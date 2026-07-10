use axum::extract::{Json, Path, Query, State};
use validator::Validate;

use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::modules::branch::request::branch_request::*;
use crate::modules::branch::response::branch_response::*;
use crate::modules::branch::service::branch_service_seaorm::BranchServiceSeaorm;

pub async fn list_branches(
    State(state): State<SharedState>,
    Query(query): Query<ListBranchesQuery>,
) -> Result<Json<crate::common::utils::helpers::PaginatedResponse<BranchResponse>>, AppError> {
    let svc = BranchServiceSeaorm::new(&state.db_seaorm);
    Ok(Json(svc.list_branches(&query).await?))
}

pub async fn create_branch(
    State(state): State<SharedState>,
    Json(req): Json<CreateBranchRequest>,
) -> Result<Json<BranchResponse>, AppError> {
    req.validate()?;
    let svc = BranchServiceSeaorm::new(&state.db_seaorm);
    Ok(Json(svc.create_branch(&req).await?))
}

pub async fn get_branch(
    State(state): State<SharedState>,
    Path(id): Path<i64>,
) -> Result<Json<BranchResponse>, AppError> {
    let svc = BranchServiceSeaorm::new(&state.db_seaorm);
    Ok(Json(svc.get_branch(id).await?))
}

pub async fn update_branch(
    State(state): State<SharedState>,
    Path(id): Path<i64>,
    Json(req): Json<UpdateBranchRequest>,
) -> Result<Json<BranchResponse>, AppError> {
    req.validate()?;
    let svc = BranchServiceSeaorm::new(&state.db_seaorm);
    Ok(Json(svc.update_branch(id, &req).await?))
}

pub async fn deactivate_branch(
    State(state): State<SharedState>,
    Path(id): Path<i64>,
) -> Result<Json<MessageResponse>, AppError> {
    let svc = BranchServiceSeaorm::new(&state.db_seaorm);
    Ok(Json(svc.deactivate_branch(id).await?))
}

// ── Working Hours ──────────────────────────────────────────

pub async fn get_working_hours(
    State(state): State<SharedState>,
    Path(id): Path<i64>,
) -> Result<Json<Vec<WorkingHourResponse>>, AppError> {
    let svc = BranchServiceSeaorm::new(&state.db_seaorm);
    Ok(Json(svc.get_working_hours(id).await?))
}

pub async fn update_working_hours(
    State(state): State<SharedState>,
    Path(id): Path<i64>,
    Json(req): Json<UpdateWorkingHoursRequest>,
) -> Result<Json<Vec<WorkingHourResponse>>, AppError> {
    let svc = BranchServiceSeaorm::new(&state.db_seaorm);
    Ok(Json(svc.update_working_hours(id, &req).await?))
}

// ── User-Branch Assignment ─────────────────────────────────

pub async fn assign_user(
    State(state): State<SharedState>,
    Path(id): Path<i64>,
    Json(req): Json<AssignUserToBranchRequest>,
) -> Result<Json<MessageResponse>, AppError> {
    let svc = BranchServiceSeaorm::new(&state.db_seaorm);
    Ok(Json(svc.assign_user(id, &req).await?))
}

pub async fn remove_user(
    State(state): State<SharedState>,
    Path((id, uid)): Path<(i64, i64)>,
) -> Result<Json<MessageResponse>, AppError> {
    let svc = BranchServiceSeaorm::new(&state.db_seaorm);
    Ok(Json(svc.remove_user(id, uid).await?))
}

pub async fn list_branch_users(
    State(state): State<SharedState>,
    Path(id): Path<i64>,
) -> Result<Json<Vec<BranchUserResponse>>, AppError> {
    let svc = BranchServiceSeaorm::new(&state.db_seaorm);
    Ok(Json(svc.list_branch_users(id).await?))
}

pub async fn get_branch_stats(
    State(state): State<SharedState>,
    Path(id): Path<i64>,
) -> Result<Json<crate::modules::branch::response::branch_response::BranchStatsResponse>, AppError> {
    let svc = BranchServiceSeaorm::new(&state.db_seaorm);
    Ok(Json(svc.get_branch_stats(id).await?))
}
