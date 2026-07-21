/// OpenAPI schemas and stub handlers for Plans endpoints.
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

// ── Request / Response types ─────────────────────────────────────────

#[derive(Debug, Serialize, ToSchema)]
pub struct PlanResponse {
    /// Plan ID
    pub id: i64,
    /// URL-friendly plan slug
    pub slug: String,
    /// Plan display name
    pub name: String,
    /// Plan description
    pub description: Option<String>,
    /// Speed label (e.g. "50 Mbps")
    pub speed_label: String,
    /// Download speed in Mbps
    pub download_mbps: i32,
    /// Upload speed in Mbps
    pub upload_mbps: i32,
    /// Burst speed in Mbps
    pub burst_mbps: Option<i32>,
    /// Whether plan is marked as popular
    pub is_popular: bool,
    /// Whether plan is for business
    pub is_business: bool,
    /// Whether plan is active
    pub is_active: bool,
    /// Pricing tiers
    pub pricing: Vec<PlanPricingResponse>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PlanPricingResponse {
    /// Billing period in months
    pub billing_period_months: i32,
    /// Price for this period
    pub price: String,
    /// Savings compared to monthly billing
    pub savings: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreatePlanRequest {
    /// URL-friendly slug
    pub slug: String,
    /// Plan display name
    pub name: String,
    /// Plan description
    #[serde(default)]
    pub description: Option<String>,
    /// Speed label
    pub speed_label: String,
    /// Download speed in Mbps
    pub download_mbps: i32,
    /// Upload speed in Mbps
    pub upload_mbps: i32,
    /// Burst speed in Mbps
    #[serde(default)]
    pub burst_mbps: Option<i32>,
    /// Whether plan is for business
    #[serde(default)]
    pub is_business: Option<bool>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdatePlanRequest {
    /// Updated plan name
    pub name: String,
    /// Updated description
    #[serde(default)]
    pub description: Option<String>,
    /// Whether plan is active
    pub is_active: bool,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdatePricingRequest {
    /// Billing period in months
    pub billing_period_months: i32,
    /// New price
    pub price: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct ClonePlanRequest {
    /// Name for the cloned plan
    pub new_name: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct SetSpeedProfileRequest {
    /// Bandwidth profile ID to assign
    pub bandwidth_profile_id: i64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct SpeedProfileResponse {
    /// Profile ID
    pub id: i64,
    /// Profile name
    pub name: String,
    /// Download speed in Kbps
    pub download_kbps: i32,
    /// Upload speed in Kbps
    pub upload_kbps: i32,
    /// Burst download speed in Kbps
    pub burst_download_kbps: Option<i32>,
    /// Burst upload speed in Kbps
    pub burst_upload_kbps: Option<i32>,
    /// Burst duration in seconds
    pub burst_duration_seconds: Option<i32>,
    /// Priority level
    pub priority: Option<i32>,
    /// Whether profile is active
    pub is_active: bool,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PlanHistoryEntry {
    /// History entry ID
    pub id: i64,
    /// Entity type
    pub entity_type: String,
    /// Entity ID
    pub entity_id: i64,
    /// Change action
    pub action: String,
    /// Change details
    pub details: serde_json::Value,
    /// User who made the change
    pub changed_by: i64,
    /// Timestamp of change
    pub changed_at: String,
}

// ── Stub handler functions (for OpenAPI spec only) ───────────────────

/// List all active plans
#[utoipa::path(
    get,
    path = "/api/v1/plans",
    tag = "Plans",
    responses(
        (status = 200, description = "List of plans", body = Vec<PlanResponse>),
        (status = 403, description = "Insufficient permissions")
    ),
    security(("bearer_auth" = []))
)]
pub async fn list_plans() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Get a single plan by ID
#[utoipa::path(
    get,
    path = "/api/v1/plans/{id}",
    tag = "Plans",
    params(("id" = i64, Path, description = "Plan ID")),
    responses(
        (status = 200, description = "Plan details", body = PlanResponse),
        (status = 404, description = "Plan not found")
    ),
    security(("bearer_auth" = []))
)]
pub async fn get_plan() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Create a new plan (company-wide only)
#[utoipa::path(
    post,
    path = "/api/v1/plans",
    tag = "Plans",
    request_body = CreatePlanRequest,
    responses(
        (status = 201, description = "Plan created", body = PlanResponse),
        (status = 403, description = "Insufficient permissions"),
        (status = 422, description = "Validation error")
    ),
    security(("bearer_auth" = []))
)]
pub async fn create_plan() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Update plan details (company-wide only)
#[utoipa::path(
    put,
    path = "/api/v1/plans/{id}",
    tag = "Plans",
    params(("id" = i64, Path, description = "Plan ID")),
    request_body = UpdatePlanRequest,
    responses(
        (status = 200, description = "Plan updated", body = PlanResponse),
        (status = 403, description = "Insufficient permissions"),
        (status = 404, description = "Plan not found")
    ),
    security(("bearer_auth" = []))
)]
pub async fn update_plan() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Delete (deactivate) a plan (company-wide only)
#[utoipa::path(
    delete,
    path = "/api/v1/plans/{id}",
    tag = "Plans",
    params(("id" = i64, Path, description = "Plan ID")),
    responses(
        (status = 204, description = "Plan deactivated"),
        (status = 403, description = "Insufficient permissions"),
        (status = 404, description = "Plan not found")
    ),
    security(("bearer_auth" = []))
)]
pub async fn delete_plan() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Update pricing for a plan
#[utoipa::path(
    put,
    path = "/api/v1/plans/{id}/pricing",
    tag = "Plans",
    params(("id" = i64, Path, description = "Plan ID")),
    request_body = UpdatePricingRequest,
    responses(
        (status = 200, description = "Pricing updated"),
        (status = 403, description = "Insufficient permissions"),
        (status = 404, description = "Plan not found"),
        (status = 422, description = "Invalid price")
    ),
    security(("bearer_auth" = []))
)]
pub async fn update_pricing() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// List pricing tiers for a plan
#[utoipa::path(
    get,
    path = "/api/v1/plans/{id}/pricing",
    tag = "Plans",
    params(("id" = i64, Path, description = "Plan ID")),
    responses(
        (status = 200, description = "Plan pricing tiers", body = Vec<PlanPricingResponse>),
        (status = 404, description = "Plan not found")
    ),
    security(("bearer_auth" = []))
)]
pub async fn list_plan_pricing() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Approve a plan for publishing (company-wide only)
#[utoipa::path(
    post,
    path = "/api/v1/plans/{id}/approve",
    tag = "Plans",
    params(("id" = i64, Path, description = "Plan ID")),
    responses(
        (status = 200, description = "Plan approved"),
        (status = 403, description = "Insufficient permissions"),
        (status = 404, description = "Plan not found")
    ),
    security(("bearer_auth" = []))
)]
pub async fn approve_plan() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Publish a plan to make it available to customers (company-wide only)
#[utoipa::path(
    post,
    path = "/api/v1/plans/{id}/publish",
    tag = "Plans",
    params(("id" = i64, Path, description = "Plan ID")),
    responses(
        (status = 200, description = "Plan published"),
        (status = 403, description = "Insufficient permissions"),
        (status = 404, description = "Plan not found")
    ),
    security(("bearer_auth" = []))
)]
pub async fn publish_plan() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Unpublish a plan to hide it from customers (company-wide only)
#[utoipa::path(
    post,
    path = "/api/v1/plans/{id}/unpublish",
    tag = "Plans",
    params(("id" = i64, Path, description = "Plan ID")),
    responses(
        (status = 200, description = "Plan unpublished"),
        (status = 403, description = "Insufficient permissions"),
        (status = 404, description = "Plan not found")
    ),
    security(("bearer_auth" = []))
)]
pub async fn unpublish_plan() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Clone a plan with a new name (company-wide only)
#[utoipa::path(
    post,
    path = "/api/v1/plans/{id}/clone",
    tag = "Plans",
    params(("id" = i64, Path, description = "Source plan ID")),
    request_body = ClonePlanRequest,
    responses(
        (status = 201, description = "Plan cloned", body = PlanResponse),
        (status = 403, description = "Insufficient permissions"),
        (status = 404, description = "Plan not found")
    ),
    security(("bearer_auth" = []))
)]
pub async fn clone_plan() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Get the speed/bandwidth profile for a plan
#[utoipa::path(
    get,
    path = "/api/v1/plans/{id}/speed-profile",
    tag = "Plans",
    params(("id" = i64, Path, description = "Plan ID")),
    responses(
        (status = 200, description = "Speed profile", body = SpeedProfileResponse),
        (status = 404, description = "Plan or profile not found")
    ),
    security(("bearer_auth" = []))
)]
pub async fn get_speed_profile() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Set the speed/bandwidth profile for a plan (company-wide only)
#[utoipa::path(
    put,
    path = "/api/v1/plans/{id}/speed-profile",
    tag = "Plans",
    params(("id" = i64, Path, description = "Plan ID")),
    request_body = SetSpeedProfileRequest,
    responses(
        (status = 200, description = "Speed profile assigned"),
        (status = 403, description = "Insufficient permissions"),
        (status = 404, description = "Plan not found"),
        (status = 422, description = "Profile not found")
    ),
    security(("bearer_auth" = []))
)]
pub async fn set_speed_profile() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Get change history for a plan
#[utoipa::path(
    get,
    path = "/api/v1/plans/{id}/history",
    tag = "Plans",
    params(("id" = i64, Path, description = "Plan ID")),
    responses(
        (status = 200, description = "Plan change history"),
        (status = 404, description = "Plan not found")
    ),
    security(("bearer_auth" = []))
)]
pub async fn get_plan_history() -> axum::Json<serde_json::Value> {
    unimplemented!()
}
