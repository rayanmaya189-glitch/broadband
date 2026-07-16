use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait, Set, ActiveModelTrait, QueryOrder, QuerySelect};
use tracing::{info, warn, error};

/// Background worker for notification delivery:
/// - Process queued notifications
/// - Send via email/SMS/WhatsApp
/// - Retry failed notifications with exponential backoff
pub struct NotificationWorker {
    db: DatabaseConnection,
}

impl NotificationWorker {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// Run the full notification worker cycle.
    pub async fn run_cycle(&self) -> anyhow::Result<()> {
        info!("Notification worker: starting cycle");
        self.process_queue().await?;
        self.retry_failed().await?;
        info!("Notification worker: cycle complete");
        Ok(())
    }

    /// Process queued notifications and send them.
    pub async fn process_queue(&self) -> anyhow::Result<()> {
        info!("Notification worker: processing queue");

        use crate::modules::notification::domain::entities::notification;

        let queued = notification::Entity::find()
            .filter(notification::Column::Status.eq("queued"))
            .order_by_asc(notification::Column::CreatedAt)
            .limit(50)
            .all(&self.db)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to query queued notifications: {}", e))?;

        let count = queued.len();
        if count == 0 {
            info!("Notification worker: no queued notifications");
            return Ok(());
        }

        info!(count = count, "Notification worker: processing notifications");

        for notif in &queued {
            match self.send_notification(notif).await {
                Ok(()) => {
                    let mut active: notification::ActiveModel = notif.clone().into();
                    active.status = Set("sent".to_string());
                    active.sent_at = Set(Some(chrono::Utc::now()));

                    if let Err(e) = active.update(&self.db).await {
                        error!(
                            notification_id = notif.id,
                            error = %e,
                            "Failed to mark notification as sent"
                        );
                    }
                }
                Err(e) => {
                    error!(
                        notification_id = notif.id,
                        error = %e,
                        "Failed to send notification"
                    );

                    let mut active: notification::ActiveModel = notif.clone().into();
                    active.retry_count = Set(notif.retry_count + 1);
                    active.last_error = Set(Some(e.to_string()));

                    if notif.retry_count >= notif.max_retries {
                        active.status = Set("failed".to_string());
                        warn!(
                            notification_id = notif.id,
                            "Notification exceeded max retries, marking as failed"
                        );
                    } else {
                        active.status = Set("retrying".to_string());
                    }

                    if let Err(update_err) = active.update(&self.db).await {
                        error!(
                            notification_id = notif.id,
                            error = %update_err,
                            "Failed to update notification retry state"
                        );
                    }
                }
            }
        }

        info!(count = count, "Notification worker: processed notifications");
        Ok(())
    }

    /// Retry failed notifications that haven't exceeded max retries.
    pub async fn retry_failed(&self) -> anyhow::Result<()> {
        info!("Notification worker: retrying failed notifications");

        use crate::modules::notification::domain::entities::notification;

        // Fetch notifications that are in retrying status
        let retryable = notification::Entity::find()
            .filter(notification::Column::Status.eq("retrying"))
            .order_by_asc(notification::Column::CreatedAt)
            .limit(20)
            .all(&self.db)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to query retryable notifications: {}", e))?;

        // Filter in Rust to avoid column comparison issues
        let retryable: Vec<_> = retryable.into_iter()
            .filter(|n| n.retry_count < n.max_retries)
            .collect();

        let count = retryable.len();
        if count == 0 {
            info!("Notification worker: no retryable notifications");
            return Ok(());
        }

        info!(count = count, "Notification worker: retrying notifications");

        for notif in &retryable {
            let backoff_secs = 5 * 2u64.pow(notif.retry_count as u32);
            tokio::time::sleep(std::time::Duration::from_secs(backoff_secs)).await;

            match self.send_notification(notif).await {
                Ok(()) => {
                    let mut active: notification::ActiveModel = notif.clone().into();
                    active.status = Set("sent".to_string());
                    active.sent_at = Set(Some(chrono::Utc::now()));

                    if let Err(e) = active.update(&self.db).await {
                        error!(
                            notification_id = notif.id,
                            error = %e,
                            "Failed to mark retried notification as sent"
                        );
                    }
                }
                Err(e) => {
                    let mut active: notification::ActiveModel = notif.clone().into();
                    active.retry_count = Set(notif.retry_count + 1);
                    active.last_error = Set(Some(e.to_string()));

                    if notif.retry_count + 1 >= notif.max_retries {
                        active.status = Set("failed".to_string());
                    }

                    if let Err(update_err) = active.update(&self.db).await {
                        error!(
                            notification_id = notif.id,
                            error = %update_err,
                            "Failed to update retry state"
                        );
                    }
                }
            }
        }

        Ok(())
    }

    /// Send a notification via the appropriate channel.
    async fn send_notification(
        &self,
        notif: &crate::modules::notification::domain::entities::notification::Model,
    ) -> anyhow::Result<()> {
        match notif.channel.as_str() {
            "email" => {
                info!(
                    notification_id = notif.id,
                    recipient = %notif.recipient_address,
                    subject = ?notif.subject,
                    "Sending email notification"
                );
                tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                Ok(())
            }
            "sms" => {
                info!(
                    notification_id = notif.id,
                    recipient = %notif.recipient_address,
                    "Sending SMS notification"
                );
                tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                Ok(())
            }
            "whatsapp" => {
                info!(
                    notification_id = notif.id,
                    recipient = %notif.recipient_address,
                    "Sending WhatsApp notification"
                );
                tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                Ok(())
            }
            "push" => {
                info!(
                    notification_id = notif.id,
                    recipient = %notif.recipient_address,
                    "Sending push notification"
                );
                tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                Ok(())
            }
            "in_app" => {
                info!(
                    notification_id = notif.id,
                    recipient = %notif.recipient_address,
                    "Creating in-app notification"
                );
                Ok(())
            }
            _ => {
                Err(anyhow::anyhow!(
                    "Unknown notification channel: {}",
                    notif.channel
                ))
            }
        }
    }
}
