use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};

use crate::common::config::config::Config;
use crate::common::errors::app_error::AppError;
use crate::common::middleware::auth_middleware::Claims;
use crate::common::security::crypto::{generate_token, sha256};

/// Generate a JWT access token for a user.
pub fn create_access_token(
    user_id: i64,
    email: &str,
    role: &str,
    role_id: i64,
    branch_id: Option<i64>,
    is_company_wide: bool,
    permissions: &[String],
) -> Result<(String, String), AppError> {
    let config = Config::get();
    let now = Utc::now();
    let exp = now + Duration::hours(config.jwt_access_expiry_hours);
    let jti = uuid::Uuid::new_v4().to_string();

    let claims = Claims {
        sub: user_id,
        email: email.to_string(),
        role: role.to_string(),
        role_id,
        branch_id,
        is_company_wide,
        permissions: permissions.to_vec(),
        jti: jti.clone(),
        iat: now.timestamp() as usize,
        exp: exp.timestamp() as usize,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.jwt_secret.as_bytes()),
    )
    .map_err(|e| AppError::Internal(anyhow::anyhow!("JWT encoding failed: {e}")))?;

    Ok((token, jti))
}

/// Generate an opaque refresh token and return (raw_token, sha256_hash).
pub fn create_refresh_token_pair() -> (String, String) {
    let raw = generate_token();
    let hash = sha256(raw.as_bytes());
    (raw, hash)
}

/// Calculate refresh token expiry timestamp.
pub fn refresh_token_expiry() -> chrono::DateTime<Utc> {
    let config = Config::get();
    Utc::now() + Duration::days(config.jwt_refresh_expiry_days)
}

/// Maximum active sessions per user.
pub const MAX_SESSIONS: i64 = 5;
