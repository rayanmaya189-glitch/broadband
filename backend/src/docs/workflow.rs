/// OpenAPI schemas and stub handlers for Workflow (Approval) endpoints.
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

// ── Request / Response types ─────────────────────────────────────────

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateApprovalRequest {
    /// Workflow type (e.g. "device_provisioning", "service_change")
    pub workflow_type: String,
    /// Resource type being submitted for approval
    pub resource_type: String,
    /// Resource ID
    pub resource_id: i64,
    /// Optional reason or note
    #[serde(default)]
    pub reason: Option<String>,
    /// JSON payload with workflow-specific data
    pub payload: serde_json::Value,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct ReviewApprovalRequest {
    /// Reviewer comment
    #[serde(default)]
    pub comment: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ApprovalRequestResponse {
    /// Request ID
    pub id: i64,
    /// Workflow type
    pub workflow_type: String,
    /// Resource type
    pub resource_type: String,
    /// Resource ID
    pub resource_id: i64,
    /// User who requested
    pub requested_by: i64,
    /// Request status (e.g. "pending", "approved", "rejected")
    pub status: String,
    /// Reason for request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// Reviewer user ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reviewer_id: Option<i64>,
    /// Reviewer comment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reviewer_comment: Option<String>,
    /// When the request was created (RFC 3339)
    pub requested_at: String,
    /// When the request was reviewed (RFC 3339, null if pending)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reviewed_at: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct WorkflowResponse {
    /// Workflow ID
    pub id: i64,
    /// Workflow type
    pub workflow_type: String,
    /// Current status
    pub status: String,
    /// Resource type
    pub resource_type: String,
    /// Resource ID
    pub resource_id: i64,
}

// ── Stub handler functions (for OpenAPI spec only) ───────────────────

/// Create a new approval request
#[utoipa::path(
    post,
    path = "/api/v1/workflow/approvals",
    tag = "Workflow",
    request_body = CreateApprovalRequest,
    responses(
        (status = 201, description = "Approval request created"),
        (status = 403, description = "Forbidden"),
        (status = 422, description = "Validation error")
    ),
    security(("bearer_auth" = []))
)]
pub async fn create_approval_request() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// List pending approval requests
#[utoipa::path(
    get,
    path = "/api/v1/workflow/approvals/pending",
    tag = "Workflow",
    responses(
        (status = 200, description = "List of pending approvals", body = Vec<ApprovalRequestResponse>),
        (status = 403, description = "Forbidden")
    ),
    security(("bearer_auth" = []))
)]
pub async fn list_pending_approvals() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Get a specific approval request by ID
#[utoipa::path(
    get,
    path = "/api/v1/workflow/approvals/{id}",
    tag = "Workflow",
    params(("id" = i64, Path, description = "Approval request ID")),
    responses(
        (status = 200, description = "Approval request details", body = ApprovalRequestResponse),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Request not found")
    ),
    security(("bearer_auth" = []))
)]
pub async fn get_approval_request() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Approve a pending approval request
#[utoipa::path(
    post,
    path = "/api/v1/workflow/approvals/{id}/approve",
    tag = "Workflow",
    params(("id" = i64, Path, description = "Approval request ID")),
    request_body = ReviewApprovalRequest,
    responses(
        (status = 200, description = "Request approved"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Request not found")
    ),
    security(("bearer_auth" = []))
)]
pub async fn approve_request() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Reject a pending approval request
#[utoipa::path(
    post,
    path = "/api/v1/workflow/approvals/{id}/reject",
    tag = "Workflow",
    params(("id" = i64, Path, description = "Approval request ID")),
    request_body = ReviewApprovalRequest,
    responses(
        (status = 200, description = "Request rejected"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Request not found")
    ),
    security(("bearer_auth" = []))
)]
pub async fn reject_request() -> axum::Json<serde_json::Value> {
    unimplemented!()
}
