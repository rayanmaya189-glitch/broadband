use axum::extract::{Json, Path, Query, State};
use validator::Validate;

use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::common::middleware::auth_middleware::{Claims, UserContext};
use crate::common::security::crypto;
use crate::common::security::jwt;
use crate::modules::role::repository::role_repository::RoleRepository;
use crate::modules::user::request::user_request::*;
use crate::modules::user::response::user_response::*;
use crate::modules::user::service::user_service::UserService;

pub async fn list_users(
    State(state): State<SharedState>,
    Query(query): Query<ListUsersQuery>,
) -> Result<Json<crate::common::utils::helpers::PaginatedResponse<UserResponse>>, AppError> {
    let svc = UserService::new(&state.db);
    Ok(Json(svc.list_users(&query).await?))
}

pub async fn create_user(
    State(state): State<SharedState>,
    Json(req): Json<CreateUserRequest>,
) -> Result<Json<UserResponse>, AppError> {
    req.validate()?;
    let svc = UserService::new(&state.db);
    Ok(Json(svc.create_user(&req).await?))
}

pub async fn get_user(
    State(state): State<SharedState>,
    Path(user_id): Path<i64>,
) -> Result<Json<UserResponse>, AppError> {
    let svc = UserService::new(&state.db);
    Ok(Json(svc.get_user(user_id).await?))
}

pub async fn update_user(
    State(state): State<SharedState>,
    Path(user_id): Path<i64>,
    Json(req): Json<UpdateUserRequest>,
) -> Result<Json<UserResponse>, AppError> {
    req.validate()?;
    let svc = UserService::new(&state.db);
    Ok(Json(svc.update_user(user_id, &req).await?))
}

pub async fn delete_user(
    State(state): State<SharedState>,
    user: UserContext,
    Path(user_id): Path<i64>,
) -> Result<Json<MessageResponse>, AppError> {
    let svc = UserService::new(&state.db);
    Ok(Json(svc.delete_user(user_id, user.user_id).await?))
}

pub async fn update_user_status(
    State(state): State<SharedState>,
    user: UserContext,
    Path(user_id): Path<i64>,
    Json(req): Json<UpdateUserStatusRequest>,
) -> Result<Json<UserResponse>, AppError> {
    req.validate()?;
    let svc = UserService::new(&state.db);
    Ok(Json(svc.update_status(user_id, &req, user.user_id).await?))
}

pub async fn get_me(
    State(state): State<SharedState>,
    user: UserContext,
) -> Result<Json<UserResponse>, AppError> {
    let svc = UserService::new(&state.db);
    Ok(Json(svc.get_user(user.user_id).await?))
}

pub async fn update_me(
    State(state): State<SharedState>,
    user: UserContext,
    Json(req): Json<UpdateProfileRequest>,
) -> Result<Json<UserResponse>, AppError> {
    req.validate()?;
    let svc = UserService::new(&state.db);
    let update = UpdateUserRequest {
        name: req.name,
        phone: req.phone,
        branch_id: None,
        avatar_url: req.avatar_url,
    };
    Ok(Json(svc.update_user(user.user_id, &update).await?))
}

pub async fn change_password(
    State(state): State<SharedState>,
    user: UserContext,
    Json(req): Json<ChangePasswordRequest>,
) -> Result<Json<MessageResponse>, AppError> {
    req.validate()?;
    let svc = UserService::new(&state.db);
    Ok(Json(svc.change_password(user.user_id, &req).await?))
}

// ── Auth endpoints ────────────────────────────────────────────

pub async fn login(
    State(state): State<SharedState>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    req.validate()?;
    let svc = UserService::new(&state.db);
    let repo = RoleRepository::new(&state.db);

    // Find user by email
    let user = svc.find_by_email(&req.email).await?
        .ok_or_else(|| AppError::Unauthorized)?;

    // Check account status
    if !user.is_active {
        return Err(AppError::Forbidden("Account is deactivated".into()));
    }
    if user.is_locked {
        return Err(AppError::Forbidden("Account is locked".into()));
    }

    // Verify password
    if !crypto::verify_password(&req.password, &user.password_hash)? {
        let _ = svc.increment_failed_attempts(user.id).await;
        return Err(AppError::Unauthorized);
    }

    // Check if2FA is enabled
    if user.two_factor_enabled {
        let temp_token = crypto::generate_token();
        let temp_hash = crypto::sha256(temp_token.as_bytes());
        // Store temp token in refresh_tokens with short expiry for 2FA verification
        let temp_expiry = chrono::Utc::now() + chrono::Duration::minutes(5);
        svc.create_refresh_token(user.id, &temp_hash, Some("2fa_temp"), None, temp_expiry).await?;
        return Ok(Json(serde_json::json!({
            "requires_2fa": true,
            "temp_token": temp_token,
            "message": "2FA verification required"
        })));
    }

    // Look up role name
    let role_name = repo.find_by_id(user.role_id).await?
        .map(|r| r.name)
        .unwrap_or_else(|| "user".to_string());

    // Generate JWT access token
    let (access_token, jti) = jwt::create_access_token(
        user.id,
        &user.email,
        &role_name,
        user.role_id,
        user.branch_id,
        user.is_company_wide,
        &[],
    )?;

    // Generate and store refresh token (store JTI in device_info for session tracking)
    let (raw_refresh, hash_refresh) = jwt::create_refresh_token_pair();
    let expires = jwt::refresh_token_expiry();
    svc.create_refresh_token(user.id, &hash_refresh, Some(&format!("jti:{}", jti)), None, expires).await?;

    // Update last login
    svc.update_last_login(user.id).await?;

    // Enforce max sessions
    let session_count = svc.count_active_tokens(user.id).await?;
    if session_count > jwt::MAX_SESSIONS {
        let _ = svc.revoke_oldest_token(user.id).await;
    }

    Ok(Json(serde_json::json!({
        "access_token": access_token,
        "refresh_token": raw_refresh,
        "token_type": "Bearer",
        "expires_in": 3600,
        "user": {
            "id": user.id,
            "email": user.email,
            "name": user.name,
            "phone": user.phone,
            "avatar_url": user.avatar_url,
            "role": role_name,
            "role_id": user.role_id,
            "branch_id": user.branch_id,
            "is_company_wide": user.is_company_wide,
        }
    })))
}

pub async fn register(
    State(state): State<SharedState>,
    Json(req): Json<CreateUserRequest>,
) -> Result<Json<UserResponse>, AppError> {
    req.validate()?;
    let svc = UserService::new(&state.db);
    Ok(Json(svc.create_user(&req).await?))
}

pub async fn logout(
    State(state): State<SharedState>,
    _user: UserContext,
    Json(req): Json<LogoutRequest>,
) -> Result<Json<MessageResponse>, AppError> {
    let svc = UserService::new(&state.db);

    // Revoke specific refresh token if provided
    if let Some(ref token) = req.refresh_token {
        let hash = crypto::sha256(token.as_bytes());
        svc.revoke_refresh_token(&hash).await?;
    }

    Ok(Json(MessageResponse { message: "Logged out successfully".into() }))
}

pub async fn logout_all(
    State(state): State<SharedState>,
    user: UserContext,
) -> Result<Json<MessageResponse>, AppError> {
    let svc = UserService::new(&state.db);
    let revoked = svc.revoke_all_user_tokens(user.user_id).await?;
    Ok(Json(MessageResponse { message: format!("All sessions logged out ({revoked} tokens revoked)") }))
}

pub async fn refresh_token(
    State(state): State<SharedState>,
    Json(req): Json<RefreshTokenRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    req.validate()?;
    let svc = UserService::new(&state.db);
    let repo = RoleRepository::new(&state.db);

    // Hash the incoming refresh token
    let hash = crypto::sha256(req.refresh_token.as_bytes());

    // Find valid refresh token
    let refresh_model = svc.find_valid_refresh_token(&hash).await?
        .ok_or_else(|| AppError::Unauthorized)?;

    // Find the user
    let user = svc.find_user_by_id(refresh_model.user_id).await?
        .ok_or_else(|| AppError::Unauthorized)?;

    if !user.is_active {
        return Err(AppError::Forbidden("Account is deactivated".into()));
    }

    // Revoke old refresh token
    svc.revoke_refresh_token(&hash).await?;

    // Look up role name
    let role_name = repo.find_by_id(user.role_id).await?
        .map(|r| r.name)
        .unwrap_or_else(|| "user".to_string());

    // Generate new token pair
    let (access_token, jti) = jwt::create_access_token(
        user.id,
        &user.email,
        &role_name,
        user.role_id,
        user.branch_id,
        user.is_company_wide,
        &[],
    )?;

    let (raw_refresh, hash_refresh) = jwt::create_refresh_token_pair();
    let expires = jwt::refresh_token_expiry();
    svc.create_refresh_token(user.id, &hash_refresh, Some(&format!("jti:{}", jti)), None, expires).await?;

    Ok(Json(serde_json::json!({
        "access_token": access_token,
        "refresh_token": raw_refresh,
        "token_type": "Bearer",
        "expires_in": 3600,
    })))
}

pub async fn me(
    State(state): State<SharedState>,
    user: UserContext,
) -> Result<Json<UserResponse>, AppError> {
    let svc = UserService::new(&state.db);
    Ok(Json(svc.get_user(user.user_id).await?))
}

pub async fn send_otp(
    State(state): State<SharedState>,
    Json(req): Json<SendOtpRequest>,
) -> Result<Json<MessageResponse>, AppError> {
    req.validate()?;
    let svc = UserService::new(&state.db);

    // Find user by phone
    let user = svc.find_by_phone(&req.phone).await?
        .ok_or_else(|| AppError::NotFound("No account found with this phone number".into()))?;

    // Generate 6-digit OTP
    use rand::Rng;
    let otp: String = {
        let mut rng = rand::thread_rng();
        (0..6).map(|_| rng.gen_range(0..10).to_string()).collect()
    };

    // Hash OTP for storage
    let otp_hash = crypto::sha256(otp.as_bytes());
    let expires_at = chrono::Utc::now() + chrono::Duration::minutes(10);

    // Store OTP in database
    svc.create_otp(user.id, &req.phone, &otp_hash, "login", expires_at).await?;

    // In production, send OTP via SMS provider
    tracing::info!(phone = %req.phone, otp = %otp, "OTP generated (not sent via SMS in dev)");

    Ok(Json(MessageResponse { message: "OTP sent successfully".into() }))
}

pub async fn verify_otp(
    State(state): State<SharedState>,
    Json(req): Json<VerifyOtpRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    req.validate()?;
    let svc = UserService::new(&state.db);
    let repo = RoleRepository::new(&state.db);

    // Find user by phone
    let user = svc.find_by_phone(&req.phone).await?
        .ok_or_else(|| AppError::NotFound("No account found with this phone number".into()))?;

    // Verify OTP
    let otp_hash = crypto::sha256(req.otp.as_bytes());
    let valid = svc.verify_otp(user.id, &otp_hash, "login").await?;
    if !valid {
        return Err(AppError::Validation("Invalid or expired OTP".into()));
    }

    // Look up role name
    let role_name = repo.find_by_id(user.role_id).await?
        .map(|r| r.name)
        .unwrap_or_else(|| "user".to_string());

    // Generate tokens
    let (access_token, jti) = jwt::create_access_token(
        user.id,
        &user.email,
        &role_name,
        user.role_id,
        user.branch_id,
        user.is_company_wide,
        &[],
    )?;

    let (raw_refresh, hash_refresh) = jwt::create_refresh_token_pair();
    let expires = jwt::refresh_token_expiry();
    svc.create_refresh_token(user.id, &hash_refresh, Some(&format!("jti:{}", jti)), None, expires).await?;
    svc.update_last_login(user.id).await?;

    Ok(Json(serde_json::json!({
        "access_token": access_token,
        "refresh_token": raw_refresh,
        "token_type": "Bearer",
        "expires_in": 3600,
        "user": {
            "id": user.id,
            "email": user.email,
            "name": user.name,
            "phone": user.phone,
            "role": role_name,
            "role_id": user.role_id,
            "branch_id": user.branch_id,
            "is_company_wide": user.is_company_wide,
        }
    })))
}

pub async fn request_password_reset(
    State(state): State<SharedState>,
    Json(req): Json<PasswordResetRequest>,
) -> Result<Json<MessageResponse>, AppError> {
    req.validate()?;
    let svc = UserService::new(&state.db);

    // Find user by email (always return success to prevent user enumeration)
    if let Some(user) = svc.find_by_email(&req.email).await? {
        let reset_token = crypto::generate_token();
        let token_hash = crypto::sha256(reset_token.as_bytes());
        let expires_at = chrono::Utc::now() + chrono::Duration::hours(1);

        svc.create_password_reset(user.id, &token_hash, expires_at).await?;

        // In production, send email via SMTP
        tracing::info!(email = %req.email, token = %reset_token, "Password reset token generated");
    }

    Ok(Json(MessageResponse { message: "If the email exists, a reset link has been sent".into() }))
}

pub async fn confirm_password_reset(
    State(state): State<SharedState>,
    Json(req): Json<PasswordResetConfirmRequest>,
) -> Result<Json<MessageResponse>, AppError> {
    let svc = UserService::new(&state.db);

    let token_hash = crypto::sha256(req.token.as_bytes());

    // Find valid reset token
    let reset_model = svc.find_valid_password_reset(&token_hash).await?
        .ok_or_else(|| AppError::Validation("Invalid or expired reset token".into()))?;

    // Hash new password
    let new_hash = crypto::hash_password(&req.new_password)?;

    // Update password
    svc.update_password(reset_model.user_id, &new_hash).await?;

    // Mark token as used
    svc.mark_password_reset_used(reset_model.id).await?;

    // Revoke all existing sessions
    svc.revoke_all_user_tokens(reset_model.user_id).await?;

    Ok(Json(MessageResponse { message: "Password reset successfully".into() }))
}

pub async fn list_sessions(
    State(state): State<SharedState>,
    user: UserContext,
    claims: Claims,
) -> Result<Json<Vec<SessionResponse>>, AppError> {
    let svc = UserService::new(&state.db);
    let sessions = svc.list_active_sessions(user.user_id).await?;

    // Mark the session whose device_info contains the current access token's JTI as current.
    // The JTI is stored when the refresh token is created during login/refresh flows.
    let current_jti = &claims.jti;
    let response: Vec<SessionResponse> = sessions.into_iter().map(|s| {
        let is_current = s.device_info.as_deref()
            .and_then(|di| di.strip_prefix("jti:"))
            .map(|stored_jti| stored_jti == current_jti)
            .unwrap_or(false);
        SessionResponse {
            id: s.id,
            device_info: s.device_info,
            ip_address: s.ip_address,
            created_at: s.created_at.into(),
            expires_at: s.expires_at.into(),
            is_current,
        }
    }).collect();

    Ok(Json(response))
}

pub async fn enable_2fa(
    State(state): State<SharedState>,
    user: UserContext,
) -> Result<Json<serde_json::Value>, AppError> {
    // Generate TOTP secret in a sync block BEFORE any .await (ThreadRng is !Send)
    let (secret_base32, temp_token, temp_hash, otpauth_url) = {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let secret_bytes: Vec<u8> = (0..20).map(|_| rng.r#gen()).collect();
        let secret_base32 = base32_encode(&secret_bytes);
        let temp_token = crypto::generate_token();
        let temp_hash = crypto::sha256(temp_token.as_bytes());
        let email = user.email.clone();
        let otpauth_url = format!(
            "otpauth://totp/{}:{}?secret={}&issuer={}",
            "AeroXe",
            email,
            secret_base32,
            "AeroXe"
        );
        (secret_base32, temp_token, temp_hash, otpauth_url)
    }; // rng is dropped here, before any .await

    let svc = UserService::new(&state.db);

    // Check if already enabled
    let user_model = svc.find_user_by_id(user.user_id).await?
        .ok_or_else(|| AppError::NotFound("User not found".into()))?;
    if user_model.two_factor_enabled {
        return Err(AppError::Validation("2FA is already enabled".into()));
    }

    // Store secret temporarily (will be confirmed in confirm_2fa)
    let temp_expiry = chrono::Utc::now() + chrono::Duration::minutes(10);
    svc.create_refresh_token(user.user_id, &temp_hash, Some("2fa_setup"), Some(&secret_base32), temp_expiry).await?;

    Ok(Json(serde_json::json!({
        "secret": secret_base32,
        "otpauth_url": otpauth_url,
        "temp_token": temp_token,
        "message": "Scan the QR code with your authenticator app, then verify with /auth/2fa/confirm"
    })))
}

/// Base32 encode helper
fn base32_encode(data: &[u8]) -> String {
    const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";
    let mut result = String::new();
    let mut bits: u32 = 0;
    let mut value: u32 = 0;
    for &byte in data {
        value = (value << 8) | (byte as u32);
        bits += 8;
        while bits >= 5 {
            bits -= 5;
            result.push(ALPHABET[((value >> bits) & 0x1F) as usize] as char);
        }
    }
    if bits > 0 {
        result.push(ALPHABET[((value << (5 - bits)) & 0x1F) as usize] as char);
    }
    result
}

pub async fn confirm_2fa(
    State(state): State<SharedState>,
    user: UserContext,
    Json(req): Json<Confirm2FaRequest>,
) -> Result<Json<MessageResponse>, AppError> {
    req.validate()?;
    let svc = UserService::new(&state.db);

    // Find the 2FA setup token
    let sessions = svc.list_active_sessions(user.user_id).await?;
    let setup_session = sessions.iter()
        .find(|s| s.device_info.as_deref() == Some("2fa_setup"))
        .ok_or_else(|| AppError::Validation("No pending 2FA setup. Call enable_2fa first".into()))?;

    // The secret is stored in ip_address field of the temp token
    let secret = setup_session.ip_address.as_deref()
        .ok_or_else(|| AppError::Internal(anyhow::anyhow!("2FA secret not found")))?;

    // Verify TOTP code
    let valid = crate::common::security::totp::verify_totp(secret, &req.code)
        .map_err(|e| AppError::Internal(anyhow::anyhow!("TOTP verification error: {e}")))?;

    if !valid {
        return Err(AppError::Validation("Invalid TOTP code".into()));
    }

    // Enable 2FA on the user
    svc.enable_two_factor(user.user_id).await?;

    // Revoke the temp setup token (token_hash is already hashed, pass directly)
    svc.revoke_refresh_token(&setup_session.token_hash).await?;

    Ok(Json(MessageResponse { message: "2FA enabled successfully".into() }))
}

pub async fn disable_2fa(
    State(state): State<SharedState>,
    user: UserContext,
    Json(req): Json<serde_json::Value>,
) -> Result<Json<MessageResponse>, AppError> {
    let svc = UserService::new(&state.db);

    let user_model = svc.find_user_by_id(user.user_id).await?
        .ok_or_else(|| AppError::NotFound("User not found".into()))?;

    if !user_model.two_factor_enabled {
        return Err(AppError::Validation("2FA is not enabled".into()));
    }

    // Verify current password for security
    if let Some(password) = req.get("password").and_then(|v| v.as_str()) {
        if !crypto::verify_password(password, &user_model.password_hash)? {
            return Err(AppError::Validation("Invalid password".into()));
        }
    } else {
        return Err(AppError::Validation("Password is required to disable2FA".into()));
    }

    svc.disable_two_factor(user.user_id).await?;

    Ok(Json(MessageResponse { message: "2FA disabled successfully".into() }))
}

pub async fn verify_2fa_login(
    State(state): State<SharedState>,
    Json(req): Json<Verify2FaRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    req.validate()?;
    let svc = UserService::new(&state.db);
    let repo = RoleRepository::new(&state.db);

    // Find the temp 2FA token
    let temp_hash = crypto::sha256(req.temp_token.as_bytes());
    let temp_session = svc.find_valid_refresh_token(&temp_hash).await?
        .filter(|s| s.device_info.as_deref() == Some("2fa_temp"))
        .ok_or_else(|| AppError::Validation("Invalid or expired 2FA token".into()))?;

    // Find user
    let user = svc.find_user_by_id(temp_session.user_id).await?
        .ok_or_else(|| AppError::NotFound("User not found".into()))?;

    // Revoke temp token
    svc.revoke_refresh_token(&temp_hash).await?;

    // Verify TOTP code if 2FA is enabled
    if user.two_factor_enabled {
        // Get the user's TOTP secret from the 2fa_setup refresh token
        let all_sessions = svc.list_active_sessions(user.id).await?;
        let setup_session = all_sessions.iter()
            .find(|s| s.device_info.as_deref() == Some("2fa_setup"));
        
        // The TOTP secret is stored in the ip_address field of the 2fa_setup token
        let secret = setup_session
            .and_then(|s| s.ip_address.as_deref())
            .ok_or_else(|| AppError::Internal(anyhow::anyhow!("2FA secret not found for user")))?;

        let valid = crate::common::security::totp::verify_totp(secret, &req.code)
            .map_err(|e| AppError::Internal(anyhow::anyhow!("TOTP verification error: {e}")))?;

        if !valid {
            return Err(AppError::Validation("Invalid 2FA code".into()));
        }
    }

    // Look up role name
    let role_name = repo.find_by_id(user.role_id).await?
        .map(|r| r.name)
        .unwrap_or_else(|| "user".to_string());

    // Generate tokens
    let (access_token, jti) = jwt::create_access_token(
        user.id,
        &user.email,
        &role_name,
        user.role_id,
        user.branch_id,
        user.is_company_wide,
        &[],
    )?;

    let (raw_refresh, hash_refresh) = jwt::create_refresh_token_pair();
    let expires = jwt::refresh_token_expiry();
    svc.create_refresh_token(user.id, &hash_refresh, Some(&format!("jti:{}", jti)), None, expires).await?;
    svc.update_last_login(user.id).await?;

    Ok(Json(serde_json::json!({
        "access_token": access_token,
        "refresh_token": raw_refresh,
        "token_type": "Bearer",
        "expires_in": 3600,
        "user": {
            "id": user.id,
            "email": user.email,
            "name": user.name,
            "role": role_name,
        }
    })))
}
