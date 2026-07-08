use utoipa::ToSchema;
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateDeviceRequest {
    pub branch_id: i64,
    pub name: String,
    pub device_model_id: i64,
    pub serial_number: String,
    pub management_ip: String,
    pub management_port: Option<i32>,
    pub firmware_version: Option<String>,
    pub location_city: Option<String>,
    pub location_area: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateDeviceRequest {
    pub name: Option<String>,
    pub firmware_version: Option<String>,
    pub status: Option<String>,
    pub location_city: Option<String>,
    pub location_area: Option<String>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateDeviceModelRequest {
    pub vendor: String,
    pub model: String,
    pub device_type: String,
    pub management_protocol: String,
    pub default_port: Option<i32>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct DeviceQuery {
    pub branch_id: Option<i64>,
    pub status: Option<String>,
    pub device_type: Option<String>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}
