/// OpenAPI schemas and stub handlers for Security (RBAC) endpoints.
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

// ── Request / Response types ─────────────────────────────────────────

#[derive(Debug, Serialize, ToSchema)]
pub struct RoleResponse {
    /// Role ID
    pub id: i64,
    /// Role name
    pub name: String,
    /// Role slug
    pub slug: String,
    /// Role description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Parent role ID for hierarchy
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_role_id: Option<i64>,
    /// Whether this is a built-in system role
    pub is_system: bool,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PermissionResponse {
    /// Permission ID
    pub id: i64,
    /// Permission name (e.g. "referral.view")
    pub name: String,
    /// Module the permission belongs to
    pub module: String,
    /// Resource within the module
    pub resource: String,
    /// Allowed action
    pub action: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateRoleRequest {
    /// Role name
    pub name: String,
    /// Role slug (unique identifier)
    pub slug: String,
    /// Role description
    #[serde(default)]
    pub description: Option<String>,
    /// Parent role ID for hierarchy
    #[serde(default)]
    pub parent_role_id: Option<i64>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateRoleRequest {
    /// Updated role name
    #[serde(default)]
    pub name: Option<String>,
    /// Updated description
    #[serde(default)]
    pub description: Option<String>,
    /// Updated parent role ID
    #[serde(default)]
    pub parent_role_id: Option<i64>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct AssignPermissionRequest {
    /// Permission ID to assign
    pub permission_id: i64,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct AssignRoleRequest {
    /// Role ID to assign
    pub role_id: i64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct RolePermissionsResponse {
    /// Role ID
    pub role_id: i64,
    /// Permissions assigned to the role
    pub permissions: Vec<PermissionResponse>,
}

// ── Stub handler functions (for OpenAPI spec only) ───────────────────

/// List all roles
#[utoipa::path(
    get,
    path = "/api/v1/rbac/roles",
    tag = "Security",
    responses(
        (status = 200, description = "List of roles", body = Vec<RoleResponse>),
        (status = 403, description = "Forbidden")
    ),
    security(("bearer_auth" = []))
)]
pub async fn list_roles() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Create a new role
#[utoipa::path(
    post,
    path = "/api/v1/rbac/roles",
    tag = "Security",
    request_body = CreateRoleRequest,
    responses(
        (status = 201, description = "Role created", body = RoleResponse),
        (status = 403, description = "Forbidden"),
        (status = 409, description = "Role slug already exists"),
        (status = 422, description = "Validation error")
    ),
    security(("bearer_auth" = []))
)]
pub async fn create_role() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Get a specific role by ID
#[utoipa::path(
    get,
    path = "/api/v1/rbac/roles/{id}",
    tag = "Security",
    params(("id" = i64, Path, description = "Role ID")),
    responses(
        (status = 200, description = "Role details", body = RoleResponse),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Role not found")
    ),
    security(("bearer_auth" = []))
)]
pub async fn get_role() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Update a role
#[utoipa::path(
    put,
    path = "/api/v1/rbac/roles/{id}",
    tag = "Security",
    params(("id" = i64, Path, description = "Role ID")),
    request_body = UpdateRoleRequest,
    responses(
        (status = 200, description = "Role updated", body = RoleResponse),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Role not found")
    ),
    security(("bearer_auth" = []))
)]
pub async fn update_role() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Delete a role
#[utoipa::path(
    delete,
    path = "/api/v1/rbac/roles/{id}",
    tag = "Security",
    params(("id" = i64, Path, description = "Role ID")),
    responses(
        (status = 204, description = "Role deleted"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Role not found")
    ),
    security(("bearer_auth" = []))
)]
pub async fn delete_role() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// List all permissions
#[utoipa::path(
    get,
    path = "/api/v1/rbac/permissions",
    tag = "Security",
    responses(
        (status = 200, description = "List of permissions", body = Vec<PermissionResponse>),
        (status = 403, description = "Forbidden")
    ),
    security(("bearer_auth" = []))
)]
pub async fn list_permissions() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Assign a permission to a role
#[utoipa::path(
    post,
    path = "/api/v1/rbac/roles/{role_id}/permissions",
    tag = "Security",
    params(("role_id" = i64, Path, description = "Role ID")),
    request_body = AssignPermissionRequest,
    responses(
        (status = 201, description = "Permission assigned"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Role or permission not found")
    ),
    security(("bearer_auth" = []))
)]
pub async fn assign_permission() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Revoke a permission from a role
#[utoipa::path(
    delete,
    path = "/api/v1/rbac/roles/{role_id}/permissions/{perm_id}",
    tag = "Security",
    params(("role_id" = i64, Path, description = "Role ID"),
           ("perm_id" = i64, Path, description = "Permission ID")),
    responses(
        (status = 204, description = "Permission revoked"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Role or permission not found")
    ),
    security(("bearer_auth" = []))
)]
pub async fn revoke_permission() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Assign a role to a user
#[utoipa::path(
    post,
    path = "/api/v1/rbac/users/{user_id}/roles",
    tag = "Security",
    params(("user_id" = i64, Path, description = "User ID")),
    request_body = AssignRoleRequest,
    responses(
        (status = 201, description = "Role assigned"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "User or role not found")
    ),
    security(("bearer_auth" = []))
)]
pub async fn assign_role() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Revoke a role from a user
#[utoipa::path(
    delete,
    path = "/api/v1/rbac/users/{user_id}/roles/{role_id}",
    tag = "Security",
    params(("user_id" = i64, Path, description = "User ID"),
           ("role_id" = i64, Path, description = "Role ID")),
    responses(
        (status = 204, description = "Role revoked"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "User or role not found")
    ),
    security(("bearer_auth" = []))
)]
pub async fn revoke_role() -> axum::Json<serde_json::Value> {
    unimplemented!()
}
