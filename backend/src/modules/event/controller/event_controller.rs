use axum::extract::{Json, Path, Query, State};
use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::modules::event::request::event_request::*;
use crate::modules::event::response::event_response::*;
use crate::modules::event::service::event_service::EventService;

#[utoipa::path(
    get,
    path = "/api/v1/events",
    tag = "Events",
    security(("bearer_auth" = [])),
    params(
        ("page" = Option<i64>, Query, description = "Page number"),
        ("limit" = Option<i64>, Query, description = "Items per page"),
        ("event_type" = Option<String>, Query, description = "Filter by event type")
    ),
    responses(
        (status = 200, description = "List of events"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn list_events(State(state): State<SharedState>, Query(q): Query<EventQuery>) -> Result<Json<EventListResponse>, AppError> {
    let svc = EventService::new(&state.db);
    Ok(Json(svc.list_events(q).await?))
}

#[utoipa::path(
    get,
    path = "/api/v1/events/{id}",
    tag = "Events",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Event ID")),
    responses(
        (status = 200, description = "Event details", body = EventResponse),
        (status = 404, description = "Event not found")
    )
)]
pub async fn get_event(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<EventResponse>, AppError> {
    let svc = EventService::new(&state.db);
    Ok(Json(svc.get_event(id).await?))
}

#[utoipa::path(
    get,
    path = "/api/v1/events/aggregate/{aggregate_type}/{aggregate_id}",
    tag = "Events",
    security(("bearer_auth" = [])),
    params(("aggregate_type" = String, Path, description = "Aggregate type"), ("aggregate_id" = i64, Path, description = "Aggregate ID")),
    responses(
        (status = 200, description = "List of aggregate events", body = Vec<EventResponse>),
        (status = 404, description = "No events found")
    )
)]
pub async fn get_aggregate_events(State(state): State<SharedState>, Path((aggregate_type, aggregate_id)): Path<(String, i64)>) -> Result<Json<Vec<EventResponse>>, AppError> {
    let svc = EventService::new(&state.db);
    Ok(Json(svc.get_aggregate_events(&aggregate_type, aggregate_id).await?))
}

#[utoipa::path(
    post,
    path = "/api/v1/events",
    tag = "Events",
    security(("bearer_auth" = [])),
    request_body = PublishEventRequest,
    responses(
        (status = 200, description = "Event published", body = EventResponse),
        (status = 422, description = "Validation error")
    )
)]
pub async fn publish_event(State(state): State<SharedState>, Json(req): Json<PublishEventRequest>) -> Result<Json<EventResponse>, AppError> {
    let svc = EventService::new(&state.db);
    Ok(Json(svc.publish_event(req).await?))
}

#[utoipa::path(
    post,
    path = "/api/v1/events/{id}/process",
    tag = "Events",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Event ID")),
    responses(
        (status = 200, description = "Event marked as processed"),
        (status = 404, description = "Event not found")
    )
)]
pub async fn mark_processed(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<MessageResponse>, AppError> {
    let svc = EventService::new(&state.db);
    Ok(Json(svc.mark_processed(id).await?))
}

#[utoipa::path(
    get,
    path = "/api/v1/events/subscriptions",
    tag = "Events",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "List of event subscriptions", body = Vec<EventSubscriptionResponse>),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn list_subscriptions(State(state): State<SharedState>) -> Result<Json<Vec<EventSubscriptionResponse>>, AppError> {
    let svc = EventService::new(&state.db);
    Ok(Json(svc.list_subscriptions().await?))
}

#[utoipa::path(
    post,
    path = "/api/v1/events/subscriptions",
    tag = "Events",
    security(("bearer_auth" = [])),
    request_body = CreateSubscriptionRequest,
    responses(
        (status = 200, description = "Subscription created", body = EventSubscriptionResponse),
        (status = 422, description = "Validation error")
    )
)]
pub async fn create_subscription(State(state): State<SharedState>, Json(req): Json<CreateSubscriptionRequest>) -> Result<Json<EventSubscriptionResponse>, AppError> {
    let svc = EventService::new(&state.db);
    Ok(Json(svc.create_subscription(req).await?))
}

#[utoipa::path(
    delete,
    path = "/api/v1/events/subscriptions/{id}",
    tag = "Events",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Subscription ID")),
    responses(
        (status = 200, description = "Subscription deleted"),
        (status = 404, description = "Subscription not found")
    )
)]
pub async fn delete_subscription(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<MessageResponse>, AppError> {
    let svc = EventService::new(&state.db);
    Ok(Json(svc.delete_subscription(id).await?))
}

#[utoipa::path(
    get,
    path = "/api/v1/events/stats",
    tag = "Events",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Event statistics", body = EventStatsResponse),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn get_stats(State(state): State<SharedState>) -> Result<Json<EventStatsResponse>, AppError> {
    let svc = EventService::new(&state.db);
    Ok(Json(svc.get_stats().await?))
}
