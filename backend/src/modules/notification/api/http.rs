use axum::extract::{Path, Query, State};
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
    let (tmpls, total) =
        NotificationService::list_templates(&state.db, p.page(), p.limit()).await?;
    let items: Vec<TemplateResponse> = tmpls
        .into_iter()
        .map(|t| TemplateResponse {
            id: t.id,
            name: t.name,
            channel: t.channel,
            is_active: t.is_active,
        })
        .collect();
    Ok(Json(
        serde_json::json!({"items": items, "total": total, "page": p.page(), "limit": p.limit()}),
    ))
}

pub async fn create_template(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Json(req): Json<CreateTemplateRequest>,
) -> Result<(StatusCode, Json<TemplateResponse>), AppError> {
    require_permission(&user, "notification.template.create")
        .map_err(|e| AppError::Forbidden(e.1))?;
    let t = NotificationService::create_template(
        &state.db,
        req.name,
        req.channel,
        req.body_template,
        req.subject_template,
    )
    .await?;
    if let Err(e) = crate::infrastructure::messaging::outbox::insert_outbox_event(
        &state.db,
        "notification.template.created",
        "notification_template",
        t.id,
        serde_json::json!({"template_id": t.id}),
        None,
        Some(user.user_id),
        user.branch_id,
    )
    .await
    {
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
        &state.db,
        "notification.sent",
        "notification",
        n.id,
        serde_json::json!({"notification_id": n.id, "channel": n.channel}),
        None,
        Some(user.user_id),
        user.branch_id,
    )
    .await
    {
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

/// GET /api/v1/notifications/list
pub async fn list_notifications(
    State(state): State<Arc<AppState>>,
    _user: UserContext,
    Query(p): Query<PaginationParams>,
) -> Result<Json<serde_json::Value>, AppError> {
    let (items, total) =
        NotificationService::list_notifications(&state.db, p.page(), p.limit()).await?;
    let resp: Vec<NotificationResponse> = items
        .into_iter()
        .map(|n| NotificationResponse {
            id: n.id,
            channel: n.channel,
            status: n.status,
            recipient_address: n.recipient_address,
        })
        .collect();
    Ok(Json(serde_json::json!({ "items": resp, "total": total })))
}

/// POST /api/v1/notifications/retry
pub async fn retry_failed_notifications(
    State(state): State<Arc<AppState>>,
    user: UserContext,
) -> Result<Json<serde_json::Value>, AppError> {
    require_permission(&user, "notification.retry").map_err(|e| AppError::Forbidden(e.1))?;
    let count = NotificationService::retry_failed_notifications(&state.db).await?;
    Ok(Json(serde_json::json!({
        "retried": count,
        "message": format!("{} notification(s) queued for retry", count),
    })))
}

#[derive(Debug, Deserialize)]
pub struct UpdateTemplateRequest {
    pub subject: Option<String>,
    pub body: Option<String>,
    pub channel: Option<String>,
}

pub async fn update_template(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Path(id): Path<i64>,
    Json(req): Json<UpdateTemplateRequest>,
) -> Result<Json<TemplateResponse>, AppError> {
    require_permission(&user, "notification.template.update")
        .map_err(|e| AppError::Forbidden(e.1))?;
    let t = NotificationService::update_template(&state.db, id, req.subject, req.body, req.channel)
        .await?;
    Ok(Json(TemplateResponse {
        id: t.id,
        name: t.name,
        channel: t.channel,
        is_active: t.is_active,
    }))
}

#[derive(Debug, Serialize)]
pub struct ChannelResponse {
    pub id: i64,
    pub name: String,
    pub channel_type: String,
    pub is_active: bool,
    pub config: Option<serde_json::Value>,
}

pub async fn list_channels(
    State(state): State<Arc<AppState>>,
    _user: UserContext,
) -> Result<Json<serde_json::Value>, AppError> {
    let channels = NotificationService::list_channels(&state.db).await?;
    let items: Vec<ChannelResponse> = channels
        .into_iter()
        .map(|c| ChannelResponse {
            id: c.id,
            name: c.name,
            channel_type: c.channel_type,
            is_active: c.is_active,
            config: c.config,
        })
        .collect();
    Ok(Json(serde_json::json!({ "items": items })))
}

#[derive(Debug, Deserialize)]
pub struct UpdateChannelRequest {
    pub is_active: Option<bool>,
    pub config: Option<serde_json::Value>,
}

pub async fn update_channel(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Path(id): Path<i64>,
    Json(req): Json<UpdateChannelRequest>,
) -> Result<Json<ChannelResponse>, AppError> {
    require_permission(&user, "notification.channel.update")
        .map_err(|e| AppError::Forbidden(e.1))?;
    let ch =
        NotificationService::update_channel(&state.db, id, req.is_active, req.config).await?;
    Ok(Json(ChannelResponse {
        id: ch.id,
        name: ch.name,
        channel_type: ch.channel_type,
        is_active: ch.is_active,
        config: ch.config,
    }))
}

#[derive(Debug, Deserialize)]
pub struct DeliveryHistoryParams {
    pub page: Option<u64>,
    pub limit: Option<u64>,
    pub status: Option<String>,
    pub channel: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct DeliveryHistoryResponse {
    pub id: i64,
    pub notification_id: i64,
    pub channel: String,
    pub status: String,
    pub attempts: i32,
    pub last_error: Option<String>,
    pub sent_at: Option<chrono::DateTime<chrono::Utc>>,
}

pub async fn list_delivery_history(
    State(state): State<Arc<AppState>>,
    _user: UserContext,
    Query(params): Query<DeliveryHistoryParams>,
) -> Result<Json<serde_json::Value>, AppError> {
    let page = params.page.unwrap_or(1);
    let limit = params.limit.unwrap_or(20);
    let (items, total) = NotificationService::list_delivery_history(
        &state.db,
        page,
        limit,
        params.status,
        params.channel,
    )
    .await?;
    let resp: Vec<DeliveryHistoryResponse> = items
        .into_iter()
        .map(|h| DeliveryHistoryResponse {
            id: h.id,
            notification_id: h.notification_id,
            channel: h.channel,
            status: h.status,
            attempts: h.attempts,
            last_error: h.last_error,
            sent_at: h.sent_at,
        })
        .collect();
    Ok(Json(
        serde_json::json!({ "items": resp, "total": total, "page": page, "limit": limit }),
    ))
}
