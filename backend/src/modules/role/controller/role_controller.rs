use axum::extract::{Json, Path, Query, State};
use validator::Validate;

use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::modules::role::request::role_request::*;
use crate::modules::role::response::role_response::*;
use crate::modules::role::service::role_service::RoleService;

#[utoipa::path(
    get,
    path = "/api/v1/roles",
    tag = "Roles",
    security(("bearer_auth" = [])),
    params(
        ("page" = Option<i64>, Query, description = "Page number"),
        ("limit" = Option<i64>, Query, description = "Items per page"),
        ("search" = Option<String>, Query, description = "Search term")
    ),
    responses(
        (status = 200, description = "List of roles"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn list_roles(State(state): State<SharedState>, Query(query): Query<ListRolesQuery>) -> Result<Json<crate::common::utils::helpers::PaginatedResponse<RoleResponse>>, AppError> {
    let svc = RoleService::new(&state.db);
    Ok(Json(svc.list_roles(&query).await?))
}

#[utoipa::path(
    post,
    path = "/api/v1/roles",
    tag = "Roles",
    security(("bearer_auth" = [])),
    request_body = CreateRoleRequest,
    responses(
        (status = 200, description = "Role created", body = RoleResponse),
        (status = 409, description = "Role name already exists"),
        (status = 422, description = "Validation error")
    )
)]
pub async fn create_role(State(state): State<SharedState>, Json(req): Json<CreateRoleRequest>) -> Result<Json<RoleResponse>, AppError> {
    req.validate()?;
    let svc = RoleService::new(&state.db);
    Ok(Json(svc.create_role(&req).await?))
}

#[utoipa::path(
    get,
    path = "/api/v1/roles/{id}",
    tag = "Roles",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Role ID")),
    responses(
        (status = 200, description = "Role details", body = RoleResponse),
        (status = 404, description = "Role not found")
    )
)]
pub async fn get_role(State(state): State<SharedState>, Path(role_id): Path<i64>) -> Result<Json<RoleResponse>, AppError> {
    let svc = RoleService::new(&state.db);
    Ok(Json(svc.get_role(role_id).await?))
}

#[utoipa::path(
    put,
    path = "/api/v1/roles/{id}",
    tag = "Roles",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Role ID")),
    request_body = UpdateRoleRequest,
    responses(
        (status = 200, description = "Role updated", body = RoleResponse),
        (status = 404, description = "Role not found"),
        (status = 422, description = "Validation error")
    )
)]
pub async fn update_role(State(state): State<SharedState>, Path(role_id): Path<i64>, Json(req): Json<UpdateRoleRequest>) -> Result<Json<RoleResponse>, AppError> {
    req.validate()?;
    let svc = RoleService::new(&state.db);
    Ok(Json(svc.update_role(role_id, &req).await?))
}

#[utoipa::path(
    delete,
    path = "/api/v1/roles/{id}",
    tag = "Roles",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Role ID")),
    responses(
        (status = 200, description = "Role deactivated"),
        (status = 404, description = "Role not found")
    )
)]
pub async fn deactivate_role(State(state): State<SharedState>, Path(role_id): Path<i64>) -> Result<Json<MessageResponse>, AppError> {
    let svc = RoleService::new(&state.db);
    Ok(Json(svc.deactivate_role(role_id).await?))
}

// ── Permission Assignment ──────────────────────────────────

#[utoipa::path(
    post,
    path = "/api/v1/roles/{id}/permissions",
    tag = "Roles",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Role ID")),
    request_body = AssignPermissionsRequest,
    responses(
        (status = 200, description = "Permissions assigned"),
        (status = 404, description = "Role not found")
    )
)]
pub async fn assign_permissions(State(state): State<SharedState>, Path(role_id): Path<i64>, Json(req): Json<AssignPermissionsRequest>) -> Result<Json<MessageResponse>, AppError> {
    let svc = RoleService::new(&state.db);
    Ok(Json(svc.assign_permissions(role_id, &req).await?))
}

#[utoipa::path(
    delete,
    path = "/api/v1/roles/{id}/permissions/{pid}",
    tag = "Roles",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Role ID"), ("pid" = i64, Path, description = "Permission ID")),
    responses(
        (status = 200, description = "Permission removed"),
        (status = 404, description = "Permission not found")
    )
)]
pub async fn remove_permission(State(state): State<SharedState>, Path((role_id, permission_id)): Path<(i64, i64)>) -> Result<Json<MessageResponse>, AppError> {
    let svc = RoleService::new(&state.db);
    Ok(Json(svc.remove_permission(role_id, permission_id).await?))
}

// ── User-Role Management ───────────────────────────────────

#[utoipa::path(
    get,
    path = "/api/v1/roles/user/{uid}/roles",
    tag = "Roles",
    security(("bearer_auth" = [])),
    params(("uid" = i64, Path, description = "User ID")),
    responses(
        (status = 200, description = "List of user roles", body = Vec<RoleResponse>),
        (status = 404, description = "User not found")
    )
)]
pub async fn list_user_roles(State(state): State<SharedState>, Path(uid): Path<i64>) -> Result<Json<Vec<RoleResponse>>, AppError> {
    let svc = RoleService::new(&state.db);
    Ok(Json(svc.list_user_roles(uid).await?))
}

#[utoipa::path(
    post,
    path = "/api/v1/roles/user/{uid}/roles",
    tag = "Roles",
    security(("bearer_auth" = [])),
    params(("uid" = i64, Path, description = "User ID")),
    request_body = AssignUserRoleRequest,
    responses(
        (status = 200, description = "Role assigned to user"),
        (status = 404, description = "User or role not found")
    )
)]
pub async fn assign_role_to_user(State(state): State<SharedState>, Path(uid): Path<i64>, Json(req): Json<AssignUserRoleRequest>) -> Result<Json<MessageResponse>, AppError> {
    let svc = RoleService::new(&state.db);
    Ok(Json(svc.assign_role_to_user(uid, &req).await?))
}

#[utoipa::path(
    delete,
    path = "/api/v1/roles/user/{uid}/roles/{rid}",
    tag = "Roles",
    security(("bearer_auth" = [])),
    params(("uid" = i64, Path, description = "User ID"), ("rid" = i64, Path, description = "Role ID")),
    responses(
        (status = 200, description = "Role revoked from user"),
        (status = 404, description = "Assignment not found")
    )
)]
pub async fn revoke_role_from_user(State(state): State<SharedState>, Path((uid, rid)): Path<(i64, i64)>) -> Result<Json<MessageResponse>, AppError> {
    let svc = RoleService::new(&state.db);
    Ok(Json(svc.revoke_role_from_user(uid, rid).await?))
}
