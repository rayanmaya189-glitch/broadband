use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder,
    QuerySelect, Set,
};
use tracing::{error, info, warn};

use crate::modules::integrations::push::FcmAdapter;
use crate::modules::integrations::sms::msg91::Msg91Adapter;
use crate::modules::integrations::sms::SmsProvider;
use crate::modules::integrations::smtp::{EmailProvider, LettreSmtpAdapter};
use crate::modules::integrations::whatsapp::WhatsAppAdapter;

/// Background worker for notification delivery:
/// - Process queued notifications
/// - Send via email/SMS/WhatsApp/Push
/// - Retry failed notifications with exponential backoff
pub struct NotificationWorker {
    db: DatabaseConnection,
    smtp_adapter: Option<LettreSmtpAdapter>,
    sms_adapter: Option<Msg91Adapter>,
    whatsapp_adapter: Option<WhatsAppAdapter>,
    push_adapter: Option<FcmAdapter>,
}

impl NotificationWorker {
    pub fn new(db: DatabaseConnection) -> Self {
        let smtp_adapter = LettreSmtpAdapter::from_env();
        let smtp = if smtp_adapter.is_configured() {
            Some(smtp_adapter)
        } else {
            warn!("SMTP not configured, email notifications disabled");
            None
        };
        let sms_adapter = Msg91Adapter::from_env();
        let sms = if sms_adapter.is_configured() {
            Some(sms_adapter)
        } else {
            warn!("MSG91 not configured (missing MSG91_AUTH_KEY), SMS notifications disabled");
            None
        };
        let whatsapp_adapter = WhatsAppAdapter::from_env();
        let whatsapp = if whatsapp_adapter.is_configured() {
            Some(whatsapp_adapter)
        } else {
            warn!("WhatsApp Business API not configured, WhatsApp notifications disabled");
            None
        };
        let push_adapter = FcmAdapter::from_env();
        let push = if push_adapter.is_configured() {
            Some(push_adapter)
        } else {
            warn!("FCM not configured, push notifications disabled");
            None
        };
        Self {
            db,
            smtp_adapter: smtp,
            sms_adapter: sms,
            whatsapp_adapter: whatsapp,
            push_adapter: push,
        }
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

        info!(
            count = count,
            "Notification worker: processing notifications"
        );

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

        info!(
            count = count,
            "Notification worker: processed notifications"
        );
        Ok(())
    }

    /// Retry failed notifications that haven't exceeded max retries.
    pub async fn retry_failed(&self) -> anyhow::Result<()> {
        info!("Notification worker: retrying failed notifications");

        use crate::modules::notification::domain::entities::notification;

        let retryable = notification::Entity::find()
            .filter(notification::Column::Status.eq("retrying"))
            .order_by_asc(notification::Column::CreatedAt)
            .limit(20)
            .all(&self.db)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to query retryable notifications: {}", e))?;

        let retryable: Vec<_> = retryable
            .into_iter()
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
                    "Sending email notification via SMTP"
                );
                // Wire actual SMTP adapter (lettre crate)
                match self
                    .send_email_via_smtp(
                        &notif.recipient_address,
                        notif.subject.as_deref().unwrap_or("AeroXe Notification"),
                        &notif.body,
                    )
                    .await
                {
                    Ok(status) => {
                        info!(
                            notification_id = notif.id,
                            message_id = %status.message_id,
                            "Email sent successfully via SMTP"
                        );
                        Ok(())
                    }
                    Err(e) => {
                        error!(
                            notification_id = notif.id,
                            error = %e,
                            "SMTP email send failed"
                        );
                        Err(anyhow::anyhow!("Email delivery failed: {}", e))
                    }
                }
            }
            "sms" => {
                info!(
                    notification_id = notif.id,
                    recipient = %notif.recipient_address,
                    "Sending SMS notification via MSG91"
                );
                // Wire actual MSG91 SMS adapter
                match self
                    .send_sms_via_msg91(&notif.recipient_address, &notif.body)
                    .await
                {
                    Ok(request_id) => {
                        info!(
                            notification_id = notif.id,
                            request_id = %request_id,
                            "SMS sent successfully via MSG91"
                        );
                        Ok(())
                    }
                    Err(e) => {
                        error!(
                            notification_id = notif.id,
                            error = %e,
                            "MSG91 SMS send failed"
                        );
                        Err(anyhow::anyhow!("SMS delivery failed: {}", e))
                    }
                }
            }
            "whatsapp" => {
                info!(
                    notification_id = notif.id,
                    recipient = %notif.recipient_address,
                    "Sending WhatsApp notification via WhatsApp Business API"
                );
                match self
                    .send_whatsapp_notification(&notif.recipient_address, &notif.body)
                    .await
                {
                    Ok(status) => {
                        info!(
                            notification_id = notif.id,
                            message_id = %status.message_id,
                            "WhatsApp message sent successfully"
                        );
                        Ok(())
                    }
                    Err(e) => {
                        error!(
                            notification_id = notif.id,
                            error = %e,
                            "WhatsApp Business API send failed"
                        );
                        Err(anyhow::anyhow!("WhatsApp delivery failed: {}", e))
                    }
                }
            }
            "push" => {
                info!(
                    notification_id = notif.id,
                    recipient = %notif.recipient_address,
                    "Sending push notification via FCM"
                );
                match self
                    .send_push_notification(
                        &notif.recipient_address,
                        notif.subject.as_deref().unwrap_or("AeroXe Notification"),
                        &notif.body,
                    )
                    .await
                {
                    Ok(status) => {
                        info!(
                            notification_id = notif.id,
                            message_id = %status.message_id,
                            "Push notification sent successfully"
                        );
                        Ok(())
                    }
                    Err(e) => {
                        error!(
                            notification_id = notif.id,
                            error = %e,
                            "FCM push notification failed"
                        );
                        Err(anyhow::anyhow!("Push delivery failed: {}", e))
                    }
                }
            }
            "in_app" => {
                info!(
                    notification_id = notif.id,
                    recipient = %notif.recipient_address,
                    "Creating in-app notification"
                );
                // In-app notifications are stored in DB, no external dispatch needed
                Ok(())
            }
            _ => Err(anyhow::anyhow!(
                "Unknown notification channel: {}",
                notif.channel
            )),
        }
    }

    /// Send email via cached SMTP adapter (lettre crate)
    async fn send_email_via_smtp(
        &self,
        to: &str,
        subject: &str,
        body: &str,
    ) -> anyhow::Result<crate::modules::integrations::smtp::EmailDeliveryStatus> {
        let adapter = self.smtp_adapter.as_ref().ok_or_else(|| {
            anyhow::anyhow!("SMTP not configured (missing SMTP_USERNAME or SMTP_PASSWORD)")
        })?;
        adapter
            .send_html_email(to, subject, body, None)
            .await
            .map_err(|e| anyhow::anyhow!("SMTP error: {}", e))
    }

    /// Send SMS via cached MSG91 adapter
    async fn send_sms_via_msg91(&self, phone: &str, message: &str) -> anyhow::Result<String> {
        let adapter = self
            .sms_adapter
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("MSG91 not configured (missing MSG91_AUTH_KEY)"))?;
        let request_id = adapter
            .send_sms(phone, message, None)
            .await
            .map_err(|e| anyhow::anyhow!("MSG91 error: {}", e))?;
        Ok(request_id)
    }

    /// Send WhatsApp notification via WhatsApp Business API
    async fn send_whatsapp_notification(
        &self,
        phone: &str,
        message: &str,
    ) -> anyhow::Result<crate::modules::integrations::whatsapp::WhatsAppDeliveryStatus> {
        let adapter = self.whatsapp_adapter.as_ref().ok_or_else(|| {
            anyhow::anyhow!("WhatsApp Business API not configured (missing WHATSAPP_ACCESS_TOKEN)")
        })?;
        adapter
            .send_text_message(phone, message)
            .await
            .map_err(|e| anyhow::anyhow!("WhatsApp error: {}", e))
    }

    /// Send push notification via FCM
    async fn send_push_notification(
        &self,
        device_token: &str,
        title: &str,
        body: &str,
    ) -> anyhow::Result<crate::modules::integrations::push::PushDeliveryStatus> {
        let _adapter = self.push_adapter.as_ref().ok_or_else(|| {
            anyhow::anyhow!("FCM not configured (missing FCM_SERVICE_ACCOUNT_KEY)")
        })?;
        // NOTE: FCM send_push requires &mut self for token caching;
        // Create a fresh adapter per call for simplicity
        let mut adapter_clone = crate::modules::integrations::push::FcmAdapter::new(
            crate::modules::integrations::push::FcmConfig::from_env(),
        );
        adapter_clone
            .send_push(device_token, title, body, None)
            .await
            .map_err(|e| anyhow::anyhow!("FCM error: {}", e))
    }
}
