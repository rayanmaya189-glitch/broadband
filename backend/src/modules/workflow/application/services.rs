use sea_orm::{DatabaseConnection, EntityTrait, ActiveModelTrait, Set, QueryFilter, ColumnTrait};
use chrono::Utc;
use crate::shared::errors::AppError;
use crate::modules::workflow::domain::entities::{
    workflow_instance, workflow_step,
    WorkflowInstance, WorkflowInstanceActiveModel,
    WorkflowStep, WorkflowStepActiveModel,
};

/// Workflow service for orchestrating sagas and long-running processes.
pub struct WorkflowService;

impl WorkflowService {
    // ── Workflow Instances ──

    pub async fn list_instances(db: &DatabaseConnection) -> Result<Vec<workflow_instance::Model>, AppError> {
        Ok(WorkflowInstance::find().all(db).await?)
    }

    pub async fn get_instance(db: &DatabaseConnection, id: i64) -> Result<workflow_instance::Model, AppError> {
        WorkflowInstance::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Workflow instance {} not found", id)))
    }

    pub async fn start_workflow(
        db: &DatabaseConnection,
        workflow_type: String,
        reference_type: String,
        reference_id: i64,
        input_data: serde_json::Value,
        total_steps: i32,
        initiated_by: Option<i64>,
        branch_id: Option<i64>,
    ) -> Result<workflow_instance::Model, AppError> {
        let now = Utc::now();
        let instance = WorkflowInstanceActiveModel {
            workflow_type: Set(workflow_type),
            reference_type: Set(reference_type),
            reference_id: Set(reference_id),
            status: Set("running".to_string()),
            current_step: Set(0),
            total_steps: Set(total_steps),
            input_data: Set(input_data),
            output_data: Set(None),
            error_message: Set(None),
            initiated_by: Set(initiated_by),
            branch_id: Set(branch_id),
            started_at: Set(now),
            completed_at: Set(None),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };
        Ok(instance.insert(db).await?)
    }

    pub async fn advance_workflow(
        db: &DatabaseConnection,
        instance_id: i64,
        step_output: serde_json::Value,
    ) -> Result<workflow_instance::Model, AppError> {
        let instance = Self::get_instance(db, instance_id).await?;
        let mut active: workflow_instance::ActiveModel = instance.into();

        let current = active.current_step.as_ref();
        let total = active.total_steps.as_ref();

        if current >= total {
            return Err(AppError::BadRequest("Workflow already completed".into()));
        }

        active.current_step = Set(current + 1);

        // Check if this is the last step
        if current + 1 >= *total {
            active.status = Set("completed".to_string());
            active.completed_at = Set(Some(Utc::now()));
            active.output_data = Set(Some(step_output));
        }

        active.updated_at = Set(Utc::now());
        Ok(active.update(db).await?)
    }

    pub async fn fail_workflow(
        db: &DatabaseConnection,
        instance_id: i64,
        error_message: String,
    ) -> Result<workflow_instance::Model, AppError> {
        let instance = Self::get_instance(db, instance_id).await?;
        let mut active: workflow_instance::ActiveModel = instance.into();
        active.status = Set("failed".to_string());
        active.error_message = Set(Some(error_message));
        active.completed_at = Set(Some(Utc::now()));
        active.updated_at = Set(Utc::now());
        Ok(active.update(db).await?)
    }

    pub async fn start_compensation(
        db: &DatabaseConnection,
        instance_id: i64,
    ) -> Result<workflow_instance::Model, AppError> {
        let instance = Self::get_instance(db, instance_id).await?;
        let mut active: workflow_instance::ActiveModel = instance.into();
        active.status = Set("compensation".to_string());
        active.updated_at = Set(Utc::now());
        Ok(active.update(db).await?)
    }

    pub async fn cancel_workflow(
        db: &DatabaseConnection,
        instance_id: i64,
    ) -> Result<workflow_instance::Model, AppError> {
        let instance = Self::get_instance(db, instance_id).await?;
        let mut active: workflow_instance::ActiveModel = instance.into();
        active.status = Set("cancelled".to_string());
        active.completed_at = Set(Some(Utc::now()));
        active.updated_at = Set(Utc::now());
        Ok(active.update(db).await?)
    }

    // ── Workflow Steps ──

    pub async fn add_step(
        db: &DatabaseConnection,
        workflow_instance_id: i64,
        step_name: String,
        step_order: i32,
        target_module: String,
        action: String,
        input_payload: serde_json::Value,
        compensation_action: Option<String>,
    ) -> Result<workflow_step::Model, AppError> {
        let now = Utc::now();
        let step = WorkflowStepActiveModel {
            workflow_instance_id: Set(workflow_instance_id),
            step_name: Set(step_name),
            step_order: Set(step_order),
            target_module: Set(target_module),
            action: Set(action),
            input_payload: Set(input_payload),
            output_payload: Set(None),
            status: Set("pending".to_string()),
            error_message: Set(None),
            retry_count: Set(0),
            max_retries: Set(3),
            compensation_action: Set(compensation_action),
            compensation_executed: Set(false),
            started_at: Set(None),
            completed_at: Set(None),
            created_at: Set(now),
            ..Default::default()
        };
        Ok(step.insert(db).await?)
    }

    pub async fn get_steps_for_workflow(
        db: &DatabaseConnection,
        workflow_instance_id: i64,
    ) -> Result<Vec<workflow_step::Model>, AppError> {
        Ok(WorkflowStep::find()
            .filter(workflow_step::Column::WorkflowInstanceId.eq(workflow_instance_id))
            .order_by_asc(workflow_step::Column::StepOrder)
            .all(db)
            .await?)
    }

    pub async fn complete_step(
        db: &DatabaseConnection,
        step_id: i64,
        output_payload: serde_json::Value,
    ) -> Result<workflow_step::Model, AppError> {
        let step = WorkflowStep::find_by_id(step_id)
            .one(db)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Workflow step {} not found", step_id)))?;
        let mut active: workflow_step::ActiveModel = step.into();
        active.status = Set("completed".to_string());
        active.output_payload = Set(Some(output_payload));
        active.completed_at = Set(Some(Utc::now()));
        Ok(active.update(db).await?)
    }

    pub async fn fail_step(
        db: &DatabaseConnection,
        step_id: i64,
        error_message: String,
    ) -> Result<workflow_step::Model, AppError> {
        let step = WorkflowStep::find_by_id(step_id)
            .one(db)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Workflow step {} not found", step_id)))?;
        let mut active: workflow_step::ActiveModel = step.into();
        let retry = active.retry_count.as_ref() + 1;
        active.status = Set(if retry >= *active.max_retries.as_ref() { "failed".to_string() } else { "pending".to_string() });
        active.error_message = Set(Some(error_message));
        active.retry_count = Set(retry);
        active.completed_at = Set(Some(Utc::now()));
        Ok(active.update(db).await?)
    }

    pub async fn get_instance_stats(db: &DatabaseConnection) -> Result<serde_json::Value, AppError> {
        let total = WorkflowInstance::find().count(db).await? as i64;
        let running = WorkflowInstance::find()
            .filter(workflow_instance::Column::Status.eq("running"))
            .count(db).await? as i64;
        let completed = WorkflowInstance::find()
            .filter(workflow_instance::Column::Status.eq("completed"))
            .count(db).await? as i64;
        let failed = WorkflowInstance::find()
            .filter(workflow_instance::Column::Status.eq("failed"))
            .count(db).await? as i64;
        let compensating = WorkflowInstance::find()
            .filter(workflow_instance::Column::Status.eq("compensation"))
            .count(db).await? as i64;

        Ok(serde_json::json!({
            "total": total,
            "running": running,
            "completed": completed,
            "failed": failed,
            "compensating": compensating,
        }))
    }
}
