use chrono::{DateTime, Utc};
use serde_json::Value;
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct DeviceModel {
    pub id: i64,
    pub vendor: String,
    pub model: String,
    pub device_type: String,
    pub management_protocol: String,
    pub default_port: Option<i32>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow)]
pub struct NetworkDevice {
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
    pub created_by: Option<i64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow)]
pub struct DevicePort {
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

#[derive(Debug, Clone, FromRow)]
pub struct FirmwareUpdate {
    pub id: i64,
    pub device_id: i64,
    pub from_version: Option<String>,
    pub to_version: String,
    pub status: String,
    pub initiated_by: Option<i64>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub failure_reason: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow)]
pub struct DeviceLog {
    pub id: i64,
    pub device_id: i64,
    pub level: String,
    pub message: String,
    pub source: Option<String>,
    pub metadata: Option<Value>,
    pub created_at: DateTime<Utc>,
}
