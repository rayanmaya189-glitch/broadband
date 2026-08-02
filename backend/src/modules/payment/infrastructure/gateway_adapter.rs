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

/// Gateway response from issuing a refund for a captured payment
#[derive(Debug, Clone)]
pub struct GatewayRefundResponse {
    pub refund_id: String,
    /// Gateway-reported refund status: `processed`, `pending`, or `failed`
    pub status: String,
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

    /// Issue a refund against an already-captured gateway transaction.
    /// `payment_id` is the gateway transaction id recorded on the payment.
    async fn refund_payment(
        &self,
        payment_id: &str,
        amount: sea_orm::prelude::Decimal,
        currency: &str,
        reason: Option<&str>,
    ) -> Result<GatewayRefundResponse, AppError>;
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

    async fn refund_payment(
        &self,
        payment_id: &str,
        amount: sea_orm::prelude::Decimal,
        currency: &str,
        reason: Option<&str>,
    ) -> Result<GatewayRefundResponse, AppError> {
        // Razorpay expects amount in paise (smallest currency unit)
        let amount_paise = (amount * sea_orm::prelude::Decimal::new(100, 0))
            .to_string()
            .parse::<i64>()
            .unwrap_or(0);

        let mut notes = serde_json::Map::new();
        if let Some(r) = reason {
            notes.insert(
                "reason".to_string(),
                serde_json::Value::String(r.to_string()),
            );
        }

        let body = serde_json::json!({
            "amount": amount_paise,
            "notes": notes,
        });

        let client = reqwest::Client::new();
        let response = client
            .post(format!(
                "https://api.razorpay.com/v1/payments/{}/refund",
                payment_id
            ))
            .basic_auth(&self.key_id, Some(&self.key_secret))
            .json(&body)
            .send()
            .await
            .map_err(|e| AppError::External(format!("Razorpay refund request failed: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_body = response.text().await.unwrap_or_default();
            warn!(status = %status, body = %error_body, "Razorpay refund failed");
            return Err(AppError::External(format!(
                "Razorpay refund error ({}): {}",
                status, error_body
            )));
        }

        let refund: serde_json::Value = response.json().await.map_err(|e| {
            AppError::External(format!("Failed to parse Razorpay refund response: {}", e))
        })?;

        let refund_id = refund["id"].as_str().unwrap_or("").to_string();
        let rz_status = refund["status"].as_str().unwrap_or("pending");
        let status = match rz_status {
            "processed" => "processed",
            "failed" => "failed",
            _ => "pending",
        };
        let amount_paise_resp = refund["amount"].as_i64().unwrap_or(0);

        info!(refund_id = %refund_id, status = %status, amount = %amount, "Razorpay refund issued");

        Ok(GatewayRefundResponse {
            refund_id,
            status: status.to_string(),
            amount: sea_orm::prelude::Decimal::new(amount_paise_resp, 2),
            currency: refund["currency"].as_str().unwrap_or(currency).to_string(),
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
        let uuid_str = crate::shared::utils::uuid_v7::new_v7_compact();
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
        hasher.update(body); // concatenated webhook parameters
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

    async fn refund_payment(
        &self,
        payment_id: &str,
        amount: sea_orm::prelude::Decimal,
        currency: &str,
        _reason: Option<&str>,
    ) -> Result<GatewayRefundResponse, AppError> {
        // PayU v3 refund API (cancel_refund_transaction) expects amount in paise.
        let amount_paise = (amount * sea_orm::prelude::Decimal::new(100, 0))
            .to_string()
            .parse::<i64>()
            .unwrap_or(0);

        let command = "cancel_refund_transaction";
        let mut vars = vec![String::new(); 20];
        vars[0] = payment_id.to_string();
        vars[1] = amount_paise.to_string();
        vars[2] = currency.to_string();

        // Hash: SHA512(key|command|var1|...|var20|salt)
        let hash_string = format!(
            "{}|{}|{}|{}",
            self.merchant_key,
            command,
            vars.join("|"),
            self.merchant_salt
        );
        let hash = {
            use sha2::{Digest, Sha512};
            let mut hasher = Sha512::new();
            hasher.update(hash_string.as_bytes());
            hex::encode(hasher.finalize())
        };

        let refund_url = if self.api_endpoint.contains("test.payu.in") {
            "https://test.payu.in/api/v3/refund"
        } else {
            "https://payu.in/api/v3/refund"
        };

        let body = serde_json::json!({
            "key": self.merchant_key,
            "command": command,
            "var1": vars[0],
            "var2": vars[1],
            "var3": vars[2],
            "hash": hash,
        });

        let client = reqwest::Client::new();
        let response = client
            .post(refund_url)
            .json(&body)
            .send()
            .await
            .map_err(|e| AppError::External(format!("PayU refund request failed: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_body = response.text().await.unwrap_or_default();
            warn!(status = %status, body = %error_body, "PayU refund failed");
            return Err(AppError::External(format!(
                "PayU refund error ({}): {}",
                status, error_body
            )));
        }

        let parsed: serde_json::Value = response.json().await.map_err(|e| {
            AppError::External(format!("Failed to parse PayU refund response: {}", e))
        })?;

        let payu_status = parsed["status"].as_i64().unwrap_or(0);
        let refund_id = parsed["refund_id"].as_str().unwrap_or("").to_string();
        let msg = parsed["msg"].as_str().unwrap_or("").to_string();
        let status = if payu_status == 1 {
            "processed"
        } else {
            "failed"
        };

        info!(refund_id = %refund_id, status = %status, msg = %msg, "PayU refund issued");

        Ok(GatewayRefundResponse {
            refund_id,
            status: status.to_string(),
            amount,
            currency: currency.to_string(),
        })
    }
}

// ============================================================================
// Stripe Adapter
// ============================================================================

/// Stripe gateway adapter with full API integration
pub struct StripeAdapter {
    pub secret_key: String,
    pub webhook_signing_secret: String,
    pub api_endpoint: String,
}

impl StripeAdapter {
    pub fn from_env() -> Self {
        Self {
            secret_key: std::env::var("STRIPE_SECRET_KEY").unwrap_or_default(),
            webhook_signing_secret: std::env::var("STRIPE_WEBHOOK_SECRET").unwrap_or_default(),
            // Same API endpoint; test vs live mode is chosen via key prefix.
            api_endpoint: "https://api.stripe.com/v1".to_string(),
        }
    }

    pub fn is_configured(&self) -> bool {
        !self.secret_key.is_empty()
    }
}

#[async_trait]
impl GatewayAdapter for StripeAdapter {
    async fn create_payment_link(
        &self,
        amount: sea_orm::prelude::Decimal,
        currency: &str,
        receipt: &str,
        metadata: serde_json::Value,
    ) -> Result<GatewayPaymentResponse, AppError> {
        // Stripe expects amount in smallest currency unit (paise for INR)
        let amount_paise = (amount * sea_orm::prelude::Decimal::new(100, 0))
            .to_string()
            .parse::<i64>()
            .unwrap_or(0);

        // Create a PaymentIntent via Stripe API
        let client = reqwest::Client::new();
        let response = client
            .post(format!("{}/payment_intents", self.api_endpoint))
            .bearer_auth(&self.secret_key)
            .form(&[
                ("amount", amount_paise.to_string()),
                ("currency", currency.to_lowercase()),
                (
                    "receipt_email",
                    metadata["customer_email"]
                        .as_str()
                        .unwrap_or("")
                        .to_string(),
                ),
                (
                    "description",
                    format!(
                        "{}: {}",
                        receipt,
                        metadata["customer_name"].as_str().unwrap_or("Customer")
                    ),
                ),
                (
                    "metadata[customer_id]",
                    metadata["customer_id"].as_str().unwrap_or("").to_string(),
                ),
                ("metadata[receipt]", receipt.to_string()),
            ])
            .send()
            .await
            .map_err(|e| AppError::External(format!("Stripe API request failed: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_body = response.text().await.unwrap_or_default();
            warn!(status = %status, body = %error_body, "Stripe PaymentIntent creation failed");
            return Err(AppError::External(format!(
                "Stripe API error ({}): {}",
                status, error_body
            )));
        }

        let intent: serde_json::Value = response
            .json()
            .await
            .map_err(|e| AppError::External(format!("Failed to parse Stripe response: {}", e)))?;

        let payment_intent_id = intent["id"].as_str().unwrap_or("").to_string();
        let client_secret = intent["client_secret"].as_str().unwrap_or("").to_string();
        let payment_url = format!(
            "https://checkout.stripe.com/pay/{}#payment_intent_client_secret={}",
            payment_intent_id, client_secret
        );

        info!(payment_intent_id = %payment_intent_id, amount = %amount, "Created Stripe PaymentIntent");

        Ok(GatewayPaymentResponse {
            order_id: payment_intent_id,
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
        // Stripe webhook verification uses HMAC-SHA256 with timestamp
        // Signature format: t=timestamp,v1=signature,v1=signature
        // We compute HMAC-SHA256(secret, "timestamp.body") and compare v1 signatures

        let parts: Vec<&str> = signature.split(',').collect();
        let mut timestamp = "";
        let mut expected_sigs: Vec<String> = Vec::new();
        for part in &parts {
            if let Some(t) = part.strip_prefix("t=") {
                timestamp = t;
            } else if let Some(v) = part.strip_prefix("v1=") {
                expected_sigs.push(v.to_string());
            }
        }

        if timestamp.is_empty() || expected_sigs.is_empty() {
            return Ok(false);
        }

        let sign_payload = format!("{}.{}", timestamp, std::str::from_utf8(body).unwrap_or(""));
        let mut mac = HmacSha256::new_from_slice(secret.as_bytes())
            .map_err(|e| AppError::Internal(anyhow::anyhow!("HMAC key error: {}", e)))?;
        mac.update(sign_payload.as_bytes());
        let computed = hex::encode(mac.finalize().into_bytes());

        Ok(expected_sigs.iter().any(|s| s == &computed))
    }

    fn parse_webhook(&self, payload: serde_json::Value) -> Result<GatewayWebhookPayload, AppError> {
        let event_type = payload["type"].as_str().unwrap_or("unknown");
        let data = &payload["data"]["object"];

        let transaction_id = data["id"].as_str().unwrap_or("").to_string();
        let order_id = data["metadata"]["receipt"].as_str().map(|s| s.to_string());
        let amount_raw = data["amount"].as_i64().unwrap_or(0);
        let amount_decimal = sea_orm::prelude::Decimal::new(amount_raw, 0);

        let stripe_status = data["status"].as_str().unwrap_or("unknown");
        let status = match stripe_status {
            "succeeded" => "success",
            "requires_payment_method" | "requires_confirmation" | "requires_action" => "pending",
            "canceled" => "failure",
            _ => "unknown",
        };

        let payment_method_type = data["payment_method_types"]
            .as_array()
            .and_then(|a| a.first())
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let error_reason = if event_type == "payment_intent.payment_failed" {
            data["last_payment_error"]["message"]
                .as_str()
                .map(|s| s.to_string())
        } else {
            None
        };

        debug!(event_type = %event_type, transaction_id = %transaction_id, "Parsed Stripe webhook");

        Ok(GatewayWebhookPayload {
            event_id: payload["id"].as_str().unwrap_or("").to_string(),
            event_type: event_type.to_string(),
            transaction_id,
            order_id,
            amount: amount_decimal,
            status: status.to_string(),
            payment_method: payment_method_type,
            error_reason,
            raw_payload: payload,
        })
    }

    async fn refund_payment(
        &self,
        payment_id: &str,
        amount: sea_orm::prelude::Decimal,
        currency: &str,
        _reason: Option<&str>,
    ) -> Result<GatewayRefundResponse, AppError> {
        // Stripe expects amount in smallest currency unit (paise for INR)
        let amount_paise = (amount * sea_orm::prelude::Decimal::new(100, 0))
            .to_string()
            .parse::<i64>()
            .unwrap_or(0);

        let client = reqwest::Client::new();
        let response = client
            .post(format!("{}/refunds", self.api_endpoint))
            .bearer_auth(&self.secret_key)
            .form(&[
                ("payment_intent", payment_id.to_string()),
                ("amount", amount_paise.to_string()),
                ("reason", "requested_by_customer".to_string()),
            ])
            .send()
            .await
            .map_err(|e| AppError::External(format!("Stripe refund request failed: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_body = response.text().await.unwrap_or_default();
            warn!(status = %status, body = %error_body, "Stripe refund failed");
            return Err(AppError::External(format!(
                "Stripe refund error ({}): {}",
                status, error_body
            )));
        }

        let refund: serde_json::Value = response.json().await.map_err(|e| {
            AppError::External(format!("Failed to parse Stripe refund response: {}", e))
        })?;

        let refund_id = refund["id"].as_str().unwrap_or("").to_string();
        let stripe_status = refund["status"].as_str().unwrap_or("pending");
        let status = match stripe_status {
            "succeeded" => "processed",
            "failed" => "failed",
            _ => "pending",
        };
        let amount_cents = refund["amount"].as_i64().unwrap_or(0);

        info!(refund_id = %refund_id, status = %status, amount = %amount, "Stripe refund issued");

        Ok(GatewayRefundResponse {
            refund_id,
            status: status.to_string(),
            amount: sea_orm::prelude::Decimal::new(amount_cents, 2),
            currency: refund["currency"].as_str().unwrap_or(currency).to_string(),
        })
    }
}
