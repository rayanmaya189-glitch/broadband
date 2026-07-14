use axum::extract::{Json, Path, Query, State};
use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::modules::workflow::request::workflow_request::*;
use crate::modules::workflow::response::workflow_response::*;
use crate::modules::workflow::service::workflow_service::WorkflowService;

pub async fn list_definitions(State(state): State<SharedState>) -> Result<Json<Vec<DefinitionResponse>>, AppError> {
    let svc = WorkflowService::new(&state.db);
    Ok(Json(svc.list_definitions(None).await?))
}
pub async fn create_definition(State(state): State<SharedState>, Json(req): Json<CreateDefinitionRequest>) -> Result<Json<DefinitionResponse>, AppError> {
    let svc = WorkflowService::new(&state.db);
    Ok(Json(svc.create_definition(None, req).await?))
}
pub async fn add_step(State(state): State<SharedState>, Path(definition_id): Path<i64>, Json(req): Json<AddStepRequest>) -> Result<Json<MessageResponse>, AppError> {
    let svc = WorkflowService::new(&state.db);
    Ok(Json(svc.add_step(definition_id, req).await?))
}
pub async fn list_instances(State(state): State<SharedState>, Query(q): Query<InstanceQuery>) -> Result<Json<Vec<InstanceResponse>>, AppError> {
    let svc = WorkflowService::new(&state.db);
    let (items, _) = svc.list_instances(q.branch_id, q.status.as_deref(), q.page.unwrap_or(1), q.per_page.unwrap_or(20)).await?;
    Ok(Json(items))
}
pub async fn start_instance(State(state): State<SharedState>, Json(req): Json<StartInstanceRequest>) -> Result<Json<InstanceResponse>, AppError> {
    let svc = WorkflowService::new(&state.db);
    Ok(Json(svc.start_instance(req.definition_id, None, req.entity_id, 0).await?))
}
pub async fn approve_step(State(state): State<SharedState>, Path((instance_id, step_id)): Path<(i64, i64)>, Json(req): Json<StepDecisionRequest>) -> Result<Json<MessageResponse>, AppError> {
    let svc = WorkflowService::new(&state.db);
    Ok(Json(svc.approve_step(instance_id, step_id, 0, req.comments.as_deref()).await?))
}
pub async fn reject_step(State(state): State<SharedState>, Path((instance_id, step_id)): Path<(i64, i64)>, Json(req): Json<StepDecisionRequest>) -> Result<Json<MessageResponse>, AppError> {
    let svc = WorkflowService::new(&state.db);
    Ok(Json(svc.reject_step(instance_id, step_id, 0, req.comments.as_deref()).await?))
}
