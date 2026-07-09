use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct GatewayConfigResponse {
    pub id: i64,
    pub gateway_id: String,
    pub name: String,
    pub is_primary: bool,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct PaymentTransactionResponse {
    pub id: i64,
    pub gateway_id: String,
    pub invoice_id: Option<i64>,
    pub customer_id: Option<i64>,
    pub amount: Decimal,
    pub currency: String,
    pub payment_method: String,
    pub gateway_transaction_id: Option<String>,
    pub status: String,
    pub failure_reason: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PaymentLinkResponse {
    pub payment_url: String,
    pub transaction_id: i64,
    pub expires_in: i64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TransactionListResponse {
    pub transactions: Vec<PaymentTransactionResponse>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct WebhookProcessResponse {
    pub status: String,
    pub message: String,
    pub transaction_id: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct MessageResponse {
    pub message: String,
}
