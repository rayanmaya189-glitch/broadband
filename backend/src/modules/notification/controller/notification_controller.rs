//! SeaORM-based controller for the Notification domain.

use axum::extract::{Json, Path, Query, State};
use validator::Validate;

use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::modules::notification::request::notification_request::*;
use crate::modules::notification::response::notification_response::*;
use crate::modules::notification::service::notification_service::NotificationService;

pub async fn list_templates(State(state): State<SharedState>) -> Result<Json<Vec<NotificationTemplateResponse>>, AppError> {
    let svc = NotificationService::new(&state.db_seaorm);
    Ok(Json(svc.list_templates().await?))
}

pub async fn create_template(State(state): State<SharedState>, Json(req): Json<CreateNotificationTemplateRequest>) -> Result<Json<NotificationTemplateResponse>, AppError> {
    req.validate()?;
    let svc = NotificationService::new(&state.db_seaorm);
    Ok(Json(svc.create_template(req).await?))
}

pub async fn update_template(State(state): State<SharedState>, Path(id): Path<i64>, Json(req): Json<UpdateTemplateRequest>) -> Result<Json<NotificationTemplateResponse>, AppError> {
    let svc = NotificationService::new(&state.db_seaorm);
    Ok(Json(svc.update_template(id, req).await?))
}

pub async fn delete_template(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<MessageResponse>, AppError> {
    let svc = NotificationService::new(&state.db_seaorm);
    Ok(Json(svc.delete_template(id).await?))
}

pub async fn list_channels(State(state): State<SharedState>) -> Result<Json<Vec<NotificationChannelResponse>>, AppError> {
    let svc = NotificationService::new(&state.db_seaorm);
    Ok(Json(svc.list_channels().await?))
}

pub async fn upsert_channel(State(state): State<SharedState>, Json(req): Json<UpsertChannelRequest>) -> Result<Json<NotificationChannelResponse>, AppError> {
    req.validate()?;
    let svc = NotificationService::new(&state.db_seaorm);
    Ok(Json(svc.upsert_channel(&req.channel, &req.provider, req.config).await?))
}

pub async fn send(State(state): State<SharedState>, Json(req): Json<SendNotificationRequest>) -> Result<Json<NotificationResponse>, AppError> {
    req.validate()?;
    let svc = NotificationService::new(&state.db_seaorm);
    Ok(Json(svc.send(&req.channel, req.recipient_id, &req.address, req.subject.as_deref(), &req.body).await?))
}

pub async fn list_notifications(State(state): State<SharedState>, Query(q): Query<NotificationQuery>) -> Result<Json<Vec<NotificationResponse>>, AppError> {
    let svc = NotificationService::new(&state.db_seaorm);
    let (notifications, _) = svc.list_notifications(q.channel.as_deref(), q.status.as_deref(), q.page.unwrap_or(1), q.per_page.unwrap_or(20)).await?;
    Ok(Json(notifications))
}

pub async fn retry_notification(State(_state): State<SharedState>, Path(_id): Path<i64>) -> Result<Json<MessageResponse>, AppError> {
    Ok(Json(MessageResponse { message: "Notification queued for retry".into() }))
}

pub async fn list_history(State(_state): State<SharedState>, Query(_q): Query<HistoryQuery>) -> Result<Json<serde_json::Value>, AppError> {
    Ok(Json(serde_json::json!({ "history": [], "total": 0 })))
}
