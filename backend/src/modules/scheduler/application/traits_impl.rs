use async_trait::async_trait;
use sea_orm::DatabaseConnection;
use crate::shared::errors::AppError;

pub type JobDefinitionModel = crate::modules::scheduler::domain::entities::job_definition::Model;
pub type JobExecutionModel = crate::modules::scheduler::domain::entities::job_execution::Model;

use crate::modules::scheduler::application::services::SchedulerService;
use crate::modules::scheduler::application::traits::SchedulerServiceTrait;

#[async_trait]
impl SchedulerServiceTrait for SchedulerService {
    async fn list_job_definitions(&self, db: &DatabaseConnection) -> Result<Vec<JobDefinitionModel>, AppError> {
        SchedulerService::list_job_definitions(db).await
    }

    async fn get_job_definition(&self, db: &DatabaseConnection, id: i64) -> Result<JobDefinitionModel, AppError> {
        SchedulerService::get_job_definition(db, id).await
    }

    async fn create_job_definition(
        &self, db: &DatabaseConnection, name: String, job_type: String, cron_expression: Option<String>,
    ) -> Result<JobDefinitionModel, AppError> {
        SchedulerService::create_job_definition(
            db, name, None, job_type, cron_expression.unwrap_or_else(|| "0 0 1 * *".to_string()),
            "system".to_string(), "default".to_string(),
            serde_json::json!({}), None,
        ).await
    }

    async fn list_executions(&self, db: &DatabaseConnection, job_id: Option<i64>) -> Result<Vec<JobExecutionModel>, AppError> {
        SchedulerService::list_executions(db, job_id).await
    }
}
