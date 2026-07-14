use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct RecordHealthCheckRequest {
    pub service_name: String,
    pub status: String,
    pub response_time_ms: Option<i32>,
    pub error_message: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct RecordMetricRequest {
    pub metric_name: String,
    pub metric_type: String,
    pub value: f64,
    pub tags: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateAlertRuleRequest {
    pub name: String,
    pub service_name: String,
    pub metric_name: String,
    pub operator: String,
    pub threshold: f64,
    pub severity: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct HealthCheckQuery {
    pub service_name: Option<String>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct AlertQuery {
    pub status: Option<String>,
    pub severity: Option<String>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}
