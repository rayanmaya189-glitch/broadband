/// OpenAPI schemas and stub handlers for Leads endpoints.
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

// ── Request / Response types ─────────────────────────────────────────

#[derive(Debug, Serialize, ToSchema)]
pub struct LeadResponse {
    /// Lead ID
    pub id: i64,
    /// Customer name
    pub name: String,
    /// Phone number
    pub phone: String,
    /// Lead status (e.g. new, contacted, quoted, converted)
    pub status: String,
    /// Lead source (e.g. website, referral, walk-in)
    pub source: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateLeadRequest {
    /// Customer name
    pub name: String,
    /// Phone number
    pub phone: String,
    /// Email address (optional)
    #[serde(default)]
    pub email: Option<String>,
    /// Lead source
    pub source: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateLeadRequest {
    /// Updated name
    #[serde(default)]
    pub name: Option<String>,
    /// Updated phone
    #[serde(default)]
    pub phone: Option<String>,
    /// Updated email
    #[serde(default)]
    pub email: Option<String>,
    /// Updated source
    #[serde(default)]
    pub source: Option<String>,
    /// Additional notes
    #[serde(default)]
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateLeadStatusRequest {
    /// New status value
    pub status: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct AssignLeadRequest {
    /// User ID to assign the lead to
    pub assigned_to: i64,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct LogActivityRequest {
    /// Activity type (e.g. call, email, visit)
    pub activity_type: String,
    /// Activity description
    pub description: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct LeadActivityResponse {
    /// Activity ID
    pub id: i64,
    /// Associated lead ID
    pub lead_id: i64,
    /// Activity type
    pub activity_type: String,
    /// Activity description
    pub description: String,
    /// User who performed the activity
    pub performed_by: i64,
    /// Creation timestamp (RFC 3339)
    pub created_at: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct ConvertLeadRequest {
    /// Optional plan ID to assign to the new customer
    #[serde(default)]
    pub plan_id: Option<i64>,
    /// Optional installation address
    #[serde(default)]
    pub address: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ConvertLeadResponse {
    /// Original lead ID
    pub lead_id: i64,
    /// Created customer ID
    pub customer_id: i64,
    /// Generated customer code
    pub customer_code: String,
    /// Conversion status
    pub status: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PipelineResponse {
    /// Pipeline stages with lead counts
    pub stages: Vec<serde_json::Value>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct LeadStatsResponse {
    /// Total leads count
    pub total: i64,
    /// Leads by status breakdown
    pub by_status: serde_json::Value,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct LeadSourceResponse {
    /// Source name
    pub source: String,
    /// Number of leads from this source
    pub count: i64,
}

// ── Stub handler functions (for OpenAPI spec only) ───────────────────

/// List all leads with pagination
#[utoipa::path(
    get,
    path = "/api/v1/leads",
    tag = "Leads",
    params(
        ("page" = Option<i64>, Query, description = "Page number"),
        ("limit" = Option<i64>, Query, description = "Items per page"),
    ),
    responses(
        (status = 200, description = "Paginated list of leads"),
        (status = 403, description = "Insufficient permissions")
    ),
    security(("bearer_auth" = []))
)]
pub async fn list_leads() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Create a new lead
#[utoipa::path(
    post,
    path = "/api/v1/leads",
    tag = "Leads",
    request_body = CreateLeadRequest,
    responses(
        (status = 201, description = "Lead created", body = LeadResponse),
        (status = 403, description = "Insufficient permissions"),
        (status = 422, description = "Validation error")
    ),
    security(("bearer_auth" = []))
)]
pub async fn create_lead() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Get a single lead by ID
#[utoipa::path(
    get,
    path = "/api/v1/leads/{id}",
    tag = "Leads",
    params(("id" = i64, Path, description = "Lead ID")),
    responses(
        (status = 200, description = "Lead details", body = LeadResponse),
        (status = 404, description = "Lead not found")
    ),
    security(("bearer_auth" = []))
)]
pub async fn get_lead() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Update lead details
#[utoipa::path(
    put,
    path = "/api/v1/leads/{id}",
    tag = "Leads",
    params(("id" = i64, Path, description = "Lead ID")),
    request_body = UpdateLeadRequest,
    responses(
        (status = 200, description = "Lead updated", body = LeadResponse),
        (status = 403, description = "Insufficient permissions"),
        (status = 404, description = "Lead not found")
    ),
    security(("bearer_auth" = []))
)]
pub async fn update_lead() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Update a lead's status
#[utoipa::path(
    patch,
    path = "/api/v1/leads/{id}/status",
    tag = "Leads",
    params(("id" = i64, Path, description = "Lead ID")),
    request_body = UpdateLeadStatusRequest,
    responses(
        (status = 200, description = "Status updated", body = LeadResponse),
        (status = 403, description = "Insufficient permissions"),
        (status = 404, description = "Lead not found")
    ),
    security(("bearer_auth" = []))
)]
pub async fn update_lead_status() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Assign a lead to a user
#[utoipa::path(
    post,
    path = "/api/v1/leads/{id}/assign",
    tag = "Leads",
    params(("id" = i64, Path, description = "Lead ID")),
    request_body = AssignLeadRequest,
    responses(
        (status = 200, description = "Lead assigned", body = LeadResponse),
        (status = 403, description = "Insufficient permissions"),
        (status = 404, description = "Lead not found")
    ),
    security(("bearer_auth" = []))
)]
pub async fn assign_lead() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Log an activity against a lead
#[utoipa::path(
    post,
    path = "/api/v1/leads/{id}/activities",
    tag = "Leads",
    params(("id" = i64, Path, description = "Lead ID")),
    request_body = LogActivityRequest,
    responses(
        (status = 201, description = "Activity logged", body = LeadActivityResponse),
        (status = 403, description = "Insufficient permissions"),
        (status = 404, description = "Lead not found")
    ),
    security(("bearer_auth" = []))
)]
pub async fn log_activity() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// List all activities for a lead
#[utoipa::path(
    get,
    path = "/api/v1/leads/{id}/activities",
    tag = "Leads",
    params(("id" = i64, Path, description = "Lead ID")),
    responses(
        (status = 200, description = "List of lead activities", body = Vec<LeadActivityResponse>),
        (status = 404, description = "Lead not found")
    ),
    security(("bearer_auth" = []))
)]
pub async fn list_activities() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Convert a qualified lead into a customer
#[utoipa::path(
    post,
    path = "/api/v1/leads/{id}/convert",
    tag = "Leads",
    params(("id" = i64, Path, description = "Lead ID")),
    request_body = ConvertLeadRequest,
    responses(
        (status = 201, description = "Lead converted to customer", body = ConvertLeadResponse),
        (status = 403, description = "Insufficient permissions"),
        (status = 404, description = "Lead not found"),
        (status = 422, description = "Lead cannot be converted in current status")
    ),
    security(("bearer_auth" = []))
)]
pub async fn convert_lead() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Get the sales pipeline overview
#[utoipa::path(
    get,
    path = "/api/v1/leads/pipeline",
    tag = "Leads",
    responses(
        (status = 200, description = "Pipeline overview", body = PipelineResponse),
        (status = 403, description = "Insufficient permissions")
    ),
    security(("bearer_auth" = []))
)]
pub async fn get_pipeline() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Get lead statistics
#[utoipa::path(
    get,
    path = "/api/v1/leads/stats",
    tag = "Leads",
    responses(
        (status = 200, description = "Lead statistics", body = LeadStatsResponse),
        (status = 403, description = "Insufficient permissions")
    ),
    security(("bearer_auth" = []))
)]
pub async fn get_stats() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// List all lead sources with counts
#[utoipa::path(
    get,
    path = "/api/v1/leads/sources",
    tag = "Leads",
    responses(
        (status = 200, description = "List of lead sources"),
        (status = 403, description = "Insufficient permissions")
    ),
    security(("bearer_auth" = []))
)]
pub async fn list_sources() -> axum::Json<serde_json::Value> {
    unimplemented!()
}
