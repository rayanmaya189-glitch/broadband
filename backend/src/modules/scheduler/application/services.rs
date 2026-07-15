use sea_orm::{DatabaseConnection, EntityTrait, ActiveModelTrait, Set, QueryFilter, ColumnTrait};
use chrono::Utc;
use crate::shared::errors::AppError;
use crate::modules::scheduler::domain::entities::{
    job_definition, job_execution,
    JobDefinition, JobDefinitionActiveModel,
    JobExecution, JobExecutionActiveModel,
};

/// Scheduler service for managing recurring and delayed jobs.
pub struct SchedulerService;

impl SchedulerService {
    // ── Job Definitions ──

    pub async fn list_job_definitions(db: &DatabaseConnection) -> Result<Vec<job_definition::Model>, AppError> {
        Ok(JobDefinition::find().all(db).await?)
    }

    pub async fn get_job_definition(db: &DatabaseConnection, id: i64) -> Result<job_definition::Model, AppError> {
        JobDefinition::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Job definition {} not found", id)))
    }

    pub async fn create_job_definition(
        db: &DatabaseConnection,
        name: String,
        description: Option<String>,
        job_type: String,
        schedule: String,
        target_module: String,
        action: String,
        payload: serde_json::Value,
        timeout_seconds: Option<i32>,
    ) -> Result<job_definition::Model, AppError> {
        let now = Utc::now();
        let job = JobDefinitionActiveModel {
            name: Set(name),
            description: Set(description),
            job_type: Set(job_type),
            schedule: Set(schedule),
            target_module: Set(target_module),
            action: Set(action),
            payload: Set(payload),
            is_active: Set(true),
            timeout_seconds: Set(timeout_seconds),
            next_run_at: Set(None),
            last_run_at: Set(None),
            last_run_status: Set(None),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };
        Ok(job.insert(db).await?)
    }

    pub async fn update_job_definition(
        db: &DatabaseConnection,
        id: i64,
        schedule: Option<String>,
        payload: Option<serde_json::Value>,
        is_active: Option<bool>,
        timeout_seconds: Option<i32>,
    ) -> Result<job_definition::Model, AppError> {
        let job = JobDefinition::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Job definition {} not found", id)))?;
        let mut active: job_definition::ActiveModel = job.into();
        if let Some(s) = schedule { active.schedule = Set(s); }
        if let Some(p) = payload { active.payload = Set(p); }
        if let Some(a) = is_active { active.is_active = Set(a); }
        if let Some(t) = timeout_seconds { active.timeout_seconds = Set(Some(t)); }
        active.updated_at = Set(Utc::now());
        Ok(active.update(db).await?)
    }

    pub async fn delete_job_definition(db: &DatabaseConnection, id: i64) -> Result<(), AppError> {
        let job = JobDefinition::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Job definition {} not found", id)))?;
        job.delete(db).await?;
        Ok(())
    }

    pub async fn get_due_jobs(db: &DatabaseConnection) -> Result<Vec<job_definition::Model>, AppError> {
        let now = Utc::now();
        Ok(JobDefinition::find()
            .filter(job_definition::Column::IsActive.eq(true))
            .filter(job_definition::Column::NextRunAt.lte(now))
            .all(db)
            .await?)
    }

    // ── Job Executions ──

    pub async fn list_executions(db: &DatabaseConnection, job_definition_id: i64) -> Result<Vec<job_execution::Model>, AppError> {
        Ok(JobExecution::find()
            .filter(job_execution::Column::JobDefinitionId.eq(job_definition_id))
            .order_by_desc(job_execution::Column::StartedAt)
            .limit(50)
            .all(db)
            .await?)
    }

    pub async fn start_execution(
        db: &DatabaseConnection,
        job_definition_id: i64,
        input_payload: serde_json::Value,
    ) -> Result<job_execution::Model, AppError> {
        let now = Utc::now();
        let execution = JobExecutionActiveModel {
            job_definition_id: Set(job_definition_id),
            status: Set("running".to_string()),
            input_payload: Set(input_payload),
            output_payload: Set(None),
            error_message: Set(None),
            duration_ms: Set(None),
            started_at: Set(now),
            completed_at: Set(None),
            ..Default::default()
        };
        Ok(execution.insert(db).await?)
    }

    pub async fn complete_execution(
        db: &DatabaseConnection,
        execution_id: i64,
        output_payload: serde_json::Value,
    ) -> Result<job_execution::Model, AppError> {
        let exec = JobExecution::find_by_id(execution_id)
            .one(db)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Job execution {} not found", execution_id)))?;
        let mut active: job_execution::ActiveModel = exec.into();
        let started = *active.started_at.as_ref();
        let duration = Utc::now().signed_duration_since(started).num_milliseconds();
        active.status = Set("completed".to_string());
        active.output_payload = Set(Some(output_payload));
        active.duration_ms = Set(Some(duration as i64));
        active.completed_at = Set(Some(Utc::now()));

        // Update parent job definition
        let job_id = *active.job_definition_id.as_ref();
        if let Ok(Some(parent)) = JobDefinition::find_by_id(job_id).one(db).await {
            let mut parent_active: job_definition::ActiveModel = parent.into();
            parent_active.last_run_at = Set(Some(Utc::now()));
            parent_active.last_run_status = Set(Some("completed".to_string()));
            parent_active.updated_at = Set(Utc::now());
            let _ = parent_active.update(db).await;
        }

        Ok(active.update(db).await?)
    }

    pub async fn fail_execution(
        db: &DatabaseConnection,
        execution_id: i64,
        error_message: String,
    ) -> Result<job_execution::Model, AppError> {
        let exec = JobExecution::find_by_id(execution_id)
            .one(db)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Job execution {} not found", execution_id)))?;
        let mut active: job_execution::ActiveModel = exec.into();
        let started = *active.started_at.as_ref();
        let duration = Utc::now().signed_duration_since(started).num_milliseconds();
        active.status = Set("failed".to_string());
        active.error_message = Set(Some(error_message));
        active.duration_ms = Set(Some(duration as i64));
        active.completed_at = Set(Some(Utc::now()));

        // Update parent job definition
        let job_id = *active.job_definition_id.as_ref();
        if let Ok(Some(parent)) = JobDefinition::find_by_id(job_id).one(db).await {
            let mut parent_active: job_definition::ActiveModel = parent.into();
            parent_active.last_run_at = Set(Some(Utc::now()));
            parent_active.last_run_status = Set(Some("failed".to_string()));
            parent_active.updated_at = Set(Utc::now());
            let _ = parent_active.update(db).await;
        }

        Ok(active.update(db).await?)
    }

    pub async fn get_scheduler_stats(db: &DatabaseConnection) -> Result<serde_json::Value, AppError> {
        let total_jobs = JobDefinition::find().count(db).await? as i64;
        let active_jobs = JobDefinition::find()
            .filter(job_definition::Column::IsActive.eq(true))
            .count(db).await? as i64;
        let total_executions = JobExecution::find().count(db).await? as i64;
        let failed_executions = JobExecution::find()
            .filter(job_execution::Column::Status.eq("failed"))
            .count(db).await? as i64;

        Ok(serde_json::json!({
            "total_definitions": total_jobs,
            "active_definitions": active_jobs,
            "total_executions": total_executions,
            "failed_executions": failed_executions,
            "success_rate": if total_executions > 0 {
                ((total_executions - failed_executions) as f64 / total_executions as f64 * 100.0).round()
            } else { 100.0 },
        }))
    }
}
