//! Lettre-based SMTP adapter for email delivery
//!
//! Uses the `lettre` crate for reliable SMTP email sending with:
//! - Async SMTP transport with connection pooling
//! - STARTTLS/TLS encryption
//! - HTML and plain text support

use async_trait::async_trait;
use lettre::message::{header::ContentType, Mailbox, Message};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{AsyncSmtpTransport, AsyncTransport, Tokio1Executor};
use tracing::{info, warn};
use uuid::Uuid;

use crate::shared::errors::AppError;

use super::{EmailDeliveryStatus, EmailProvider, SmtpConfig};

/// Lettre-based SMTP email adapter
pub struct LettreSmtpAdapter {
    #[allow(dead_code)]
    config: SmtpConfig,
    transport: Option<AsyncSmtpTransport<Tokio1Executor>>,
}

impl LettreSmtpAdapter {
    /// Create a new SMTP adapter from config
    pub fn new(config: SmtpConfig) -> Self {
        let transport = Self::create_transport(&config);
        Self { config, transport }
    }

    /// Create adapter from environment variables
    pub fn from_env() -> Self {
        let config = SmtpConfig::from_env();
        Self::new(config)
    }

    /// Create the SMTP transport
    fn create_transport(config: &SmtpConfig) -> Option<AsyncSmtpTransport<Tokio1Executor>> {
        if config.username.is_empty() || config.password.is_empty() {
            warn!("SMTP credentials not configured, email sending disabled");
            return None;
        }

        if config.use_tls {
            if let Ok(builder) = AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&config.host) {
                let creds = Credentials::new(config.username.clone(), config.password.clone());
                return Some(builder.port(config.port).credentials(creds).build());
            }
            warn!("STARTTLS failed, trying direct TLS on port 465");
            if let Ok(builder) = AsyncSmtpTransport::<Tokio1Executor>::relay(&config.host) {
                let creds = Credentials::new(config.username.clone(), config.password.clone());
                return Some(builder.port(465).credentials(creds).build());
            }
            warn!("Direct TLS also failed");
            None
        } else {
            warn!("SMTP running without TLS - use only for local development");
            let creds = Credentials::new(config.username.clone(), config.password.clone());
            Some(
                AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(&config.host)
                    .port(config.port)
                    .credentials(creds)
                    .build(),
            )
        }
    }

    /// Parse email address (supports "Name <email>" and plain "email" formats)
    fn parse_email(email: &str) -> Result<Mailbox, AppError> {
        email.parse().map_err(|_| {
            AppError::Internal(anyhow::anyhow!("Invalid email address: {}", email))
        })
    }

    /// Build and send an email message
    async fn send_message(
        &self,
        to: &str,
        subject: &str,
        message: Message,
    ) -> Result<EmailDeliveryStatus, AppError> {
        let transport = self
            .transport
            .as_ref()
            .ok_or_else(|| AppError::Internal(anyhow::anyhow!("SMTP not configured")))?;

        let tracking_id = Uuid::new_v4().to_string();

        match transport.send(message).await {
            Ok(_response) => {
                info!(
                    to = %to,
                    subject = %subject,
                    tracking_id = %tracking_id,
                    "Email sent successfully via SMTP"
                );
                Ok(EmailDeliveryStatus {
                    message_id: tracking_id,
                    status: "sent".to_string(),
                    recipient: to.to_string(),
                    delivered: true,
                    delivered_at: Some(chrono::Utc::now().to_rfc3339()),
                    error: None,
                })
            }
            Err(e) => {
                warn!(to = %to, error = %e, "Failed to send email via SMTP");
                Ok(EmailDeliveryStatus {
                    message_id: tracking_id,
                    status: "failed".to_string(),
                    recipient: to.to_string(),
                    delivered: false,
                    delivered_at: None,
                    error: Some(e.to_string()),
                })
            }
        }
    }
}

#[async_trait]
impl EmailProvider for LettreSmtpAdapter {
    async fn send_email(
        &self,
        to: &str,
        subject: &str,
        body: &str,
    ) -> Result<EmailDeliveryStatus, AppError> {
        self.send_html_email(to, subject, body, None).await
    }

    async fn send_html_email(
        &self,
        to: &str,
        subject: &str,
        html_body: &str,
        text_body: Option<&str>,
    ) -> Result<EmailDeliveryStatus, AppError> {
        let from_mailbox = Self::parse_email(&self.config.from_email)?;
        let to_mailbox = Self::parse_email(to)?;

        let message = if let Some(text) = text_body {
            // In lettre 0.11, SinglePart::builder().header().body() returns SinglePart directly
            let text_part = lettre::message::SinglePart::builder()
                .header(ContentType::TEXT_PLAIN)
                .body(text.to_string());

            let html_part = lettre::message::SinglePart::builder()
                .header(ContentType::TEXT_HTML)
                .body(html_body.to_string());

            Message::builder()
                .from(from_mailbox)
                .to(to_mailbox)
                .subject(subject)
                .multipart(
                    lettre::message::MultiPart::alternative()
                        .singlepart(text_part)
                        .singlepart(html_part),
                )
                .map_err(|e| AppError::Internal(anyhow::anyhow!("Message build error: {}", e)))?
        } else {
            Message::builder()
                .from(from_mailbox)
                .to(to_mailbox)
                .subject(subject)
                .header(ContentType::TEXT_HTML)
                .body(html_body.to_string())
                .map_err(|e| AppError::Internal(anyhow::anyhow!("Message build error: {}", e)))?
        };

        self.send_message(to, subject, message).await
    }

    async fn send_email_with_reply_to(
        &self,
        to: &str,
        subject: &str,
        body: &str,
        reply_to: &str,
    ) -> Result<EmailDeliveryStatus, AppError> {
        let from_mailbox = Self::parse_email(&self.config.from_email)?;
        let to_mailbox = Self::parse_email(to)?;
        let reply_to_mailbox = Self::parse_email(reply_to)?;

        let message = Message::builder()
            .from(from_mailbox)
            .to(to_mailbox)
            .reply_to(reply_to_mailbox)
            .subject(subject)
            .header(ContentType::TEXT_PLAIN)
            .body(body.to_string())
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Message build error: {}", e)))?;

        self.send_message(to, subject, message).await
    }

    fn is_configured(&self) -> bool {
        self.transport.is_some()
    }
}
