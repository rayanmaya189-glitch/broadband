/// OpenAPI schemas and stub handlers for Bandwidth endpoints.
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

// ── Request Types ────────────────────────────────────────────────────

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateProfileRequest {
    /// Profile name
    pub name: String,
    /// Download speed in kbps
    pub download_kbps: i32,
    /// Upload speed in kbps
    pub upload_kbps: i32,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateProfileRequest {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub download_kbps: Option<i32>,
    #[serde(default)]
    pub upload_kbps: Option<i32>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreatePolicyRequest {
    /// Policy name
    pub name: String,
    /// Policy type (rate_limit, burst, schedule)
    pub policy_type: String,
    /// Policy configuration as JSON
    pub config: serde_json::Value,
    /// Priority (lower = higher priority)
    #[serde(default)]
    pub priority: Option<i32>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdatePolicyRequest {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub config: Option<serde_json::Value>,
    #[serde(default)]
    pub priority: Option<i32>,
    #[serde(default)]
    pub is_active: Option<bool>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct ApplyProfileRequest {
    /// Bandwidth profile ID to apply
    pub profile_id: i64,
}

// ── Response Types ───────────────────────────────────────────────────

#[derive(Debug, Serialize, ToSchema)]
pub struct BandwidthProfileResponse {
    pub id: i64,
    pub name: String,
    pub download_kbps: i32,
    pub upload_kbps: i32,
    pub is_active: bool,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct BandwidthPolicyResponse {
    pub id: i64,
    pub name: String,
    pub policy_type: String,
    pub config: serde_json::Value,
    pub priority: i32,
    pub is_active: bool,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct BandwidthApplicationResponse {
    pub id: i64,
    pub profile_id: i64,
    pub subscription_id: i64,
    pub status: String,
    pub applied_at: Option<String>,
    pub retry_count: i32,
    pub created_at: String,
}

// ── Stub handler functions (for OpenAPI spec only) ───────────────────

/// List bandwidth profiles with pagination
#[utoipa::path(
    get,
    path = "/api/v1/bandwidth/profiles",
    tag = "Bandwidth",
    params(
        ("page" = Option<i64>, Query, description = "Page number"),
        ("limit" = Option<i64>, Query, description = "Items per page")
    ),
    responses(
        (status = 200, description = "Paginated list of bandwidth profiles"),
        (status = 401, description = "Unauthorized")
    ),
    security(("bearer_auth" = []))
)]
pub async fn list_profiles() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Create a new bandwidth profile
#[utoipa::path(
    post,
    path = "/api/v1/bandwidth/profiles",
    tag = "Bandwidth",
    request_body = CreateProfileRequest,
    responses(
        (status = 201, description = "Profile created", body = BandwidthProfileResponse),
        (status = 409, description = "Profile name already exists"),
        (status = 422, description = "Validation error")
    ),
    security(("bearer_auth" = []))
)]
pub async fn create_profile() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Update an existing bandwidth profile
#[utoipa::path(
    put,
    path = "/api/v1/bandwidth/profiles/{id}",
    tag = "Bandwidth",
    params(("id" = i64, Path, description = "Profile ID")),
    request_body = UpdateProfileRequest,
    responses(
        (status = 200, description = "Profile updated", body = BandwidthProfileResponse),
        (status = 404, description = "Profile not found")
    ),
    security(("bearer_auth" = []))
)]
pub async fn update_profile() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Delete a bandwidth profile
#[utoipa::path(
    delete,
    path = "/api/v1/bandwidth/profiles/{id}",
    tag = "Bandwidth",
    params(("id" = i64, Path, description = "Profile ID")),
    responses(
        (status = 204, description = "Profile deleted"),
        (status = 404, description = "Profile not found")
    ),
    security(("bearer_auth" = []))
)]
pub async fn delete_profile() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Get a single bandwidth profile
#[utoipa::path(
    get,
    path = "/api/v1/bandwidth/profiles/{id}",
    tag = "Bandwidth",
    params(("id" = i64, Path, description = "Profile ID")),
    responses(
        (status = 200, description = "Bandwidth profile", body = BandwidthProfileResponse),
        (status = 404, description = "Profile not found")
    ),
    security(("bearer_auth" = []))
)]
pub async fn get_profile() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// List all bandwidth policies
#[utoipa::path(
    get,
    path = "/api/v1/bandwidth/policies",
    tag = "Bandwidth",
    responses(
        (status = 200, description = "List of bandwidth policies", body = Vec<BandwidthPolicyResponse>),
        (status = 401, description = "Unauthorized")
    ),
    security(("bearer_auth" = []))
)]
pub async fn list_policies() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Create a new bandwidth policy
#[utoipa::path(
    post,
    path = "/api/v1/bandwidth/policies",
    tag = "Bandwidth",
    request_body = CreatePolicyRequest,
    responses(
        (status = 201, description = "Policy created", body = BandwidthPolicyResponse),
        (status = 422, description = "Validation error")
    ),
    security(("bearer_auth" = []))
)]
pub async fn create_policy() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Update a bandwidth policy
#[utoipa::path(
    put,
    path = "/api/v1/bandwidth/policies/{id}",
    tag = "Bandwidth",
    params(("id" = i64, Path, description = "Policy ID")),
    request_body = UpdatePolicyRequest,
    responses(
        (status = 200, description = "Policy updated", body = BandwidthPolicyResponse),
        (status = 404, description = "Policy not found")
    ),
    security(("bearer_auth" = []))
)]
pub async fn update_policy() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Delete a bandwidth policy
#[utoipa::path(
    delete,
    path = "/api/v1/bandwidth/policies/{id}",
    tag = "Bandwidth",
    params(("id" = i64, Path, description = "Policy ID")),
    responses(
        (status = 204, description = "Policy deleted"),
        (status = 404, description = "Policy not found")
    ),
    security(("bearer_auth" = []))
)]
pub async fn delete_policy() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Apply a bandwidth profile to all subscriptions
#[utoipa::path(
    post,
    path = "/api/v1/bandwidth/profiles/{id}/apply",
    tag = "Bandwidth",
    params(("id" = i64, Path, description = "Profile ID")),
    responses(
        (status = 200, description = "Profile applied to all subscriptions"),
        (status = 404, description = "Profile not found")
    ),
    security(("bearer_auth" = []))
)]
pub async fn apply_profile_to_all() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Apply a bandwidth profile to a specific subscription
#[utoipa::path(
    post,
    path = "/api/v1/bandwidth/apply/{subscription_id}",
    tag = "Bandwidth",
    params(("subscription_id" = i64, Path, description = "Subscription ID")),
    request_body = ApplyProfileRequest,
    responses(
        (status = 201, description = "Profile applied to subscription", body = BandwidthApplicationResponse),
        (status = 404, description = "Subscription not found")
    ),
    security(("bearer_auth" = []))
)]
pub async fn apply_to_subscription() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// List all bandwidth applications
#[utoipa::path(
    get,
    path = "/api/v1/bandwidth/applications",
    tag = "Bandwidth",
    responses(
        (status = 200, description = "List of bandwidth applications", body = Vec<BandwidthApplicationResponse>),
        (status = 401, description = "Unauthorized")
    ),
    security(("bearer_auth" = []))
)]
pub async fn list_bandwidth_applications() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Get bandwidth usage for a subscription
#[utoipa::path(
    get,
    path = "/api/v1/bandwidth/usage/{subscription_id}",
    tag = "Bandwidth",
    params(("subscription_id" = i64, Path, description = "Subscription ID")),
    responses(
        (status = 200, description = "Bandwidth usage data"),
        (status = 404, description = "Subscription not found")
    ),
    security(("bearer_auth" = []))
)]
pub async fn get_bandwidth_usage() -> axum::Json<serde_json::Value> {
    unimplemented!()
}
