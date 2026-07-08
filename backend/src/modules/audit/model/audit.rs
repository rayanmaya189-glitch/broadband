use chrono::{DateTime, Utc};
use serde_json::Value;
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct AuditLog {
    pub id: i64,
    pub user_id: Option<i64>,
    pub user_email: Option<String>,
    pub user_role: Option<String>,
    pub action: String,
    pub resource_type: Option<String>,
    pub resource_id: Option<String>,
    pub ip_address: Option<String>,
    pub result: String,
    pub old_data: Option<Value>,
    pub new_data: Option<Value>,
    pub created_at: DateTime<Utc>,
}
