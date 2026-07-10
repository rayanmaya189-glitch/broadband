//! SeaORM-based controller for the Device domain.

use axum::extract::{Json, Path, Query, State};
use validator::Validate;

use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::modules::device::request::device_request::*;
use crate::modules::device::response::device_response::*;
use crate::modules::device::service::device_service_seaorm::DeviceServiceSeaorm;

pub async fn list_devices(State(state): State<SharedState>, Query(query): Query<DeviceQuery>) -> Result<Json<DeviceListResponse>, AppError> {
    let svc = DeviceServiceSeaorm::new(&state.db_seaorm);
    Ok(Json(svc.list_devices(query).await?))
}

pub async fn get_device(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<DeviceResponse>, AppError> {
    let svc = DeviceServiceSeaorm::new(&state.db_seaorm);
    Ok(Json(svc.get_device(id).await?))
}

pub async fn create_device(State(state): State<SharedState>, Json(req): Json<CreateDeviceRequest>) -> Result<Json<DeviceResponse>, AppError> {
    req.validate()?;
    let svc = DeviceServiceSeaorm::new(&state.db_seaorm);
    Ok(Json(svc.create_device(req).await?))
}

pub async fn update_device(State(state): State<SharedState>, Path(id): Path<i64>, Json(req): Json<UpdateDeviceRequest>) -> Result<Json<DeviceResponse>, AppError> {
    let svc = DeviceServiceSeaorm::new(&state.db_seaorm);
    Ok(Json(svc.update_device(id, req).await?))
}

pub async fn delete_device(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<MessageResponse>, AppError> {
    let svc = DeviceServiceSeaorm::new(&state.db_seaorm);
    Ok(Json(svc.delete_device(id).await?))
}

pub async fn list_models(State(state): State<SharedState>) -> Result<Json<Vec<DeviceModelResponse>>, AppError> {
    let svc = DeviceServiceSeaorm::new(&state.db_seaorm);
    Ok(Json(svc.list_models().await?))
}

pub async fn create_model(State(state): State<SharedState>, Json(req): Json<CreateDeviceModelRequest>) -> Result<Json<DeviceModelResponse>, AppError> {
    req.validate()?;
    let svc = DeviceServiceSeaorm::new(&state.db_seaorm);
    Ok(Json(svc.create_model(req).await?))
}

pub async fn list_ports(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<Vec<DevicePortResponse>>, AppError> {
    let svc = DeviceServiceSeaorm::new(&state.db_seaorm);
    Ok(Json(svc.list_ports(id).await?))
}

pub async fn update_port_status(State(state): State<SharedState>, Path((id, port_id)): Path<(i64, i64)>, Json(req): Json<PortStatusRequest>) -> Result<Json<DevicePortResponse>, AppError> {
    let svc = DeviceServiceSeaorm::new(&state.db_seaorm);
    Ok(Json(svc.update_port_status(id, port_id, req).await?))
}

pub async fn restart_device(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<MessageResponse>, AppError> {
    let svc = DeviceServiceSeaorm::new(&state.db_seaorm);
    Ok(Json(svc.restart_device(id).await?))
}

pub async fn shutdown_device(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<MessageResponse>, AppError> {
    let svc = DeviceServiceSeaorm::new(&state.db_seaorm);
    Ok(Json(svc.shutdown_device(id).await?))
}

pub async fn list_firmware_updates(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<Vec<FirmwareUpdateResponse>>, AppError> {
    let svc = DeviceServiceSeaorm::new(&state.db_seaorm);
    Ok(Json(svc.list_firmware_updates(id).await?))
}

pub async fn create_firmware_update(State(state): State<SharedState>, Path(id): Path<i64>, Json(req): Json<FirmwareUpdateRequest>) -> Result<Json<FirmwareUpdateResponse>, AppError> {
    req.validate()?;
    let svc = DeviceServiceSeaorm::new(&state.db_seaorm);
    Ok(Json(svc.create_firmware_update(id, req).await?))
}

pub async fn update_firmware_status(State(state): State<SharedState>, Path(update_id): Path<i64>, Json(req): Json<FirmwareStatusRequest>) -> Result<Json<FirmwareUpdateResponse>, AppError> {
    let svc = DeviceServiceSeaorm::new(&state.db_seaorm);
    Ok(Json(svc.update_firmware_status(update_id, req).await?))
}

pub async fn get_device_metrics(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<Vec<DeviceMetricResponse>>, AppError> {
    let svc = DeviceServiceSeaorm::new(&state.db_seaorm);
    Ok(Json(svc.get_device_metrics(id).await?))
}

pub async fn list_device_logs(State(state): State<SharedState>, Path(id): Path<i64>, Query(query): Query<DeviceLogQuery>) -> Result<Json<DeviceLogListResponse>, AppError> {
    let svc = DeviceServiceSeaorm::new(&state.db_seaorm);
    Ok(Json(svc.list_logs(id, query).await?))
}

pub async fn create_device_log(State(state): State<SharedState>, Path(id): Path<i64>, Json(req): Json<serde_json::Value>) -> Result<Json<DeviceLogResponse>, AppError> {
    let level = req["level"].as_str().unwrap_or("info");
    let message = req["message"].as_str().ok_or_else(|| AppError::Validation("message required".into()))?;
    let source = req["source"].as_str();
    let metadata = req.get("metadata").cloned();
    let svc = DeviceServiceSeaorm::new(&state.db_seaorm);
    Ok(Json(svc.insert_log(id, level, message, source, metadata).await?))
}
