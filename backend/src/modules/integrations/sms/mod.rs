//! SMS Provider Integration Module
//!
//! Supports multiple SMS providers for India:
//! - MSG91: Popular Indian transactional SMS & OTP provider
//! - Twilio: Global provider with excellent India support
//!
//! Features:
//! - OTP generation and verification
//! - Transactional SMS (invoices, alerts, notifications)
//! - DLT (Distributed Ledger Technology) compliance for TRAI
//! - Retry logic with exponential backoff

use async_trait::async_trait;
use crate::shared::errors::AppError;

pub mod msg91;
pub mod twilio;

pub use msg91::Msg91Adapter;
pub use twilio::TwilioSmsAdapter;

/// SMS delivery status
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SmsDeliveryStatus {
    pub request_id: String,
    pub status: String,
    pub phone: String,
    pub delivered: bool,
    pub delivered_at: Option<String>,
    pub error: Option<String>,
}

/// Trait for SMS providers
#[async_trait]
pub trait SmsProvider: Send + Sync {
    /// Send OTP to a phone number
    async fn send_otp(
        &self,
        phone: &str,
        template_id: Option<&str>,
    ) -> Result<String, AppError>;

    /// Verify OTP
    async fn verify_otp(&self, phone: &str, otp: &str) -> Result<bool, AppError>;

    /// Send transactional SMS
    async fn send_sms(
        &self,
        phone: &str,
        message: &str,
        template_id: Option<&str>,
    ) -> Result<String, AppError>;

    /// Send bulk SMS
    async fn send_bulk_sms(
        &self,
        phones: &[String],
        message: &str,
        template_id: Option<&str>,
    ) -> Result<String, AppError>;

    /// Check delivery status
    async fn check_delivery(&self, request_id: &str) -> Result<SmsDeliveryStatus, AppError>;
}
