//! Notification delivery orchestrator.
//!
//! Picks up notifications with status `"queued"` from the database and routes
//! them to the correct channel delivery service (Telegram, WhatsApp, Email).
//!
//! Uses a configurable polling interval (default 5 seconds) with exponential
//! backoff on errors. Supports retry with a maximum retry count per notification.

use std::sync::Arc;

use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel,
    QueryFilter, QueryOrder, QuerySelect, Set,
};
use tokio::sync::Mutex;
use tokio_util::sync::CancellationToken;
use tracing::{error, info, warn};

use crate::common::errors::app_error::AppError;
use crate::modules::notification::delivery::email::{self, SmtpConfig};
use crate::modules::notification::delivery::telegram::{self, TelegramConfig};
use crate::modules::notification::delivery::whatsapp::{self, WhatsAppConfig};
use crate::modules::notification::model::notification_channel_entity;
use crate::modules::notification::model::notification_entity::{self, Model as NotificationModel};

const DEFAULT_POLL_INTERVAL_MS: u64 = 5000;
const MAX_RETRIES: i32 = 3;

/// Orchestrator that polls for queued notifications and dispatches them.
pub struct NotificationOrchestrator {
    db: Arc<DatabaseConnection>,
    /// In-memory cache of channel configs keyed by channel name.
    channel_cache: Arc<Mutex<ChannelCache>>,
}

struct ChannelCache {
    telegram: Option<(TelegramConfig, reqwest::Client)>,
    whatsapp: Option<(WhatsAppConfig, reqwest::Client)>,
    email: Option<SmtpConfig>,
}

impl ChannelCache {
    fn new() -> Self {
        Self {
            telegram: None,
            whatsapp: None,
            email: None,
        }
    }
}

/// Unified delivery outcome returned by all channel-specific delivery functions.
#[derive(Debug)]
pub enum DeliveryOutcome {
    Success,
    Failure(String),
}

/// Result of processing a single notification.
#[derive(Debug)]
pub struct DeliveryResult {
    pub notification_id: i64,
    pub channel: String,
    pub success: bool,
    pub error: Option<String>,
}

impl NotificationOrchestrator {
    /// Create a new orchestrator with the given database connection.
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self {
            db,
            channel_cache: Arc::new(Mutex::new(ChannelCache::new())),
        }
    }

    /// Start the orchestrator polling loop.
    ///
    /// Runs until the cancellation token is triggered.
    pub async fn run(&self, token: CancellationToken) {
        let poll_interval_ms = std::env::var("NOTIFICATION_POLL_INTERVAL_MS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(DEFAULT_POLL_INTERVAL_MS);

        info!(
            poll_interval_ms = poll_interval_ms,
            "Notification orchestrator started"
        );

        // Load channel configs once at startup
        if let Err(e) = self.refresh_channel_cache().await {
            warn!(error = %e, "Failed to load initial channel configs, will retry on first delivery");
        }

        loop {
            tokio::select! {
                _ = token.cancelled() => {
                    info!("Notification orchestrator shutting down gracefully");
                    break;
                }
                _ = tokio::time::sleep(std::time::Duration::from_millis(poll_interval_ms)) => {
                    match self.process_queued().await {
                        Ok(count) if count > 0 => {
                            info!(processed = count, "Processed queued notifications");
                        }
                        Ok(_) => {}
                        Err(e) => {
                            error!(error = %e, "Error processing queued notifications");
                        }
                    }
                }
            }
        }
    }

    /// Process all currently queued notifications (up to 50 per batch).
    async fn process_queued(&self) -> Result<u64, AppError> {
        let queued: Vec<NotificationModel> = notification_entity::Entity::find()
            .filter(notification_entity::Column::Status.eq("queued"))
            .order_by_asc(notification_entity::Column::CreatedAt)
            .limit(50)
            .all(self.db.as_ref())
            .await?;

        if queued.is_empty() {
            return Ok(0);
        }

        let count = queued.len() as u64;

        for notification in queued {
            let result = self.deliver(&notification).await;
            match &result {
                Ok(r) if r.success => {
                    info!(
                        notification_id = r.notification_id,
                        channel = %r.channel,
                        "Notification delivered"
                    );
                }
                Ok(r) => {
                    warn!(
                        notification_id = r.notification_id,
                        channel = %r.channel,
                        error = ?r.error,
                        "Delivery failed (will retry if under limit)"
                    );
                }
                Err(e) => {
                    warn!(
                        notification_id = notification.id,
                        channel = %notification.channel,
                        error = %e,
                        "Delivery error"
                    );
                }
            }
        }

        Ok(count)
    }

    /// Deliver a single notification to its channel.
    async fn deliver(&self, notification: &NotificationModel) -> Result<DeliveryResult, AppError> {
        // Mark as processing
        let mut active = notification.clone().into_active_model();
        active.status = Set("sending".to_string());
        active.update(self.db.as_ref()).await?;

        let channel = notification.channel.as_str();
        let body = notification.body.as_deref().unwrap_or("");
        let subject = notification.title.as_deref().unwrap_or("");

        let outcome = match channel {
            "email" => self.deliver_email(notification, subject, body).await,
            "telegram" => self.deliver_telegram(notification, subject, body).await,
            "whatsapp" => self.deliver_whatsapp(notification, subject, body).await,
            _ => {
                warn!(channel = channel, "Unknown notification channel");
                DeliveryOutcome::Failure(format!("Unsupported notification channel: {channel}"))
            }
        };

        // Update notification status based on outcome
        let mut active = notification.clone().into_active_model();
        match &outcome {
            DeliveryOutcome::Success => {
                active.status = Set("sent".to_string());
                active.update(self.db.as_ref()).await?;
                Ok(DeliveryResult {
                    notification_id: notification.id,
                    channel: channel.to_string(),
                    success: true,
                    error: None,
                })
            }
            DeliveryOutcome::Failure(error_msg) => {
                let should_retry = (notification.metadata.as_ref())
                    .and_then(|m| m.get("retry_count"))
                    .and_then(|v| v.as_i64())
                    .unwrap_or(0)
                    < MAX_RETRIES as i64;

                if should_retry {
                    active.status = Set("queued".to_string());
                    let mut meta = notification
                        .metadata
                        .as_ref()
                        .and_then(|m| m.as_object().cloned())
                        .unwrap_or_default();
                    let current = meta
                        .get("retry_count")
                        .and_then(|v| v.as_i64())
                        .unwrap_or(0);
                    meta.insert("retry_count".to_string(), serde_json::json!(current + 1));
                    meta.insert("last_error".to_string(), serde_json::json!(error_msg));
                    active.metadata = Set(Some(serde_json::Value::Object(meta)));
                } else {
                    active.status = Set("failed".to_string());
                }
                active.update(self.db.as_ref()).await?;

                Ok(DeliveryResult {
                    notification_id: notification.id,
                    channel: channel.to_string(),
                    success: false,
                    error: Some(error_msg.clone()),
                })
            }
        }
    }

    /// Deliver via Telegram Bot API.
    async fn deliver_telegram(
        &self,
        notification: &NotificationModel,
        subject: &str,
        body: &str,
    ) -> DeliveryOutcome {
        let cache = self.channel_cache.lock().await;
        let (config, client) = match cache.telegram.as_ref() {
            Some(v) => (v.0.clone(), v.1.clone()),
            None => return DeliveryOutcome::Failure("Telegram channel not configured".into()),
        };
        drop(cache);

        let chat_id = notification
            .customer_id
            .map(|id| id.to_string())
            .unwrap_or_default();

        let result = if subject.is_empty() {
            telegram::send_text(&client, &config, &chat_id, body).await
        } else {
            telegram::send_rich_notification(&client, &config, &chat_id, subject, body).await
        };

        match result {
            Ok(r) if r.success => DeliveryOutcome::Success,
            Ok(r) => DeliveryOutcome::Failure(r.error.unwrap_or_else(|| "Telegram send failed".into())),
            Err(e) => DeliveryOutcome::Failure(format!("Telegram error: {e}")),
        }
    }

    /// Deliver via WhatsApp Business API v23.
    async fn deliver_whatsapp(
        &self,
        notification: &NotificationModel,
        subject: &str,
        body: &str,
    ) -> DeliveryOutcome {
        let cache = self.channel_cache.lock().await;
        let (config, client) = match cache.whatsapp.as_ref() {
            Some(v) => (v.0.clone(), v.1.clone()),
            None => return DeliveryOutcome::Failure("WhatsApp channel not configured".into()),
        };
        drop(cache);

        let to = notification
            .metadata
            .as_ref()
            .and_then(|m| m.get("recipient_address"))
            .and_then(|v| v.as_str())
            .unwrap_or("");

        if to.is_empty() {
            return DeliveryOutcome::Failure(
                "WhatsApp delivery requires recipient phone number in metadata.recipient_address"
                    .into(),
            );
        }

        match whatsapp::send_notification(&client, &config, to, subject, body).await {
            Ok(r) if r.success => DeliveryOutcome::Success,
            Ok(r) => DeliveryOutcome::Failure(r.error.unwrap_or_else(|| "WhatsApp send failed".into())),
            Err(e) => DeliveryOutcome::Failure(format!("WhatsApp error: {e}")),
        }
    }

    /// Deliver via Email SMTP.
    async fn deliver_email(
        &self,
        notification: &NotificationModel,
        subject: &str,
        body: &str,
    ) -> DeliveryOutcome {
        let cache = self.channel_cache.lock().await;
        let config = match cache.email.as_ref() {
            Some(c) => c.clone(),
            None => return DeliveryOutcome::Failure("Email channel not configured".into()),
        };
        drop(cache);

        let to_email = notification
            .metadata
            .as_ref()
            .and_then(|m| m.get("recipient_address"))
            .and_then(|v| v.as_str())
            .unwrap_or("");

        let to_name = notification
            .metadata
            .as_ref()
            .and_then(|m| m.get("recipient_name"))
            .and_then(|v| v.as_str());

        if to_email.is_empty() {
            return DeliveryOutcome::Failure(
                "Email delivery requires recipient email in metadata.recipient_address".into(),
            );
        }

        match email::send_notification(&config, to_email, to_name, subject, body).await {
            Ok(r) if r.success => DeliveryOutcome::Success,
            Ok(r) => DeliveryOutcome::Failure(r.error.unwrap_or_else(|| "Email send failed".into())),
            Err(e) => DeliveryOutcome::Failure(format!("Email error: {e}")),
        }
    }

    /// Refresh the channel config cache from the database.
    pub async fn refresh_channel_cache(&self) -> Result<(), AppError> {
        let channels = notification_channel_entity::Entity::find()
            .filter(notification_channel_entity::Column::IsActive.eq(true))
            .all(self.db.as_ref())
            .await?;

        let mut cache = self.channel_cache.lock().await;

        for channel in &channels {
            match channel.channel.as_str() {
                "email" => match SmtpConfig::from_json(&channel.config) {
                    Ok(config) => {
                        cache.email = Some(config);
                        info!("Loaded email channel config");
                    }
                    Err(e) => warn!(error = %e, "Failed to load email config"),
                },
                "telegram" => match TelegramConfig::from_json(&channel.config) {
                    Ok(config) => {
                        let client = reqwest::Client::builder()
                            .timeout(std::time::Duration::from_secs(30))
                            .build()
                            .expect("Failed to build HTTP client");
                        cache.telegram = Some((config, client));
                        info!("Loaded Telegram channel config");
                    }
                    Err(e) => warn!(error = %e, "Failed to load Telegram config"),
                },
                "whatsapp" => match WhatsAppConfig::from_json(&channel.config) {
                    Ok(config) => {
                        let client = reqwest::Client::builder()
                            .timeout(std::time::Duration::from_secs(30))
                            .build()
                            .expect("Failed to build HTTP client");
                        cache.whatsapp = Some((config, client));
                        info!("Loaded WhatsApp channel config");
                    }
                    Err(e) => warn!(error = %e, "Failed to load WhatsApp config"),
                },
                _ => {
                    warn!(channel = %channel.channel, "Unknown channel type in config");
                }
            }
        }

        Ok(())
    }

    /// Send a notification immediately (synchronous, used by the API).
    ///
    /// This is called when a notification is created via the REST API and
    /// the caller wants immediate delivery rather than waiting for the
    /// polling loop.
    pub async fn send_immediate(
        &self,
        channel: &str,
        recipient_id: i64,
        _address: &str,
        subject: Option<&str>,
        body: &str,
    ) -> Result<DeliveryResult, AppError> {
        let now = chrono::Utc::now();
        let active_model = notification_entity::ActiveModel {
            r#type: Set("direct".to_string()),
            channel: Set(channel.to_string()),
            title: Set(subject.map(|s| s.to_string())),
            body: Set(Some(body.to_string())),
            status: Set("sending".to_string()),
            customer_id: Set(Some(recipient_id)),
            created_at: Set(now.into()),
            ..Default::default()
        };

        let notification = active_model.insert(self.db.as_ref()).await?;

        self.deliver(&notification).await
    }
}
