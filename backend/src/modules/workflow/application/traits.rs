use async_trait::async_trait;
use sea_orm::{DatabaseConnection};
use crate::shared::errors::AppError;

pub type WorkflowInstanceModel = crate::modules::workflow::domain::entities::workflow_instance::Model;
pub type WorkflowStepModel = crate::modules::workflow::domain::entities::workflow_step::Model;

#[async_trait]
pub trait WorkflowServiceTrait: Send + Sync {
    async fn list_approval_requests(
        &self,
        db: &DatabaseConnection,
        status: Option<&str>,
    ) -> Result<Vec<WorkflowInstanceModel>, AppError>;

    async fn create_approval_request(
        &self,
        db: &DatabaseConnection,
        operation: String,
        resource_type: String,
        resource_id: i64,
        requested_by: i64,
        payload: Option<serde_json::Value>,
    ) -> Result<WorkflowInstanceModel, AppError>;

    async fn approve_request(
        &self,
        db: &DatabaseConnection,
        id: i64,
        approved_by: i64,
        comments: Option<String>,
    ) -> Result<WorkflowInstanceModel, AppError>;

    async fn reject_request(
        &self,
        db: &DatabaseConnection,
        id: i64,
        rejected_by: i64,
        reason: String,
    ) -> Result<WorkflowInstanceModel, AppError>;

    async fn list_workflow_steps(
        &self,
        db: &DatabaseConnection,
        instance_id: i64,
    ) -> Result<Vec<WorkflowStepModel>, AppError>;
}
