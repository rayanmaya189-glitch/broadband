use chrono::{DateTime, Utc};
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
