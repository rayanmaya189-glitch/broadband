use axum::extract::{Json, Path, Query, State};
use validator::Validate;
use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::modules::notification::request::notification_request::*;
use crate::modules::notification::response::notification_response::*;
use crate::modules::notification::service::notification_service::NotificationService;

// ── Templates ───────────────────────────────────────────────

pub async fn list_templates(State(state): State<SharedState>) -> Result<Json<Vec<TemplateResponse>>, AppError> {
    let svc = NotificationService::new(&state.db);
    Ok(Json(svc.list_templates().await?))
}

pub async fn create_template(State(state): State<SharedState>, Json(req): Json<CreateTemplateRequest>) -> Result<Json<TemplateResponse>, AppError> {
    req.validate()?;
    let svc = NotificationService::new(&state.db);
    Ok(Json(svc.create_template(req).await?))
}

pub async fn update_template(State(state): State<SharedState>, Path(id): Path<i64>, Json(req): Json<UpdateTemplateRequest>) -> Result<Json<TemplateResponse>, AppError> {
    let svc = NotificationService::new(&state.db);
    Ok(Json(svc.update_template(id, req).await?))
}

pub async fn delete_template(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<MessageResponse>, AppError> {
    let svc = NotificationService::new(&state.db);
    Ok(Json(svc.delete_template(id).await?))
}

// ── Channels ────────────────────────────────────────────────

pub async fn list_channels(State(state): State<SharedState>) -> Result<Json<Vec<ChannelResponse>>, AppError> {
    let svc = NotificationService::new(&state.db);
    Ok(Json(svc.list_channels().await?))
}

pub async fn upsert_channel(State(state): State<SharedState>, Json(req): Json<UpsertChannelRequest>) -> Result<Json<ChannelResponse>, AppError> {
    req.validate()?;
    let svc = NotificationService::new(&state.db);
    Ok(Json(svc.upsert_channel(req).await?))
}

// ── Notifications ───────────────────────────────────────────

pub async fn send_notification(State(state): State<SharedState>, Json(req): Json<SendNotificationRequest>) -> Result<Json<NotificationResponse>, AppError> {
    req.validate()?;
    let svc = NotificationService::new(&state.db);
    Ok(Json(svc.send(req).await?))
}

pub async fn list_notifications(State(state): State<SharedState>, Query(query): Query<NotificationQuery>) -> Result<Json<NotificationListResponse>, AppError> {
    let svc = NotificationService::new(&state.db);
    Ok(Json(svc.list_notifications(query).await?))
}

pub async fn retry_notification(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<NotificationDetailResponse>, AppError> {
    let svc = NotificationService::new(&state.db);
    Ok(Json(svc.retry_notification(id).await?))
}

// ── History ─────────────────────────────────────────────────

pub async fn list_history(State(state): State<SharedState>, Query(query): Query<HistoryQuery>) -> Result<Json<HistoryListResponse>, AppError> {
    let svc = NotificationService::new(&state.db);
    Ok(Json(svc.list_history(query).await?))
}
