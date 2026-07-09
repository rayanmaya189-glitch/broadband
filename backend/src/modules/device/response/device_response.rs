use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct DeviceResponse {
    pub id: i64,
    pub branch_id: i64,
    pub name: String,
    pub device_model_id: i64,
    pub serial_number: String,
    pub management_ip: String,
    pub management_port: Option<i32>,
    pub firmware_version: Option<String>,
    pub status: String,
    pub health_score: Option<i32>,
    pub location_city: Option<String>,
    pub location_area: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DeviceListResponse {
    pub devices: Vec<DeviceResponse>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
    pub total_pages: i64,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct DeviceModelResponse {
    pub id: i64,
    pub vendor: String,
    pub model: String,
    pub device_type: String,
    pub management_protocol: String,
    pub default_port: Option<i32>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct DevicePortResponse {
    pub id: i64,
    pub device_id: i64,
    pub port_number: i32,
    pub port_name: Option<String>,
    pub port_type: Option<String>,
    pub speed_mbps: Option<i32>,
    pub status: String,
    pub connected_device_id: Option<i64>,
    pub customer_id: Option<i64>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct FirmwareUpdateResponse {
    pub id: i64,
    pub device_id: i64,
    pub from_version: Option<String>,
    pub to_version: String,
    pub status: String,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub failure_reason: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DeviceMetricResponse {
    pub metric_name: String,
    pub metric_value: f64,
    pub unit: Option<String>,
    pub recorded_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DeviceLogResponse {
    pub id: i64,
    pub device_id: i64,
    pub level: String,
    pub message: String,
    pub source: Option<String>,
    pub metadata: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DeviceLogListResponse {
    pub logs: Vec<DeviceLogResponse>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
    pub total_pages: i64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct MessageResponse {
    pub message: String,
}
