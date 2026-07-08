use axum::extract::{Json, Path, Query, State};
use validator::Validate;

use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::modules::device::request::device_request::*;
use crate::modules::device::response::device_response::*;
use crate::modules::device::service::device_service::DeviceService;

pub async fn list_devices(State(state): State<SharedState>, Query(query): Query<DeviceQuery>) -> Result<Json<DeviceListResponse>, AppError> {
    let svc = DeviceService::new(&state.db);
    Ok(Json(svc.list_devices(query).await?))
}

pub async fn get_device(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<DeviceResponse>, AppError> {
    let svc = DeviceService::new(&state.db);
    Ok(Json(svc.get_device(id).await?))
}

pub async fn create_device(State(state): State<SharedState>, Json(req): Json<CreateDeviceRequest>) -> Result<Json<DeviceResponse>, AppError> {
    req.validate()?;
    let svc = DeviceService::new(&state.db);
    Ok(Json(svc.create_device(req).await?))
}

pub async fn update_device(State(state): State<SharedState>, Path(id): Path<i64>, Json(req): Json<UpdateDeviceRequest>) -> Result<Json<DeviceResponse>, AppError> {
    let svc = DeviceService::new(&state.db);
    Ok(Json(svc.update_device(id, req).await?))
}

pub async fn delete_device(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<MessageResponse>, AppError> {
    let svc = DeviceService::new(&state.db);
    Ok(Json(svc.delete_device(id).await?))
}

pub async fn list_models(State(state): State<SharedState>) -> Result<Json<Vec<DeviceModelResponse>>, AppError> {
    let svc = DeviceService::new(&state.db);
    Ok(Json(svc.list_models().await?))
}

pub async fn create_model(State(state): State<SharedState>, Json(req): Json<CreateDeviceModelRequest>) -> Result<Json<DeviceModelResponse>, AppError> {
    req.validate()?;
    let svc = DeviceService::new(&state.db);
    Ok(Json(svc.create_model(req).await?))
}
