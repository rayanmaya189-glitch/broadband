use axum::extract::{Json, Query, State};
use validator::Validate;
use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::modules::discovery::request::discovery_request::*;
use crate::modules::discovery::response::discovery_response::*;
use crate::modules::discovery::service::discovery_service::DiscoveryService;

pub async fn list_scans(State(state): State<SharedState>) -> Result<Json<Vec<ScanResponse>>, AppError> {
    let svc = DiscoveryService::new(&state.db);
    Ok(Json(svc.list_scans().await?))
}

pub async fn create_scan(State(state): State<SharedState>, Json(req): Json<CreateScanRequest>) -> Result<Json<ScanResponse>, AppError> {
    req.validate()?;
    let svc = DiscoveryService::new(&state.db);
    Ok(Json(svc.create_scan(req).await?))
}

pub async fn list_results(State(state): State<SharedState>, Query(_q): Query<DiscoveryQuery>) -> Result<Json<Vec<ResultResponse>>, AppError> {
    let svc = DiscoveryService::new(&state.db);
    Ok(Json(svc.list_results().await?))
}
