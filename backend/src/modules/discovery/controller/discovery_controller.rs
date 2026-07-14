//! SeaORM-based controller for the Discovery domain.

use axum::extract::{Json, Path, Query, State};
use validator::Validate;

use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::modules::discovery::request::discovery_request::*;
use crate::modules::discovery::response::discovery_response::*;
use crate::modules::discovery::service::discovery_service::DiscoveryService;

pub async fn list_scans(State(state): State<SharedState>, Query(q): Query<DiscoveryQuery>) -> Result<Json<Vec<DiscoveryScanResponse>>, AppError> {
    let svc = DiscoveryService::new(&state.db);
    Ok(Json(svc.list_scans(q.branch_id).await?))
}

pub async fn create_scan(State(state): State<SharedState>, Json(req): Json<CreateDiscoveryScanRequest>) -> Result<Json<DiscoveryScanResponse>, AppError> {
    req.validate()?;
    let svc = DiscoveryService::new(&state.db);
    Ok(Json(svc.create_scan(req).await?))
}

pub async fn start_scan(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<MessageResponse>, AppError> {
    let svc = DiscoveryService::new(&state.db);
    Ok(Json(svc.start_scan(id).await?))
}

pub async fn stop_scan(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<MessageResponse>, AppError> {
    let svc = DiscoveryService::new(&state.db);
    Ok(Json(svc.stop_scan(id).await?))
}

pub async fn list_results(State(state): State<SharedState>, Query(q): Query<DiscoveryResultQuery>) -> Result<Json<Vec<DiscoveryResultResponse>>, AppError> {
    let svc = DiscoveryService::new(&state.db);
    Ok(Json(svc.list_results(q.status.as_deref(), q.branch_id).await?))
}

pub async fn approve_result(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<DiscoveryResultResponse>, AppError> {
    let svc = DiscoveryService::new(&state.db);
    Ok(Json(svc.approve_result(id, 1).await?))
}

pub async fn reject_result(State(state): State<SharedState>, Path(id): Path<i64>, Json(req): Json<RejectDiscoveryRequest>) -> Result<Json<DiscoveryResultResponse>, AppError> {
    let svc = DiscoveryService::new(&state.db);
    Ok(Json(svc.reject_result(id, 1, &req.reason).await?))
}

pub async fn dashboard(State(state): State<SharedState>) -> Result<Json<serde_json::Value>, AppError> {
    let svc = DiscoveryService::new(&state.db);
    Ok(Json(svc.dashboard().await?))
}
