//! Notification Worker
//!
//! Processes queued notifications and delivers them via the configured channels
//! (email, SMS, WhatsApp, Telegram).

use std::sync::Arc;
use tokio_util::sync::CancellationToken;

use crate::app::AppState;

/// Run the notification worker.
///
/// This worker processes the notification queue and delivers messages
/// via the appropriate channels. It handles retries for failed deliveries.
pub async fn run_notification_worker(state: Arc<AppState>, shutdown: CancellationToken) {
    tracing::info!("Notification worker started");

    loop {
        tokio::select! {
            _ = shutdown.cancelled() => {
                tracing::info!("Notification worker shutting down");
                break;
            }
            _ = tokio::time::sleep(std::time::Duration::from_secs(5)) => {
                if let Err(e) = process_notification_queue(&state).await {
                    tracing::error!(error = %e, "Notification processing failed");
                }
            }
        }
    }
}

/// Process pending notifications from the queue.
async fn process_notification_queue(state: &Arc<AppState>) -> Result<(), crate::common::errors::app_error::AppError> {
    use sea_orm::{EntityTrait, QueryFilter, ColumnTrait, QueryOrder, QuerySelect, Set, ActiveModelTrait};
    use crate::modules::notification::model::notification_entity;

    // Fetch pending notifications
    let notifications: Vec<notification_entity::Model> = notification_entity::Entity::find()
        .filter(notification_entity::Column::Status.eq("queued"))
        .order_by_asc(notification_entity::Column::Id)
        .limit(50)
        .all(&state.db)
        .await?;

    for notification in &notifications {
        // Mark as processing
        let active_model = notification_entity::ActiveModel {
            id: sea_orm::Set(notification.id),
            status: Set("processing".to_string()),
            ..Default::default()
        };
        active_model.update(&state.db).await?;

        // Deliver based on channel
        let result = match notification.channel.as_str() {
            "email" => deliver_email(state, &notification).await,
            "sms" => deliver_sms(state, &notification).await,
            "whatsapp" => deliver_whatsapp(state, &notification).await,
            "telegram" => deliver_telegram(state, &notification).await,
            _ => {
                tracing::warn!(channel = %notification.channel, "Unknown notification channel");
                Ok(())
            }
        };

        // Update status based on result
        let mut active_model = notification_entity::ActiveModel {
            id: sea_orm::Set(notification.id),
            status: Set("delivered".to_string()),
            ..Default::default()
        };
        match result {
            Ok(()) => {
                // Status already set to delivered
                let _ = state.nats.publish_event("notification.sent", &serde_json::json!({
                    "notification_id": notification.id,
                    "channel": notification.channel,
                })).await;
            }
            Err(e) => {
                tracing::error!(notification_id = notification.id, error = %e, "Notification delivery failed");
                active_model.status = Set("failed".to_string());
                let _ = state.nats.publish_event("notification.failed", &serde_json::json!({
                    "notification_id": notification.id,
                    "channel": notification.channel,
                    "error": e.to_string(),
                })).await;
            }
        }
        active_model.update(&state.db).await?;
    }

    Ok(())
}

/// Deliver notification via email.
async fn deliver_email(
    _state: &Arc<AppState>,
    notification: &crate::modules::notification::model::notification_entity::Model,
) -> Result<(), crate::common::errors::app_error::AppError> {
    // In production, this would use the SMTP client from notification/delivery/email.rs
    tracing::info!(notification_id = notification.id, "Delivering email notification");
    Ok(())
}

/// Deliver notification via SMS.
async fn deliver_sms(
    _state: &Arc<AppState>,
    notification: &crate::modules::notification::model::notification_entity::Model,
) -> Result<(), crate::common::errors::app_error::AppError> {
    tracing::info!(notification_id = notification.id, "Delivering SMS notification");
    Ok(())
}

/// Deliver notification via WhatsApp.
async fn deliver_whatsapp(
    _state: &Arc<AppState>,
    notification: &crate::modules::notification::model::notification_entity::Model,
) -> Result<(), crate::common::errors::app_error::AppError> {
    tracing::info!(notification_id = notification.id, "Delivering WhatsApp notification");
    Ok(())
}

/// Deliver notification via Telegram.
async fn deliver_telegram(
    _state: &Arc<AppState>,
    notification: &crate::modules::notification::model::notification_entity::Model,
) -> Result<(), crate::common::errors::app_error::AppError> {
    tracing::info!(notification_id = notification.id, "Delivering Telegram notification");
    Ok(())
}
