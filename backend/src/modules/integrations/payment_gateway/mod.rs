//! Payment gateway integration adapter.
//!
//! Adapter for payment gateway integration.

use serde::{Deserialize, Serialize};

/// Payment gateway configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentGatewayConfig {
    pub api_key: String,
    pub api_secret: String,
    pub merchant_id: String,
    pub gateway_url: String,
    pub sandbox: bool,
}

/// Payment gateway adapter trait.
#[async_trait::async_trait]
pub trait PaymentGatewayAdapter: Send + Sync {
    /// Create a payment link.
    async fn create_payment_link(
        &self,
        amount: f64,
        currency: &str,
        order_id: &str,
        description: &str,
    ) -> Result<PaymentLink, PaymentGatewayError>;

    /// Verify a payment.
    async fn verify_payment(
        &self,
        transaction_id: &str,
    ) -> Result<PaymentStatus, PaymentGatewayError>;

    /// Process a refund.
    async fn process_refund(
        &self,
        transaction_id: &str,
        amount: f64,
        reason: &str,
    ) -> Result<RefundResult, PaymentGatewayError>;
}

/// Payment link response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentLink {
    pub link_url: String,
    pub transaction_id: String,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// Payment status.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentStatus {
    pub transaction_id: String,
    pub status: String,
    pub amount: f64,
    pub currency: String,
    pub paid_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// Refund result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundResult {
    pub refund_id: String,
    pub transaction_id: String,
    pub amount: f64,
    pub status: String,
}

/// Payment gateway errors.
#[derive(Debug, Clone)]
pub enum PaymentGatewayError {
    ApiError(String),
    InvalidRequest(String),
    PaymentFailed(String),
    NetworkError(String),
}

impl std::fmt::Display for PaymentGatewayError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PaymentGatewayError::ApiError(msg) => write!(f, "API error: {}", msg),
            PaymentGatewayError::InvalidRequest(msg) => write!(f, "Invalid request: {}", msg),
            PaymentGatewayError::PaymentFailed(msg) => write!(f, "Payment failed: {}", msg),
            PaymentGatewayError::NetworkError(msg) => write!(f, "Network error: {}", msg),
        }
    }
}

impl std::error::Error for PaymentGatewayError {}
