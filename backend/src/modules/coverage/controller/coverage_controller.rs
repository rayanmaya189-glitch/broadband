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

pub async fn create_area(State(state): State<SharedState>, Json(req): Json<CreateCoverageAreaRequest>) -> Result<Json<CoverageAreaResponse>, AppError> {
    req.validate()?;
    let svc = CoverageService::new(&state.db);
    Ok(Json(svc.create_area(req).await?))
}

pub async fn check_availability(State(state): State<SharedState>, Json(req): Json<CheckAvailabilityRequest>) -> Result<Json<AvailabilityCheckResponse>, AppError> {
    let svc = CoverageService::new(&state.db);
    Ok(Json(svc.check_availability(req).await?))
}

pub async fn delete_area(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<MessageResponse>, AppError> {
    let svc = CoverageService::new(&state.db);
    Ok(Json(svc.delete_area(id).await?))
}
