use axum::extract::{Json, Path, Query, State};
use validator::Validate;
use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::modules::notification::request::notification_request::*;
use crate::modules::notification::response::notification_response::*;
use crate::modules::notification::service::notification_service::NotificationService;

// ── Templates ───────────────────────────────────────────────

#[utoipa::path(
    get,
    path = "/api/v1/notifications/templates",
    tag = "Notifications",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "List of templates", body = Vec<TemplateResponse>),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn list_templates(State(state): State<SharedState>) -> Result<Json<Vec<TemplateResponse>>, AppError> {
    let svc = NotificationService::new(&state.db);
    Ok(Json(svc.list_templates().await?))
}

#[utoipa::path(
    post,
    path = "/api/v1/notifications/templates",
    tag = "Notifications",
    security(("bearer_auth" = [])),
    request_body = CreateTemplateRequest,
    responses(
        (status = 200, description = "Template created", body = TemplateResponse),
        (status = 422, description = "Validation error")
    )
)]
pub async fn create_template(State(state): State<SharedState>, Json(req): Json<CreateTemplateRequest>) -> Result<Json<TemplateResponse>, AppError> {
    req.validate()?;
    let svc = NotificationService::new(&state.db);
    Ok(Json(svc.create_template(req).await?))
}

#[utoipa::path(
    put,
    path = "/api/v1/notifications/templates/{id}",
    tag = "Notifications",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Template ID")),
    request_body = UpdateTemplateRequest,
    responses(
        (status = 200, description = "Template updated", body = TemplateResponse),
        (status = 404, description = "Template not found")
    )
)]
pub async fn update_template(State(state): State<SharedState>, Path(id): Path<i64>, Json(req): Json<UpdateTemplateRequest>) -> Result<Json<TemplateResponse>, AppError> {
    let svc = NotificationService::new(&state.db);
    Ok(Json(svc.update_template(id, req).await?))
}

#[utoipa::path(
    delete,
    path = "/api/v1/notifications/templates/{id}",
    tag = "Notifications",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Template ID")),
    responses(
        (status = 200, description = "Template deleted"),
        (status = 404, description = "Template not found")
    )
)]
pub async fn delete_template(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<MessageResponse>, AppError> {
    let svc = NotificationService::new(&state.db);
    Ok(Json(svc.delete_template(id).await?))
}

// ── Channels ────────────────────────────────────────────────

#[utoipa::path(
    get,
    path = "/api/v1/notifications/channels",
    tag = "Notifications",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "List of channels", body = Vec<ChannelResponse>),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn list_channels(State(state): State<SharedState>) -> Result<Json<Vec<ChannelResponse>>, AppError> {
    let svc = NotificationService::new(&state.db);
    Ok(Json(svc.list_channels().await?))
}

#[utoipa::path(
    post,
    path = "/api/v1/notifications/channels",
    tag = "Notifications",
    security(("bearer_auth" = [])),
    request_body = UpsertChannelRequest,
    responses(
        (status = 200, description = "Channel upserted", body = ChannelResponse),
        (status = 422, description = "Validation error")
    )
)]
pub async fn upsert_channel(State(state): State<SharedState>, Json(req): Json<UpsertChannelRequest>) -> Result<Json<ChannelResponse>, AppError> {
    req.validate()?;
    let svc = NotificationService::new(&state.db);
    Ok(Json(svc.upsert_channel(req).await?))
}

// ── Notifications ───────────────────────────────────────────

#[utoipa::path(
    post,
    path = "/api/v1/notifications/send",
    tag = "Notifications",
    security(("bearer_auth" = [])),
    request_body = SendNotificationRequest,
    responses(
        (status = 200, description = "Notification sent", body = NotificationResponse),
        (status = 422, description = "Validation error")
    )
)]
pub async fn send_notification(State(state): State<SharedState>, Json(req): Json<SendNotificationRequest>) -> Result<Json<NotificationResponse>, AppError> {
    req.validate()?;
    let svc = NotificationService::new(&state.db);
    Ok(Json(svc.send(req).await?))
}

#[utoipa::path(
    get,
    path = "/api/v1/notifications",
    tag = "Notifications",
    security(("bearer_auth" = [])),
    params(
        ("page" = Option<i64>, Query, description = "Page number"),
        ("limit" = Option<i64>, Query, description = "Items per page"),
        ("status" = Option<String>, Query, description = "Filter by status")
    ),
    responses(
        (status = 200, description = "List of notifications"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn list_notifications(State(state): State<SharedState>, Query(query): Query<NotificationQuery>) -> Result<Json<NotificationListResponse>, AppError> {
    let svc = NotificationService::new(&state.db);
    Ok(Json(svc.list_notifications(query).await?))
}

#[utoipa::path(
    post,
    path = "/api/v1/notifications/{id}/retry",
    tag = "Notifications",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Notification ID")),
    responses(
        (status = 200, description = "Notification retried", body = NotificationDetailResponse),
        (status = 404, description = "Notification not found")
    )
)]
pub async fn retry_notification(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<NotificationDetailResponse>, AppError> {
    let svc = NotificationService::new(&state.db);
    Ok(Json(svc.retry_notification(id).await?))
}

// ── History ─────────────────────────────────────────────────

#[utoipa::path(
    get,
    path = "/api/v1/notifications/history",
    tag = "Notifications",
    security(("bearer_auth" = [])),
    params(
        ("page" = Option<i64>, Query, description = "Page number"),
        ("limit" = Option<i64>, Query, description = "Items per page")
    ),
    responses(
        (status = 200, description = "Notification history"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn list_history(State(state): State<SharedState>, Query(query): Query<HistoryQuery>) -> Result<Json<HistoryListResponse>, AppError> {
    let svc = NotificationService::new(&state.db);
    Ok(Json(svc.list_history(query).await?))
}
