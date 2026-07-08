use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateGatewayConfigRequest {
    pub gateway_id: String,
    pub name: String,
    pub is_primary: Option<bool>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreatePaymentLinkRequest {
    pub invoice_id: i64,
    pub amount: rust_decimal::Decimal,
    pub payment_method: String,
}
