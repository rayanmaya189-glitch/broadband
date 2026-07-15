use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::shared::app_state::AppState;
use crate::shared::errors::AppError;
use crate::shared::middleware::auth::UserContext;
use crate::modules::bandwidth::application::services::BandwidthService;

#[derive(Debug, Serialize)]
pub struct BandwidthProfileResponse {
    pub id: i64, pub name: String, pub download_kbps: i32, pub upload_kbps: i32, pub is_active: bool,
}

#[derive(Debug, Deserialize)]
pub struct CreateProfileRequest {
    pub name: String, pub download_kbps: i32, pub upload_kbps: i32,
}

pub async fn list_profiles(State(state): State<Arc<AppState>>, _user: UserContext) -> Result<Json<Vec<BandwidthProfileResponse>>, AppError> {
    let profiles = BandwidthService::list_profiles(&state.db).await?;
    Ok(Json(profiles.into_iter().map(|p| BandwidthProfileResponse { id: p.id, name: p.name, download_kbps: p.download_kbps, upload_kbps: p.upload_kbps, is_active: p.is_active }).collect()))
}

pub async fn create_profile(State(state): State<Arc<AppState>>, _user: UserContext, Json(req): Json<CreateProfileRequest>) -> Result<(StatusCode, Json<BandwidthProfileResponse>), AppError> {
    let p = BandwidthService::create_profile(&state.db, req.name, req.download_kbps, req.upload_kbps).await?;
    Ok((StatusCode::CREATED, Json(BandwidthProfileResponse { id: p.id, name: p.name, download_kbps: p.download_kbps, upload_kbps: p.upload_kbps, is_active: p.is_active })))
}

pub async fn update_profile(State(state): State<Arc<AppState>>, _user: UserContext, Path(id): Path<i64>, Json(req): Json<UpdateProfileRequest>) -> Result<Json<BandwidthProfileResponse>, AppError> {
    let p = BandwidthService::update_profile(&state.db, id, req.name, req.download_kbps, req.upload_kbps).await?;
    Ok(Json(BandwidthProfileResponse { id: p.id, name: p.name, download_kbps: p.download_kbps, upload_kbps: p.upload_kbps, is_active: p.is_active }))
}

#[derive(Debug, Deserialize)]
pub struct UpdateProfileRequest {
    pub name: Option<String>, pub download_kbps: Option<i32>, pub upload_kbps: Option<i32>,
}

pub async fn delete_profile(State(state): State<Arc<AppState>>, _user: UserContext, Path(id): Path<i64>) -> Result<StatusCode, AppError> {
    BandwidthService::delete_profile(&state.db, id).await?;
    Ok(StatusCode::NO_CONTENT)
}
