/// OpenAPI schemas and stub handlers for Branches endpoints.
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

// ── Request Types ────────────────────────────────────────────────────

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateBranchRequest {
    /// Branch name
    pub name: String,
    /// URL-friendly slug
    pub slug: String,
    /// Short branch code
    pub code: String,
    /// City
    pub city: String,
    #[serde(default)]
    pub state: Option<String>,
    #[serde(default)]
    pub address: Option<String>,
    #[serde(default)]
    pub phone: Option<String>,
    #[serde(default)]
    pub email: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateBranchRequest {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub city: Option<String>,
    #[serde(default)]
    pub state: Option<String>,
    #[serde(default)]
    pub address: Option<String>,
    #[serde(default)]
    pub phone: Option<String>,
    #[serde(default)]
    pub email: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct WorkingHoursEntry {
    /// Day of week (0=Sunday, 6=Saturday)
    pub day_of_week: i32,
    /// Opening time (HH:MM)
    pub open_time: String,
    /// Closing time (HH:MM)
    pub close_time: String,
    /// Whether the branch is closed on this day
    pub is_closed: bool,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct AssignBranchUserRequest {
    /// User ID to assign
    pub user_id: i64,
    /// Role for the user in this branch
    pub role: String,
}

// ── Response Types ───────────────────────────────────────────────────

#[derive(Debug, Serialize, ToSchema)]
pub struct BranchResponse {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub code: String,
    pub city: String,
    pub state: String,
    pub address: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub timezone: String,
    pub is_active: bool,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct WorkingHoursResponse {
    pub id: i64,
    pub branch_id: i64,
    pub day_of_week: i32,
    pub open_time: String,
    pub close_time: String,
    pub is_closed: bool,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct BranchUserResponse {
    pub id: i64,
    pub branch_id: i64,
    pub user_id: i64,
    pub role: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct BranchStatsResponse {
    pub branch_id: i64,
    pub total_customers: i64,
    pub total_subscriptions: i64,
    pub total_users: i64,
}

// ── Stub handler functions (for OpenAPI spec only) ───────────────────

/// List all branches
#[utoipa::path(
    get,
    path = "/api/v1/branches",
    tag = "Branches",
    responses(
        (status = 200, description = "List of branches", body = Vec<BranchResponse>),
        (status = 401, description = "Unauthorized")
    ),
    security(("bearer_auth" = []))
)]
pub async fn list_branches() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Create a new branch
#[utoipa::path(
    post,
    path = "/api/v1/branches",
    tag = "Branches",
    request_body = CreateBranchRequest,
    responses(
        (status = 201, description = "Branch created", body = BranchResponse),
        (status = 409, description = "Slug or code already exists"),
        (status = 422, description = "Validation error")
    ),
    security(("bearer_auth" = []))
)]
pub async fn create_branch() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Get a branch by ID
#[utoipa::path(
    get,
    path = "/api/v1/branches/{id}",
    tag = "Branches",
    params(("id" = i64, Path, description = "Branch ID")),
    responses(
        (status = 200, description = "Branch details", body = BranchResponse),
        (status = 404, description = "Branch not found")
    ),
    security(("bearer_auth" = []))
)]
pub async fn get_branch() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Update a branch
#[utoipa::path(
    put,
    path = "/api/v1/branches/{id}",
    tag = "Branches",
    params(("id" = i64, Path, description = "Branch ID")),
    request_body = UpdateBranchRequest,
    responses(
        (status = 200, description = "Branch updated", body = BranchResponse),
        (status = 404, description = "Branch not found")
    ),
    security(("bearer_auth" = []))
)]
pub async fn update_branch() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Delete (deactivate) a branch
#[utoipa::path(
    delete,
    path = "/api/v1/branches/{id}",
    tag = "Branches",
    params(("id" = i64, Path, description = "Branch ID")),
    responses(
        (status = 204, description = "Branch deactivated"),
        (status = 404, description = "Branch not found")
    ),
    security(("bearer_auth" = []))
)]
pub async fn delete_branch() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Get branch hierarchy tree
#[utoipa::path(
    get,
    path = "/api/v1/branches/hierarchy",
    tag = "Branches",
    responses(
        (status = 200, description = "Branch hierarchy tree"),
        (status = 401, description = "Unauthorized")
    ),
    security(("bearer_auth" = []))
)]
pub async fn get_branch_hierarchy() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Get working hours for a branch
#[utoipa::path(
    get,
    path = "/api/v1/branches/{id}/working-hours",
    tag = "Branches",
    params(("id" = i64, Path, description = "Branch ID")),
    responses(
        (status = 200, description = "Working hours", body = Vec<WorkingHoursResponse>),
        (status = 404, description = "Branch not found")
    ),
    security(("bearer_auth" = []))
)]
pub async fn get_working_hours() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Update working hours for a branch
#[utoipa::path(
    put,
    path = "/api/v1/branches/{id}/working-hours",
    tag = "Branches",
    params(("id" = i64, Path, description = "Branch ID")),
    request_body = Vec<WorkingHoursEntry>,
    responses(
        (status = 200, description = "Working hours updated", body = Vec<WorkingHoursResponse>),
        (status = 404, description = "Branch not found")
    ),
    security(("bearer_auth" = []))
)]
pub async fn update_working_hours() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Get statistics for a branch
#[utoipa::path(
    get,
    path = "/api/v1/branches/{id}/stats",
    tag = "Branches",
    params(("id" = i64, Path, description = "Branch ID")),
    responses(
        (status = 200, description = "Branch statistics", body = BranchStatsResponse),
        (status = 404, description = "Branch not found")
    ),
    security(("bearer_auth" = []))
)]
pub async fn get_branch_stats() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Assign a user to a branch
#[utoipa::path(
    post,
    path = "/api/v1/branches/{id}/users",
    tag = "Branches",
    params(("id" = i64, Path, description = "Branch ID")),
    request_body = AssignBranchUserRequest,
    responses(
        (status = 201, description = "User assigned", body = BranchUserResponse),
        (status = 404, description = "Branch or user not found")
    ),
    security(("bearer_auth" = []))
)]
pub async fn assign_branch_user() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Remove a user from a branch
#[utoipa::path(
    delete,
    path = "/api/v1/branches/{id}/users/{uid}",
    tag = "Branches",
    params(
        ("id" = i64, Path, description = "Branch ID"),
        ("uid" = i64, Path, description = "User ID")
    ),
    responses(
        (status = 204, description = "User removed from branch"),
        (status = 404, description = "Branch or user assignment not found")
    ),
    security(("bearer_auth" = []))
)]
pub async fn remove_branch_user() -> axum::Json<serde_json::Value> {
    unimplemented!()
}
