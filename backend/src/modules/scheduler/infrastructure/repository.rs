use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, QuerySelect, Set,
};

use crate::modules::scheduler::application::traits::SchedulerRepositoryTrait;
use crate::modules::scheduler::domain::entities::{
    job_definition, job_execution, JobDefinition, JobDefinitionActiveModel, JobExecution,
    JobExecutionActiveModel,
};
use crate::modules::scheduler::domain::value_objects::Schedule;
use crate::shared::errors::AppError;
use async_trait::async_trait;
use chrono::Utc;

pub struct SchedulerRepository<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> SchedulerRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }

    // ── Job Definitions ──

    pub async fn list_job_definitions(&self) -> Result<Vec<job_definition::Model>, AppError> {
        Ok(JobDefinition::find().all(self.db).await?)
    }

    pub async fn get_job_definition(&self, id: i64) -> Result<job_definition::Model, AppError> {
        JobDefinition::find_by_id(id)
            .one(self.db)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Job definition {} not found", id)))
    }

    pub async fn create_job_definition(
        &self,
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
        let next_run = match Schedule::parse(&job_type, &schedule) {
            Ok(sched) => sched.next_run_after(now),
            Err(e) => {
                tracing::warn!(error = %e, "Failed to parse schedule for initial next_run_at");
                None
            }
        };

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
            next_run_at: Set(next_run),
            last_run_at: Set(None),
            last_run_status: Set(None),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };
        Ok(job.insert(self.db).await?)
    }

    pub async fn update_job_definition(
        &self,
        id: i64,
        schedule: Option<String>,
        payload: Option<serde_json::Value>,
        is_active: Option<bool>,
        timeout_seconds: Option<i32>,
    ) -> Result<job_definition::Model, AppError> {
        let job = self.get_job_definition(id).await?;
        let mut active: job_definition::ActiveModel = job.into();

        let mut recalc = false;
        if let Some(ref s) = schedule {
            active.schedule = Set(s.clone());
            recalc = true;
        }
        if let Some(p) = payload {
            active.payload = Set(p);
        }
        if let Some(a) = is_active {
            active.is_active = Set(a);
            recalc = true;
        }
        if let Some(t) = timeout_seconds {
            active.timeout_seconds = Set(Some(t));
        }

        if recalc {
            let job_type_str = active.job_type.as_ref().clone();
            let schedule_str = active.schedule.as_ref().clone();
            let active_flag = *active.is_active.as_ref();
            let next_run = if active_flag {
                match Schedule::parse(&job_type_str, &schedule_str) {
                    Ok(sched) => sched.next_run_after(Utc::now()),
                    Err(_) => None,
                }
            } else {
                None
            };
            active.next_run_at = Set(next_run);
        }

        active.updated_at = Set(Utc::now());
        Ok(active.update(self.db).await?)
    }

    pub async fn delete_job_definition(&self, id: i64) -> Result<(), AppError> {
        let job = self.get_job_definition(id).await?;
        let active: job_definition::ActiveModel = job.into();
        active.delete(self.db).await?;
        Ok(())
    }

    pub async fn get_due_jobs(&self) -> Result<Vec<job_definition::Model>, AppError> {
        let now = Utc::now();
        Ok(JobDefinition::find()
            .filter(job_definition::Column::IsActive.eq(true))
            .filter(job_definition::Column::NextRunAt.lte(now))
            .all(self.db)
            .await?)
    }

    // ── Job Executions ──

    pub async fn list_executions(
        &self,
        job_definition_id: Option<i64>,
    ) -> Result<Vec<job_execution::Model>, AppError> {
        let mut query = JobExecution::find();
        if let Some(jid) = job_definition_id {
            query = query.filter(job_execution::Column::JobDefinitionId.eq(jid));
        }
        Ok(query
            .order_by_desc(job_execution::Column::StartedAt)
            .limit(50)
            .all(self.db)
            .await?)
    }

    pub async fn start_execution(
        &self,
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
        Ok(execution.insert(self.db).await?)
    }

    pub async fn complete_execution(
        &self,
        execution_id: i64,
        output_payload: serde_json::Value,
    ) -> Result<job_execution::Model, AppError> {
        let exec = JobExecution::find_by_id(execution_id)
            .one(self.db)
            .await?
            .ok_or_else(|| {
                AppError::NotFound(format!("Job execution {} not found", execution_id))
            })?;
        let mut active: job_execution::ActiveModel = exec.into();
        let started = *active.started_at.as_ref();
        let duration = Utc::now().signed_duration_since(started).num_milliseconds();
        active.status = Set("completed".to_string());
        active.output_payload = Set(Some(output_payload));
        active.duration_ms = Set(Some(duration as i64));
        active.completed_at = Set(Some(Utc::now()));

        let job_id = *active.job_definition_id.as_ref();
        if let Ok(Some(parent)) = JobDefinition::find_by_id(job_id).one(self.db).await {
            let mut parent_active: job_definition::ActiveModel = parent.into();
            parent_active.last_run_at = Set(Some(Utc::now()));
            parent_active.last_run_status = Set(Some("completed".to_string()));
            parent_active.updated_at = Set(Utc::now());
            let _ = parent_active.update(self.db).await;
        }

        Ok(active.update(self.db).await?)
    }

    pub async fn fail_execution(
        &self,
        execution_id: i64,
        error_message: String,
    ) -> Result<job_execution::Model, AppError> {
        let exec = JobExecution::find_by_id(execution_id)
            .one(self.db)
            .await?
            .ok_or_else(|| {
                AppError::NotFound(format!("Job execution {} not found", execution_id))
            })?;
        let mut active: job_execution::ActiveModel = exec.into();
        let started = *active.started_at.as_ref();
        let duration = Utc::now().signed_duration_since(started).num_milliseconds();
        active.status = Set("failed".to_string());
        active.error_message = Set(Some(error_message));
        active.duration_ms = Set(Some(duration as i64));
        active.completed_at = Set(Some(Utc::now()));

        let job_id = *active.job_definition_id.as_ref();
        if let Ok(Some(parent)) = JobDefinition::find_by_id(job_id).one(self.db).await {
            let mut parent_active: job_definition::ActiveModel = parent.into();
            parent_active.last_run_at = Set(Some(Utc::now()));
            parent_active.last_run_status = Set(Some("failed".to_string()));
            parent_active.updated_at = Set(Utc::now());
            let _ = parent_active.update(self.db).await;
        }

        Ok(active.update(self.db).await?)
    }

    pub async fn get_scheduler_stats(&self) -> Result<serde_json::Value, AppError> {
        let total_jobs = JobDefinition::find().count(self.db).await? as i64;
        let active_jobs = JobDefinition::find()
            .filter(job_definition::Column::IsActive.eq(true))
            .count(self.db)
            .await? as i64;
        let total_executions = JobExecution::find().count(self.db).await? as i64;
        let failed_executions = JobExecution::find()
            .filter(job_execution::Column::Status.eq("failed"))
            .count(self.db)
            .await? as i64;

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

#[async_trait]
impl SchedulerRepositoryTrait for SchedulerRepository<'_> {
    async fn list_job_definitions(
        &self,
        _db: &DatabaseConnection,
    ) -> Result<Vec<job_definition::Model>, AppError> {
        self.list_job_definitions().await
    }

    async fn get_job_definition(
        &self,
        _db: &DatabaseConnection,
        id: i64,
    ) -> Result<job_definition::Model, AppError> {
        self.get_job_definition(id).await
    }

    async fn create_job_definition(
        &self,
        _db: &DatabaseConnection,
        name: String,
        description: Option<String>,
        job_type: String,
        schedule: String,
        target_module: String,
        action: String,
        payload: serde_json::Value,
        timeout_seconds: Option<i32>,
    ) -> Result<job_definition::Model, AppError> {
        self.create_job_definition(
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

    async fn update_job_definition(
        &self,
        _db: &DatabaseConnection,
        id: i64,
        schedule: Option<String>,
        payload: Option<serde_json::Value>,
        is_active: Option<bool>,
        timeout_seconds: Option<i32>,
    ) -> Result<job_definition::Model, AppError> {
        self.update_job_definition(id, schedule, payload, is_active, timeout_seconds)
            .await
    }

    async fn delete_job_definition(
        &self,
        _db: &DatabaseConnection,
        id: i64,
    ) -> Result<(), AppError> {
        self.delete_job_definition(id).await
    }

    async fn get_due_jobs(
        &self,
        _db: &DatabaseConnection,
    ) -> Result<Vec<job_definition::Model>, AppError> {
        self.get_due_jobs().await
    }

    async fn list_executions(
        &self,
        _db: &DatabaseConnection,
        job_definition_id: Option<i64>,
    ) -> Result<Vec<job_execution::Model>, AppError> {
        self.list_executions(job_definition_id).await
    }

    async fn start_execution(
        &self,
        _db: &DatabaseConnection,
        job_definition_id: i64,
        input_payload: serde_json::Value,
    ) -> Result<job_execution::Model, AppError> {
        self.start_execution(job_definition_id, input_payload).await
    }

    async fn complete_execution(
        &self,
        _db: &DatabaseConnection,
        execution_id: i64,
        output_payload: serde_json::Value,
    ) -> Result<job_execution::Model, AppError> {
        self.complete_execution(execution_id, output_payload).await
    }

    async fn fail_execution(
        &self,
        _db: &DatabaseConnection,
        execution_id: i64,
        error_message: String,
    ) -> Result<job_execution::Model, AppError> {
        self.fail_execution(execution_id, error_message).await
    }

    async fn get_scheduler_stats(
        &self,
        _db: &DatabaseConnection,
    ) -> Result<serde_json::Value, AppError> {
        self.get_scheduler_stats().await
    }
}
