use crate::modules::workflow::domain::approval::ApprovalWorkflowType;
use crate::modules::workflow::infrastructure::entities::approval_request;
use crate::shared::errors::AppError;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};

pub struct ApprovalRepository;

impl ApprovalRepository {
    /// Create a new approval request
    pub async fn create(
        db: &DatabaseConnection,
        request: &crate::modules::workflow::domain::approval::ApprovalRequest,
    ) -> Result<i64, AppError> {
        let active = approval_request::ActiveModel {
            workflow_type: Set(request.workflow_type.to_string()),
            resource_type: Set(request.resource_type.clone()),
            resource_id: Set(request.resource_id),
            requested_by: Set(request.requested_by),
            branch_id: Set(request.branch_id),
            status: Set(request.status.to_string()),
            payload: Set(request.payload.clone()),
            reason: Set(request.reason.clone()),
            reviewer_id: Set(request.reviewer_id),
            reviewer_comment: Set(request.reviewer_comment.clone()),
            requested_at: Set(request.requested_at.into()),
            reviewed_at: Set(request.reviewed_at.map(|dt| dt.into())),
            expires_at: Set(request.expires_at.map(|dt| dt.into())),
            created_at: Set(chrono::Utc::now().into()),
            updated_at: Set(chrono::Utc::now().into()),
            ..Default::default()
        };

        let result = active.insert(db).await.map_err(|e| {
            AppError::Internal(anyhow::anyhow!("Failed to create approval request: {}", e))
        })?;

        Ok(result.id)
    }

    /// Find an approval request by ID
    pub async fn find_by_id(
        db: &DatabaseConnection,
        id: i64,
    ) -> Result<Option<approval_request::Model>, AppError> {
        approval_request::Entity::find_by_id(id)
            .one(db)
            .await
            .map_err(|e| {
                AppError::Internal(anyhow::anyhow!("Failed to find approval request: {}", e))
            })
    }

    /// Find pending approval requests
    pub async fn find_pending(
        db: &DatabaseConnection,
    ) -> Result<Vec<approval_request::Model>, AppError> {
        approval_request::Entity::find()
            .filter(approval_request::Column::Status.eq("pending"))
            .all(db)
            .await
            .map_err(|e| {
                AppError::Internal(anyhow::anyhow!("Failed to find pending requests: {}", e))
            })
    }

    /// Find pending requests for a specific workflow type
    pub async fn find_pending_by_type(
        db: &DatabaseConnection,
        workflow_type: &ApprovalWorkflowType,
    ) -> Result<Vec<approval_request::Model>, AppError> {
        approval_request::Entity::find()
            .filter(approval_request::Column::Status.eq("pending"))
            .filter(approval_request::Column::WorkflowType.eq(workflow_type.to_string()))
            .all(db)
            .await
            .map_err(|e| {
                AppError::Internal(anyhow::anyhow!("Failed to find pending requests: {}", e))
            })
    }

    /// Approve a request
    pub async fn approve(
        db: &DatabaseConnection,
        id: i64,
        reviewer_id: i64,
        comment: Option<String>,
    ) -> Result<(), AppError> {
        let model = approval_request::Entity::find_by_id(id)
            .one(db)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to find request: {}", e)))?
            .ok_or_else(|| AppError::NotFound("Approval request not found".to_string()))?;

        if model.status != "pending" {
            return Err(AppError::BadRequest("Request is not pending".to_string()));
        }

        let now = chrono::Utc::now();
        let mut active: approval_request::ActiveModel = model.into();
        active.status = Set("approved".to_string());
        active.reviewer_id = Set(Some(reviewer_id));
        active.reviewer_comment = Set(comment);
        active.reviewed_at = Set(Some(now.into()));
        active.updated_at = Set(now.into());

        active
            .update(db)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to approve request: {}", e)))?;

        Ok(())
    }

    /// Reject a request
    pub async fn reject(
        db: &DatabaseConnection,
        id: i64,
        reviewer_id: i64,
        comment: String,
    ) -> Result<(), AppError> {
        let model = approval_request::Entity::find_by_id(id)
            .one(db)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to find request: {}", e)))?
            .ok_or_else(|| AppError::NotFound("Approval request not found".to_string()))?;

        if model.status != "pending" {
            return Err(AppError::BadRequest("Request is not pending".to_string()));
        }

        let now = chrono::Utc::now();
        let mut active: approval_request::ActiveModel = model.into();
        active.status = Set("rejected".to_string());
        active.reviewer_id = Set(Some(reviewer_id));
        active.reviewer_comment = Set(Some(comment));
        active.reviewed_at = Set(Some(now.into()));
        active.updated_at = Set(now.into());

        active
            .update(db)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to reject request: {}", e)))?;

        Ok(())
    }
}
