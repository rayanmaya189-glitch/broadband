use utoipa::ToSchema;
use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::FromRow;

/// User list item response.
#[derive(Debug, Serialize, FromRow, ToSchema)]
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
