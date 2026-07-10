use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;


#[derive(Debug, Serialize, Deserialize, ToSchema)]
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

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DeviceModelResponse {
    pub id: i64,
    pub vendor: String,
    pub model: String,
    pub device_type: String,
    pub management_protocol: String,
    pub default_port: Option<i32>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
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

#[derive(Debug, Serialize, Deserialize, ToSchema)]
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

impl DeviceResponse {
    pub fn from_model(m: crate::modules::device::model::network_device_entity::Model) -> Self {
        Self {
            id: m.id, branch_id: m.branch_id, name: m.name, device_model_id: m.device_model_id,
            serial_number: m.serial_number, management_ip: m.management_ip,
            management_port: m.management_port, firmware_version: m.firmware_version,
            status: m.status, health_score: m.health_score,
            location_city: m.location_city, location_area: m.location_area,
            created_at: m.created_at.into(),
        }
    }
}

impl DeviceModelResponse {
    pub fn from_model(m: crate::modules::device::model::device_model_entity::Model) -> Self {
        Self {
            id: m.id, vendor: m.vendor, model: m.model, device_type: m.device_type,
            management_protocol: m.management_protocol, default_port: m.default_port,
            created_at: m.created_at.into(),
        }
    }
}

impl DevicePortResponse {
    pub fn from_model(m: crate::modules::device::model::device_port_entity::Model) -> Self {
        Self {
            id: m.id, device_id: m.device_id, port_number: m.port_number,
            port_name: m.port_name, port_type: m.port_type, speed_mbps: m.speed_mbps,
            status: m.status, connected_device_id: m.connected_device_id,
            customer_id: m.customer_id, created_at: m.created_at.into(),
        }
    }
}

impl FirmwareUpdateResponse {
    pub fn from_model(m: crate::modules::device::model::firmware_update_entity::Model) -> Self {
        Self {
            id: m.id, device_id: m.device_id, from_version: m.from_version,
            to_version: m.to_version, status: m.status,
            started_at: m.started_at.map(|v| v.into()), completed_at: m.completed_at.map(|v| v.into()),
            failure_reason: m.failure_reason, created_at: m.created_at.into(),
        }
    }
}

impl DeviceLogResponse {
    pub fn from_model(m: crate::modules::device::model::device_log_entity::Model) -> Self {
        Self {
            id: m.id, device_id: m.device_id, level: m.level, message: m.message,
            source: m.source, metadata: m.metadata, created_at: m.created_at.into(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct MessageResponse {
    pub message: String,
}
