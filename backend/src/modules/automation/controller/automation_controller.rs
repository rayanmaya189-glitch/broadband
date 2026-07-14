use axum::extract::{Json, Path, Query, State};
use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::modules::automation::request::automation_request::*;
use crate::modules::automation::response::automation_response::*;
use crate::modules::automation::service::automation_service::AutomationService;

pub async fn list_rules(State(state): State<SharedState>) -> Result<Json<Vec<RuleResponse>>, AppError> {
    let svc = AutomationService::new(&state.db);
    Ok(Json(svc.list_rules(None).await?))
}
pub async fn create_rule(State(state): State<SharedState>, Json(req): Json<CreateRuleRequest>) -> Result<Json<RuleResponse>, AppError> {
    let svc = AutomationService::new(&state.db);
    Ok(Json(svc.create_rule(None, req).await?))
}
pub async fn add_trigger(State(state): State<SharedState>, Path(rule_id): Path<i64>, Json(req): Json<AddTriggerRequest>) -> Result<Json<MessageResponse>, AppError> {
    let svc = AutomationService::new(&state.db);
    Ok(Json(svc.add_trigger(rule_id, req).await?))
}
pub async fn add_action(State(state): State<SharedState>, Path(rule_id): Path<i64>, Json(req): Json<AddActionRequest>) -> Result<Json<MessageResponse>, AppError> {
    let svc = AutomationService::new(&state.db);
    Ok(Json(svc.add_action(rule_id, req).await?))
}
pub async fn list_executions(State(state): State<SharedState>, Query(q): Query<ExecutionQuery>) -> Result<Json<Vec<ExecutionResponse>>, AppError> {
    let svc = AutomationService::new(&state.db);
    let (items, _) = svc.list_executions(q.rule_id, q.page.unwrap_or(1), q.per_page.unwrap_or(20)).await?;
    Ok(Json(items))
}
pub async fn delete_rule(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<MessageResponse>, AppError> {
    let svc = AutomationService::new(&state.db);
    Ok(Json(svc.delete_rule(id).await?))
}
