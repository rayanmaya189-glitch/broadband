use chrono::{DateTime, Utc};
use sqlx::FromRow;

/// Row type mapping to the `roles` table.
#[derive(Debug, Clone, FromRow)]
pub struct Role {
    pub id: i64,
    pub name: String,
    pub display_name: String,
    pub description: Option<String>,
    pub is_system: bool,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
