use axum::extract::{Json, State};
use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::modules::realtime::response::realtime_response::*;
use crate::modules::realtime::service::realtime_service::RealtimeService;

pub async fn health(State(_state): State<SharedState>) -> Result<Json<HealthResponse>, AppError> {
    let svc = RealtimeService::new();
    Ok(Json(svc.health().await?))
}

pub async fn channels(State(_state): State<SharedState>) -> Result<Json<Vec<ChannelInfo>>, AppError> {
    let svc = RealtimeService::new();
    Ok(Json(svc.list_channels().await?))
}
