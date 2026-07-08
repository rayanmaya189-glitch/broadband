use axum::extract::{Json, Path, Query, State};
use validator::Validate;

use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::modules::branch::request::branch_request::*;
use crate::modules::branch::response::branch_response::*;
use crate::modules::branch::service::branch_service::BranchService;

pub async fn list_branches(State(state): State<SharedState>, Query(query): Query<ListBranchesQuery>) -> Result<Json<crate::common::utils::helpers::PaginatedResponse<BranchResponse>>, AppError> {
    let svc = BranchService::new(&state.db);
    Ok(Json(svc.list_branches(&query).await?))
}

pub async fn create_branch(State(state): State<SharedState>, Json(req): Json<CreateBranchRequest>) -> Result<Json<BranchResponse>, AppError> {
    req.validate()?;
    let svc = BranchService::new(&state.db);
    Ok(Json(svc.create_branch(&req).await?))
}

pub async fn get_branch(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<BranchResponse>, AppError> {
    let svc = BranchService::new(&state.db);
    Ok(Json(svc.get_branch(id).await?))
}

pub async fn update_branch(State(state): State<SharedState>, Path(id): Path<i64>, Json(req): Json<UpdateBranchRequest>) -> Result<Json<BranchResponse>, AppError> {
    req.validate()?;
    let svc = BranchService::new(&state.db);
    Ok(Json(svc.update_branch(id, &req).await?))
}

pub async fn deactivate_branch(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<MessageResponse>, AppError> {
    let svc = BranchService::new(&state.db);
    Ok(Json(svc.deactivate_branch(id).await?))
}
