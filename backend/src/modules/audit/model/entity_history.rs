use chrono::{DateTime, Utc};
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct EntityHistory {
    pub id: i64,
    pub entity_type: String,
    pub entity_id: i64,
    pub action: String,
    pub old_data: Option<Value>,
    pub new_data: Option<Value>,
    pub changed_fields: Option<Vec<String>>,
    pub user_id: Option<i64>,
    pub branch_id: Option<i64>,
    pub ip_address: Option<String>,
    pub reason: Option<String>,
    pub rollback_reference: Option<i64>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct EntityHistoryStats {
    pub total_entries: i64,
    pub total_entities: i64,
    pub total_rollbacks: i64,
}
