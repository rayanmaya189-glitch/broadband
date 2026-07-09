use axum::extract::{Json, Path, Query, State};
use validator::Validate;
use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::common::middleware::auth_middleware::UserContext;
use crate::modules::discovery::request::discovery_request::*;
use crate::modules::discovery::response::discovery_response::*;
use crate::modules::discovery::service::discovery_service::DiscoveryService;

// ── Scans ───────────────────────────────────────────────────

pub async fn list_scans(State(state): State<SharedState>, Query(q): Query<DiscoveryQuery>) -> Result<Json<Vec<ScanResponse>>, AppError> {
    let svc = DiscoveryService::new(&state.db);
    Ok(Json(svc.list_scans(q.branch_id).await?))
}

pub async fn create_scan(State(state): State<SharedState>, Json(req): Json<CreateScanRequest>) -> Result<Json<ScanResponse>, AppError> {
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

// ── Results ─────────────────────────────────────────────────

pub async fn list_results(State(state): State<SharedState>, Query(q): Query<DiscoveryQuery>) -> Result<Json<Vec<ResultResponse>>, AppError> {
    let svc = DiscoveryService::new(&state.db);
    Ok(Json(svc.list_results(q).await?))
}

pub async fn approve_result(State(state): State<SharedState>, user: UserContext, Path(id): Path<i64>) -> Result<Json<ResultResponse>, AppError> {
    let svc = DiscoveryService::new(&state.db);
    Ok(Json(svc.approve_result(id, user.user_id).await?))
}

pub async fn reject_result(State(state): State<SharedState>, user: UserContext, Path(id): Path<i64>, Json(req): Json<RejectRequest>) -> Result<Json<ResultResponse>, AppError> {
    req.validate()?;
    let svc = DiscoveryService::new(&state.db);
    Ok(Json(svc.reject_result(id, user.user_id, &req.reason).await?))
}

// ── Dashboard ───────────────────────────────────────────────

pub async fn dashboard(State(state): State<SharedState>) -> Result<Json<DashboardResponse>, AppError> {
    let svc = DiscoveryService::new(&state.db);
    Ok(Json(svc.get_dashboard().await?))
}
