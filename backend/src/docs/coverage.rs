/// OpenAPI schemas and stub handlers for Coverage endpoints.
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

// ── Request / Response types ─────────────────────────────────────────

#[derive(Debug, Serialize, ToSchema)]
pub struct CoverageAreaResponse {
    /// Area ID
    pub id: i64,
    /// Area name
    pub name: String,
    /// Area type (e.g. "city", "region")
    pub area_type: String,
    /// Whether the area is active
    pub is_active: bool,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateCoverageAreaRequest {
    /// Area name
    pub name: String,
    /// Area type (e.g. "city", "region")
    pub area_type: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CheckPincodeRequest {
    /// Pincode to check
    pub pincode: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AvailabilityResponse {
    /// Whether the pincode is covered
    pub available: bool,
    /// Area name if covered
    pub area_name: Option<String>,
    /// Estimated installation days if covered
    pub estimated_days: Option<i32>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CoverageStatsResponse {
    /// Total number of coverage areas
    pub total_areas: i64,
    /// Number of active areas
    pub active_areas: i64,
}

// ── Stub handler functions (for OpenAPI spec only) ───────────────────

/// List all coverage areas (paginated)
#[utoipa::path(
    get,
    path = "/api/v1/coverage",
    tag = "Coverage",
    params(("page" = Option<i64>, Query, description = "Page number"),
           ("limit" = Option<i64>, Query, description = "Items per page")),
    responses(
        (status = 200, description = "Paginated list of coverage areas"),
        (status = 403, description = "Forbidden")
    ),
    security(("bearer_auth" = []))
)]
pub async fn list_coverage_areas() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Check if a pincode is covered and get availability info
#[utoipa::path(
    post,
    path = "/api/v1/coverage/check",
    tag = "Coverage",
    request_body = CheckPincodeRequest,
    responses(
        (status = 200, description = "Availability result", body = AvailabilityResponse),
        (status = 422, description = "Validation error")
    )
)]
pub async fn check_availability() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Create a new coverage area
#[utoipa::path(
    post,
    path = "/api/v1/coverage",
    tag = "Coverage",
    request_body = CreateCoverageAreaRequest,
    responses(
        (status = 201, description = "Coverage area created", body = CoverageAreaResponse),
        (status = 403, description = "Forbidden"),
        (status = 422, description = "Validation error")
    ),
    security(("bearer_auth" = []))
)]
pub async fn create_coverage_area() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Get coverage statistics
#[utoipa::path(
    get,
    path = "/api/v1/coverage/stats",
    tag = "Coverage",
    responses(
        (status = 200, description = "Coverage statistics", body = CoverageStatsResponse),
        (status = 403, description = "Forbidden")
    ),
    security(("bearer_auth" = []))
)]
pub async fn get_coverage_stats() -> axum::Json<serde_json::Value> {
    unimplemented!()
}
