use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::modules::security::application::services::SecurityService;
use crate::shared::app_state::AppState;
use crate::shared::errors::AppError;
use crate::shared::middleware::auth::{require_permission, UserContext};

#[derive(Debug, Serialize)]
pub struct RoleResponse {
    pub id: i64,
    pub name: String,
    pub slug: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub parent_role_id: Option<i64>,
    pub is_system: bool,
}

#[derive(Debug, Serialize)]
pub struct PermissionResponse {
    pub id: i64,
    pub name: String,
    pub module: String,
    pub resource: String,
    pub action: String,
}

#[derive(Debug, Deserialize)]
pub struct AssignRoleRequest {
    pub role_id: i64,
}

#[derive(Debug, Deserialize)]
pub struct CreateRoleRequest {
    pub name: String,
    pub slug: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub parent_role_id: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateRoleRequest {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub parent_role_id: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct AssignPermissionRequest {
    pub permission_id: i64,
}

/// GET /api/v1/rbac/roles
pub async fn list_roles(
    State(state): State<Arc<AppState>>,
    user: UserContext,
) -> Result<Json<Vec<RoleResponse>>, AppError> {
    require_permission(&user, "rbac.role.view").map_err(|e| AppError::Forbidden(e.1))?;
    let roles = SecurityService::list_roles(&state.db).await?;
    let resp: Vec<RoleResponse> = roles
        .into_iter()
        .map(|r| RoleResponse {
            id: r.id,
            name: r.name,
            slug: r.slug,
            description: r.description,
            parent_role_id: r.parent_role_id,
            is_system: r.is_system,
        })
        .collect();
    Ok(Json(resp))
}

/// POST /api/v1/rbac/roles
pub async fn create_role(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Json(req): Json<CreateRoleRequest>,
) -> Result<(StatusCode, Json<RoleResponse>), AppError> {
    require_permission(&user, "rbac.role.create").map_err(|e| AppError::Forbidden(e.1))?;
    let role = SecurityService::create_role(
        &state.db,
        req.name,
        req.slug,
        req.description,
        req.parent_role_id,
    )
    .await?;
    Ok((
        StatusCode::CREATED,
        Json(RoleResponse {
            id: role.id,
            name: role.name,
            slug: role.slug,
            description: role.description,
            parent_role_id: role.parent_role_id,
            is_system: role.is_system,
        }),
    ))
}

/// GET /api/v1/rbac/roles/:id
pub async fn get_role(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Path(id): Path<i64>,
) -> Result<Json<RoleResponse>, AppError> {
    require_permission(&user, "rbac.role.view").map_err(|e| AppError::Forbidden(e.1))?;
    let role = SecurityService::get_role(&state.db, id).await?;
    Ok(Json(RoleResponse {
        id: role.id,
        name: role.name,
        slug: role.slug,
        description: role.description,
        parent_role_id: role.parent_role_id,
        is_system: role.is_system,
    }))
}

/// PUT /api/v1/rbac/roles/:id
pub async fn update_role(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Path(id): Path<i64>,
    Json(req): Json<UpdateRoleRequest>,
) -> Result<Json<RoleResponse>, AppError> {
    require_permission(&user, "rbac.role.update").map_err(|e| AppError::Forbidden(e.1))?;
    let role = SecurityService::update_role(&state.db, id, req.name, req.description, req.parent_role_id).await?;
    Ok(Json(RoleResponse {
        id: role.id,
        name: role.name,
        slug: role.slug,
        description: role.description,
        parent_role_id: role.parent_role_id,
        is_system: role.is_system,
    }))
}

/// DELETE /api/v1/rbac/roles/:id
pub async fn delete_role(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Path(id): Path<i64>,
) -> Result<StatusCode, AppError> {
    require_permission(&user, "rbac.role.delete").map_err(|e| AppError::Forbidden(e.1))?;
    SecurityService::delete_role(&state.db, id).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// GET /api/v1/rbac/permissions
pub async fn list_permissions(
    State(state): State<Arc<AppState>>,
    user: UserContext,
) -> Result<Json<Vec<PermissionResponse>>, AppError> {
    require_permission(&user, "rbac.permission.view").map_err(|e| AppError::Forbidden(e.1))?;
    let perms = SecurityService::list_permissions(&state.db).await?;
    let resp: Vec<PermissionResponse> = perms
        .into_iter()
        .map(|p| PermissionResponse {
            id: p.id,
            name: p.name,
            module: p.module,
            resource: p.resource,
            action: p.action,
        })
        .collect();
    Ok(Json(resp))
}

/// POST /api/v1/rbac/roles/:id/permissions
pub async fn assign_permission(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Path(role_id): Path<i64>,
    Json(req): Json<AssignPermissionRequest>,
) -> Result<StatusCode, AppError> {
    require_permission(&user, "rbac.role.permission.assign").map_err(|e| AppError::Forbidden(e.1))?;
    SecurityService::assign_permission(&state.db, role_id, req.permission_id).await?;
    Ok(StatusCode::CREATED)
}

/// DELETE /api/v1/rbac/roles/:id/permissions/:perm_id
pub async fn revoke_permission(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Path((role_id, perm_id)): Path<(i64, i64)>,
) -> Result<StatusCode, AppError> {
    require_permission(&user, "rbac.role.permission.revoke").map_err(|e| AppError::Forbidden(e.1))?;
    SecurityService::revoke_permission(&state.db, role_id, perm_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// POST /api/v1/rbac/users/:id/roles
pub async fn assign_role(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Path(user_id): Path<i64>,
    Json(req): Json<AssignRoleRequest>,
) -> Result<StatusCode, AppError> {
    require_permission(&user, "rbac.user.role.assign").map_err(|e| AppError::Forbidden(e.1))?;
    let mut redis = state.redis.clone();
    SecurityService::assign_role(
        &state.db,
        &mut redis,
        user_id,
        req.role_id,
        Some(user.user_id),
    )
    .await?;
    Ok(StatusCode::CREATED)
}

/// DELETE /api/v1/rbac/users/:id/roles/:role_id
pub async fn revoke_role(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Path((user_id, role_id)): Path<(i64, i64)>,
) -> Result<StatusCode, AppError> {
    require_permission(&user, "rbac.user.role.revoke").map_err(|e| AppError::Forbidden(e.1))?;
    let mut redis = state.redis.clone();
    SecurityService::revoke_role(&state.db, &mut redis, user_id, role_id).await?;
    Ok(StatusCode::NO_CONTENT)
}
