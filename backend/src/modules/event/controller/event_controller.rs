//! SeaORM-based controller for the Event domain.

use axum::extract::{Json, Path, Query, State};
use validator::Validate;

use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::modules::event::request::event_request::*;
use crate::modules::event::response::event_response::*;
use crate::modules::event::service::event_service::EventService;

pub async fn list(State(state): State<SharedState>, Query(q): Query<EventQuery>) -> Result<Json<Vec<EventResponse>>, AppError> {
    let svc = EventService::new(&state.db);
    let (events, _) = svc.list(q.event_type.as_deref(), q.page.unwrap_or(1), q.per_page.unwrap_or(20)).await?;
    Ok(Json(events.into_iter().map(|e| EventResponse {
        id: e.id, event_type: e.event_type, aggregate_type: e.aggregate_type, aggregate_id: e.aggregate_id,
        payload: e.payload, metadata: e.metadata, caused_by_user_id: e.caused_by_user_id,
        caused_by_branch_id: e.caused_by_branch_id, sequence_number: e.sequence_number,
        published_at: e.published_at.into(), processed: e.processed,
    }).collect()))
}

pub async fn get_by_id(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<EventResponse>, AppError> {
    let svc = EventService::new(&state.db);
    let e = svc.get_by_id(id).await?;
    Ok(Json(EventResponse {
        id: e.id, event_type: e.event_type, aggregate_type: e.aggregate_type, aggregate_id: e.aggregate_id,
        payload: e.payload, metadata: e.metadata, caused_by_user_id: e.caused_by_user_id,
        caused_by_branch_id: e.caused_by_branch_id, sequence_number: e.sequence_number,
        published_at: e.published_at.into(), processed: e.processed,
    }))
}

pub async fn publish_event(State(state): State<SharedState>, Json(req): Json<PublishEventRequest>) -> Result<Json<EventResponse>, AppError> {
    req.validate()?;
    let svc = EventService::new(&state.db);
    let e = svc.publish(&req.event_type, &req.aggregate_type, req.aggregate_id, req.payload, req.metadata, None, None).await?;
    Ok(Json(EventResponse {
        id: e.id, event_type: e.event_type, aggregate_type: e.aggregate_type, aggregate_id: e.aggregate_id,
        payload: e.payload, metadata: e.metadata, caused_by_user_id: e.caused_by_user_id,
        caused_by_branch_id: e.caused_by_branch_id, sequence_number: e.sequence_number,
        published_at: e.published_at.into(), processed: e.processed,
    }))
}

pub async fn mark_processed(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<MessageResponse>, AppError> {
    let svc = EventService::new(&state.db);
    Ok(Json(svc.mark_processed(id).await?))
}

pub async fn list_subscriptions(State(state): State<SharedState>) -> Result<Json<Vec<EventSubscriptionResponse>>, AppError> {
    let svc = EventService::new(&state.db);
    let subs = svc.list_subscriptions().await?;
    Ok(Json(subs.into_iter().map(|s| EventSubscriptionResponse {
        id: s.id, subscriber_name: s.subscriber_name, event_type: s.event_type,
        last_processed_at: s.last_processed_at.map(|v| v.into()),
        is_active: s.is_active, created_at: s.created_at.into(),
    }).collect()))
}

pub async fn create_subscription(State(state): State<SharedState>, Json(req): Json<CreateSubscriptionRequest>) -> Result<Json<EventSubscriptionResponse>, AppError> {
    req.validate()?;
    let svc = EventService::new(&state.db);
    let s = svc.create_subscription(&req.subscriber_name, &req.event_type).await?;
    Ok(Json(EventSubscriptionResponse {
        id: s.id, subscriber_name: s.subscriber_name, event_type: s.event_type,
        last_processed_at: s.last_processed_at.map(|v| v.into()),
        is_active: s.is_active, created_at: s.created_at.into(),
    }))
}

pub async fn delete_subscription(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<MessageResponse>, AppError> {
    let svc = EventService::new(&state.db);
    svc.delete_subscription(id).await?;
    Ok(Json(MessageResponse { message: "Subscription deleted".into() }))
}

pub async fn get_aggregate_events(State(state): State<SharedState>, Path((aggregate_type, aggregate_id)): Path<(String, i64)>) -> Result<Json<Vec<EventResponse>>, AppError> {
    let svc = EventService::new(&state.db);
    let events = svc.get_aggregate_events(&aggregate_type, aggregate_id).await?;
    Ok(Json(events.into_iter().map(|e| EventResponse {
        id: e.id, event_type: e.event_type, aggregate_type: e.aggregate_type, aggregate_id: e.aggregate_id,
        payload: e.payload, metadata: e.metadata, caused_by_user_id: e.caused_by_user_id,
        caused_by_branch_id: e.caused_by_branch_id, sequence_number: e.sequence_number,
        published_at: e.published_at.into(), processed: e.processed,
    }).collect()))
}
