use crate::shared::errors::AppError;
use async_trait::async_trait;
use sea_orm::DatabaseConnection;

pub type JobDefinitionModel = crate::modules::scheduler::domain::entities::job_definition::Model;
pub type JobExecutionModel = crate::modules::scheduler::domain::entities::job_execution::Model;

#[async_trait]
pub trait SchedulerRepositoryTrait: Send + Sync {
    async fn list_job_definitions(
        &self,
        db: &DatabaseConnection,
    ) -> Result<Vec<JobDefinitionModel>, AppError>;

    async fn get_job_definition(
        &self,
        db: &DatabaseConnection,
        id: i64,
    ) -> Result<JobDefinitionModel, AppError>;

    async fn create_job_definition(
        &self,
        db: &DatabaseConnection,
        name: String,
        description: Option<String>,
        job_type: String,
        schedule: String,
        target_module: String,
        action: String,
        payload: serde_json::Value,
        timeout_seconds: Option<i32>,
    ) -> Result<JobDefinitionModel, AppError>;

    async fn update_job_definition(
        &self,
        db: &DatabaseConnection,
        id: i64,
        schedule: Option<String>,
        payload: Option<serde_json::Value>,
        is_active: Option<bool>,
        timeout_seconds: Option<i32>,
    ) -> Result<JobDefinitionModel, AppError>;

    async fn delete_job_definition(&self, db: &DatabaseConnection, id: i64)
        -> Result<(), AppError>;

    async fn get_due_jobs(
        &self,
        db: &DatabaseConnection,
    ) -> Result<Vec<JobDefinitionModel>, AppError>;

    async fn list_executions(
        &self,
        db: &DatabaseConnection,
        job_definition_id: Option<i64>,
    ) -> Result<Vec<JobExecutionModel>, AppError>;

    async fn start_execution(
        &self,
        db: &DatabaseConnection,
        job_definition_id: i64,
        input_payload: serde_json::Value,
    ) -> Result<JobExecutionModel, AppError>;

    async fn complete_execution(
        &self,
        db: &DatabaseConnection,
        execution_id: i64,
        output_payload: serde_json::Value,
    ) -> Result<JobExecutionModel, AppError>;

    async fn fail_execution(
        &self,
        db: &DatabaseConnection,
        execution_id: i64,
        error_message: String,
    ) -> Result<JobExecutionModel, AppError>;

    async fn get_scheduler_stats(
        &self,
        db: &DatabaseConnection,
    ) -> Result<serde_json::Value, AppError>;
}
