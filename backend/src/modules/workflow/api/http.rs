use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::modules::workflow::application::services::ApprovalService;
use crate::modules::workflow::domain::approval::ApprovalWorkflowType;
use crate::shared::app_state::AppState;
use crate::shared::errors::AppError;
use crate::shared::middleware::auth::{has_permission, UserContext};

#[derive(Deserialize)]
pub struct CreateApprovalRequest {
    pub workflow_type: String,
    pub resource_type: String,
    pub resource_id: i64,
    pub reason: Option<String>,
    pub payload: serde_json::Value,
}

#[derive(Deserialize)]
pub struct ReviewApprovalRequest {
    pub comment: Option<String>,
}

#[derive(Serialize)]
pub struct ApprovalRequestResponse {
    pub id: i64,
    pub workflow_type: String,
    pub resource_type: String,
    pub resource_id: i64,
    pub requested_by: i64,
    pub status: String,
    pub reason: Option<String>,
    pub reviewer_id: Option<i64>,
    pub reviewer_comment: Option<String>,
    pub requested_at: String,
    pub reviewed_at: Option<String>,
}

/// Create a new approval request
pub async fn create_approval_request(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Json(input): Json<CreateApprovalRequest>,
) -> Result<(StatusCode, Json<serde_json::Value>), AppError> {
    if !has_permission(&user, "workflow.approval.create") {
        return Err(AppError::Forbidden("Permission required".to_string()));
    }

    let workflow_type = ApprovalWorkflowType::from_str(&input.workflow_type);

    let id = ApprovalService::create_request(
        &state.db,
        workflow_type,
        input.resource_type,
        input.resource_id,
        user.user_id,
        user.branch_id,
        input.payload,
    )
    .await?;
    if let Err(e) = crate::infrastructure::messaging::outbox::insert_outbox_event(
        &state.db,
        "workflow.approval.created",
        "approval_request",
        id,
        serde_json::json!({"request_id": id, "workflow_type": input.workflow_type}),
        None,
        Some(user.user_id),
        user.branch_id,
    )
    .await
    {
        tracing::error!(error = %e, "Failed to publish workflow.approval.created event");
    }

    Ok((
        StatusCode::CREATED,
        Json(serde_json::json!({
            "id": id,
            "message": "Approval request created"
        })),
    ))
}

/// List pending approval requests
pub async fn list_pending_approvals(
    State(state): State<Arc<AppState>>,
    user: UserContext,
) -> Result<Json<Vec<ApprovalRequestResponse>>, AppError> {
    if !has_permission(&user, "workflow.approval.read") {
        return Err(AppError::Forbidden("Permission required".to_string()));
    }

    let requests = ApprovalService::list_pending(&state.db).await?;

    let responses: Vec<ApprovalRequestResponse> = requests
        .into_iter()
        .map(|r| ApprovalRequestResponse {
            id: r.id,
            workflow_type: r.workflow_type.to_string(),
            resource_type: r.resource_type,
            resource_id: r.resource_id,
            requested_by: r.requested_by,
            status: r.status.to_string(),
            reason: r.reason,
            reviewer_id: r.reviewer_id,
            reviewer_comment: r.reviewer_comment,
            requested_at: r.requested_at.to_rfc3339(),
            reviewed_at: r.reviewed_at.map(|dt| dt.to_rfc3339()),
        })
        .collect();

    Ok(Json(responses))
}

/// Get a specific approval request
pub async fn get_approval_request(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Path(id): Path<i64>,
) -> Result<Json<ApprovalRequestResponse>, AppError> {
    if !has_permission(&user, "workflow.approval.read") {
        return Err(AppError::Forbidden("Permission required".to_string()));
    }

    let request = ApprovalService::get_request(&state.db, id).await?;

    Ok(Json(ApprovalRequestResponse {
        id: request.id,
        workflow_type: request.workflow_type.to_string(),
        resource_type: request.resource_type,
        resource_id: request.resource_id,
        requested_by: request.requested_by,
        status: request.status.to_string(),
        reason: request.reason,
        reviewer_id: request.reviewer_id,
        reviewer_comment: request.reviewer_comment,
        requested_at: request.requested_at.to_rfc3339(),
        reviewed_at: request.reviewed_at.map(|dt| dt.to_rfc3339()),
    }))
}

/// Approve an approval request
pub async fn approve_request(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Path(id): Path<i64>,
    Json(input): Json<ReviewApprovalRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    if !has_permission(&user, "workflow.approval.review") {
        return Err(AppError::Forbidden("Permission required".to_string()));
    }

    ApprovalService::approve_request(&state.db, id, user.user_id, input.comment).await?;
    if let Err(e) = crate::infrastructure::messaging::outbox::insert_outbox_event(
        &state.db,
        "workflow.approval.approved",
        "approval_request",
        id,
        serde_json::json!({"request_id": id}),
        None,
        Some(user.user_id),
        user.branch_id,
    )
    .await
    {
        tracing::error!(error = %e, "Failed to publish workflow.approval.approved event");
    }

    Ok(Json(serde_json::json!({
        "message": "Approval request approved"
    })))
}

/// Reject an approval request
pub async fn reject_request(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Path(id): Path<i64>,
    Json(input): Json<ReviewApprovalRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    if !has_permission(&user, "workflow.approval.review") {
        return Err(AppError::Forbidden("Permission required".to_string()));
    }

    let comment = input
        .comment
        .unwrap_or_else(|| "No comment provided".to_string());
    ApprovalService::reject_request(&state.db, id, user.user_id, comment).await?;
    if let Err(e) = crate::infrastructure::messaging::outbox::insert_outbox_event(
        &state.db,
        "workflow.approval.rejected",
        "approval_request",
        id,
        serde_json::json!({"request_id": id}),
        None,
        Some(user.user_id),
        user.branch_id,
    )
    .await
    {
        tracing::error!(error = %e, "Failed to publish workflow.approval.rejected event");
    }

    Ok(Json(serde_json::json!({
        "message": "Approval request rejected"
    })))
}
