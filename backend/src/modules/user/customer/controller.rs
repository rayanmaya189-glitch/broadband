use axum::extract::{Json, State};
use validator::Validate;

use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::common::middleware::auth_middleware::{Claims, UserContext};
use crate::common::security::crypto;
use crate::modules::user::request::user_request::*;
use crate::modules::user::response::user_response::*;
use crate::modules::user::service::user_service::UserService;

/// Get current user's profile.
pub async fn get_me(
    State(state): State<SharedState>,
    user: UserContext,
) -> Result<Json<UserResponse>, AppError> {
    let svc = UserService::new(&state.db);
    Ok(Json(svc.get_user(user.user_id).await?))
}

/// Update current user's profile (name, phone, avatar).
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

/// Change password.
pub async fn change_password(
    State(state): State<SharedState>,
    user: UserContext,
    Json(req): Json<ChangePasswordRequest>,
) -> Result<Json<MessageResponse>, AppError> {
    req.validate()?;
    let svc = UserService::new(&state.db);
    Ok(Json(svc.change_password(user.user_id, &req).await?))
}

/// List active sessions for the current user.
pub async fn list_sessions(
    State(state): State<SharedState>,
    user: UserContext,
    claims: Claims,
) -> Result<Json<Vec<SessionResponse>>, AppError> {
    let svc = UserService::new(&state.db);
    let sessions = svc.list_active_sessions(user.user_id).await?;
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

/// Logout current session.
pub async fn logout(
    State(state): State<SharedState>,
    _user: UserContext,
    Json(req): Json<LogoutRequest>,
) -> Result<Json<MessageResponse>, AppError> {
    let svc = UserService::new(&state.db);
    if let Some(ref token) = req.refresh_token {
        let hash = crypto::sha256(token.as_bytes());
        svc.revoke_refresh_token(&hash).await?;
    }
    Ok(Json(MessageResponse { message: "Logged out successfully".into() }))
}

/// Logout all sessions.
pub async fn logout_all(
    State(state): State<SharedState>,
    user: UserContext,
) -> Result<Json<MessageResponse>, AppError> {
    let svc = UserService::new(&state.db);
    let revoked = svc.revoke_all_user_tokens(user.user_id).await?;
    Ok(Json(MessageResponse { message: format!("All sessions logged out ({revoked} tokens revoked)") }))
}

// ── 2FA Management ──────────────────────────────────────

pub async fn enable_2fa(
    State(state): State<SharedState>,
    user: UserContext,
) -> Result<Json<serde_json::Value>, AppError> {
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
            "AeroXe", email, secret_base32, "AeroXe"
        );
        (secret_base32, temp_token, temp_hash, otpauth_url)
    };

    let svc = UserService::new(&state.db);
    let user_model = svc.find_user_by_id(user.user_id).await?
        .ok_or_else(|| AppError::NotFound("User not found".into()))?;
    if user_model.two_factor_enabled {
        return Err(AppError::Validation("2FA is already enabled".into()));
    }

    let temp_expiry = chrono::Utc::now() + chrono::Duration::minutes(10);
    svc.create_refresh_token(user.user_id, &temp_hash, Some("2fa_setup"), Some(&secret_base32), temp_expiry).await?;

    Ok(Json(serde_json::json!({
        "secret": secret_base32,
        "otpauth_url": otpauth_url,
        "temp_token": temp_token,
        "message": "Scan the QR code with your authenticator app, then verify via /auth/2fa/confirm"
    })))
}

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
    let sessions = svc.list_active_sessions(user.user_id).await?;
    let setup_session = sessions.iter()
        .find(|s| s.device_info.as_deref() == Some("2fa_setup"))
        .ok_or_else(|| AppError::Validation("No pending 2FA setup. Call enable_2fa first".into()))?;
    let secret = setup_session.ip_address.as_deref()
        .ok_or_else(|| AppError::Internal(anyhow::anyhow!("2FA secret not found")))?;
    let valid = crate::common::security::totp::verify_totp(secret, &req.code)
        .map_err(|e| AppError::Internal(anyhow::anyhow!("TOTP verification error: {e}")))?;
    if !valid {
        return Err(AppError::Validation("Invalid TOTP code".into()));
    }
    svc.enable_two_factor(user.user_id).await?;
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
    if let Some(password) = req.get("password").and_then(|v| v.as_str()) {
        if !crypto::verify_password(password, &user_model.password_hash)? {
            return Err(AppError::Validation("Invalid password".into()));
        }
    } else {
        return Err(AppError::Validation("Password is required to disable 2FA".into()));
    }
    svc.disable_two_factor(user.user_id).await?;
    Ok(Json(MessageResponse { message: "2FA disabled successfully".into() }))
}
