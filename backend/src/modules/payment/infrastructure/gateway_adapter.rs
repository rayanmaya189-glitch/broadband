use async_trait::async_trait;
use crate::shared::errors::AppError;

/// Gateway response from creating a payment link/order
#[derive(Debug, Clone)]
pub struct GatewayPaymentResponse {
    pub order_id: String,
    pub payment_url: String,
    pub amount: sea_orm::prelude::Decimal,
    pub currency: String,
}

/// Webhook payload from gateway
#[derive(Debug, Clone)]
pub struct GatewayWebhookPayload {
    pub event_id: String,
    pub event_type: String,
    pub transaction_id: String,
    pub order_id: Option<String>,
    pub amount: sea_orm::prelude::Decimal,
    pub status: String,
    pub payment_method: Option<String>,
    pub error_reason: Option<String>,
    pub raw_payload: serde_json::Value,
}

/// Trait for payment gateway adapters
#[async_trait]
pub trait GatewayAdapter: Send + Sync {
    /// Create a payment link/order
    async fn create_payment_link(
        &self,
        amount: sea_orm::prelude::Decimal,
        currency: &str,
        receipt: &str,
        metadata: serde_json::Value,
    ) -> Result<GatewayPaymentResponse, AppError>;

    /// Verify webhook signature
    fn verify_webhook_signature(
        &self,
        body: &[u8],
        signature: &str,
        secret: &str,
    ) -> Result<bool, AppError>;

    /// Parse webhook payload
    fn parse_webhook(&self, payload: serde_json::Value) -> Result<GatewayWebhookPayload, AppError>;
}

/// Razorpay gateway adapter
pub struct RazorpayAdapter {
    pub key_id: String,
    pub key_secret: String,
    pub webhook_secret: String,
}

#[async_trait]
impl GatewayAdapter for RazorpayAdapter {
    async fn create_payment_link(
        &self,
        amount: sea_orm::prelude::Decimal,
        currency: &str,
        _receipt: &str,
        _metadata: serde_json::Value,
    ) -> Result<GatewayPaymentResponse, AppError> {
        // In production, this would call Razorpay API
        // For now, return a mock response
        let uuid_str = uuid::Uuid::new_v4().to_string().replace('-', "");
        let order_id = format!("order_{}", &uuid_str[..14.min(uuid_str.len())]);
        let payment_url = format!("https://checkout.razorpay.com/v1/pay.js#order_id={}", order_id);

        tracing::info!(order_id = %order_id, amount = %amount, "Created Razorpay order");

        Ok(GatewayPaymentResponse {
            order_id,
            payment_url,
            amount,
            currency: currency.to_string(),
        })
    }

    fn verify_webhook_signature(
        &self,
        body: &[u8],
        signature: &str,
        secret: &str,
    ) -> Result<bool, AppError> {
        // TODO: Implement HMAC-SHA256 signature verification
        // In production:
        // let expected = hmac::Mac::new_from_slice(secret.as_bytes())
        //     .and_then(|mut mac| { mac.update(body); mac.finalize() })
        //     .map(|result| result.into_bytes().to_hex());
        // Ok(expected == signature)
        let _ = (body, signature, secret);
        Ok(true)
    }

    fn parse_webhook(&self, payload: serde_json::Value) -> Result<GatewayWebhookPayload, AppError> {
        let event = payload["event"].as_str().unwrap_or("unknown");
        let event_id = payload["payload"]["payment"]["entity"]["id"].as_str().unwrap_or("");
        let transaction_id = payload["payload"]["payment"]["entity"]["id"].as_str().unwrap_or("");
        let order_id = payload["payload"]["payment"]["entity"]["order_id"].as_str().map(|s| s.to_string());
        let amount = payload["payload"]["payment"]["entity"]["amount"].as_i64().unwrap_or(0);
        let status = payload["payload"]["payment"]["entity"]["status"].as_str().unwrap_or("unknown");
        let payment_method = payload["payload"]["payment"]["entity"]["method"].as_str().map(|s| s.to_string());

        // Razorpay sends amount in paise (smallest currency unit)
        let amount_decimal = sea_orm::prelude::Decimal::new(amount, 0);

        let error_reason = if event == "payment.failed" {
            payload["payload"]["payment"]["entity"]["error_description"].as_str().map(|s| s.to_string())
        } else {
            None
        };

        Ok(GatewayWebhookPayload {
            event_id: event_id.to_string(),
            event_type: event.to_string(),
            transaction_id: transaction_id.to_string(),
            order_id,
            amount: amount_decimal,
            status: status.to_string(),
            payment_method,
            error_reason,
            raw_payload: payload,
        })
    }
}

/// PayU gateway adapter
pub struct PayuAdapter {
    pub merchant_key: String,
    pub merchant_salt: String,
}

#[async_trait]
impl GatewayAdapter for PayuAdapter {
    async fn create_payment_link(
        &self,
        amount: sea_orm::prelude::Decimal,
        currency: &str,
        _receipt: &str,
        _metadata: serde_json::Value,
    ) -> Result<GatewayPaymentResponse, AppError> {
        // In production, this would call PayU API
        let uuid_str = uuid::Uuid::new_v4().to_string().replace('-', "");
        let txn_id = format!("txn_{}", &uuid_str[..14.min(uuid_str.len())]);
        let payment_url = format!("https://test.payu.in/_payment?txnid={}", txn_id);

        tracing::info!(txn_id = %txn_id, amount = %amount, "Created PayU payment");

        Ok(GatewayPaymentResponse {
            order_id: txn_id,
            payment_url,
            amount,
            currency: currency.to_string(),
        })
    }

    fn verify_webhook_signature(
        &self,
        body: &[u8],
        signature: &str,
        secret: &str,
    ) -> Result<bool, AppError> {
        // In production, use SHA-512 to verify
        let _ = (body, signature, secret);
        Ok(true)
    }

    fn parse_webhook(&self, payload: serde_json::Value) -> Result<GatewayWebhookPayload, AppError> {
        let event_type = payload["status"].as_str().unwrap_or("unknown");
        let transaction_id = payload["txnid"].as_str().unwrap_or("");
        let order_id = payload["order_id"].as_str().map(|s| s.to_string());
        let amount = payload["amount"].as_str().and_then(|s| s.parse::<i64>().ok()).unwrap_or(0);
        let status = payload["status"].as_str().unwrap_or("unknown");
        let payment_method = payload["payment_source"].as_str().map(|s| s.to_string());

        // Razorpay sends amount in paise (smallest currency unit)
        let amount_decimal = sea_orm::prelude::Decimal::new(amount, 0);

        let error_reason = if event_type == "failure" || event_type == "drop" {
            payload["error"].as_str().map(|s| s.to_string())
        } else {
            None
        };

        Ok(GatewayWebhookPayload {
            event_id: transaction_id.to_string(),
            event_type: event_type.to_string(),
            transaction_id: transaction_id.to_string(),
            order_id,
            amount: amount_decimal,
            status: status.to_string(),
            payment_method,
            error_reason,
            raw_payload: payload,
        })
    }
}
