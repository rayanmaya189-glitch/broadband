use chrono::{DateTime, Utc};
use serde_json::Value;
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct Event {
    pub id: i64,
    pub event_type: String,
    pub aggregate_type: String,
    pub aggregate_id: i64,
    pub payload: Value,
    pub caused_by_user_id: Option<i64>,
    pub caused_by_branch_id: Option<i64>,
    pub sequence_number: i64,
    pub published_at: DateTime<Utc>,
    pub processed: bool,
}
