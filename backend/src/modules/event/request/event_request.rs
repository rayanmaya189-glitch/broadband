use utoipa::ToSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, ToSchema)]
pub struct EventQuery {
    pub event_type: Option<String>,
    pub aggregate_type: Option<String>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct PublishEventRequest {
    pub event_type: String,
    pub aggregate_type: String,
    pub aggregate_id: i64,
    pub payload: serde_json::Value,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateSubscriptionRequest {
    pub subscriber_name: String,
    pub event_type: String,
}
