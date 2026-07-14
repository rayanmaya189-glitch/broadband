use sea_orm::DatabaseConnection;
use crate::common::errors::app_error::AppError;
use crate::modules::scheduler::repository::scheduled_task_repository::ScheduledTaskRepository;
use crate::modules::scheduler::repository::task_execution_repository::TaskExecutionRepository;
use crate::modules::scheduler::request::scheduler_request::*;
use crate::modules::scheduler::response::scheduler_response::*;

pub struct SchedulerService<'a> { db: &'a DatabaseConnection }
impl<'a> SchedulerService<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self { Self { db } }
    pub async fn list_tasks(&self, branch_id: Option<i64>) -> Result<Vec<TaskResponse>, AppError> {
        let repo = ScheduledTaskRepository::new(self.db);
        let items = repo.list(branch_id).await?;
        Ok(items.into_iter().map(|t| TaskResponse { id: t.id, name: t.name, task_type: t.task_type, config: t.config, schedule_type: t.schedule_type, schedule_value: t.schedule_value, next_run_at: t.next_run_at.map(|v| v.into()), last_run_at: t.last_run_at.map(|v| v.into()), is_active: t.is_active, created_at: t.created_at.into() }).collect())
    }
    pub async fn create_task(&self, branch_id: Option<i64>, req: CreateTaskRequest) -> Result<TaskResponse, AppError> {
        let repo = ScheduledTaskRepository::new(self.db);
        let t = repo.create(branch_id, &req.name, &req.task_type, req.config, &req.schedule_type, &req.schedule_value).await?;
        Ok(TaskResponse { id: t.id, name: t.name, task_type: t.task_type, config: t.config, schedule_type: t.schedule_type, schedule_value: t.schedule_value, next_run_at: t.next_run_at.map(|v| v.into()), last_run_at: t.last_run_at.map(|v| v.into()), is_active: t.is_active, created_at: t.created_at.into() })
    }
    pub async fn toggle_task(&self, id: i64) -> Result<TaskResponse, AppError> {
        let repo = ScheduledTaskRepository::new(self.db);
        let t = repo.toggle_active(id).await?;
        Ok(TaskResponse { id: t.id, name: t.name, task_type: t.task_type, config: t.config, schedule_type: t.schedule_type, schedule_value: t.schedule_value, next_run_at: t.next_run_at.map(|v| v.into()), last_run_at: t.last_run_at.map(|v| v.into()), is_active: t.is_active, created_at: t.created_at.into() })
    }
    pub async fn delete_task(&self, id: i64) -> Result<MessageResponse, AppError> {
        let repo = ScheduledTaskRepository::new(self.db);
        if !repo.delete(id).await? { return Err(AppError::NotFound("Task not found".into())); }
        Ok(MessageResponse { message: "Task deleted".into() })
    }
    pub async fn list_executions(&self, task_id: Option<i64>, page: i64, per_page: i64) -> Result<(Vec<ExecutionResponse>, i64), AppError> {
        let repo = TaskExecutionRepository::new(self.db);
        let (items, total) = repo.list(task_id, page, per_page).await?;
        Ok((items.into_iter().map(|e| ExecutionResponse { id: e.id, task_id: e.task_id, status: e.status, result: e.result, error_message: e.error_message, started_at: e.started_at.into(), completed_at: e.completed_at.map(|v| v.into()), duration_ms: e.duration_ms }).collect(), total))
    }
}
