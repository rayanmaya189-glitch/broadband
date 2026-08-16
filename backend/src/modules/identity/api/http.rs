use axum::extract::connect_info::ConnectInfo;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::Json;
use sea_orm::{ActiveModelTrait, EntityTrait, Set};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use validator::Validate;
use validator::ValidationError;

use crate::modules::identity::application::otp as otp_service;
use crate::modules::identity::application::services::IdentityService;
use crate::modules::identity::application::two_factor;
use crate::modules::identity::domain::entities::user;
use crate::modules::identity::domain::rules::auth_rules::AuthRules;
use crate::shared::app_state::AppState;
use crate::shared::errors::AppError;
use crate::shared::middleware::auth::{require_permission, UserContext};
use crate::shared::primitives::{ClientIp, PaginationParams};
use crate::shared::utils::login_anomaly;

/// Run validator-crate validation on a request DTO and map failures to a 400.
fn validate_input<T: Validate>(req: &T) -> Result<(), AppError> {
    req.validate()
        .map_err(|e| AppError::BadRequest(format!("Invalid request: {}", e)))
}

fn validate_phone(phone: &str) -> Result<(), ValidationError> {
    let normalized = otp_service::normalize_phone(phone);
    if AuthRules::is_valid_indian_phone(&normalized) {
        Ok(())
    } else {
        Err(ValidationError::new("invalid_indian_phone"))
    }
}

fn validate_password_strength(password: &str) -> Result<(), ValidationError> {
    if AuthRules::is_strong_password(password) {
        Ok(())
    } else {
        Err(ValidationError::new("weak_password"))
    }
}

fn validate_otp_code(code: &str) -> Result<(), ValidationError> {
    let ok = code.len() >= 4 && code.len() <= 16 && code.chars().all(|c| c.is_ascii_digit());
    if ok {
        Ok(())
    } else {
        Err(ValidationError::new("invalid_otp_code"))
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(email)]
    pub email: String,
    #[validate(custom(function = "validate_phone"))]
    pub phone: String,
    #[validate(length(min = 2, max = 120))]
    pub name: String,
    #[validate(
        length(min = 8, max = 128),
        custom(function = "validate_password_strength")
    )]
    pub password: String,
    #[serde(default)]
    pub branch_id: Option<i64>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(length(min = 3, max = 320))]
    pub email: String,
    #[validate(length(min = 1, max = 256))]
    pub password: String,
}

/// Two-step 2FA login: verify TOTP code after password
#[derive(Debug, Deserialize, Validate)]
pub struct Login2FARequest {
    #[validate(length(min = 8, max = 512))]
    pub pending_token: String,
    #[validate(custom(function = "validate_otp_code"))]
    pub code: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct RefreshTokenRequest {
    #[validate(length(min = 8, max = 2048))]
    pub refresh_token: String,
}

#[derive(Debug, Serialize)]
pub struct RefreshTokenResponse {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub requires_2fa: bool,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub user: Option<UserResponse>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: i64,
    pub email: String,
    pub phone: String,
    pub name: String,
    pub avatar_url: Option<String>,
    #[serde(default)]
    pub branch_id: Option<i64>,
    pub status: String,
    pub last_login_at: Option<String>,
}

fn to_user_response(u: user::Model) -> UserResponse {
    UserResponse {
        id: u.id,
        email: u.email,
        phone: u.phone,
        name: u.name,
        avatar_url: u.avatar_url,
        branch_id: u.branch_id,
        status: u.status,
        last_login_at: u.last_login_at.map(|dt| dt.to_rfc3339()),
    }
}

/// POST /api/v1/auth/register
pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(req): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<AuthResponse>), AppError> {
    validate_input(&req)?;
    let email = req.email.clone();
    let password = req.password.clone();
    let user = IdentityService::register(
        &state.db,
        req.email,
        req.phone,
        req.name,
        req.password,
        req.branch_id,
    )
    .await?;

    let mut redis = state.redis.clone();
    let (access_token, refresh_token, _) = IdentityService::login(
        &state.db,
        &mut redis,
        &state.settings,
        &email,
        &password,
        &state.jwt_keys,
    )
    .await?;

    Ok((
        StatusCode::CREATED,
        Json(AuthResponse {
            requires_2fa: false,
            access_token: Some(access_token),
            refresh_token: Some(refresh_token),
            user: Some(to_user_response(user)),
            pending_token: None,
            message: None,
        }),
    ))
}

/// POST /api/v1/auth/login — Step 1: verify password, return pending_token if 2FA required
pub async fn login(
    State(state): State<Arc<AppState>>,
    headers: axum::http::HeaderMap,
    ConnectInfo(peer): ConnectInfo<std::net::SocketAddr>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    validate_input(&req)?;
    let user_model =
        IdentityService::verify_password_only(&state.db, &req.email, &req.password).await?;

    // Login anomaly detection — check for new IP (non-blocking, best-effort)
    // The real peer socket address is authoritative; forwarding headers are
    // only honored when TRUST_PROXY=true (see ClientIp::from_peer).
    let client_ip = ClientIp::from_peer(Some(peer), &headers);
    let ip_str = client_ip.as_str().to_string();
    let mut redis = state.redis.clone();
    let anomaly = login_anomaly::check_login_anomaly(&mut redis, user_model.id, &ip_str)
        .await
        .ok(); // Non-blocking: don't fail login on anomaly check failure

    if let Some(ref check) = anomaly {
        if check.is_anomaly {
            tracing::warn!(
                user_id = user_model.id,
                ip = %ip_str,
                "Login anomaly detected — new IP address"
            );
            // In production: spawn notification task to alert user via email/SMS
        }
    }

    if user_model.two_factor_enabled {
        let pending_token =
            IdentityService::generate_pending_2fa_token(&user_model, &state.jwt_keys)?;
        return Ok(Json(AuthResponse {
            requires_2fa: true,
            access_token: None,
            refresh_token: None,
            user: None,
            pending_token: Some(pending_token),
            message: Some("2FA verification required. Call POST /auth/login/2fa with the pending_token and TOTP code.".to_string()),
        }));
    }

    let (access_token, refresh_token, user) = IdentityService::login(
        &state.db,
        &mut redis,
        &state.settings,
        &req.email,
        &req.password,
        &state.jwt_keys,
    )
    .await?;

    Ok(Json(AuthResponse {
        requires_2fa: false,
        access_token: Some(access_token),
        refresh_token: Some(refresh_token),
        user: Some(to_user_response(user)),
        pending_token: None,
        message: None,
    }))
}

/// POST /api/v1/auth/login/2fa — Step 2: verify TOTP code, complete login
pub async fn login_2fa(
    State(state): State<Arc<AppState>>,
    Json(req): Json<Login2FARequest>,
) -> Result<Json<AuthResponse>, AppError> {
    validate_input(&req)?;
    let user_id = IdentityService::verify_pending_2fa_token(&req.pending_token, &state.jwt_keys)?;

    let user_model = user::Entity::find_by_id(user_id)
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".into()))?;

    if user_model.status != "active" {
        return Err(AppError::Unauthorized);
    }

    let secret = user_model
        .two_factor_secret
        .as_deref()
        .ok_or_else(|| AppError::BadRequest("2FA not configured".into()))?;

    if !two_factor::verify_totp(secret, &req.code) {
        return Err(AppError::BadRequest("Invalid TOTP code".into()));
    }

    let mut redis = state.redis.clone();
    let (access_token, refresh_token, user) = IdentityService::complete_2fa_login(
        &state.db,
        &mut redis,
        &state.settings,
        &user_model,
        &state.jwt_keys,
    )
    .await?;

    Ok(Json(AuthResponse {
        requires_2fa: false,
        access_token: Some(access_token),
        refresh_token: Some(refresh_token),
        user: Some(to_user_response(user)),
        pending_token: None,
        message: None,
    }))
}

/// POST /api/v1/auth/refresh
pub async fn refresh_token(
    State(state): State<Arc<AppState>>,
    Json(req): Json<RefreshTokenRequest>,
) -> Result<Json<RefreshTokenResponse>, AppError> {
    validate_input(&req)?;
    let mut redis = state.redis.clone();
    let (access_token, refresh_token, _) = IdentityService::refresh_token(
        &state.db,
        &mut redis,
        &state.settings,
        &req.refresh_token,
        &state.jwt_keys,
    )
    .await?;
    Ok(Json(RefreshTokenResponse {
        access_token,
        refresh_token,
    }))
}

/// GET /api/v1/users/me
pub async fn get_current_user(
    State(state): State<Arc<AppState>>,
    user: UserContext,
) -> Result<Json<UserResponse>, AppError> {
    let user_model = IdentityService::get_user(&state.db, user.user_id).await?;
    Ok(Json(to_user_response(user_model)))
}

/// GET /api/v1/users
pub async fn list_users(
    State(state): State<Arc<AppState>>,
    Query(p): Query<PaginationParams>,
    user: UserContext,
) -> Result<Json<serde_json::Value>, AppError> {
    require_permission(&user, "user.account.view").map_err(|e| AppError::Forbidden(e.1))?;
    let (users, total) = IdentityService::list_users(&state.db, p.page(), p.limit()).await?;
    let items: Vec<UserResponse> = users.into_iter().map(to_user_response).collect();
    Ok(Json(
        serde_json::json!({ "items": items, "total": total, "page": p.page(), "limit": p.limit() }),
    ))
}

// ──────────────────────────────────────────────
// 2FA / TOTP Endpoints (§28 Security)
// ──────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct TwoFactorSetupResponse {
    pub secret_base32: String,
    pub otpauth_uri: String,
    pub backup_codes: Vec<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct TwoFactorVerifyRequest {
    #[validate(custom(function = "validate_otp_code"))]
    pub code: String,
}

/// POST /api/v1/auth/2fa/setup — Generate TOTP secret + backup codes.
pub async fn setup_2fa(
    State(state): State<Arc<AppState>>,
    user: UserContext,
) -> Result<Json<TwoFactorSetupResponse>, AppError> {
    let user_model = IdentityService::get_user(&state.db, user.user_id).await?;
    if user_model.two_factor_enabled {
        return Err(AppError::Conflict("2FA is already enabled".to_string()));
    }

    let setup = two_factor::setup_two_factor(&user_model.email)?;

    let mut active: user::ActiveModel = user_model.into();
    active.two_factor_secret = Set(Some(setup.secret_base32.clone()));
    active.two_factor_backup_codes = Set(Some(
        serde_json::to_string(&setup.backup_code_hashes).unwrap_or_default(),
    ));
    active.updated_at = Set(chrono::Utc::now());
    active.update(&state.db).await?;

    Ok(Json(TwoFactorSetupResponse {
        secret_base32: setup.secret_base32,
        otpauth_uri: setup.otpauth_uri,
        backup_codes: setup.backup_codes,
    }))
}

/// POST /api/v1/auth/2fa/confirm — Confirm TOTP setup by verifying a code.
pub async fn confirm_2fa(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Json(req): Json<TwoFactorVerifyRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    validate_input(&req)?;
    let user_model = IdentityService::get_user(&state.db, user.user_id).await?;
    let secret = user_model.two_factor_secret.as_deref().ok_or_else(|| {
        AppError::BadRequest("2FA not initialized. Call /2fa/setup first.".to_string())
    })?;

    if user_model.two_factor_enabled {
        return Err(AppError::Conflict("2FA is already enabled".to_string()));
    }

    if !two_factor::verify_totp(secret, &req.code) {
        return Err(AppError::BadRequest("Invalid TOTP code".to_string()));
    }

    let mut active: user::ActiveModel = user_model.into();
    active.two_factor_enabled = Set(true);
    active.updated_at = Set(chrono::Utc::now());
    active.update(&state.db).await?;

    Ok(Json(serde_json::json!({ "status": "2fa_enabled" })))
}

/// POST /api/v1/auth/2fa/verify — Verify a TOTP code.
pub async fn verify_2fa(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Json(req): Json<TwoFactorVerifyRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    validate_input(&req)?;
    let user_model = user::Entity::find_by_id(user.user_id)
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

    let secret = user_model
        .two_factor_secret
        .as_deref()
        .ok_or_else(|| AppError::BadRequest("2FA not enabled for this user".to_string()))?;

    if two_factor::verify_totp(secret, &req.code) {
        Ok(Json(serde_json::json!({ "verified": true })))
    } else {
        Err(AppError::Unauthorized)
    }
}

/// POST /api/v1/auth/2fa/backup-verify — Verify a backup code.
pub async fn verify_backup_code(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Json(req): Json<TwoFactorVerifyRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    validate_input(&req)?;
    let user_model = user::Entity::find_by_id(user.user_id)
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

    let backup_hashes_json = user_model
        .two_factor_backup_codes
        .as_deref()
        .ok_or_else(|| AppError::BadRequest("No backup codes found".to_string()))?;
    let stored_hashes: Vec<String> = serde_json::from_str(backup_hashes_json)
        .map_err(|_| AppError::Internal(anyhow::anyhow!("Invalid backup codes format")))?;

    let (valid, remaining) = two_factor::verify_backup_code(&req.code, &stored_hashes)?;

    if valid {
        let mut active: user::ActiveModel = user_model.into();
        active.two_factor_backup_codes =
            Set(Some(serde_json::to_string(&remaining).unwrap_or_default()));
        active.updated_at = Set(chrono::Utc::now());
        active.update(&state.db).await?;
        Ok(Json(
            serde_json::json!({ "verified": true, "remaining_codes": remaining.len() }),
        ))
    } else {
        Err(AppError::Unauthorized)
    }
}

/// DELETE /api/v1/auth/2fa/disable — Disable 2FA (requires valid TOTP code).
pub async fn disable_2fa(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Json(req): Json<TwoFactorVerifyRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    let user_model = IdentityService::get_user(&state.db, user.user_id).await?;
    if !user_model.two_factor_enabled {
        return Err(AppError::BadRequest("2FA is not enabled".to_string()));
    }

    let secret = user_model
        .two_factor_secret
        .as_deref()
        .ok_or_else(|| AppError::BadRequest("2FA secret not found".to_string()))?;

    if !two_factor::verify_totp(secret, &req.code) {
        return Err(AppError::BadRequest(
            "Invalid TOTP code. Cannot disable 2FA without valid code.".to_string(),
        ));
    }

    let mut active: user::ActiveModel = user_model.into();
    active.two_factor_enabled = Set(false);
    active.two_factor_secret = Set(None);
    active.two_factor_backup_codes = Set(None);
    active.updated_at = Set(chrono::Utc::now());
    active.update(&state.db).await?;

    Ok(Json(serde_json::json!({ "status": "2fa_disabled" })))
}

// ──────────────────────────────────────────────
// Auth — Logout, Password, Sessions
// ──────────────────────────────────────────────

/// POST /api/v1/auth/logout — Invalidate all refresh tokens for this user.
pub async fn logout_all(
    State(state): State<Arc<AppState>>,
    user: UserContext,
) -> Result<Json<serde_json::Value>, AppError> {
    let mut redis = state.redis.clone();
    IdentityService::logout_all(&mut redis, user.user_id).await?;
    Ok(Json(serde_json::json!({ "status": "logged_out" })))
}

#[derive(Debug, Deserialize, Validate)]
pub struct ChangePasswordRequest {
    #[validate(length(min = 1, max = 256))]
    pub current_password: String,
    #[validate(
        length(min = 8, max = 128),
        custom(function = "validate_password_strength")
    )]
    pub new_password: String,
}

/// POST /api/v1/auth/change-password
pub async fn change_password(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Json(req): Json<ChangePasswordRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    validate_input(&req)?;
    IdentityService::change_password(
        &state.db,
        user.user_id,
        &req.current_password,
        &req.new_password,
    )
    .await?;
    Ok(Json(serde_json::json!({ "status": "password_changed" })))
}

#[derive(Debug, Deserialize, Validate)]
pub struct PasswordResetRequest {
    #[validate(length(min = 3, max = 320))]
    pub email: String,
}

/// POST /api/v1/auth/password-reset — Send reset token (best-effort).
pub async fn request_password_reset(
    State(state): State<Arc<AppState>>,
    Json(req): Json<PasswordResetRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    validate_input(&req)?;
    let mut redis = state.redis.clone();
    IdentityService::request_password_reset(&state.db, &mut redis, &req.email).await?;
    Ok(Json(serde_json::json!({
        "message": "If the email exists, a reset link has been sent."
    })))
}

#[derive(Debug, Deserialize, Validate)]
pub struct PasswordResetConfirmRequest {
    #[validate(length(min = 16, max = 512))]
    pub token: String,
    #[validate(
        length(min = 8, max = 128),
        custom(function = "validate_password_strength")
    )]
    pub new_password: String,
}

/// POST /api/v1/auth/password-reset/confirm
pub async fn confirm_password_reset(
    State(state): State<Arc<AppState>>,
    Json(req): Json<PasswordResetConfirmRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    validate_input(&req)?;
    let mut redis = state.redis.clone();
    IdentityService::confirm_password_reset(&state.db, &mut redis, &req.token, &req.new_password)
        .await?;
    Ok(Json(serde_json::json!({ "status": "password_reset" })))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionResponse {
    pub session_id: String,
    pub user_agent: Option<String>,
    pub ip_address: Option<String>,
    pub created_at: String,
    pub last_active_at: String,
}

/// GET /api/v1/auth/sessions
pub async fn list_sessions(
    State(state): State<Arc<AppState>>,
    user: UserContext,
) -> Result<Json<Vec<SessionResponse>>, AppError> {
    let mut redis = state.redis.clone();
    let sessions = IdentityService::list_sessions(&mut redis, user.user_id).await?;
    Ok(Json(sessions))
}

/// DELETE /api/v1/auth/sessions/:session_id
pub async fn revoke_session(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Path(session_id): Path<String>,
) -> Result<Json<serde_json::Value>, AppError> {
    let mut redis = state.redis.clone();
    IdentityService::revoke_session(&mut redis, user.user_id, &session_id).await?;
    Ok(Json(serde_json::json!({ "status": "session_revoked" })))
}

// ──────────────────────────────────────────────
// OTP Login (§28 Security)
// ──────────────────────────────────────────────

#[derive(Debug, Deserialize, Validate)]
pub struct OtpRequestRequest {
    #[validate(custom(function = "validate_phone"))]
    pub phone: String,
    #[serde(default)]
    pub channel: Option<String>,
    #[serde(default)]
    pub fcm_token: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct OtpRequestResponse {
    pub success: bool,
    pub channel: String,
    pub expires_in_secs: u64,
    pub message: String,
}

/// POST /api/v1/auth/otp/request — Generate and deliver an OTP via
/// sms (MSG91), whatsapp, telegram or firebase (FCM push).
pub async fn request_otp(
    State(state): State<Arc<AppState>>,
    Json(req): Json<OtpRequestRequest>,
) -> Result<Json<OtpRequestResponse>, AppError> {
    validate_input(&req)?;
    let channel = otp_service::OtpChannel::from_str(req.channel.as_deref().unwrap_or("sms"))?;

    let mut redis = state.redis.clone();
    let sent = otp_service::send_otp(
        &state.db,
        &mut redis,
        &state.settings.app_name,
        &req.phone,
        &channel,
        req.fcm_token.as_deref(),
    )
    .await?;

    let channel_name = sent.channel.clone();

    Ok(Json(OtpRequestResponse {
        success: true,
        channel: sent.channel,
        expires_in_secs: sent.expires_in_secs,
        message: format!(
            "OTP sent via {}. It expires in {} seconds.",
            channel_name, sent.expires_in_secs
        ),
    }))
}

#[derive(Debug, Deserialize, Validate)]
pub struct OtpVerifyRequest {
    #[validate(custom(function = "validate_phone"))]
    pub phone: String,
    #[serde(default)]
    pub channel: Option<String>,
    #[validate(custom(function = "validate_otp_code"))]
    pub code: String,
}

/// POST /api/v1/auth/otp/verify — Verify the OTP and complete login.
/// Issues access/refresh tokens for the account linked to the phone.
pub async fn verify_otp(
    State(state): State<Arc<AppState>>,
    Json(req): Json<OtpVerifyRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    validate_input(&req)?;
    let channel = otp_service::OtpChannel::from_str(req.channel.as_deref().unwrap_or("sms"))?;

    let mut redis = state.redis.clone();
    otp_service::verify_otp(&state.db, &mut redis, &req.phone, &channel, &req.code).await?;

    let user_model = otp_service::find_user_by_phone(&state.db, &req.phone)
        .await?
        .ok_or_else(|| {
            AppError::NotFound("No account is linked to this phone number".to_string())
        })?;

    if user_model.status != "active" {
        return Err(AppError::Unauthorized);
    }

    let (access_token, refresh_token, user) = IdentityService::complete_2fa_login(
        &state.db,
        &mut redis,
        &state.settings,
        &user_model,
        &state.jwt_keys,
    )
    .await?;

    Ok(Json(AuthResponse {
        requires_2fa: false,
        access_token: Some(access_token),
        refresh_token: Some(refresh_token),
        user: Some(to_user_response(user)),
        pending_token: None,
        message: None,
    }))
}

/// POST /api/v1/auth/otp/telegram-webhook — Receive Telegram bot updates.
/// A user binds their phone by messaging the bot:
///   `/login <phone>`   — sends a verification code to the phone via SMS
///   `/confirm <code>`  — confirms the binding from the same chat
pub async fn telegram_webhook(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    let raw = serde_json::to_string(&payload)
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to serialize webhook: {}", e)))?;

    let Some(update) = crate::modules::integrations::telegram::parse_update(&raw) else {
        return Ok(Json(serde_json::json!({ "ok": true })));
    };
    let Some(msg) = update.message else {
        return Ok(Json(serde_json::json!({ "ok": true })));
    };

    let chat_id = msg.chat.id;
    let text = msg.text.unwrap_or_default();
    let parts: Vec<&str> = text.split_whitespace().collect();

    let mut redis = state.redis.clone();
    // Two-step binding proves phone ownership before a chat is linked:
    //   1. /login <phone>  -> a verification code is sent to the phone via SMS
    //   2. /confirm <code> -> the same chat that started the flow confirms it
    let mut reply =
        String::from("Usage: /login <phone> to link a phone for OTP login, then /confirm <code>.");
    match parts.as_slice() {
        ["/start" | "/login", phone] => {
            let normalized = otp_service::normalize_phone(phone);
            if !AuthRules::is_valid_indian_phone(&normalized) {
                reply = "Please provide a valid 10-digit Indian phone number.".to_string();
            } else {
                match otp_service::initiate_telegram_bind(
                    &mut redis,
                    &normalized,
                    chat_id,
                    &state.settings.app_name,
                )
                .await
                {
                    Ok(()) => {
                        reply = format!(
                            "A verification code has been sent to {}. Reply with /confirm <code> within 5 minutes.",
                            normalized
                        );
                    }
                    Err(e) => reply = format!("Could not start linking phone: {}", e),
                }
            }
        }
        ["/confirm", code] => {
            match otp_service::confirm_telegram_bind(&mut redis, chat_id, code).await {
                Ok(phone) => {
                    reply = format!(
                        "Phone {} is now linked. You can use it for OTP login.",
                        phone
                    );
                }
                Err(e) => reply = format!("Could not confirm phone: {}", e),
            }
        }
        _ => {}
    }

    let adapter = crate::modules::integrations::telegram::TelegramBotAdapter::from_env();
    if adapter.is_configured() {
        let _ = adapter.send_message(chat_id, &reply).await;
    }

    Ok(Json(serde_json::json!({ "ok": true })))
}
