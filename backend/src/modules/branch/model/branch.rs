use chrono::{DateTime, Utc};
use sqlx::FromRow;

/// Row type mapping to the `branches` table.
#[derive(Debug, Clone, FromRow)]
pub struct Branch {
    pub id: i64,
    pub name: String,
    pub code: String,
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub pincode: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub is_active: bool,
    pub timezone: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
