use axum::extract::{Json, State};
use validator::Validate;
use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::modules::notification::request::notification_request::*;
use crate::modules::notification::response::notification_response::*;
use crate::modules::notification::service::notification_service::NotificationService;

pub async fn list_templates(State(state): State<SharedState>) -> Result<Json<Vec<TemplateResponse>>, AppError> {
    let svc = NotificationService::new(&state.db);
    Ok(Json(svc.list_templates().await?))
}

pub async fn create_template(State(state): State<SharedState>, Json(req): Json<CreateTemplateRequest>) -> Result<Json<TemplateResponse>, AppError> {
    req.validate()?;
    let svc = NotificationService::new(&state.db);
    Ok(Json(svc.create_template(req).await?))
}

pub async fn send_notification(State(state): State<SharedState>, Json(req): Json<SendNotificationRequest>) -> Result<Json<NotificationResponse>, AppError> {
    req.validate()?;
    let svc = NotificationService::new(&state.db);
    Ok(Json(svc.send(req).await?))
}
