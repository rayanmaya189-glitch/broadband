use axum::extract::{Json, Path, State};

use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::common::utils::helpers::{PaginatedResponse, PaginationParams};
use crate::modules::plan::request::plan_request::ListPlansQuery;
use crate::modules::plan::response::plan_response::*;
use crate::modules::plan::service::plan_service::PlanService;

/// View available (published) plans.
pub async fn list_plans(
    State(state): State<SharedState>,
) -> Result<Json<PaginatedResponse<PlanResponse>>, AppError> {
    let svc = PlanService::new(&state.db_seaorm, &state.redis);
    let query = ListPlansQuery {
        pagination: PaginationParams { page: 1, limit: 50, sort_by: None, sort_order: None, search: None },
        is_active: Some(true),
        category: None,
    };
    Ok(Json(svc.list_plans(&query).await?))
}

/// Get a single published plan.
pub async fn get_plan(
    State(state): State<SharedState>,
    Path(id): Path<i64>,
) -> Result<Json<PlanDetailResponse>, AppError> {
    let svc = PlanService::new(&state.db_seaorm, &state.redis);
    let plan = svc.get_plan(id).await?;
    if !plan.is_active {
        return Err(AppError::NotFound("Plan not found".into()));
    }
    Ok(Json(plan))
}
