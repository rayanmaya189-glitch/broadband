use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use sea_orm::{ActiveModelTrait, EntityTrait, Set};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::modules::identity::application::services::IdentityService;
use crate::modules::identity::application::two_factor;
use crate::modules::identity::domain::entities::user;
use crate::shared::app_state::AppState;
use crate::shared::errors::AppError;
use crate::shared::middleware::auth::UserContext;
use crate::shared::utils::login_anomaly;

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub phone: String,
    pub name: String,
    pub password: String,
    #[serde(default)]
    pub branch_id: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

/// Two-step 2FA login: verify TOTP code after password
#[derive(Debug, Deserialize)]
pub struct Login2FARequest {
    pub pending_token: String,
    pub code: String,
}

#[derive(Debug, Deserialize)]
pub struct RefreshTokenRequest {
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
    Json(req): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    let user_model =
        IdentityService::verify_password_only(&state.db, &req.email, &req.password).await?;

    // Login anomaly detection — check for new IP (non-blocking, best-effort)
    // In production, extract real IP from X-Forwarded-For or connecting socket
    let client_ip = "0.0.0.0"; // TODO: extract from request extensions via middleware
    let mut redis = state.redis.clone();
    let anomaly = login_anomaly::check_login_anomaly(&mut redis, user_model.id, client_ip)
        .await
        .ok(); // Non-blocking: don't fail login on anomaly check failure

    if let Some(ref check) = anomaly {
        if check.is_anomaly {
            tracing::warn!(
                user_id = user_model.id,
                ip = client_ip,
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
    user: UserContext,
) -> Result<Json<Vec<UserResponse>>, AppError> {
    if !user.is_company_wide {
        return Err(AppError::Forbidden("Insufficient permissions".to_string()));
    }
    let users = IdentityService::list_users(&state.db).await?;
    Ok(Json(users.into_iter().map(to_user_response).collect()))
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

#[derive(Debug, Deserialize)]
pub struct TwoFactorVerifyRequest {
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
