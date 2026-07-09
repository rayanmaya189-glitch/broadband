use axum::extract::{Json, Path, Query, State};
use validator::Validate;
use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::modules::coverage::request::coverage_request::*;
use crate::modules::coverage::response::coverage_response::*;
use crate::modules::coverage::service::coverage_service::CoverageService;

pub async fn list_areas(State(state): State<SharedState>, Query(q): Query<CoverageQuery>) -> Result<Json<Vec<CoverageAreaResponse>>, AppError> {
    let svc = CoverageService::new(&state.db);
    Ok(Json(svc.list_areas(q.branch_id).await?))
}

pub async fn get_area(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<CoverageAreaResponse>, AppError> {
    let svc = CoverageService::new(&state.db);
    Ok(Json(svc.get_area(id).await?))
}

pub async fn create_area(State(state): State<SharedState>, Json(req): Json<CreateCoverageAreaRequest>) -> Result<Json<CoverageAreaResponse>, AppError> {
    req.validate()?;
    let svc = CoverageService::new(&state.db);
    Ok(Json(svc.create_area(req).await?))
}

pub async fn update_area(State(state): State<SharedState>, Path(id): Path<i64>, Json(req): Json<UpdateCoverageAreaRequest>) -> Result<Json<CoverageAreaResponse>, AppError> {
    let svc = CoverageService::new(&state.db);
    Ok(Json(svc.update_area(id, req).await?))
}

pub async fn check_availability(State(state): State<SharedState>, Json(req): Json<CheckAvailabilityRequest>) -> Result<Json<AvailabilityCheckResponse>, AppError> {
    let svc = CoverageService::new(&state.db);
    Ok(Json(svc.check_availability(req).await?))
}

pub async fn delete_area(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<MessageResponse>, AppError> {
    let svc = CoverageService::new(&state.db);
    Ok(Json(svc.delete_area(id).await?))
}

// ── Pincode Management ──────────────────────────────────

pub async fn list_pincodes(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<Vec<CoveragePincodeResponse>>, AppError> {
    let svc = CoverageService::new(&state.db);
    Ok(Json(svc.list_pincodes(id).await?))
}

pub async fn add_pincode(State(state): State<SharedState>, Path(id): Path<i64>, Json(req): Json<AddPincodeRequest>) -> Result<Json<CoveragePincodeResponse>, AppError> {
    let svc = CoverageService::new(&state.db);
    Ok(Json(svc.add_pincode(id, req).await?))
}

pub async fn remove_pincode(State(state): State<SharedState>, Path((id, pincode)): Path<(i64, String)>) -> Result<Json<MessageResponse>, AppError> {
    let svc = CoverageService::new(&state.db);
    Ok(Json(svc.remove_pincode(id, &pincode).await?))
}

// ── Stats ───────────────────────────────────────────────

pub async fn get_stats(State(state): State<SharedState>) -> Result<Json<CoverageStatsResponse>, AppError> {
    let svc = CoverageService::new(&state.db);
    Ok(Json(svc.get_stats().await?))
}
