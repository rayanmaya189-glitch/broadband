use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceRegisteredV1 {
    pub device_id: i64,
    pub name: String,
    pub device_type: String,
    pub vendor: String,
    pub model: String,
    pub management_ip: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceStatusChangedV1 {
    pub device_id: i64,
    pub old_status: String,
    pub new_status: String,
    pub health_score: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceDiscoveredV1 {
    pub discovery_result_id: i64,
    pub discovered_ip: String,
    pub vendor: Option<String>,
    pub model: Option<String>,
    pub auto_registered: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceFirmwareUpdateStartedV1 {
    pub device_id: i64,
    pub from_version: String,
    pub to_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceFirmwareUpdateCompletedV1 {
    pub device_id: i64,
    pub new_version: String,
}
