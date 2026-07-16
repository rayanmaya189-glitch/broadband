//! Email Provider Integration Module
//!
//! Supports SMTP email delivery via lettre crate:
//! - Transactional emails (invoices, alerts, notifications)
//! - HTML and plain text support
//! - TLS/STARTTLS encryption
//! - Connection pooling via lettre's AsyncSmtpTransport

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::shared::errors::AppError;

pub mod smtp_adapter;

pub use smtp_adapter::LettreSmtpAdapter;

/// Email delivery status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailDeliveryStatus {
    pub message_id: String,
    pub status: String,
    pub recipient: String,
    pub delivered: bool,
    pub delivered_at: Option<String>,
    pub error: Option<String>,
}

/// SMTP configuration loaded from environment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmtpConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub from_email: String,
    pub from_name: String,
    pub use_tls: bool,
}

impl SmtpConfig {
    /// Load SMTP config from environment variables
    pub fn from_env() -> Self {
        Self {
            host: std::env::var("SMTP_HOST").unwrap_or_else(|_| "smtp.gmail.com".to_string()),
            port: std::env::var("SMTP_PORT")
                .unwrap_or_else(|_| "587".to_string())
                .parse()
                .unwrap_or(587),
            username: std::env::var("SMTP_USERNAME").unwrap_or_default(),
            password: std::env::var("SMTP_PASSWORD").unwrap_or_default(),
            from_email: std::env::var("SMTP_FROM_EMAIL")
                .unwrap_or_else(|_| "noreply@aeroxebroadband.com".to_string()),
            from_name: std::env::var("SMTP_FROM_NAME")
                .unwrap_or_else(|_| "AeroXe Broadband".to_string()),
            use_tls: std::env::var("SMTP_USE_TLS")
                .unwrap_or_else(|_| "true".to_string())
                .parse()
                .unwrap_or(true),
        }
    }
}

/// Trait for email providers
#[async_trait]
pub trait EmailProvider: Send + Sync {
    /// Send a plain text email
    async fn send_email(
        &self,
        to: &str,
        subject: &str,
        body: &str,
    ) -> Result<EmailDeliveryStatus, AppError>;

    /// Send an HTML email
    async fn send_html_email(
        &self,
        to: &str,
        subject: &str,
        html_body: &str,
        text_body: Option<&str>,
    ) -> Result<EmailDeliveryStatus, AppError>;

    /// Send an email with attachments (future use)
    async fn send_email_with_reply_to(
        &self,
        to: &str,
        subject: &str,
        body: &str,
        reply_to: &str,
    ) -> Result<EmailDeliveryStatus, AppError>;

    /// Check if the provider is configured and ready
    fn is_configured(&self) -> bool;
}
