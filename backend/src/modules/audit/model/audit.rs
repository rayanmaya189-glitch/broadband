use chrono::{DateTime, Utc};
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct AuditLog {
    pub id: i64,
    pub user_id: Option<i64>,
    pub user_email: Option<String>,
    pub user_role: Option<String>,
    pub action: String,
    pub resource_type: Option<String>,
    pub resource_id: Option<String>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub result: String,
    pub old_data: Option<Value>,
    pub new_data: Option<Value>,
    pub metadata: Option<Value>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct AuditStats {
    pub total_logs: i64,
    pub granted_count: i64,
    pub denied_count: i64,
    pub unique_users: i64,
}
