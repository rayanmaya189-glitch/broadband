use axum::extract::{Json, Path, Query, State};
use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::modules::scheduler::request::scheduler_request::*;
use crate::modules::scheduler::response::scheduler_response::*;
use crate::modules::scheduler::service::scheduler_service::SchedulerService;

pub async fn list_tasks(State(state): State<SharedState>) -> Result<Json<Vec<TaskResponse>>, AppError> {
    let svc = SchedulerService::new(&state.db);
    Ok(Json(svc.list_tasks(None).await?))
}
pub async fn create_task(State(state): State<SharedState>, Json(req): Json<CreateTaskRequest>) -> Result<Json<TaskResponse>, AppError> {
    let svc = SchedulerService::new(&state.db);
    Ok(Json(svc.create_task(None, req).await?))
}
pub async fn toggle_task(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<TaskResponse>, AppError> {
    let svc = SchedulerService::new(&state.db);
    Ok(Json(svc.toggle_task(id).await?))
}
pub async fn delete_task(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<MessageResponse>, AppError> {
    let svc = SchedulerService::new(&state.db);
    Ok(Json(svc.delete_task(id).await?))
}
pub async fn list_executions(State(state): State<SharedState>, Query(q): Query<ExecutionQuery>) -> Result<Json<Vec<ExecutionResponse>>, AppError> {
    let svc = SchedulerService::new(&state.db);
    let (items, _) = svc.list_executions(q.task_id, q.page.unwrap_or(1), q.per_page.unwrap_or(20)).await?;
    Ok(Json(items))
}
