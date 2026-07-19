/// OpenAPI schemas and stub handlers for Device endpoints.
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

// ── Request / Response types ─────────────────────────────────────────

#[derive(Debug, Serialize, ToSchema)]
pub struct DeviceResponse {
    pub id: i64,
    pub name: String,
    pub serial_number: String,
    pub management_ip: String,
    pub status: String,
    pub health_score: Option<i32>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct RegisterDeviceRequest {
    pub branch_id: i64,
    pub name: String,
    pub device_model_id: i64,
    pub serial_number: String,
    pub management_ip: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateDeviceStatusRequest {
    pub status: String,
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct DeviceListParams {
    pub status: Option<String>,
    pub branch_id: Option<i64>,
}

// ── Stub handler functions ───────────────────────────────────────────

/// List all network devices
#[utoipa::path(
    get,
    path = "/api/v1/devices",
    tag = "Devices",
    params(DeviceListParams),
    responses(
        (status = 200, description = "List of devices"),
        (status = 403, description = "Insufficient permissions")
    ),
    security(("bearer_auth" = []))
)]
pub async fn list_devices() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Register a new network device
#[utoipa::path(
    post,
    path = "/api/v1/devices",
    tag = "Devices",
    request_body = RegisterDeviceRequest,
    responses(
        (status = 201, description = "Device registered", body = DeviceResponse),
        (status = 409, description = "Serial number already exists")
    ),
    security(("bearer_auth" = []))
)]
pub async fn register_device() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Get a device by ID
#[utoipa::path(
    get,
    path = "/api/v1/devices/{id}",
    tag = "Devices",
    params(("id" = i64, Path, description = "Device ID")),
    responses(
        (status = 200, description = "Device details", body = DeviceResponse),
        (status = 404, description = "Device not found")
    ),
    security(("bearer_auth" = []))
)]
pub async fn get_device() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Update device status
#[utoipa::path(
    put,
    path = "/api/v1/devices/{id}/status",
    tag = "Devices",
    params(("id" = i64, Path, description = "Device ID")),
    request_body = UpdateDeviceStatusRequest,
    responses(
        (status = 200, description = "Status updated"),
        (status = 404, description = "Device not found")
    ),
    security(("bearer_auth" = []))
)]
pub async fn update_device_status() -> axum::Json<serde_json::Value> {
    unimplemented!()
}
