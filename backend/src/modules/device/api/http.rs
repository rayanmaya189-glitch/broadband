use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::shared::app_state::AppState;
use crate::shared::errors::AppError;
use crate::shared::middleware::auth::{UserContext, require_permission};
use crate::modules::device::application::services::DeviceService;

#[derive(Debug, Serialize)]
pub struct DeviceResponse {
    pub id: i64, pub name: String, pub serial_number: String,
    pub management_ip: String, pub status: String, pub health_score: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct RegisterDeviceRequest {
    pub branch_id: i64, pub name: String, pub device_model_id: i64,
    pub serial_number: String, pub management_ip: String,
}

pub async fn list_devices(State(state): State<Arc<AppState>>, user: UserContext) -> Result<Json<Vec<DeviceResponse>>, AppError> {
    require_permission(&user, "device.router.view").map_err(|e| AppError::Forbidden(e.1))?;
    let bid = if user.is_company_wide { None } else { user.branch_id };
    let devices = DeviceService::list_devices(&state.db, bid).await?;
    Ok(Json(devices.into_iter().map(|d| DeviceResponse { id: d.id, name: d.name, serial_number: d.serial_number, management_ip: d.management_ip, status: d.status, health_score: d.health_score }).collect()))
}

pub async fn get_device(State(state): State<Arc<AppState>>, _user: UserContext, Path(id): Path<i64>) -> Result<Json<DeviceResponse>, AppError> {
    let d = DeviceService::get_device(&state.db, id).await?;
    Ok(Json(DeviceResponse { id: d.id, name: d.name, serial_number: d.serial_number, management_ip: d.management_ip, status: d.status, health_score: d.health_score }))
}

pub async fn register_device(State(state): State<Arc<AppState>>, user: UserContext, Json(req): Json<RegisterDeviceRequest>) -> Result<(StatusCode, Json<DeviceResponse>), AppError> {
    require_permission(&user, "device.router.register").map_err(|e| AppError::Forbidden(e.1))?;
    let d = DeviceService::register_device(&state.db, req.branch_id, req.name, req.device_model_id, req.serial_number, req.management_ip).await?;
    Ok((StatusCode::CREATED, Json(DeviceResponse { id: d.id, name: d.name, serial_number: d.serial_number, management_ip: d.management_ip, status: d.status, health_score: d.health_score })))
}

pub async fn update_device_status(State(state): State<Arc<AppState>>, _user: UserContext, Path(id): Path<i64>, Json(req): Json<UpdateStatusRequest>) -> Result<Json<DeviceResponse>, AppError> {
    let d = DeviceService::update_device_status(&state.db, id, &req.status).await?;
    Ok(Json(DeviceResponse { id: d.id, name: d.name, serial_number: d.serial_number, management_ip: d.management_ip, status: d.status, health_score: d.health_score }))
}

#[derive(Debug, Deserialize)]
pub struct UpdateStatusRequest { pub status: String }
