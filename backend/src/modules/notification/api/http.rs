use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::modules::notification::application::services::NotificationService;
use crate::shared::app_state::AppState;
use crate::shared::errors::AppError;
use crate::shared::middleware::auth::{require_permission, UserContext};
use crate::shared::primitives::PaginationParams;

#[derive(Debug, Serialize)]
pub struct TemplateResponse {
    pub id: i64,
    pub name: String,
    pub channel: String,
    pub is_active: bool,
}

#[derive(Debug, Deserialize)]
pub struct CreateTemplateRequest {
    pub name: String,
    pub channel: String,
    pub body_template: String,
    #[serde(default)]
    pub subject_template: Option<String>,
}

pub async fn list_templates(
    State(state): State<Arc<AppState>>,
    _user: UserContext,
    Query(p): Query<PaginationParams>,
) -> Result<Json<serde_json::Value>, AppError> {
    let (tmpls, total) = NotificationService::list_templates(&state.db, p.page(), p.limit()).await?;
    let items: Vec<TemplateResponse> = tmpls
            .into_iter()
            .map(|t| TemplateResponse {
                id: t.id,
                name: t.name,
                channel: t.channel,
                is_active: t.is_active,
            })
            .collect();
    Ok(Json(serde_json::json!({"items": items, "total": total, "page": p.page(), "limit": p.limit()})))
}

pub async fn create_template(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Json(req): Json<CreateTemplateRequest>,
) -> Result<(StatusCode, Json<TemplateResponse>), AppError> {
    require_permission(&user, "notification.template.create").map_err(|e| AppError::Forbidden(e.1))?;
    let t = NotificationService::create_template(
        &state.db,
        req.name,
        req.channel,
        req.body_template,
        req.subject_template,
    )
    .await?;
    if let Err(e) = crate::infrastructure::messaging::outbox::insert_outbox_event(
        &state.db, "notification.template.created", "notification_template", t.id,
        serde_json::json!({"template_id": t.id}), None,
        Some(user.user_id), user.branch_id,
    ).await {
        tracing::error!(error = %e, "Failed to publish notification.template.created event");
    }
    Ok((
        StatusCode::CREATED,
        Json(TemplateResponse {
            id: t.id,
            name: t.name,
            channel: t.channel,
            is_active: t.is_active,
        }),
    ))
}

#[derive(Debug, Deserialize)]
pub struct SendNotificationRequest {
    pub channel: String,
    pub recipient_type: String,
    pub recipient_id: i64,
    pub recipient_address: String,
    #[serde(default)]
    pub subject: Option<String>,
    pub body: String,
}

#[derive(Debug, Serialize)]
pub struct NotificationResponse {
    pub id: i64,
    pub channel: String,
    pub status: String,
    pub recipient_address: String,
}

pub async fn send_notification(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Json(req): Json<SendNotificationRequest>,
) -> Result<(StatusCode, Json<NotificationResponse>), AppError> {
    require_permission(&user, "notification.send").map_err(|e| AppError::Forbidden(e.1))?;
    let n = NotificationService::send_notification(
        &state.db,
        req.channel.clone(),
        req.recipient_type,
        req.recipient_id,
        req.recipient_address.clone(),
        req.subject,
        req.body,
    )
    .await?;
    if let Err(e) = crate::infrastructure::messaging::outbox::insert_outbox_event(
        &state.db, "notification.sent", "notification", n.id,
        serde_json::json!({"notification_id": n.id, "channel": n.channel}), None,
        Some(user.user_id), user.branch_id,
    ).await {
        tracing::error!(error = %e, "Failed to publish notification.sent event");
    }
    Ok((
        StatusCode::CREATED,
        Json(NotificationResponse {
            id: n.id,
            channel: n.channel,
            status: n.status,
            recipient_address: n.recipient_address,
        }),
    ))
}
