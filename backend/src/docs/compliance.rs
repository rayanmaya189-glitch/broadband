/// OpenAPI schemas and stub handlers for Compliance endpoints.
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

// ── Request Types ────────────────────────────────────────────────────

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateComplianceItemRequest {
    /// Item name
    pub name: String,
    /// Item type (kyc, consent, data_retention)
    pub item_type: String,
    /// Description
    pub description: String,
    /// Regulatory reference
    #[serde(default)]
    pub regulatory_reference: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateComplianceItemRequest {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateAuditRequest {
    /// Compliance item ID to audit
    pub compliance_item_id: i64,
    /// Audit description
    pub description: String,
    /// Scheduled date (YYYY-MM-DD)
    pub scheduled_date: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateAuditRequest {
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub findings: Option<String>,
    #[serde(default)]
    pub completed_date: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct ReportViolationRequest {
    /// Audit ID
    pub audit_id: i64,
    /// Violation description
    pub description: String,
    /// Severity (low, medium, high, critical)
    pub severity: String,
    #[serde(default)]
    pub corrective_action: Option<String>,
}

// ── Response Types ───────────────────────────────────────────────────

#[derive(Debug, Serialize, ToSchema)]
pub struct ComplianceItemResponse {
    pub id: i64,
    pub name: String,
    pub item_type: String,
    pub description: String,
    pub status: String,
    pub regulatory_reference: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AuditResponse {
    pub id: i64,
    pub compliance_item_id: i64,
    pub description: String,
    pub status: String,
    pub scheduled_date: String,
    pub completed_date: Option<String>,
    pub findings: Option<String>,
    pub created_by: Option<i64>,
    pub created_at: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ViolationResponse {
    pub id: i64,
    pub audit_id: i64,
    pub description: String,
    pub severity: String,
    pub status: String,
    pub corrective_action: Option<String>,
    pub resolved_at: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ComplianceReport {
    pub total_items: i64,
    pub compliant_items: i64,
    pub non_compliant_items: i64,
    pub total_audits: i64,
    pub pending_audits: i64,
    pub total_violations: i64,
    pub unresolved_violations: i64,
    pub compliance_score: f64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct KycResponse {
    pub id: i64,
    pub customer_id: i64,
    pub document_type: String,
    pub status: String,
    pub provider: Option<String>,
    pub verified_at: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ConsentResponse {
    pub id: i64,
    pub customer_id: i64,
    pub consent_type: String,
    pub granted: bool,
    pub collection_channel: String,
    pub revoked_at: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct RetentionPolicyResponse {
    pub id: i64,
    pub entity_type: String,
    pub retention_days: i32,
    pub action: String,
    pub is_active: bool,
    pub description: Option<String>,
    pub legal_basis: Option<String>,
}

// ── Stub handler functions (for OpenAPI spec only) ───────────────────

// ── Compliance Items ──

/// List all compliance items
#[utoipa::path(
    get,
    path = "/api/v1/compliance/items",
    tag = "Compliance",
    responses(
        (status = 200, description = "List of compliance items", body = Vec<ComplianceItemResponse>),
        (status = 401, description = "Unauthorized")
    ),
    security(("bearer_auth" = []))
)]
pub async fn list_compliance_items() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Create a new compliance item
#[utoipa::path(
    post,
    path = "/api/v1/compliance/items",
    tag = "Compliance",
    request_body = CreateComplianceItemRequest,
    responses(
        (status = 201, description = "Compliance item created", body = ComplianceItemResponse),
        (status = 422, description = "Validation error")
    ),
    security(("bearer_auth" = []))
)]
pub async fn create_compliance_item() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Update an existing compliance item
#[utoipa::path(
    put,
    path = "/api/v1/compliance/items/{id}",
    tag = "Compliance",
    params(("id" = i64, Path, description = "Compliance item ID")),
    request_body = UpdateComplianceItemRequest,
    responses(
        (status = 200, description = "Compliance item updated", body = ComplianceItemResponse),
        (status = 404, description = "Compliance item not found")
    ),
    security(("bearer_auth" = []))
)]
pub async fn update_compliance_item() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Get compliance report summary
#[utoipa::path(
    get,
    path = "/api/v1/compliance/report",
    tag = "Compliance",
    responses(
        (status = 200, description = "Compliance report", body = ComplianceReport),
        (status = 401, description = "Unauthorized")
    ),
    security(("bearer_auth" = []))
)]
pub async fn get_compliance_report() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

// ── Compliance Audits ──

/// List all compliance audits
#[utoipa::path(
    get,
    path = "/api/v1/compliance/audits",
    tag = "Compliance",
    responses(
        (status = 200, description = "List of audits", body = Vec<AuditResponse>),
        (status = 401, description = "Unauthorized")
    ),
    security(("bearer_auth" = []))
)]
pub async fn list_audits() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Create a new compliance audit
#[utoipa::path(
    post,
    path = "/api/v1/compliance/audits",
    tag = "Compliance",
    request_body = CreateAuditRequest,
    responses(
        (status = 201, description = "Audit created", body = AuditResponse),
        (status = 404, description = "Compliance item not found"),
        (status = 422, description = "Validation error")
    ),
    security(("bearer_auth" = []))
)]
pub async fn create_audit() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Update a compliance audit
#[utoipa::path(
    put,
    path = "/api/v1/compliance/audits/{id}",
    tag = "Compliance",
    params(("id" = i64, Path, description = "Audit ID")),
    request_body = UpdateAuditRequest,
    responses(
        (status = 200, description = "Audit updated", body = AuditResponse),
        (status = 404, description = "Audit not found")
    ),
    security(("bearer_auth" = []))
)]
pub async fn update_audit() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Get audit report
#[utoipa::path(
    get,
    path = "/api/v1/compliance/audits/{id}/report",
    tag = "Compliance",
    params(("id" = i64, Path, description = "Audit ID")),
    responses(
        (status = 200, description = "Audit report"),
        (status = 404, description = "Audit not found")
    ),
    security(("bearer_auth" = []))
)]
pub async fn get_audit_report() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

// ── Violations ──

/// List all violations
#[utoipa::path(
    get,
    path = "/api/v1/compliance/violations",
    tag = "Compliance",
    responses(
        (status = 200, description = "List of violations", body = Vec<ViolationResponse>),
        (status = 401, description = "Unauthorized")
    ),
    security(("bearer_auth" = []))
)]
pub async fn list_violations() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Report a new violation
#[utoipa::path(
    post,
    path = "/api/v1/compliance/violations",
    tag = "Compliance",
    request_body = ReportViolationRequest,
    responses(
        (status = 201, description = "Violation reported", body = ViolationResponse),
        (status = 404, description = "Audit not found"),
        (status = 422, description = "Validation error")
    ),
    security(("bearer_auth" = []))
)]
pub async fn report_violation() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Resolve a violation
#[utoipa::path(
    put,
    path = "/api/v1/compliance/violations/{id}/resolve",
    tag = "Compliance",
    params(("id" = i64, Path, description = "Violation ID")),
    responses(
        (status = 200, description = "Violation resolved", body = ViolationResponse),
        (status = 404, description = "Violation not found")
    ),
    security(("bearer_auth" = []))
)]
pub async fn resolve_violation() -> axum::Json<serde_json::Value> {
    unimplemented!()
}
