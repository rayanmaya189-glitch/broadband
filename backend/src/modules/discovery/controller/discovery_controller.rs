use axum::extract::{Json, Path, Query, State};
use validator::Validate;
use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::common::middleware::auth_middleware::UserContext;
use crate::modules::discovery::request::discovery_request::*;
use crate::modules::discovery::response::discovery_response::*;
use crate::modules::discovery::service::discovery_service::DiscoveryService;

// ── Scans ───────────────────────────────────────────────────

#[utoipa::path(
    get,
    path = "/api/v1/discovery/scans",
    tag = "Discovery",
    security(("bearer_auth" = [])),
    params(("branch_id" = Option<i64>, Query, description = "Filter by branch")),
    responses(
        (status = 200, description = "List of scans", body = Vec<ScanResponse>),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn list_scans(State(state): State<SharedState>, Query(q): Query<DiscoveryQuery>) -> Result<Json<Vec<ScanResponse>>, AppError> {
    let svc = DiscoveryService::new(&state.db);
    Ok(Json(svc.list_scans(q.branch_id).await?))
}

#[utoipa::path(
    post,
    path = "/api/v1/discovery/scans",
    tag = "Discovery",
    security(("bearer_auth" = [])),
    request_body = CreateScanRequest,
    responses(
        (status = 200, description = "Scan created", body = ScanResponse),
        (status = 422, description = "Validation error")
    )
)]
pub async fn create_scan(State(state): State<SharedState>, Json(req): Json<CreateScanRequest>) -> Result<Json<ScanResponse>, AppError> {
    req.validate()?;
    let svc = DiscoveryService::new(&state.db);
    Ok(Json(svc.create_scan(req).await?))
}

#[utoipa::path(
    post,
    path = "/api/v1/discovery/scans/{id}/start",
    tag = "Discovery",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Scan ID")),
    responses(
        (status = 200, description = "Scan started"),
        (status = 404, description = "Scan not found")
    )
)]
pub async fn start_scan(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<MessageResponse>, AppError> {
    let svc = DiscoveryService::new(&state.db);
    Ok(Json(svc.start_scan(id).await?))
}

#[utoipa::path(
    post,
    path = "/api/v1/discovery/scans/{id}/stop",
    tag = "Discovery",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Scan ID")),
    responses(
        (status = 200, description = "Scan stopped"),
        (status = 404, description = "Scan not found")
    )
)]
pub async fn stop_scan(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<MessageResponse>, AppError> {
    let svc = DiscoveryService::new(&state.db);
    Ok(Json(svc.stop_scan(id).await?))
}

// ── Results ─────────────────────────────────────────────────

#[utoipa::path(
    get,
    path = "/api/v1/discovery/results",
    tag = "Discovery",
    security(("bearer_auth" = [])),
    params(
        ("branch_id" = Option<i64>, Query, description = "Filter by branch"),
        ("status" = Option<String>, Query, description = "Filter by status")
    ),
    responses(
        (status = 200, description = "List of results", body = Vec<ResultResponse>),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn list_results(State(state): State<SharedState>, Query(q): Query<DiscoveryQuery>) -> Result<Json<Vec<ResultResponse>>, AppError> {
    let svc = DiscoveryService::new(&state.db);
    Ok(Json(svc.list_results(q).await?))
}

#[utoipa::path(
    post,
    path = "/api/v1/discovery/results/{id}/approve",
    tag = "Discovery",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Result ID")),
    responses(
        (status = 200, description = "Result approved", body = ResultResponse),
        (status = 404, description = "Result not found")
    )
)]
pub async fn approve_result(State(state): State<SharedState>, user: UserContext, Path(id): Path<i64>) -> Result<Json<ResultResponse>, AppError> {
    let svc = DiscoveryService::new(&state.db);
    Ok(Json(svc.approve_result(id, user.user_id).await?))
}

#[utoipa::path(
    post,
    path = "/api/v1/discovery/results/{id}/reject",
    tag = "Discovery",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Result ID")),
    request_body = RejectRequest,
    responses(
        (status = 200, description = "Result rejected", body = ResultResponse),
        (status = 404, description = "Result not found")
    )
)]
pub async fn reject_result(State(state): State<SharedState>, user: UserContext, Path(id): Path<i64>, Json(req): Json<RejectRequest>) -> Result<Json<ResultResponse>, AppError> {
    req.validate()?;
    let svc = DiscoveryService::new(&state.db);
    Ok(Json(svc.reject_result(id, user.user_id, &req.reason).await?))
}

// ── Dashboard ───────────────────────────────────────────────

#[utoipa::path(
    get,
    path = "/api/v1/discovery/dashboard",
    tag = "Discovery",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Discovery dashboard", body = DashboardResponse),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn dashboard(State(state): State<SharedState>) -> Result<Json<DashboardResponse>, AppError> {
    let svc = DiscoveryService::new(&state.db);
    Ok(Json(svc.get_dashboard().await?))
}
