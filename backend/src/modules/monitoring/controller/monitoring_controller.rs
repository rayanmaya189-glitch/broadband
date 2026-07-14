use axum::extract::{Json, Path, Query, State};
use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::modules::monitoring::request::monitoring_request::*;
use crate::modules::monitoring::response::monitoring_response::*;
use crate::modules::monitoring::service::monitoring_service::MonitoringService;

pub async fn list_health_checks(State(state): State<SharedState>, Query(q): Query<HealthCheckQuery>) -> Result<Json<Vec<HealthCheckResponse>>, AppError> {
    let svc = MonitoringService::new(&state.db);
    let (items, _) = svc.list_health_checks(q.service_name.as_deref(), q.page.unwrap_or(1), q.per_page.unwrap_or(20)).await?;
    Ok(Json(items))
}
pub async fn record_health_check(State(state): State<SharedState>, Json(req): Json<RecordHealthCheckRequest>) -> Result<Json<HealthCheckResponse>, AppError> {
    let svc = MonitoringService::new(&state.db);
    Ok(Json(svc.record_health_check(&req.service_name, &req.status, req.response_time_ms, req.error_message.as_deref()).await?))
}
pub async fn record_metric(State(state): State<SharedState>, Json(req): Json<RecordMetricRequest>) -> Result<Json<MetricResponse>, AppError> {
    let svc = MonitoringService::new(&state.db);
    Ok(Json(svc.record_metric(&req.metric_name, &req.metric_type, req.value, req.tags).await?))
}
pub async fn list_alerts(State(state): State<SharedState>, Query(q): Query<AlertQuery>) -> Result<Json<Vec<AlertResponse>>, AppError> {
    let svc = MonitoringService::new(&state.db);
    let (items, _) = svc.list_alerts(q.status.as_deref(), q.severity.as_deref(), q.page.unwrap_or(1), q.per_page.unwrap_or(20)).await?;
    Ok(Json(items))
}
pub async fn acknowledge_alert(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<AlertResponse>, AppError> {
    let svc = MonitoringService::new(&state.db);
    Ok(Json(svc.acknowledge_alert(id, 0).await?))
}
pub async fn resolve_alert(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<AlertResponse>, AppError> {
    let svc = MonitoringService::new(&state.db);
    Ok(Json(svc.resolve_alert(id).await?))
}
pub async fn list_alert_rules(State(state): State<SharedState>) -> Result<Json<Vec<AlertRuleResponse>>, AppError> {
    let svc = MonitoringService::new(&state.db);
    Ok(Json(svc.list_alert_rules().await?))
}
pub async fn create_alert_rule(State(state): State<SharedState>, Json(req): Json<CreateAlertRuleRequest>) -> Result<Json<AlertRuleResponse>, AppError> {
    let svc = MonitoringService::new(&state.db);
    Ok(Json(svc.create_alert_rule(req).await?))
}
