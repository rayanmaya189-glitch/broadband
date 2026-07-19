use crate::modules::scheduler::application::traits::SchedulerRepositoryTrait;
use crate::modules::scheduler::domain::entities::{job_definition, job_execution};
use crate::shared::errors::AppError;

/// Scheduler service for managing recurring and delayed jobs.
/// Uses the repository trait for testability and DDD compliance.
pub struct SchedulerService;

impl SchedulerService {
    // ── Job Definitions ──

    pub async fn list_job_definitions(
        repo: &dyn SchedulerRepositoryTrait,
        db: &sea_orm::DatabaseConnection,
    ) -> Result<Vec<job_definition::Model>, AppError> {
        repo.list_job_definitions(db).await
    }

    pub async fn get_job_definition(
        repo: &dyn SchedulerRepositoryTrait,
        db: &sea_orm::DatabaseConnection,
        id: i64,
    ) -> Result<job_definition::Model, AppError> {
        repo.get_job_definition(db, id).await
    }

    pub async fn create_job_definition(
        repo: &dyn SchedulerRepositoryTrait,
        db: &sea_orm::DatabaseConnection,
        name: String,
        description: Option<String>,
        job_type: String,
        schedule: String,
        target_module: String,
        action: String,
        payload: serde_json::Value,
        timeout_seconds: Option<i32>,
    ) -> Result<job_definition::Model, AppError> {
        repo.create_job_definition(
            db,
            name,
            description,
            job_type,
            schedule,
            target_module,
            action,
            payload,
            timeout_seconds,
        )
        .await
    }

    pub async fn update_job_definition(
        repo: &dyn SchedulerRepositoryTrait,
        db: &sea_orm::DatabaseConnection,
        id: i64,
        schedule: Option<String>,
        payload: Option<serde_json::Value>,
        is_active: Option<bool>,
        timeout_seconds: Option<i32>,
    ) -> Result<job_definition::Model, AppError> {
        repo.update_job_definition(db, id, schedule, payload, is_active, timeout_seconds)
            .await
    }

    pub async fn delete_job_definition(
        repo: &dyn SchedulerRepositoryTrait,
        db: &sea_orm::DatabaseConnection,
        id: i64,
    ) -> Result<(), AppError> {
        repo.delete_job_definition(db, id).await
    }

    pub async fn get_due_jobs(
        repo: &dyn SchedulerRepositoryTrait,
        db: &sea_orm::DatabaseConnection,
    ) -> Result<Vec<job_definition::Model>, AppError> {
        repo.get_due_jobs(db).await
    }

    // ── Job Executions ──

    pub async fn list_executions(
        repo: &dyn SchedulerRepositoryTrait,
        db: &sea_orm::DatabaseConnection,
        job_definition_id: Option<i64>,
    ) -> Result<Vec<job_execution::Model>, AppError> {
        repo.list_executions(db, job_definition_id).await
    }

    pub async fn start_execution(
        repo: &dyn SchedulerRepositoryTrait,
        db: &sea_orm::DatabaseConnection,
        job_definition_id: i64,
        input_payload: serde_json::Value,
    ) -> Result<job_execution::Model, AppError> {
        repo.start_execution(db, job_definition_id, input_payload)
            .await
    }

    pub async fn complete_execution(
        repo: &dyn SchedulerRepositoryTrait,
        db: &sea_orm::DatabaseConnection,
        execution_id: i64,
        output_payload: serde_json::Value,
    ) -> Result<job_execution::Model, AppError> {
        repo.complete_execution(db, execution_id, output_payload)
            .await
    }

    pub async fn fail_execution(
        repo: &dyn SchedulerRepositoryTrait,
        db: &sea_orm::DatabaseConnection,
        execution_id: i64,
        error_message: String,
    ) -> Result<job_execution::Model, AppError> {
        repo.fail_execution(db, execution_id, error_message).await
    }

    pub async fn get_scheduler_stats(
        repo: &dyn SchedulerRepositoryTrait,
        db: &sea_orm::DatabaseConnection,
    ) -> Result<serde_json::Value, AppError> {
        repo.get_scheduler_stats(db).await
    }
}
