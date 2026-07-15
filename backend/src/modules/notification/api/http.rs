use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::shared::app_state::AppState;
use crate::shared::errors::AppError;
use crate::shared::middleware::auth::UserContext;
use crate::modules::notification::application::services::NotificationService;

#[derive(Debug, Serialize)]
pub struct TemplateResponse { pub id: i64, pub name: String, pub channel: String, pub is_active: bool }

#[derive(Debug, Deserialize)]
pub struct CreateTemplateRequest { pub name: String, pub channel: String, pub body_template: String, #[serde(default)] pub subject_template: Option<String> }

pub async fn list_templates(State(state): State<Arc<AppState>>, _user: UserContext) -> Result<Json<Vec<TemplateResponse>>, AppError> {
    let tmpls = NotificationService::list_templates(&state.db).await?;
    Ok(Json(tmpls.into_iter().map(|t| TemplateResponse { id: t.id, name: t.name, channel: t.channel, is_active: t.is_active }).collect()))
}

pub async fn create_template(State(state): State<Arc<AppState>>, _user: UserContext, Json(req): Json<CreateTemplateRequest>) -> Result<(StatusCode, Json<TemplateResponse>), AppError> {
    let t = NotificationService::create_template(&state.db, req.name, req.channel, req.body_template, req.subject_template).await?;
    Ok((StatusCode::CREATED, Json(TemplateResponse { id: t.id, name: t.name, channel: t.channel, is_active: t.is_active })))
}

#[derive(Debug, Deserialize)]
pub struct SendNotificationRequest {
    pub channel: String, pub recipient_type: String, pub recipient_id: i64,
    pub recipient_address: String,    #[serde(default)]
    pub subject: Option<String>, pub body: String,
}

#[derive(Debug, Serialize)]
pub struct NotificationResponse { pub id: i64, pub channel: String, pub status: String, pub recipient_address: String }

pub async fn send_notification(State(state): State<Arc<AppState>>, _user: UserContext, Json(req): Json<SendNotificationRequest>) -> Result<(StatusCode, Json<NotificationResponse>), AppError> {
    let n = NotificationService::send_notification(&state.db, req.channel.clone(), req.recipient_type, req.recipient_id, req.recipient_address.clone(), req.subject, req.body).await?;
    Ok((StatusCode::CREATED, Json(NotificationResponse { id: n.id, channel: n.channel, status: n.status, recipient_address: n.recipient_address })))
}
