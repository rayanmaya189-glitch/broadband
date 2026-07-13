use axum::extract::{Json, Path, Query, State};
use validator::Validate;

use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::modules::branch::request::branch_request::*;
use crate::modules::branch::response::branch_response::*;
use crate::modules::branch::service::branch_service::BranchService;

#[utoipa::path(
    get,
    path = "/api/v1/branches",
    tag = "Branches",
    security(("bearer_auth" = [])),
    params(
        ("page" = Option<i64>, Query, description = "Page number"),
        ("limit" = Option<i64>, Query, description = "Items per page")
    ),
    responses(
        (status = 200, description = "List of branches"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn list_branches(State(state): State<SharedState>, Query(query): Query<ListBranchesQuery>) -> Result<Json<crate::common::utils::helpers::PaginatedResponse<BranchResponse>>, AppError> {
    let svc = BranchService::new(&state.db);
    Ok(Json(svc.list_branches(&query).await?))
}

#[utoipa::path(
    post,
    path = "/api/v1/branches",
    tag = "Branches",
    security(("bearer_auth" = [])),
    request_body = CreateBranchRequest,
    responses(
        (status = 200, description = "Branch created", body = BranchResponse),
        (status = 409, description = "Branch already exists"),
        (status = 422, description = "Validation error")
    )
)]
pub async fn create_branch(State(state): State<SharedState>, Json(req): Json<CreateBranchRequest>) -> Result<Json<BranchResponse>, AppError> {
    req.validate()?;
    let svc = BranchService::new(&state.db);
    Ok(Json(svc.create_branch(&req).await?))
}

#[utoipa::path(
    get,
    path = "/api/v1/branches/{id}",
    tag = "Branches",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Branch ID")),
    responses(
        (status = 200, description = "Branch details", body = BranchResponse),
        (status = 404, description = "Branch not found")
    )
)]
pub async fn get_branch(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<BranchResponse>, AppError> {
    let svc = BranchService::new(&state.db);
    Ok(Json(svc.get_branch(id).await?))
}

#[utoipa::path(
    put,
    path = "/api/v1/branches/{id}",
    tag = "Branches",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Branch ID")),
    request_body = UpdateBranchRequest,
    responses(
        (status = 200, description = "Branch updated", body = BranchResponse),
        (status = 404, description = "Branch not found"),
        (status = 422, description = "Validation error")
    )
)]
pub async fn update_branch(State(state): State<SharedState>, Path(id): Path<i64>, Json(req): Json<UpdateBranchRequest>) -> Result<Json<BranchResponse>, AppError> {
    req.validate()?;
    let svc = BranchService::new(&state.db);
    Ok(Json(svc.update_branch(id, &req).await?))
}

#[utoipa::path(
    delete,
    path = "/api/v1/branches/{id}",
    tag = "Branches",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Branch ID")),
    responses(
        (status = 200, description = "Branch deactivated"),
        (status = 404, description = "Branch not found")
    )
)]
pub async fn deactivate_branch(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<MessageResponse>, AppError> {
    let svc = BranchService::new(&state.db);
    Ok(Json(svc.deactivate_branch(id).await?))
}

// ── Working Hours ──────────────────────────────────────────

#[utoipa::path(
    get,
    path = "/api/v1/branches/{id}/working-hours",
    tag = "Branches",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Branch ID")),
    responses(
        (status = 200, description = "Working hours list", body = Vec<WorkingHourResponse>),
        (status = 404, description = "Branch not found")
    )
)]
pub async fn get_working_hours(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<Vec<WorkingHourResponse>>, AppError> {
    let svc = BranchService::new(&state.db);
    Ok(Json(svc.get_working_hours(id).await?))
}

#[utoipa::path(
    put,
    path = "/api/v1/branches/{id}/working-hours",
    tag = "Branches",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Branch ID")),
    request_body = UpdateWorkingHoursRequest,
    responses(
        (status = 200, description = "Working hours updated", body = Vec<WorkingHourResponse>),
        (status = 404, description = "Branch not found"),
        (status = 422, description = "Validation error")
    )
)]
pub async fn update_working_hours(State(state): State<SharedState>, Path(id): Path<i64>, Json(req): Json<UpdateWorkingHoursRequest>) -> Result<Json<Vec<WorkingHourResponse>>, AppError> {
    let svc = BranchService::new(&state.db);
    Ok(Json(svc.update_working_hours(id, &req).await?))
}

// ── User-Branch Assignment ─────────────────────────────────

#[utoipa::path(
    post,
    path = "/api/v1/branches/{id}/users",
    tag = "Branches",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Branch ID")),
    request_body = AssignUserToBranchRequest,
    responses(
        (status = 200, description = "User assigned to branch"),
        (status = 404, description = "Branch or user not found"),
        (status = 409, description = "User already assigned")
    )
)]
pub async fn assign_user(State(state): State<SharedState>, Path(id): Path<i64>, Json(req): Json<AssignUserToBranchRequest>) -> Result<Json<MessageResponse>, AppError> {
    let svc = BranchService::new(&state.db);
    Ok(Json(svc.assign_user(id, &req).await?))
}

#[utoipa::path(
    delete,
    path = "/api/v1/branches/{id}/users/{uid}",
    tag = "Branches",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Branch ID"), ("uid" = i64, Path, description = "User ID")),
    responses(
        (status = 200, description = "User removed from branch"),
        (status = 404, description = "Assignment not found")
    )
)]
pub async fn remove_user(State(state): State<SharedState>, Path((id, uid)): Path<(i64, i64)>) -> Result<Json<MessageResponse>, AppError> {
    let svc = BranchService::new(&state.db);
    Ok(Json(svc.remove_user(id, uid).await?))
}

#[utoipa::path(
    get,
    path = "/api/v1/branches/{id}/users",
    tag = "Branches",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Branch ID")),
    responses(
        (status = 200, description = "List of branch users", body = Vec<BranchUserResponse>),
        (status = 404, description = "Branch not found")
    )
)]
pub async fn list_branch_users(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<Vec<BranchUserResponse>>, AppError> {
    let svc = BranchService::new(&state.db);
    Ok(Json(svc.list_branch_users(id).await?))
}

// ── Branch Statistics ──────────────────────────────────────

#[utoipa::path(
    get,
    path = "/api/v1/branches/{id}/stats",
    tag = "Branches",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Branch ID")),
    responses(
        (status = 200, description = "Branch statistics", body = BranchStatsResponse),
        (status = 404, description = "Branch not found")
    )
)]
pub async fn get_branch_stats(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<BranchStatsResponse>, AppError> {
    let svc = BranchService::new(&state.db);
    Ok(Json(svc.get_branch_stats(id).await?))
}
