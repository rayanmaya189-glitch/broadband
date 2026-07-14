//! SMS provider adapter.
//!
//! Trait and types for SMS provider integration.

use serde::{Deserialize, Serialize};

/// SMS provider adapter trait.
#[async_trait::async_trait]
pub trait SmsProviderAdapter: Send + Sync {
    /// Send an SMS message.
    async fn send_sms(&self, __message: &SmsMessage) -> Result<SmsResult, SmsError>;

    /// Check delivery status.
    async fn check_status(&self, message_id: &str) -> Result<SmsStatus, SmsError>;
}

/// SMS message to send.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmsMessage {
    pub to: String,
    pub from: String,
    pub body: String,
    pub priority: SmsPriority,
}

/// SMS priority levels.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SmsPriority {
    Low,
    Normal,
    High,
}

/// SMS send result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmsResult {
    pub message_id: String,
    pub status: String,
    pub sent_at: chrono::DateTime<chrono::Utc>,
}

/// SMS status.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmsStatus {
    pub message_id: String,
    pub status: String,
    pub delivered_at: Option<chrono::DateTime<chrono::Utc>>,
    pub error_code: Option<String>,
}

/// SMS errors.
#[derive(Debug, Clone)]
pub enum SmsError {
    ProviderUnavailable(String),
    InvalidNumber(String),
    MessageTooLong(usize),
    RateLimited,
    NetworkError(String),
}

impl std::fmt::Display for SmsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SmsError::ProviderUnavailable(msg) => write!(f, "Provider unavailable: {}", msg),
            SmsError::InvalidNumber(msg) => write!(f, "Invalid number: {}", msg),
            SmsError::MessageTooLong(len) => write!(f, "Message too long: {} characters", len),
            SmsError::RateLimited => write!(f, "Rate limited"),
            SmsError::NetworkError(msg) => write!(f, "Network error: {}", msg),
        }
    }
}

impl std::error::Error for SmsError {}

/// Demo SMS provider adapter for testing.
pub struct DemoSmsProviderAdapter;

#[async_trait::async_trait]
impl SmsProviderAdapter for DemoSmsProviderAdapter {
    async fn send_sms(&self, __message: &SmsMessage) -> Result<SmsResult, SmsError> {
        // Demo implementation - always succeeds
        Ok(SmsResult {
            message_id: format!("SMS-{}", uuid::Uuid::new_v4()),
            status: "sent".to_string(),
            sent_at: chrono::Utc::now(),
        })
    }

    async fn check_status(&self, message_id: &str) -> Result<SmsStatus, SmsError> {
        // Demo implementation - always delivered
        Ok(SmsStatus {
            message_id: message_id.to_string(),
            status: "delivered".to_string(),
            delivered_at: Some(chrono::Utc::now()),
            error_code: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_demo_sms_provider() {
        let adapter = DemoSmsProviderAdapter;
        let message = SmsMessage {
            to: "+1234567890".to_string(),
            from: "AeroXe".to_string(),
            body: "Test message".to_string(),
            priority: SmsPriority::Normal,
        };

        let result = adapter.send_sms(&message).await.unwrap();
        assert!(!result.message_id.is_empty());
    }
}
