use utoipa::ToSchema;
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateGatewayConfigRequest {
    pub gateway_id: String,
    pub name: String,
    pub is_primary: Option<bool>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateGatewayRequest {
    pub name: Option<String>,
    pub is_primary: Option<bool>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreatePaymentLinkRequest {
    pub invoice_id: i64,
    pub customer_id: Option<i64>,
    pub amount: rust_decimal::Decimal,
    pub payment_method: String,
    pub gateway_id: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct WebhookPayload {
    pub event_type: String,
    pub gateway_id: String,
    pub payload: serde_json::Value,
    pub signature: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct TransactionQuery {
    pub gateway_id: Option<String>,
    pub status: Option<String>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct RetryPaymentRequest {
    pub transaction_id: i64,
    pub gateway_id: Option<String>,
}

// Type aliases for backward compatibility
pub type CreateGatewayRequest = CreateGatewayConfigRequest;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateTransactionRequest {
    pub gateway_id: String,
    pub customer_id: i64,
    pub invoice_id: Option<i64>,
    pub amount: rust_decimal::Decimal,
    pub currency: Option<String>,
    pub payment_method: String,
    pub description: Option<String>,
}
