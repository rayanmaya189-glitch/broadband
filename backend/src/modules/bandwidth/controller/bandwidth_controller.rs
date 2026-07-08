use axum::extract::{Json, Path, State};
use validator::Validate;
use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::modules::bandwidth::request::bandwidth_request::*;
use crate::modules::bandwidth::response::bandwidth_response::*;
use crate::modules::bandwidth::service::bandwidth_service::BandwidthService;

pub async fn list_profiles(State(state): State<SharedState>) -> Result<Json<BandwidthProfileListResponse>, AppError> {
    let svc = BandwidthService::new(&state.db);
    Ok(Json(svc.list_profiles(1, 100).await?))
}

pub async fn get_profile(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<BandwidthProfileResponse>, AppError> {
    let svc = BandwidthService::new(&state.db);
    Ok(Json(svc.get_profile(id).await?))
}

pub async fn create_profile(State(state): State<SharedState>, Json(req): Json<CreateBandwidthProfileRequest>) -> Result<Json<BandwidthProfileResponse>, AppError> {
    req.validate()?;
    let svc = BandwidthService::new(&state.db);
    Ok(Json(svc.create_profile(req).await?))
}

pub async fn update_profile(State(state): State<SharedState>, Path(id): Path<i64>, Json(req): Json<UpdateBandwidthProfileRequest>) -> Result<Json<BandwidthProfileResponse>, AppError> {
    let svc = BandwidthService::new(&state.db);
    Ok(Json(svc.update_profile(id, req).await?))
}

pub async fn delete_profile(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<MessageResponse>, AppError> {
    let svc = BandwidthService::new(&state.db);
    Ok(Json(svc.delete_profile(id).await?))
}
