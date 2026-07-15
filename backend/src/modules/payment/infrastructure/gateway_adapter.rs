use async_trait::async_trait;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use tracing::{debug, info, warn};

use crate::shared::errors::AppError;

type HmacSha256 = Hmac<Sha256>;

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

// ============================================================================
// Razorpay Adapter
// ============================================================================

/// Razorpay gateway adapter with full API integration
pub struct RazorpayAdapter {
    pub key_id: String,
    pub key_secret: String,
    pub webhook_secret: String,
}

impl RazorpayAdapter {
    /// Create a new Razorpay adapter from environment variables
    pub fn from_env() -> Self {
        Self {
            key_id: std::env::var("RAZORPAY_KEY_ID").unwrap_or_default(),
            key_secret: std::env::var("RAZORPAY_KEY_SECRET").unwrap_or_default(),
            webhook_secret: std::env::var("RAZORPAY_WEBHOOK_SECRET").unwrap_or_default(),
        }
    }
}

#[async_trait]
impl GatewayAdapter for RazorpayAdapter {
    async fn create_payment_link(
        &self,
        amount: sea_orm::prelude::Decimal,
        currency: &str,
        receipt: &str,
        metadata: serde_json::Value,
    ) -> Result<GatewayPaymentResponse, AppError> {
        // Razorpay expects amount in paise (smallest currency unit)
        let amount_paise = (amount * sea_orm::prelude::Decimal::new(100, 0))
            .to_string()
            .parse::<i64>()
            .unwrap_or(0);

        let body = serde_json::json!({
            "amount": amount_paise,
            "currency": currency,
            "receipt": receipt,
            "metadata": metadata,
        });

        let client = reqwest::Client::new();
        let response = client
            .post("https://api.razorpay.com/v1/orders")
            .basic_auth(&self.key_id, Some(&self.key_secret))
            .json(&body)
            .send()
            .await
            .map_err(|e| AppError::External(format!("Razorpay API request failed: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_body = response.text().await.unwrap_or_default();
            warn!(status = %status, body = %error_body, "Razorpay order creation failed");
            return Err(AppError::External(format!(
                "Razorpay API error ({}): {}",
                status, error_body
            )));
        }

        let order: serde_json::Value = response
            .json()
            .await
            .map_err(|e| AppError::External(format!("Failed to parse Razorpay response: {}", e)))?;

        let order_id = order["id"].as_str().unwrap_or("").to_string();
        let payment_url = format!(
            "https://checkout.razorpay.com/v1/pay.js#order_id={}",
            order_id
        );

        info!(order_id = %order_id, amount = %amount, "Created Razorpay order");

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
        let mut mac = HmacSha256::new_from_slice(secret.as_bytes())
            .map_err(|e| AppError::Internal(anyhow::anyhow!("HMAC key error: {}", e)))?;
        mac.update(body);
        let expected = hex::encode(mac.finalize().into_bytes());
        Ok(expected == signature)
    }

    fn parse_webhook(&self, payload: serde_json::Value) -> Result<GatewayWebhookPayload, AppError> {
        let event = payload["event"].as_str().unwrap_or("unknown");
        let entity = &payload["payload"]["payment"]["entity"];

        let event_id = entity["id"].as_str().unwrap_or("").to_string();
        let transaction_id = entity["id"].as_str().unwrap_or("").to_string();
        let order_id = entity["order_id"].as_str().map(|s| s.to_string());
        let amount_paise = entity["amount"].as_i64().unwrap_or(0);
        let status = entity["status"].as_str().unwrap_or("unknown");
        let payment_method = entity["method"].as_str().map(|s| s.to_string());

        // Razorpay sends amount in paise (smallest currency unit)
        let amount_decimal = sea_orm::prelude::Decimal::new(amount_paise, 0);

        let error_reason = if event == "payment.failed" {
            entity["error_description"].as_str().map(|s| s.to_string())
        } else {
            None
        };

        debug!(event = %event, transaction_id = %transaction_id, "Parsed Razorpay webhook");

        Ok(GatewayWebhookPayload {
            event_id,
            event_type: event.to_string(),
            transaction_id,
            order_id,
            amount: amount_decimal,
            status: status.to_string(),
            payment_method,
            error_reason,
            raw_payload: payload,
        })
    }
}

// ============================================================================
// PayU Adapter
// ============================================================================

/// PayU gateway adapter with full API integration
pub struct PayuAdapter {
    pub merchant_key: String,
    pub merchant_salt: String,
    pub api_endpoint: String,
}

impl PayuAdapter {
    /// Create a new PayU adapter from environment variables
    pub fn from_env() -> Self {
        let is_production = std::env::var("PAYU_ENV").unwrap_or_default() == "production";
        Self {
            merchant_key: std::env::var("PAYU_MERCHANT_KEY").unwrap_or_default(),
            merchant_salt: std::env::var("PAYU_MERCHANT_SALT").unwrap_or_default(),
            api_endpoint: if is_production {
                "https://secure.payu.in/_payment".to_string()
            } else {
                "https://test.payu.in/_payment".to_string()
            },
        }
    }
}

#[async_trait]
impl GatewayAdapter for PayuAdapter {
    async fn create_payment_link(
        &self,
        amount: sea_orm::prelude::Decimal,
        currency: &str,
        receipt: &str,
        metadata: serde_json::Value,
    ) -> Result<GatewayPaymentResponse, AppError> {
        let uuid_str = uuid::Uuid::new_v4().to_string().replace('-', "");
        let txn_id = format!("txn_{}", &uuid_str[..14.min(uuid_str.len())]);

        // PayU uses SHA-512 hash for hash generation
        let hash_string = format!(
            "{}|{}|{}|{}|{}|||||||||||{}",
            self.merchant_key, txn_id, amount, receipt, currency, self.merchant_salt
        );
        let hash = {
            use sha2::{Digest, Sha512};
            let mut hasher = Sha512::new();
            hasher.update(hash_string.as_bytes());
            hex::encode(hasher.finalize())
        };

        let body = serde_json::json!({
            "key": self.merchant_key,
            "txnid": txn_id,
            "amount": amount.to_string(),
            "productinfo": receipt,
            "firstname": metadata["customer_name"].as_str().unwrap_or("Customer"),
            "email": metadata["customer_email"].as_str().unwrap_or(""),
            "phone": metadata["customer_phone"].as_str().unwrap_or(""),
            "surl": metadata["success_url"].as_str().unwrap_or(""),
            "furl": metadata["failure_url"].as_str().unwrap_or(""),
            "hash": hash,
        });

        let client = reqwest::Client::new();
        let response = client
            .post("https://secure.payu.in/merchant/postcollector.php")
            .json(&body)
            .send()
            .await
            .map_err(|e| AppError::External(format!("PayU API request failed: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_body = response.text().await.unwrap_or_default();
            warn!(status = %status, body = %error_body, "PayU payment creation failed");
            return Err(AppError::External(format!(
                "PayU API error ({}): {}",
                status, error_body
            )));
        }

        // PayU returns form data for redirect, we construct the payment URL
        let payment_url = format!(
            "{}?key={}&txnid={}&amount={}&productinfo={}&firstname={}&email={}&phone={}&hash={}",
            self.api_endpoint,
            self.merchant_key,
            txn_id,
            amount,
            receipt,
            metadata["customer_name"].as_str().unwrap_or("Customer"),
            metadata["customer_email"].as_str().unwrap_or(""),
            metadata["customer_phone"].as_str().unwrap_or(""),
            hash
        );

        info!(txn_id = %txn_id, amount = %amount, "Created PayU payment");

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
        // PayU uses SHA-512: hash = SHA512(salt|status|||||||key|txnid|amount|productinfo|firstname|email|phone)
        // For webhook verification, PayU sends the hash and we verify it against the expected format
        use sha2::{Digest, Sha512};
        let mut hasher = Sha512::new();
        hasher.update(secret.as_bytes()); // salt
        hasher.update(body.as_ref()); // concatenated webhook parameters
        let expected = hex::encode(hasher.finalize());
        Ok(expected == signature)
    }

    fn parse_webhook(&self, payload: serde_json::Value) -> Result<GatewayWebhookPayload, AppError> {
        let status = payload["status"].as_str().unwrap_or("unknown");
        let transaction_id = payload["txnid"].as_str().unwrap_or("").to_string();
        let order_id = payload["order_id"].as_str().map(|s| s.to_string());
        let amount_str = payload["amount"].as_str().unwrap_or("0");
        let amount = amount_str.parse::<i64>().unwrap_or(0);
        let payment_method = payload["payment_source"].as_str().map(|s| s.to_string());

        let amount_decimal = sea_orm::prelude::Decimal::new(amount, 0);

        let error_reason = if status == "failure" || status == "drop" {
            payload["error"].as_str().map(|s| s.to_string())
        } else {
            None
        };

        let event_type = match status {
            "success" => "payment.success",
            "failure" => "payment.failed",
            "drop" => "payment.dropped",
            _ => "payment.unknown",
        };

        debug!(status = %status, transaction_id = %transaction_id, "Parsed PayU webhook");

        Ok(GatewayWebhookPayload {
            event_id: transaction_id.clone(),
            event_type: event_type.to_string(),
            transaction_id,
            order_id,
            amount: amount_decimal,
            status: status.to_string(),
            payment_method,
            error_reason,
            raw_payload: payload,
        })
    }
}
