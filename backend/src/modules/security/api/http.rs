use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::shared::app_state::AppState;
use crate::shared::errors::AppError;
use crate::shared::middleware::auth::{UserContext, require_permission};
use crate::modules::security::application::services::SecurityService;

#[derive(Debug, Serialize)]
pub struct RoleResponse {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
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

/// GET /api/v1/rbac/roles
pub async fn list_roles(
    State(state): State<Arc<AppState>>,
    user: UserContext,
) -> Result<Json<Vec<RoleResponse>>, AppError> {
    require_permission(&user, "rbac.role.view").map_err(|e| AppError::Forbidden(e.1))?;
    let roles = SecurityService::list_roles(&state.db).await?;
    let resp: Vec<RoleResponse> = roles.into_iter().map(|r| RoleResponse {
        id: r.id, name: r.name, slug: r.slug,
        description: r.description, parent_role_id: r.parent_role_id, is_system: r.is_system,
    }).collect();
    Ok(Json(resp))
}

/// GET /api/v1/rbac/permissions
pub async fn list_permissions(
    State(state): State<Arc<AppState>>,
    user: UserContext,
) -> Result<Json<Vec<PermissionResponse>>, AppError> {
    require_permission(&user, "rbac.permission.view").map_err(|e| AppError::Forbidden(e.1))?;
    let perms = SecurityService::list_permissions(&state.db).await?;
    let resp: Vec<PermissionResponse> = perms.into_iter().map(|p| PermissionResponse {
        id: p.id, name: p.name, module: p.module, resource: p.resource, action: p.action,
    }).collect();
    Ok(Json(resp))
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
    SecurityService::assign_role(&state.db, &mut redis, user_id, req.role_id, Some(user.user_id)).await?;
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
