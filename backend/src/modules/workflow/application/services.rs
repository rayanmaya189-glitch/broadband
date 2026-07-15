use sea_orm::DatabaseConnection;
use crate::shared::errors::AppError;
use crate::modules::workflow::domain::approval::{ApprovalRequest as DomainApprovalRequest, ApprovalWorkflowType, ApprovalStatus};
use crate::modules::workflow::infrastructure::repository::ApprovalRepository;

pub struct ApprovalService;

impl ApprovalService {
    /// Create a new approval request
    pub async fn create_request(
        db: &DatabaseConnection,
        workflow_type: ApprovalWorkflowType,
        resource_type: String,
        resource_id: i64,
        requested_by: i64,
        branch_id: Option<i64>,
        payload: serde_json::Value,
    ) -> Result<i64, AppError> {
        let request = DomainApprovalRequest::new(
            workflow_type,
            resource_type,
            resource_id,
            requested_by,
            branch_id,
            payload,
        );

        ApprovalRepository::create(db, &request).await
    }

    /// Get an approval request by ID
    pub async fn get_request(
        db: &DatabaseConnection,
        id: i64,
    ) -> Result<DomainApprovalRequest, AppError> {
        let model = ApprovalRepository::find_by_id(db, id).await?
            .ok_or_else(|| AppError::NotFound("Approval request not found".to_string()))?;

        Ok(DomainApprovalRequest {
            id: model.id,
            workflow_type: ApprovalWorkflowType::from_str(&model.workflow_type),
            resource_type: model.resource_type,
            resource_id: model.resource_id,
            requested_by: model.requested_by,
            branch_id: model.branch_id,
            status: ApprovalStatus::from_str(&model.status),
            payload: model.payload,
            reason: model.reason,
            reviewer_id: model.reviewer_id,
            reviewer_comment: model.reviewer_comment,
            requested_at: model.requested_at.with_timezone(&chrono::Utc),
            reviewed_at: model.reviewed_at.map(|dt| dt.with_timezone(&chrono::Utc)),
            expires_at: model.expires_at.map(|dt| dt.with_timezone(&chrono::Utc)),
        })
    }

    /// List pending approval requests
    pub async fn list_pending(
        db: &DatabaseConnection,
    ) -> Result<Vec<DomainApprovalRequest>, AppError> {
        let models = ApprovalRepository::find_pending(db).await?;

        Ok(models.into_iter().map(|model| {
            DomainApprovalRequest {
                id: model.id,
                workflow_type: ApprovalWorkflowType::from_str(&model.workflow_type),
                resource_type: model.resource_type,
                resource_id: model.resource_id,
                requested_by: model.requested_by,
                branch_id: model.branch_id,
                status: ApprovalStatus::from_str(&model.status),
                payload: model.payload,
                reason: model.reason,
                reviewer_id: model.reviewer_id,
                reviewer_comment: model.reviewer_comment,
                requested_at: model.requested_at.with_timezone(&chrono::Utc),
                reviewed_at: model.reviewed_at.map(|dt| dt.with_timezone(&chrono::Utc)),
                expires_at: model.expires_at.map(|dt| dt.with_timezone(&chrono::Utc)),
            }
        }).collect())
    }

    /// Approve a request
    pub async fn approve_request(
        db: &DatabaseConnection,
        id: i64,
        reviewer_id: i64,
        comment: Option<String>,
    ) -> Result<(), AppError> {
        let request = Self::get_request(db, id).await?;
        if !request.can_review() {
            return Err(AppError::BadRequest("Request cannot be reviewed".to_string()));
        }

        ApprovalRepository::approve(db, id, reviewer_id, comment).await
    }

    /// Reject a request
    pub async fn reject_request(
        db: &DatabaseConnection,
        id: i64,
        reviewer_id: i64,
        comment: String,
    ) -> Result<(), AppError> {
        let request = Self::get_request(db, id).await?;
        if !request.can_review() {
            return Err(AppError::BadRequest("Request cannot be reviewed".to_string()));
        }

        ApprovalRepository::reject(db, id, reviewer_id, comment).await
    }
}
