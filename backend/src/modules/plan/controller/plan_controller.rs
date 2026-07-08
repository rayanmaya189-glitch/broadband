use axum::extract::{Json, Path, Query, State};
use validator::Validate;

use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::modules::plan::request::plan_request::*;
use crate::modules::plan::response::plan_response::*;
use crate::modules::plan::service::plan_service::PlanService;

pub async fn list_plans(State(state): State<SharedState>, Query(query): Query<ListPlansQuery>) -> Result<Json<crate::common::utils::helpers::PaginatedResponse<PlanResponse>>, AppError> {
    let svc = PlanService::new(&state.db);
    Ok(Json(svc.list_plans(&query).await?))
}

pub async fn create_plan(State(state): State<SharedState>, Json(req): Json<CreatePlanRequest>) -> Result<Json<PlanResponse>, AppError> {
    req.validate()?;
    let svc = PlanService::new(&state.db);
    Ok(Json(svc.create_plan(&req).await?))
}

pub async fn get_plan(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<PlanResponse>, AppError> {
    let svc = PlanService::new(&state.db);
    Ok(Json(svc.get_plan(id).await?))
}

pub async fn update_plan(State(state): State<SharedState>, Path(id): Path<i64>, Json(req): Json<UpdatePlanRequest>) -> Result<Json<PlanResponse>, AppError> {
    req.validate()?;
    let svc = PlanService::new(&state.db);
    Ok(Json(svc.update_plan(id, &req).await?))
}

pub async fn delete_plan(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<MessageResponse>, AppError> {
    let svc = PlanService::new(&state.db);
    Ok(Json(svc.delete_plan(id).await?))
}
