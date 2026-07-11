//! SeaORM-based controller for the Event domain.

use axum::extract::{Json, Path, Query, State};

use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::modules::event::request::event_request::*;
use crate::modules::event::response::event_response::*;
use crate::modules::event::service::event_service::EventService;

pub async fn list(State(state): State<SharedState>, Query(q): Query<EventQuery>) -> Result<Json<Vec<EventResponse>>, AppError> {
    let svc = EventService::new(&state.db_seaorm);
    let (events, _) = svc.list(q.event_type.as_deref(), q.page.unwrap_or(1), q.per_page.unwrap_or(20)).await?;
    Ok(Json(events.into_iter().map(|e| EventResponse {
        id: e.id, event_type: e.event_type, aggregate_type: e.aggregate_type, aggregate_id: e.aggregate_id,
        payload: e.payload, metadata: e.metadata, caused_by_user_id: e.caused_by_user_id,
        caused_by_branch_id: e.caused_by_branch_id, sequence_number: e.sequence_number,
        published_at: e.published_at.into(), processed: e.processed,
    }).collect()))
}

pub async fn get_by_id(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<EventResponse>, AppError> {
    let svc = EventService::new(&state.db_seaorm);
    let e = svc.get_by_id(id).await?;
    Ok(Json(EventResponse {
        id: e.id, event_type: e.event_type, aggregate_type: e.aggregate_type, aggregate_id: e.aggregate_id,
        payload: e.payload, metadata: e.metadata, caused_by_user_id: e.caused_by_user_id,
        caused_by_branch_id: e.caused_by_branch_id, sequence_number: e.sequence_number,
        published_at: e.published_at.into(), processed: e.processed,
    }))
}

pub async fn mark_processed(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<MessageResponse>, AppError> {
    let svc = EventService::new(&state.db_seaorm);
    Ok(Json(svc.mark_processed(id).await?))
}
