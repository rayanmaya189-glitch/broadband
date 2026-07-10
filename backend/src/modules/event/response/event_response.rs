use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use serde_json::Value;


#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct EventResponse {
    pub id: i64,
    pub event_type: String,
    pub aggregate_type: String,
    pub aggregate_id: i64,
    pub payload: Value,
    pub metadata: Option<Value>,
    pub caused_by_user_id: Option<i64>,
    pub caused_by_branch_id: Option<i64>,
    pub sequence_number: i64,
    pub published_at: DateTime<Utc>,
    pub processed: bool,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct EventListResponse {
    pub events: Vec<EventResponse>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct EventSubscriptionResponse {
    pub id: i64,
    pub subscriber_name: String,
    pub event_type: String,
    pub last_processed_at: Option<DateTime<Utc>>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct EventStatsResponse {
    pub total_events: i64,
    pub processed_events: i64,
    pub unprocessed_events: i64,
    pub unique_event_types: i64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct MessageResponse {
    pub message: String,
}
