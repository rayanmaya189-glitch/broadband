use axum::extract::{Json, Path, Query, State};
use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::modules::event::request::event_request::*;
use crate::modules::event::response::event_response::*;
use crate::modules::event::service::event_service::EventService;

pub async fn list_events(State(state): State<SharedState>, Query(q): Query<EventQuery>) -> Result<Json<EventListResponse>, AppError> {
    let svc = EventService::new(&state.db);
    Ok(Json(svc.list_events(q).await?))
}

pub async fn get_event(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<EventResponse>, AppError> {
    let svc = EventService::new(&state.db);
    Ok(Json(svc.get_event(id).await?))
}

pub async fn get_aggregate_events(State(state): State<SharedState>, Path((aggregate_type, aggregate_id)): Path<(String, i64)>) -> Result<Json<Vec<EventResponse>>, AppError> {
    let svc = EventService::new(&state.db);
    Ok(Json(svc.get_aggregate_events(&aggregate_type, aggregate_id).await?))
}

pub async fn publish_event(State(state): State<SharedState>, Json(req): Json<PublishEventRequest>) -> Result<Json<EventResponse>, AppError> {
    let svc = EventService::new(&state.db);
    Ok(Json(svc.publish_event(req).await?))
}

pub async fn mark_processed(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<MessageResponse>, AppError> {
    let svc = EventService::new(&state.db);
    Ok(Json(svc.mark_processed(id).await?))
}

pub async fn list_subscriptions(State(state): State<SharedState>) -> Result<Json<Vec<EventSubscriptionResponse>>, AppError> {
    let svc = EventService::new(&state.db);
    Ok(Json(svc.list_subscriptions().await?))
}

pub async fn create_subscription(State(state): State<SharedState>, Json(req): Json<CreateSubscriptionRequest>) -> Result<Json<EventSubscriptionResponse>, AppError> {
    let svc = EventService::new(&state.db);
    Ok(Json(svc.create_subscription(req).await?))
}

pub async fn delete_subscription(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<MessageResponse>, AppError> {
    let svc = EventService::new(&state.db);
    Ok(Json(svc.delete_subscription(id).await?))
}

pub async fn get_stats(State(state): State<SharedState>) -> Result<Json<EventStatsResponse>, AppError> {
    let svc = EventService::new(&state.db);
    Ok(Json(svc.get_stats().await?))
}
