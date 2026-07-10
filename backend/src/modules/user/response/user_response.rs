use utoipa::ToSchema;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};


/// User list item response.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserResponse {
    pub id: i64,
    pub email: String,
    pub name: String,
    pub phone: Option<String>,
    pub avatar_url: Option<String>,
    pub role_id: i64,
    pub role_name: Option<String>,
    pub branch_id: Option<i64>,
    pub is_company_wide: bool,
    pub is_active: bool,
    pub is_locked: bool,
    pub two_factor_enabled: bool,
    pub last_login_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// User detail response (same as UserResponse, extensible).
pub type UserDetailResponse = UserResponse;

impl UserResponse {
    pub fn from_model(m: crate::modules::user::model::user_entity::Model, role_name: Option<String>) -> Self {
        Self {
            id: m.id, email: m.email, name: m.name, phone: m.phone, avatar_url: m.avatar_url,
            role_id: m.role_id, role_name, branch_id: m.branch_id, is_company_wide: m.is_company_wide,
            is_active: m.is_active, is_locked: m.is_locked, two_factor_enabled: m.two_factor_enabled,
            last_login_at: m.last_login_at.map(|v| v.into()),
            created_at: m.created_at.into(), updated_at: m.updated_at.into(),
        }
    }
}

/// Auth user response (minimal user info for auth endpoints).
#[derive(Debug, Serialize, ToSchema)]
pub struct AuthUserResponse {
    pub id: i64,
    pub email: String,
    pub name: String,
    pub phone: Option<String>,
    pub avatar_url: Option<String>,
    pub role: String,
    pub branch_id: Option<i64>,
    pub is_company_wide: bool,
}

/// Login response with tokens.
#[derive(Debug, Serialize, ToSchema)]
pub struct LoginResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: &'static str,
    pub expires_in: i64,
    pub user: AuthUserResponse,
}

/// Registration response.
#[derive(Debug, Serialize, ToSchema)]
pub struct RegisterResponse {
    pub user: AuthUserResponse,
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: &'static str,
    pub expires_in: i64,
}

/// Token refresh response.
#[derive(Debug, Serialize, ToSchema)]
pub struct TokenRefreshResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: &'static str,
    pub expires_in: i64,
}

/// Session info for listing active sessions.
#[derive(Debug, Serialize, ToSchema)]
pub struct SessionResponse {
    pub id: i64,
    pub device_info: Option<String>,
    pub ip_address: Option<String>,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub is_current: bool,
}

/// Generic message response.
#[derive(Debug, Serialize, ToSchema)]
pub struct MessageResponse {
    pub message: String,
}

// ── OTP Login ──────────────────────────────────────────────

/// Response for OTP send request.
#[derive(Debug, Serialize, ToSchema)]
pub struct OtpSentResponse {
    pub message: String,
    pub expires_in: i64,
}

// ── Password Reset ─────────────────────────────────────────

/// Response for password reset request.
#[derive(Debug, Serialize, ToSchema)]
pub struct PasswordResetResponse {
    pub message: String,
}

// ── 2FA (TOTP) ────────────────────────────────────────────

/// Response when 2FA is required during login.
#[derive(Debug, Serialize, ToSchema)]
pub struct Requires2FaResponse {
    pub requires_2fa: bool,
    pub temp_token: String,
}

/// Response for 2FA setup (enable).
#[derive(Debug, Serialize, ToSchema)]
pub struct TwoFaSetupResponse {
    pub secret: String,
    pub otpauth_url: String,
}

/// Response for 2FA enable confirm.
#[derive(Debug, Serialize, ToSchema)]
pub struct TwoFaEnabledResponse {
    pub message: String,
    pub backup_codes: Vec<String>,
}
