use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct GatewayConfig {
    pub id: i64,
    pub gateway_id: String,
    pub name: String,
    pub is_primary: bool,
    pub is_active: bool,
    pub supported_methods: Vec<String>,
    pub currency: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow)]
pub struct PaymentTransaction {
    pub id: i64,
    pub gateway_id: String,
    pub invoice_id: Option<i64>,
    pub customer_id: Option<i64>,
    pub amount: Decimal,
    pub currency: String,
    pub payment_method: String,
    pub gateway_transaction_id: Option<String>,
    pub status: String,
    pub idempotency_key: Option<String>,
    pub failure_reason: Option<String>,
    pub webhook_received_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow)]
pub struct PaymentLink {
    pub id: i64,
    pub transaction_id: i64,
    pub payment_url: String,
    pub short_url: Option<String>,
    pub expires_at: DateTime<Utc>,
    pub is_used: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow)]
pub struct WebhookLog {
    pub id: i64,
    pub gateway_id: String,
    pub event_type: String,
    pub payload: serde_json::Value,
    pub processed: bool,
    pub error_message: Option<String>,
    pub created_at: DateTime<Utc>,
}
