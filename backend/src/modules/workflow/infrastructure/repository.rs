use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait};
use crate::shared::errors::AppError;
use crate::modules::workflow::domain::entities::{WorkflowInstance, WorkflowStep};

/// Workflow repository for database queries.
pub struct WorkflowRepository;

impl WorkflowRepository {
    /// Find running workflow instances.
    pub async fn find_running(db: &DatabaseConnection) -> Result<Vec<workflow_instance::Model>, AppError> {
        Ok(WorkflowInstance::find()
            .filter(workflow_instance::Column::Status.eq("running"))
            .all(db)
            .await?)
    }

    /// Find workflow instances by reference entity.
    pub async fn find_by_reference(
        db: &DatabaseConnection,
        reference_type: &str,
        reference_id: i64,
    ) -> Result<Vec<workflow_instance::Model>, AppError> {
        Ok(WorkflowInstance::find()
            .filter(workflow_instance::Column::ReferenceType.eq(reference_type))
            .filter(workflow_instance::Column::ReferenceId.eq(reference_id))
            .all(db)
            .await?)
    }

    /// Find failed workflow instances for retry.
    pub async fn find_failed(db: &DatabaseConnection) -> Result<Vec<workflow_instance::Model>, AppError> {
        Ok(WorkflowInstance::find()
            .filter(workflow_instance::Column::Status.eq("failed"))
            .all(db)
            .await?)
    }

    /// Find pending steps for a workflow.
    pub async fn find_pending_steps(
        db: &DatabaseConnection,
        workflow_instance_id: i64,
    ) -> Result<Vec<workflow_step::Model>, AppError> {
        Ok(WorkflowStep::find()
            .filter(workflow_step::Column::WorkflowInstanceId.eq(workflow_instance_id))
            .filter(workflow_step::Column::Status.eq("pending"))
            .order_by_asc(workflow_step::Column::StepOrder)
            .all(db)
            .await?)
    }

    /// Find failed steps for retry.
    pub async fn find_failed_steps(
        db: &DatabaseConnection,
        workflow_instance_id: i64,
    ) -> Result<Vec<workflow_step::Model>, AppError> {
        Ok(WorkflowStep::find()
            .filter(workflow_step::Column::WorkflowInstanceId.eq(workflow_instance_id))
            .filter(workflow_step::Column::Status.eq("failed"))
            .all(db)
            .await?)
    }
}

use crate::modules::workflow::domain::entities::{workflow_instance, workflow_step};
