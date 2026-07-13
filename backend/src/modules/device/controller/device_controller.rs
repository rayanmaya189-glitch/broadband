use axum::extract::{Json, Path, Query, State};
use validator::Validate;

use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::modules::device::request::device_request::*;
use crate::modules::device::response::device_response::*;
use crate::modules::device::service::device_service::DeviceService;

// ── Device CRUD ─────────────────────────────────────────────

#[utoipa::path(
    get,
    path = "/api/v1/devices",
    tag = "Devices",
    security(("bearer_auth" = [])),
    params(
        ("page" = Option<i64>, Query, description = "Page number"),
        ("limit" = Option<i64>, Query, description = "Items per page"),
        ("status" = Option<String>, Query, description = "Filter by status"),
        ("device_type" = Option<String>, Query, description = "Filter by type")
    ),
    responses(
        (status = 200, description = "List of devices"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn list_devices(State(state): State<SharedState>, Query(query): Query<DeviceQuery>) -> Result<Json<DeviceListResponse>, AppError> {
    let svc = DeviceService::new(&state.db);
    Ok(Json(svc.list_devices(query).await?))
}

#[utoipa::path(
    get,
    path = "/api/v1/devices/{id}",
    tag = "Devices",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Device ID")),
    responses(
        (status = 200, description = "Device details", body = DeviceResponse),
        (status = 404, description = "Device not found")
    )
)]
pub async fn get_device(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<DeviceResponse>, AppError> {
    let svc = DeviceService::new(&state.db);
    Ok(Json(svc.get_device(id).await?))
}

#[utoipa::path(
    post,
    path = "/api/v1/devices",
    tag = "Devices",
    security(("bearer_auth" = [])),
    request_body = CreateDeviceRequest,
    responses(
        (status = 200, description = "Device created", body = DeviceResponse),
        (status = 409, description = "Serial number already exists"),
        (status = 422, description = "Validation error")
    )
)]
pub async fn create_device(State(state): State<SharedState>, Json(req): Json<CreateDeviceRequest>) -> Result<Json<DeviceResponse>, AppError> {
    req.validate()?;
    let svc = DeviceService::new(&state.db);
    Ok(Json(svc.create_device(req).await?))
}

#[utoipa::path(
    put,
    path = "/api/v1/devices/{id}",
    tag = "Devices",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Device ID")),
    request_body = UpdateDeviceRequest,
    responses(
        (status = 200, description = "Device updated", body = DeviceResponse),
        (status = 404, description = "Device not found")
    )
)]
pub async fn update_device(State(state): State<SharedState>, Path(id): Path<i64>, Json(req): Json<UpdateDeviceRequest>) -> Result<Json<DeviceResponse>, AppError> {
    let svc = DeviceService::new(&state.db);
    Ok(Json(svc.update_device(id, req).await?))
}

#[utoipa::path(
    delete,
    path = "/api/v1/devices/{id}",
    tag = "Devices",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Device ID")),
    responses(
        (status = 200, description = "Device deleted"),
        (status = 404, description = "Device not found")
    )
)]
pub async fn delete_device(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<MessageResponse>, AppError> {
    let svc = DeviceService::new(&state.db);
    Ok(Json(svc.delete_device(id).await?))
}

// ── Device Models ───────────────────────────────────────────

#[utoipa::path(
    get,
    path = "/api/v1/devices/models",
    tag = "Devices",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "List of device models", body = Vec<DeviceModelResponse>),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn list_models(State(state): State<SharedState>) -> Result<Json<Vec<DeviceModelResponse>>, AppError> {
    let svc = DeviceService::new(&state.db);
    Ok(Json(svc.list_models().await?))
}

#[utoipa::path(
    post,
    path = "/api/v1/devices/models",
    tag = "Devices",
    security(("bearer_auth" = [])),
    request_body = CreateDeviceModelRequest,
    responses(
        (status = 200, description = "Model created", body = DeviceModelResponse),
        (status = 422, description = "Validation error")
    )
)]
pub async fn create_model(State(state): State<SharedState>, Json(req): Json<CreateDeviceModelRequest>) -> Result<Json<DeviceModelResponse>, AppError> {
    req.validate()?;
    let svc = DeviceService::new(&state.db);
    Ok(Json(svc.create_model(req).await?))
}

// ── Ports ───────────────────────────────────────────────────

#[utoipa::path(
    get,
    path = "/api/v1/devices/{id}/ports",
    tag = "Devices",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Device ID")),
    responses(
        (status = 200, description = "List of ports", body = Vec<DevicePortResponse>),
        (status = 404, description = "Device not found")
    )
)]
pub async fn list_ports(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<Vec<DevicePortResponse>>, AppError> {
    let svc = DeviceService::new(&state.db);
    Ok(Json(svc.list_ports(id).await?))
}

#[utoipa::path(
    post,
    path = "/api/v1/devices/{id}/ports/{port_id}",
    tag = "Devices",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Device ID"), ("port_id" = i64, Path, description = "Port ID")),
    request_body = PortStatusRequest,
    responses(
        (status = 200, description = "Port status updated", body = DevicePortResponse),
        (status = 404, description = "Port not found")
    )
)]
pub async fn update_port_status(State(state): State<SharedState>, Path((id, port_id)): Path<(i64, i64)>, Json(req): Json<PortStatusRequest>) -> Result<Json<DevicePortResponse>, AppError> {
    let svc = DeviceService::new(&state.db);
    Ok(Json(svc.update_port_status(id, port_id, req).await?))
}

// ── Device Control ──────────────────────────────────────────

#[utoipa::path(
    post,
    path = "/api/v1/devices/{id}/restart",
    tag = "Devices",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Device ID")),
    responses(
        (status = 200, description = "Device restart initiated"),
        (status = 404, description = "Device not found")
    )
)]
pub async fn restart_device(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<MessageResponse>, AppError> {
    let svc = DeviceService::new(&state.db);
    Ok(Json(svc.restart_device(id).await?))
}

#[utoipa::path(
    post,
    path = "/api/v1/devices/{id}/shutdown",
    tag = "Devices",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Device ID")),
    responses(
        (status = 200, description = "Device shutdown initiated"),
        (status = 404, description = "Device not found")
    )
)]
pub async fn shutdown_device(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<MessageResponse>, AppError> {
    let svc = DeviceService::new(&state.db);
    Ok(Json(svc.shutdown_device(id).await?))
}

// ── Firmware ────────────────────────────────────────────────

#[utoipa::path(
    get,
    path = "/api/v1/devices/{id}/firmware",
    tag = "Devices",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Device ID")),
    responses(
        (status = 200, description = "List of firmware updates", body = Vec<FirmwareUpdateResponse>),
        (status = 404, description = "Device not found")
    )
)]
pub async fn list_firmware_updates(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<Vec<FirmwareUpdateResponse>>, AppError> {
    let svc = DeviceService::new(&state.db);
    Ok(Json(svc.list_firmware_updates(id).await?))
}

#[utoipa::path(
    post,
    path = "/api/v1/devices/{id}/firmware/update",
    tag = "Devices",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Device ID")),
    request_body = FirmwareUpdateRequest,
    responses(
        (status = 200, description = "Firmware update initiated", body = FirmwareUpdateResponse),
        (status = 404, description = "Device not found"),
        (status = 422, description = "Validation error")
    )
)]
pub async fn create_firmware_update(State(state): State<SharedState>, Path(id): Path<i64>, Json(req): Json<FirmwareUpdateRequest>) -> Result<Json<FirmwareUpdateResponse>, AppError> {
    req.validate()?;
    let svc = DeviceService::new(&state.db);
    Ok(Json(svc.create_firmware_update(id, req).await?))
}

#[utoipa::path(
    post,
    path = "/api/v1/devices/firmware/{update_id}/status",
    tag = "Devices",
    security(("bearer_auth" = [])),
    params(("update_id" = i64, Path, description = "Firmware update ID")),
    request_body = FirmwareStatusRequest,
    responses(
        (status = 200, description = "Firmware status updated", body = FirmwareUpdateResponse),
        (status = 404, description = "Update not found")
    )
)]
pub async fn update_firmware_status(State(state): State<SharedState>, Path(update_id): Path<i64>, Json(req): Json<FirmwareStatusRequest>) -> Result<Json<FirmwareUpdateResponse>, AppError> {
    let svc = DeviceService::new(&state.db);
    Ok(Json(svc.update_firmware_status(update_id, req).await?))
}

// ── Metrics ─────────────────────────────────────────────────

#[utoipa::path(
    get,
    path = "/api/v1/devices/{id}/metrics",
    tag = "Devices",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Device ID")),
    responses(
        (status = 200, description = "Device metrics", body = Vec<DeviceMetricResponse>),
        (status = 404, description = "Device not found")
    )
)]
pub async fn get_device_metrics(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<Vec<DeviceMetricResponse>>, AppError> {
    let svc = DeviceService::new(&state.db);
    Ok(Json(svc.get_device_metrics(id).await?))
}

// ── Device Logs ────────────────────────────────────────────

#[utoipa::path(
    get,
    path = "/api/v1/devices/{id}/logs",
    tag = "Devices",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Device ID")),
    responses(
        (status = 200, description = "Device logs", body = DeviceLogListResponse),
        (status = 404, description = "Device not found")
    )
)]
pub async fn list_device_logs(State(state): State<SharedState>, Path(id): Path<i64>, Query(query): Query<DeviceLogQuery>) -> Result<Json<DeviceLogListResponse>, AppError> {
    let svc = DeviceService::new(&state.db);
    Ok(Json(svc.list_logs(id, query).await?))
}

#[utoipa::path(
    post,
    path = "/api/v1/devices/{id}/logs",
    tag = "Devices",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Device ID")),
    request_body = serde_json::Value,
    responses(
        (status = 200, description = "Log created", body = DeviceLogResponse),
        (status = 404, description = "Device not found"),
        (status = 422, description = "Validation error")
    )
)]
pub async fn create_device_log(State(state): State<SharedState>, Path(id): Path<i64>, Json(req): Json<serde_json::Value>) -> Result<Json<DeviceLogResponse>, AppError> {
    let level = req["level"].as_str().unwrap_or("info");
    let message = req["message"].as_str().ok_or_else(|| AppError::Validation("message required".into()))?;
    let source = req["source"].as_str();
    let metadata = req.get("metadata").cloned();
    let svc = DeviceService::new(&state.db);
    Ok(Json(svc.insert_log(id, level, message, source, metadata).await?))
}
