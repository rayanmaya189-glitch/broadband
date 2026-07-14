use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct RecordSampleRequest {
    pub customer_id: Option<i64>, pub subscription_id: Option<i64>, pub branch_id: Option<i64>,
    pub interface_name: Option<String>, pub bytes_in: i64, pub bytes_out: i64,
    pub packets_in: i64, pub packets_out: i64, pub sample_duration_seconds: i32,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreatePolicyRequest {
    pub name: String, pub priority: i32, pub criteria: serde_json::Value, pub action: serde_json::Value,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct SampleQuery { pub customer_id: Option<i64>, pub page: Option<i64>, pub per_page: Option<i64> }

#[derive(Debug, Deserialize, ToSchema)]
pub struct AggregateQuery { pub customer_id: Option<i64>, pub period: Option<String>, pub page: Option<i64>, pub per_page: Option<i64> }
