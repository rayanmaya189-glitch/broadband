use axum::extract::{Json, Path, Query, State};
use validator::Validate;
use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::modules::coverage::request::coverage_request::*;
use crate::modules::coverage::response::coverage_response::*;
use crate::modules::coverage::service::coverage_service::CoverageService;

#[utoipa::path(
    get,
    path = "/api/v1/coverage/areas",
    tag = "Coverage",
    security(("bearer_auth" = [])),
    params(("branch_id" = Option<i64>, Query, description = "Filter by branch")),
    responses(
        (status = 200, description = "List of coverage areas", body = Vec<CoverageAreaResponse>),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn list_areas(State(state): State<SharedState>, Query(q): Query<CoverageQuery>) -> Result<Json<Vec<CoverageAreaResponse>>, AppError> {
    let svc = CoverageService::new(&state.db);
    Ok(Json(svc.list_areas(q.branch_id).await?))
}

#[utoipa::path(
    get,
    path = "/api/v1/coverage/areas/{id}",
    tag = "Coverage",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Area ID")),
    responses(
        (status = 200, description = "Coverage area details", body = CoverageAreaResponse),
        (status = 404, description = "Area not found")
    )
)]
pub async fn get_area(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<CoverageAreaResponse>, AppError> {
    let svc = CoverageService::new(&state.db);
    Ok(Json(svc.get_area(id).await?))
}

#[utoipa::path(
    post,
    path = "/api/v1/coverage/areas",
    tag = "Coverage",
    security(("bearer_auth" = [])),
    request_body = CreateCoverageAreaRequest,
    responses(
        (status = 200, description = "Coverage area created", body = CoverageAreaResponse),
        (status = 422, description = "Validation error")
    )
)]
pub async fn create_area(State(state): State<SharedState>, Json(req): Json<CreateCoverageAreaRequest>) -> Result<Json<CoverageAreaResponse>, AppError> {
    req.validate()?;
    let svc = CoverageService::new(&state.db);
    Ok(Json(svc.create_area(req).await?))
}

#[utoipa::path(
    put,
    path = "/api/v1/coverage/areas/{id}",
    tag = "Coverage",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Area ID")),
    request_body = UpdateCoverageAreaRequest,
    responses(
        (status = 200, description = "Coverage area updated", body = CoverageAreaResponse),
        (status = 404, description = "Area not found")
    )
)]
pub async fn update_area(State(state): State<SharedState>, Path(id): Path<i64>, Json(req): Json<UpdateCoverageAreaRequest>) -> Result<Json<CoverageAreaResponse>, AppError> {
    let svc = CoverageService::new(&state.db);
    Ok(Json(svc.update_area(id, req).await?))
}

#[utoipa::path(
    post,
    path = "/api/v1/coverage/check",
    tag = "Coverage",
    security(("bearer_auth" = [])),
    request_body = CheckAvailabilityRequest,
    responses(
        (status = 200, description = "Availability check result", body = AvailabilityCheckResponse),
        (status = 422, description = "Validation error")
    )
)]
pub async fn check_availability(State(state): State<SharedState>, Json(req): Json<CheckAvailabilityRequest>) -> Result<Json<AvailabilityCheckResponse>, AppError> {
    let svc = CoverageService::new(&state.db);
    Ok(Json(svc.check_availability(req).await?))
}

#[utoipa::path(
    delete,
    path = "/api/v1/coverage/areas/{id}",
    tag = "Coverage",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Area ID")),
    responses(
        (status = 200, description = "Coverage area deleted"),
        (status = 404, description = "Area not found")
    )
)]
pub async fn delete_area(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<MessageResponse>, AppError> {
    let svc = CoverageService::new(&state.db);
    Ok(Json(svc.delete_area(id).await?))
}

// ── Pincode Management ──────────────────────────────────

#[utoipa::path(
    get,
    path = "/api/v1/coverage/areas/{id}/pincodes",
    tag = "Coverage",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Area ID")),
    responses(
        (status = 200, description = "List of pincodes", body = Vec<CoveragePincodeResponse>),
        (status = 404, description = "Area not found")
    )
)]
pub async fn list_pincodes(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<Vec<CoveragePincodeResponse>>, AppError> {
    let svc = CoverageService::new(&state.db);
    Ok(Json(svc.list_pincodes(id).await?))
}

#[utoipa::path(
    post,
    path = "/api/v1/coverage/areas/{id}/pincodes",
    tag = "Coverage",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Area ID")),
    request_body = AddPincodeRequest,
    responses(
        (status = 200, description = "Pincode added", body = CoveragePincodeResponse),
        (status = 404, description = "Area not found")
    )
)]
pub async fn add_pincode(State(state): State<SharedState>, Path(id): Path<i64>, Json(req): Json<AddPincodeRequest>) -> Result<Json<CoveragePincodeResponse>, AppError> {
    let svc = CoverageService::new(&state.db);
    Ok(Json(svc.add_pincode(id, req).await?))
}

#[utoipa::path(
    delete,
    path = "/api/v1/coverage/areas/{id}/pincodes/{pincode}",
    tag = "Coverage",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Area ID"), ("pincode" = String, Path, description = "Pincode")),
    responses(
        (status = 200, description = "Pincode removed"),
        (status = 404, description = "Pincode not found")
    )
)]
pub async fn remove_pincode(State(state): State<SharedState>, Path((id, pincode)): Path<(i64, String)>) -> Result<Json<MessageResponse>, AppError> {
    let svc = CoverageService::new(&state.db);
    Ok(Json(svc.remove_pincode(id, &pincode).await?))
}

// ── Stats ───────────────────────────────────────────────

#[utoipa::path(
    get,
    path = "/api/v1/coverage/stats",
    tag = "Coverage",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Coverage statistics", body = CoverageStatsResponse),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn get_stats(State(state): State<SharedState>) -> Result<Json<CoverageStatsResponse>, AppError> {
    let svc = CoverageService::new(&state.db);
    Ok(Json(svc.get_stats().await?))
}
