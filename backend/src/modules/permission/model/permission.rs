use chrono::{DateTime, Utc};
use sqlx::FromRow;

/// Row type mapping to the `permissions` table.
#[derive(Debug, Clone, FromRow)]
pub struct Permission {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub module: String,
    pub created_at: DateTime<Utc>,
}
