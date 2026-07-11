use axum::extract::{Json, Path, Query, State};

use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::common::utils::helpers::PaginatedResponse;
use crate::modules::plan::request::plan_request::*;
use crate::modules::plan::response::plan_response::*;
use crate::modules::plan::service::plan_service::PlanService;

/// List all plans (admin: including unpublished).
pub async fn list_plans(
    State(state): State<SharedState>,
    Query(query): Query<ListPlansQuery>,
) -> Result<Json<PaginatedResponse<PlanResponse>>, AppError> {
    let svc = PlanService::new(&state.db_seaorm, &state.redis);
    Ok(Json(svc.list_plans(&query).await?))
}

/// Get plan by ID (admin: any plan).
pub async fn get_plan(
    State(state): State<SharedState>,
    Path(id): Path<i64>,
) -> Result<Json<PlanDetailResponse>, AppError> {
    let svc = PlanService::new(&state.db_seaorm, &state.redis);
    Ok(Json(svc.get_plan(id).await?))
}

/// Create a new plan (admin).
pub async fn create_plan(
    State(state): State<SharedState>,
    Json(req): Json<CreatePlanRequest>,
) -> Result<Json<PlanDetailResponse>, AppError> {
    let svc = PlanService::new(&state.db_seaorm, &state.redis);
    Ok(Json(svc.create_plan(&req).await?))
}

/// Update a plan (admin).
pub async fn update_plan(
    State(state): State<SharedState>,
    Path(id): Path<i64>,
    Json(req): Json<UpdatePlanRequest>,
) -> Result<Json<PlanDetailResponse>, AppError> {
    let svc = PlanService::new(&state.db_seaorm, &state.redis);
    Ok(Json(svc.update_plan(id, &req).await?))
}

/// Delete/deactivate a plan (admin).
pub async fn delete_plan(
    State(state): State<SharedState>,
    Path(id): Path<i64>,
) -> Result<Json<MessageResponse>, AppError> {
    let svc = PlanService::new(&state.db_seaorm, &state.redis);
    Ok(Json(svc.delete_plan(id).await?))
}

/// Publish a plan (admin).
pub async fn publish_plan(
    State(state): State<SharedState>,
    Path(id): Path<i64>,
) -> Result<Json<PlanDetailResponse>, AppError> {
    let svc = PlanService::new(&state.db_seaorm, &state.redis);
    Ok(Json(svc.publish_plan(id).await?))
}

/// Unpublish a plan (admin).
pub async fn unpublish_plan(
    State(state): State<SharedState>,
    Path(id): Path<i64>,
) -> Result<Json<PlanDetailResponse>, AppError> {
    let svc = PlanService::new(&state.db_seaorm, &state.redis);
    Ok(Json(svc.unpublish_plan(id).await?))
}

/// Clone a plan (admin).
pub async fn clone_plan(
    State(state): State<SharedState>,
    Path(id): Path<i64>,
) -> Result<Json<PlanCloneResponse>, AppError> {
    let svc = PlanService::new(&state.db_seaorm, &state.redis);
    Ok(Json(svc.clone_plan(id).await?))
}

/// Get speed profile for a plan (admin).
pub async fn get_speed_profile(
    State(state): State<SharedState>,
    Path(id): Path<i64>,
) -> Result<Json<SpeedProfileResponse>, AppError> {
    let svc = PlanService::new(&state.db_seaorm, &state.redis);
    Ok(Json(svc.get_speed_profile(id).await?))
}

/// Create/update speed profile (admin).
pub async fn create_speed_profile(
    State(state): State<SharedState>,
    Path(id): Path<i64>,
    Json(req): Json<CreateSpeedProfileRequest>,
) -> Result<Json<SpeedProfileResponse>, AppError> {
    let svc = PlanService::new(&state.db_seaorm, &state.redis);
    Ok(Json(svc.create_speed_profile(id, &req).await?))
}

/// Delete speed profile (admin).
pub async fn delete_speed_profile(
    State(state): State<SharedState>,
    Path(id): Path<i64>,
) -> Result<Json<MessageResponse>, AppError> {
    let svc = PlanService::new(&state.db_seaorm, &state.redis);
    Ok(Json(svc.delete_speed_profile(id).await?))
}

/// List plan pricing (admin).
pub async fn list_plan_pricing(
    State(state): State<SharedState>,
    Path(id): Path<i64>,
) -> Result<Json<Vec<PlanPricingResponse>>, AppError> {
    let svc = PlanService::new(&state.db_seaorm, &state.redis);
    Ok(Json(svc.list_pricing(id).await?))
}

/// Update plan pricing (admin).
pub async fn update_plan_pricing(
    State(state): State<SharedState>,
    Path(id): Path<i64>,
    Json(req): Json<UpdatePlanPricingRequest>,
) -> Result<Json<PlanPricingResponse>, AppError> {
    let svc = PlanService::new(&state.db_seaorm, &state.redis);
    Ok(Json(svc.update_pricing(id, &req).await?))
}
