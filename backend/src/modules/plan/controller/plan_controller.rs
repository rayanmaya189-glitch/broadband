use axum::extract::{Json, Path, Query, State};
use validator::Validate;

use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::modules::plan::request::plan_request::*;
use crate::modules::plan::response::plan_response::*;
use crate::modules::plan::service::plan_service::PlanService;

// ── Plan CRUD ───────────────────────────────────────────────

#[utoipa::path(
    get,
    path = "/api/v1/plans",
    tag = "Plans",
    params(
        ("page" = Option<i64>, Query, description = "Page number"),
        ("limit" = Option<i64>, Query, description = "Items per page"),
        ("active_only" = Option<bool>, Query, description = "Filter active plans only")
    ),
    responses(
        (status = 200, description = "List of plans"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn list_plans(State(state): State<SharedState>, Query(query): Query<ListPlansQuery>) -> Result<Json<crate::common::utils::helpers::PaginatedResponse<PlanResponse>>, AppError> {
    let svc = PlanService::new(&state.db, &state.redis);
    Ok(Json(svc.list_plans(&query).await?))
}

#[utoipa::path(
    post,
    path = "/api/v1/plans",
    tag = "Plans",
    security(("bearer_auth" = [])),
    request_body = CreatePlanRequest,
    responses(
        (status = 200, description = "Plan created", body = PlanResponse),
        (status = 409, description = "Plan slug already exists"),
        (status = 422, description = "Validation error")
    )
)]
pub async fn create_plan(State(state): State<SharedState>, Json(req): Json<CreatePlanRequest>) -> Result<Json<PlanResponse>, AppError> {
    req.validate()?;
    let svc = PlanService::new(&state.db, &state.redis);
    Ok(Json(svc.create_plan(&req).await?))
}

#[utoipa::path(
    get,
    path = "/api/v1/plans/{id}",
    tag = "Plans",
    params(("id" = i64, Path, description = "Plan ID")),
    responses(
        (status = 200, description = "Plan details", body = PlanResponse),
        (status = 404, description = "Plan not found")
    )
)]
pub async fn get_plan(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<PlanResponse>, AppError> {
    let svc = PlanService::new(&state.db, &state.redis);
    Ok(Json(svc.get_plan(id).await?))
}

#[utoipa::path(
    put,
    path = "/api/v1/plans/{id}",
    tag = "Plans",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Plan ID")),
    request_body = UpdatePlanRequest,
    responses(
        (status = 200, description = "Plan updated", body = PlanResponse),
        (status = 404, description = "Plan not found"),
        (status = 422, description = "Validation error")
    )
)]
pub async fn update_plan(State(state): State<SharedState>, Path(id): Path<i64>, Json(req): Json<UpdatePlanRequest>) -> Result<Json<PlanResponse>, AppError> {
    req.validate()?;
    let svc = PlanService::new(&state.db, &state.redis);
    Ok(Json(svc.update_plan(id, &req).await?))
}

#[utoipa::path(
    delete,
    path = "/api/v1/plans/{id}",
    tag = "Plans",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Plan ID")),
    responses(
        (status = 200, description = "Plan deleted"),
        (status = 404, description = "Plan not found")
    )
)]
pub async fn delete_plan(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<MessageResponse>, AppError> {
    let svc = PlanService::new(&state.db, &state.redis);
    Ok(Json(svc.delete_plan(id).await?))
}

// ── Publish / Unpublish ────────────────────────────────────

#[utoipa::path(
    post,
    path = "/api/v1/plans/{id}/publish",
    tag = "Plans",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Plan ID")),
    responses(
        (status = 200, description = "Plan published", body = PlanResponse),
        (status = 404, description = "Plan not found")
    )
)]
pub async fn publish_plan(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<PlanResponse>, AppError> {
    let svc = PlanService::new(&state.db, &state.redis);
    Ok(Json(svc.publish_plan(id).await?))
}

#[utoipa::path(
    post,
    path = "/api/v1/plans/{id}/unpublish",
    tag = "Plans",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Plan ID")),
    responses(
        (status = 200, description = "Plan unpublished", body = PlanResponse),
        (status = 404, description = "Plan not found")
    )
)]
pub async fn unpublish_plan(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<PlanResponse>, AppError> {
    let svc = PlanService::new(&state.db, &state.redis);
    Ok(Json(svc.unpublish_plan(id).await?))
}

// ── Clone ──────────────────────────────────────────────────

#[utoipa::path(
    post,
    path = "/api/v1/plans/{id}/clone",
    tag = "Plans",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Plan ID to clone")),
    responses(
        (status = 200, description = "Plan cloned", body = PlanCloneResponse),
        (status = 404, description = "Plan not found")
    )
)]
pub async fn clone_plan(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<PlanCloneResponse>, AppError> {
    let svc = PlanService::new(&state.db, &state.redis);
    Ok(Json(svc.clone_plan(id).await?))
}

// ── Speed Profiles ─────────────────────────────────────────

#[utoipa::path(
    get,
    path = "/api/v1/plans/{id}/speed-profile",
    tag = "Plans",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Plan ID")),
    responses(
        (status = 200, description = "Speed profile", body = SpeedProfileResponse),
        (status = 404, description = "Speed profile not found")
    )
)]
pub async fn get_speed_profile(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<SpeedProfileResponse>, AppError> {
    let svc = PlanService::new(&state.db, &state.redis);
    Ok(Json(svc.get_speed_profile(id).await?))
}

#[utoipa::path(
    post,
    path = "/api/v1/plans/{id}/speed-profile",
    tag = "Plans",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Plan ID")),
    request_body = CreateSpeedProfileRequest,
    responses(
        (status = 200, description = "Speed profile created/updated", body = SpeedProfileResponse),
        (status = 404, description = "Plan not found"),
        (status = 422, description = "Validation error")
    )
)]
pub async fn create_speed_profile(State(state): State<SharedState>, Path(id): Path<i64>, Json(req): Json<CreateSpeedProfileRequest>) -> Result<Json<SpeedProfileResponse>, AppError> {
    req.validate()?;
    let svc = PlanService::new(&state.db, &state.redis);
    Ok(Json(svc.create_speed_profile(id, &req).await?))
}

#[utoipa::path(
    post,
    path = "/api/v1/plans/{id}/speed-profile/delete",
    tag = "Plans",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Plan ID")),
    responses(
        (status = 200, description = "Speed profile deleted"),
        (status = 404, description = "Speed profile not found")
    )
)]
pub async fn delete_speed_profile(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<MessageResponse>, AppError> {
    let svc = PlanService::new(&state.db, &state.redis);
    Ok(Json(svc.delete_speed_profile(id).await?))
}

// ── Plan Pricing ──────────────────────────────────────────

#[utoipa::path(
    get,
    path = "/api/v1/plans/{id}/pricing",
    tag = "Plans",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Plan ID")),
    responses(
        (status = 200, description = "List of pricing tiers", body = Vec<PlanPricingResponse>),
        (status = 404, description = "Plan not found")
    )
)]
pub async fn list_plan_pricing(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<Vec<PlanPricingResponse>>, AppError> {
    let svc = PlanService::new(&state.db, &state.redis);
    Ok(Json(svc.list_pricing(id).await?))
}

#[utoipa::path(
    put,
    path = "/api/v1/plans/{id}/pricing",
    tag = "Plans",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Plan ID")),
    request_body = UpdatePlanPricingRequest,
    responses(
        (status = 200, description = "Pricing updated", body = PlanPricingResponse),
        (status = 404, description = "Plan not found"),
        (status = 422, description = "Validation error")
    )
)]
pub async fn update_plan_pricing(State(state): State<SharedState>, Path(id): Path<i64>, Json(req): Json<UpdatePlanPricingRequest>) -> Result<Json<PlanPricingResponse>, AppError> {
    req.validate()?;
    let svc = PlanService::new(&state.db, &state.redis);
    Ok(Json(svc.update_pricing(id, &req).await?))
}
