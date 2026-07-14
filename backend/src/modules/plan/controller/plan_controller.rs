//! SeaORM-based controller for the Plan domain.
//!
//! Uses `state.db` (SeaORM `DatabaseConnection`) instead of `state.db` (sqlx `PgPool`).

use axum::extract::{Json, Path, Query, State};
use validator::Validate;

use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::common::utils::helpers::PaginatedResponse;
use crate::modules::plan::request::plan_request::*;
use crate::modules::plan::response::plan_response::*;
use crate::modules::plan::service::plan_service::PlanService;

// ── Plan CRUD ───────────────────────────────────────────────

pub async fn list_plans(
    State(state): State<SharedState>,
    Query(query): Query<ListPlansQuery>,
) -> Result<Json<PaginatedResponse<PlanResponse>>, AppError> {
    let svc = PlanService::new(&state.db, &state.redis);
    Ok(Json(svc.list_plans(&query).await?))
}

pub async fn create_plan(
    State(state): State<SharedState>,
    Json(req): Json<CreatePlanRequest>,
) -> Result<Json<PlanResponse>, AppError> {
    req.validate()?;
    let svc = PlanService::new(&state.db, &state.redis);
    Ok(Json(svc.create_plan(&req).await?))
}

pub async fn get_plan(
    State(state): State<SharedState>,
    Path(id): Path<i64>,
) -> Result<Json<PlanResponse>, AppError> {
    let svc = PlanService::new(&state.db, &state.redis);
    Ok(Json(svc.get_plan(id).await?))
}

pub async fn update_plan(
    State(state): State<SharedState>,
    Path(id): Path<i64>,
    Json(req): Json<UpdatePlanRequest>,
) -> Result<Json<PlanResponse>, AppError> {
    req.validate()?;
    let svc = PlanService::new(&state.db, &state.redis);
    Ok(Json(svc.update_plan(id, &req).await?))
}

pub async fn delete_plan(
    State(state): State<SharedState>,
    Path(id): Path<i64>,
) -> Result<Json<MessageResponse>, AppError> {
    let svc = PlanService::new(&state.db, &state.redis);
    Ok(Json(svc.delete_plan(id).await?))
}

// ── Publish / Unpublish ────────────────────────────────────

pub async fn publish_plan(
    State(state): State<SharedState>,
    Path(id): Path<i64>,
) -> Result<Json<PlanResponse>, AppError> {
    let svc = PlanService::new(&state.db, &state.redis);
    Ok(Json(svc.publish_plan(id).await?))
}

pub async fn unpublish_plan(
    State(state): State<SharedState>,
    Path(id): Path<i64>,
) -> Result<Json<PlanResponse>, AppError> {
    let svc = PlanService::new(&state.db, &state.redis);
    Ok(Json(svc.unpublish_plan(id).await?))
}

// ── Clone ──────────────────────────────────────────────────

pub async fn clone_plan(
    State(state): State<SharedState>,
    Path(id): Path<i64>,
) -> Result<Json<PlanCloneResponse>, AppError> {
    let svc = PlanService::new(&state.db, &state.redis);
    Ok(Json(svc.clone_plan(id).await?))
}

// ── Speed Profiles ─────────────────────────────────────────

pub async fn get_speed_profile(
    State(state): State<SharedState>,
    Path(id): Path<i64>,
) -> Result<Json<SpeedProfileResponse>, AppError> {
    let svc = PlanService::new(&state.db, &state.redis);
    Ok(Json(svc.get_speed_profile(id).await?))
}

pub async fn create_speed_profile(
    State(state): State<SharedState>,
    Path(id): Path<i64>,
    Json(req): Json<CreateSpeedProfileRequest>,
) -> Result<Json<SpeedProfileResponse>, AppError> {
    req.validate()?;
    let svc = PlanService::new(&state.db, &state.redis);
    Ok(Json(svc.create_speed_profile(id, &req).await?))
}

pub async fn delete_speed_profile(
    State(state): State<SharedState>,
    Path(id): Path<i64>,
) -> Result<Json<MessageResponse>, AppError> {
    let svc = PlanService::new(&state.db, &state.redis);
    Ok(Json(svc.delete_speed_profile(id).await?))
}

// ── Plan Pricing ──────────────────────────────────────────

pub async fn list_plan_pricing(
    State(state): State<SharedState>,
    Path(id): Path<i64>,
) -> Result<Json<Vec<PlanPricingResponse>>, AppError> {
    let svc = PlanService::new(&state.db, &state.redis);
    Ok(Json(svc.list_pricing(id).await?))
}

pub async fn update_plan_pricing(
    State(state): State<SharedState>,
    Path(id): Path<i64>,
    Json(req): Json<UpdatePlanPricingRequest>,
) -> Result<Json<PlanPricingResponse>, AppError> {
    req.validate()?;
    let svc = PlanService::new(&state.db, &state.redis);
    Ok(Json(svc.update_pricing(id, &req).await?))
}
