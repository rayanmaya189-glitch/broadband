use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SampleResponse { pub id: i64, pub customer_id: Option<i64>, pub subscription_id: Option<i64>, pub branch_id: Option<i64>, pub interface_name: Option<String>, pub bytes_in: i64, pub bytes_out: i64, pub packets_in: i64, pub packets_out: i64, pub sample_duration_seconds: i32, pub recorded_at: DateTime<Utc> }

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PolicyResponse { pub id: i64, pub name: String, pub priority: i32, pub criteria: serde_json::Value, pub action: serde_json::Value, pub is_active: bool, pub created_at: DateTime<Utc>, pub updated_at: DateTime<Utc> }

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AggregateResponse { pub id: i64, pub customer_id: Option<i64>, pub subscription_id: Option<i64>, pub branch_id: Option<i64>, pub period: String, pub total_bytes_in: i64, pub total_bytes_out: i64, pub peak_bytes_in: i64, pub peak_bytes_out: i64, pub sample_count: i64, pub period_start: DateTime<Utc>, pub period_end: DateTime<Utc> }
