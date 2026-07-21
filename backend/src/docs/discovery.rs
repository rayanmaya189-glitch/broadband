/// OpenAPI schemas and stub handlers for Discovery endpoints.
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

// ── Request / Response types ─────────────────────────────────────────

#[derive(Debug, Serialize, ToSchema)]
pub struct ScanResponse {
    /// Scan ID
    pub id: i64,
    /// Scan name
    pub name: String,
    /// Scan type (e.g. "network", "snmp")
    pub scan_type: String,
    /// Whether the scan is active
    pub is_active: bool,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateScanRequest {
    /// Scan name
    pub name: String,
    /// Scan type
    pub scan_type: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ResultResponse {
    /// Result ID
    pub id: i64,
    /// Discovered IP address
    pub discovered_ip: String,
    /// Device vendor
    pub vendor: Option<String>,
    /// Device model
    pub model: Option<String>,
    /// Device status (e.g. "pending", "approved")
    pub status: String,
}

// ── Stub handler functions (for OpenAPI spec only) ───────────────────

/// List all discovery scans (paginated)
#[utoipa::path(
    get,
    path = "/api/v1/discovery/scans",
    tag = "Discovery",
    params(("page" = Option<i64>, Query, description = "Page number"),
           ("limit" = Option<i64>, Query, description = "Items per page")),
    responses(
        (status = 200, description = "Paginated list of scans"),
        (status = 403, description = "Forbidden")
    ),
    security(("bearer_auth" = []))
)]
pub async fn list_scans() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Create a new network scan
#[utoipa::path(
    post,
    path = "/api/v1/discovery/scans",
    tag = "Discovery",
    request_body = CreateScanRequest,
    responses(
        (status = 201, description = "Scan created", body = ScanResponse),
        (status = 403, description = "Forbidden"),
        (status = 422, description = "Validation error")
    ),
    security(("bearer_auth" = []))
)]
pub async fn create_scan() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// List all discovery results (paginated)
#[utoipa::path(
    get,
    path = "/api/v1/discovery/results",
    tag = "Discovery",
    params(("page" = Option<i64>, Query, description = "Page number"),
           ("limit" = Option<i64>, Query, description = "Items per page")),
    responses(
        (status = 200, description = "Paginated list of results"),
        (status = 403, description = "Forbidden")
    ),
    security(("bearer_auth" = []))
)]
pub async fn list_results() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Approve a discovered device result
#[utoipa::path(
    post,
    path = "/api/v1/discovery/results/{id}/approve",
    tag = "Discovery",
    params(("id" = i64, Path, description = "Result ID")),
    responses(
        (status = 200, description = "Result approved"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Result not found")
    ),
    security(("bearer_auth" = []))
)]
pub async fn approve_result() -> axum::Json<serde_json::Value> {
    unimplemented!()
}
