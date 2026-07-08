use chrono::{DateTime, Utc};
use sqlx::FromRow;

/// Row type mapping to the `users` table.
#[derive(Debug, Clone, FromRow)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub password_hash: String,
    pub name: String,
    pub phone: Option<String>,
    pub avatar_url: Option<String>,
    pub role_id: i64,
    pub branch_id: Option<i64>,
    pub is_company_wide: bool,
    pub is_active: bool,
    pub is_locked: bool,
    pub locked_until: Option<DateTime<Utc>>,
    pub failed_attempts: i32,
    pub last_login_at: Option<DateTime<Utc>>,
    pub two_factor_enabled: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Row type mapping to the `refresh_tokens` table.
#[derive(Debug, Clone, FromRow)]
pub struct RefreshToken {
    pub id: i64,
    pub user_id: i64,
    pub token_hash: String,
    pub device_info: Option<String>,
    pub ip_address: Option<String>,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub revoked_at: Option<DateTime<Utc>>,
}
