//! SeaORM-based controller for the Role domain.

use axum::extract::{Json, Path, Query, State};
use validator::Validate;

use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::common::utils::helpers::PaginatedResponse;
use crate::modules::role::request::role_request::*;
use crate::modules::role::response::role_response::*;
use crate::modules::role::service::role_service_seaorm::RoleServiceSeaorm;

pub async fn list_roles(
    State(state): State<SharedState>,
    Query(query): Query<ListRolesQuery>,
) -> Result<Json<PaginatedResponse<RoleResponse>>, AppError> {
    let svc = RoleServiceSeaorm::new(&state.db_seaorm);
    Ok(Json(svc.list_roles(&query).await?))
}

pub async fn create_role(
    State(state): State<SharedState>,
    Json(req): Json<CreateRoleRequest>,
) -> Result<Json<RoleResponse>, AppError> {
    req.validate()?;
    let svc = RoleServiceSeaorm::new(&state.db_seaorm);
    Ok(Json(svc.create_role(&req).await?))
}

pub async fn get_role(
    State(state): State<SharedState>,
    Path(role_id): Path<i64>,
) -> Result<Json<RoleResponse>, AppError> {
    let svc = RoleServiceSeaorm::new(&state.db_seaorm);
    Ok(Json(svc.get_role(role_id).await?))
}

pub async fn update_role(
    State(state): State<SharedState>,
    Path(role_id): Path<i64>,
    Json(req): Json<UpdateRoleRequest>,
) -> Result<Json<RoleResponse>, AppError> {
    req.validate()?;
    let svc = RoleServiceSeaorm::new(&state.db_seaorm);
    Ok(Json(svc.update_role(role_id, &req).await?))
}

pub async fn deactivate_role(
    State(state): State<SharedState>,
    Path(role_id): Path<i64>,
) -> Result<Json<MessageResponse>, AppError> {
    let svc = RoleServiceSeaorm::new(&state.db_seaorm);
    Ok(Json(svc.deactivate_role(role_id).await?))
}

// ── Permission Assignment ──────────────────────────────────

pub async fn assign_permissions(
    State(state): State<SharedState>,
    Path(role_id): Path<i64>,
    Json(req): Json<AssignPermissionsRequest>,
) -> Result<Json<MessageResponse>, AppError> {
    let svc = RoleServiceSeaorm::new(&state.db_seaorm);
    Ok(Json(svc.assign_permissions(role_id, &req).await?))
}

pub async fn remove_permission(
    State(state): State<SharedState>,
    Path((role_id, permission_id)): Path<(i64, i64)>,
) -> Result<Json<MessageResponse>, AppError> {
    let svc = RoleServiceSeaorm::new(&state.db_seaorm);
    Ok(Json(svc.remove_permission(role_id, permission_id).await?))
}

// ── User-Role Management ───────────────────────────────────

pub async fn list_user_roles(
    State(state): State<SharedState>,
    Path(uid): Path<i64>,
) -> Result<Json<Vec<RoleResponse>>, AppError> {
    let svc = RoleServiceSeaorm::new(&state.db_seaorm);
    Ok(Json(svc.list_user_roles(uid).await?))
}

pub async fn assign_role_to_user(
    State(state): State<SharedState>,
    Path(uid): Path<i64>,
    Json(req): Json<AssignUserRoleRequest>,
) -> Result<Json<MessageResponse>, AppError> {
    let svc = RoleServiceSeaorm::new(&state.db_seaorm);
    Ok(Json(svc.assign_role_to_user(uid, &req).await?))
}

pub async fn revoke_role_from_user(
    State(state): State<SharedState>,
    Path((uid, rid)): Path<(i64, i64)>,
) -> Result<Json<MessageResponse>, AppError> {
    let svc = RoleServiceSeaorm::new(&state.db_seaorm);
    Ok(Json(svc.revoke_role_from_user(uid, rid).await?))
}
