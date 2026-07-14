use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct HealthCheckResponse {
    pub id: i64, pub service_name: String, pub status: String,
    pub response_time_ms: Option<i32>, pub error_message: Option<String>,
    pub checked_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct MetricResponse {
    pub id: i64, pub metric_name: String, pub metric_type: String,
    pub value: f64, pub tags: Option<serde_json::Value>,
    pub recorded_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AlertResponse {
    pub id: i64, pub rule_id: Option<i64>, pub service_name: String,
    pub severity: String, pub message: String, pub status: String,
    pub acknowledged_by: Option<i64>, pub acknowledged_at: Option<DateTime<Utc>>,
    pub resolved_at: Option<DateTime<Utc>>, pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AlertRuleResponse {
    pub id: i64, pub name: String, pub service_name: String,
    pub metric_name: String, pub operator: String, pub threshold: f64,
    pub severity: String, pub is_active: bool, pub created_at: DateTime<Utc>,
}
