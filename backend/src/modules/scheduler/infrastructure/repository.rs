use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait, QueryOrder, QuerySelect};
use crate::shared::errors::AppError;

use crate::modules::scheduler::domain::entities::{
    job_definition, job_execution,
    JobDefinition, JobExecution,
};

pub struct SchedulerRepository;

impl SchedulerRepository {
    pub async fn list_job_definitions(db: &DatabaseConnection) -> Result<Vec<job_definition::Model>, AppError> {
        Ok(JobDefinition::find().all(db).await?)
    }

    pub async fn get_due_jobs(db: &DatabaseConnection) -> Result<Vec<job_definition::Model>, AppError> {
        let now = chrono::Utc::now();
        Ok(JobDefinition::find()
            .filter(job_definition::Column::IsActive.eq(true))
            .filter(job_definition::Column::NextRunAt.lte(now))
            .all(db)
            .await?)
    }

    pub async fn get_job_definition(db: &DatabaseConnection, id: i64) -> Result<job_definition::Model, AppError> {
        JobDefinition::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Job definition {} not found", id)))
    }

    pub async fn list_executions(
        db: &DatabaseConnection,
        job_definition_id: Option<i64>,
    ) -> Result<Vec<job_execution::Model>, AppError> {
        let mut query = JobExecution::find();
        if let Some(jid) = job_definition_id {
            query = query.filter(job_execution::Column::JobDefinitionId.eq(jid));
        }
        Ok(query.order_by_desc(job_execution::Column::StartedAt).limit(100).all(db).await?)
    }
}
