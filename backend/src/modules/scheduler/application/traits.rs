use async_trait::async_trait;
use sea_orm::{DatabaseConnection};
use crate::shared::errors::AppError;

pub type JobDefinitionModel = crate::modules::scheduler::domain::entities::job_definition::Model;
pub type JobExecutionModel = crate::modules::scheduler::domain::entities::job_execution::Model;

#[async_trait]
pub trait SchedulerServiceTrait: Send + Sync {
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
        job_type: String,
        cron_expression: Option<String>,
    ) -> Result<JobDefinitionModel, AppError>;

    async fn list_executions(
        &self,
        db: &DatabaseConnection,
        job_id: Option<i64>,
    ) -> Result<Vec<JobExecutionModel>, AppError>;
}
