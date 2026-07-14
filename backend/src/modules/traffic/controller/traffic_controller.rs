use axum::extract::{Json, Query, State};
use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::modules::traffic::request::traffic_request::*;
use crate::modules::traffic::response::traffic_response::*;
use crate::modules::traffic::service::traffic_service::TrafficService;

pub async fn record_sample(State(state): State<SharedState>, Json(req): Json<RecordSampleRequest>) -> Result<Json<SampleResponse>, AppError> {
    let svc = TrafficService::new(&state.db);
    Ok(Json(svc.record_sample(req).await?))
}
pub async fn list_samples(State(state): State<SharedState>, Query(q): Query<SampleQuery>) -> Result<Json<Vec<SampleResponse>>, AppError> {
    let svc = TrafficService::new(&state.db);
    let (items, _) = svc.list_samples(q.customer_id, q.page.unwrap_or(1), q.per_page.unwrap_or(20)).await?;
    Ok(Json(items))
}
pub async fn list_policies(State(state): State<SharedState>) -> Result<Json<Vec<PolicyResponse>>, AppError> {
    let svc = TrafficService::new(&state.db);
    Ok(Json(svc.list_policies(None).await?))
}
pub async fn create_policy(State(state): State<SharedState>, Json(req): Json<CreatePolicyRequest>) -> Result<Json<PolicyResponse>, AppError> {
    let svc = TrafficService::new(&state.db);
    Ok(Json(svc.create_policy(None, req).await?))
}
pub async fn list_aggregates(State(state): State<SharedState>, Query(q): Query<AggregateQuery>) -> Result<Json<Vec<AggregateResponse>>, AppError> {
    let svc = TrafficService::new(&state.db);
    let (items, _) = svc.list_aggregates(q.customer_id, q.period.as_deref(), q.page.unwrap_or(1), q.per_page.unwrap_or(20)).await?;
    Ok(Json(items))
}
