use async_trait::async_trait;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use tracing::{debug, info, warn};

use crate::shared::errors::AppError;

type HmacSha256 = Hmac<Sha256>;

/// Shared HTTP client for all gateway calls. A bare `reqwest::Client::new()`
/// has no timeout, so a hung gateway would block the request thread forever.
/// All gateway adapters use explicit connect + total timeouts instead.
fn gateway_http_client() -> reqwest::Client {
    reqwest::Client::builder()
        .connect_timeout(std::time::Duration::from_secs(10))
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .unwrap_or_default()
}

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

        let client = gateway_http_client();
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

        // Razorpay sends amount in paise (smallest currency unit): scale 2 -> rupees
        let amount_decimal = sea_orm::prelude::Decimal::new(amount_paise, 2);

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

        let client = gateway_http_client();
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

/// PayU SHA-512 hash: sha512(key|txnid|amount|productinfo|firstname|email|
/// udf1..udf10|salt). Firstname and email are required in their exact
/// positions — substituting any other field (e.g. currency) produces a hash
/// PayU's server rejects.
pub fn payu_hash(
    merchant_key: &str,
    txn_id: &str,
    amount: &str,
    productinfo: &str,
    firstname: &str,
    email: &str,
    merchant_salt: &str,
) -> String {
    use sha2::{Digest, Sha512};
    let hash_string = format!(
        "{}|{}|{}|{}|{}|{}|||||||||||{}",
        merchant_key, txn_id, amount, productinfo, firstname, email, merchant_salt
    );
    let mut hasher = Sha512::new();
    hasher.update(hash_string.as_bytes());
    hex::encode(hasher.finalize())
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

        // PayU uses SHA-512: sha512(key|txnid|amount|productinfo|firstname|
        // email|udf1..udf10|salt). The 5th and 6th fields are firstname/email,
        // NOT currency — a mismatched layout fails PayU's server-side hash check
        // and rejects every payment.
        let firstname = metadata["customer_name"].as_str().unwrap_or("Customer");
        let email = metadata["customer_email"].as_str().unwrap_or("");
        let hash = payu_hash(
            &self.merchant_key,
            &txn_id,
            &amount.to_string(),
            receipt,
            firstname,
            email,
            &self.merchant_salt,
        );

        let body = serde_json::json!({
            "key": self.merchant_key,
            "txnid": txn_id,
            "amount": amount.to_string(),
            "productinfo": receipt,
            "firstname": firstname,
            "email": email,
            "phone": metadata["customer_phone"].as_str().unwrap_or(""),
            "surl": metadata["success_url"].as_str().unwrap_or(""),
            "furl": metadata["failure_url"].as_str().unwrap_or(""),
            "hash": hash,
        });

        // Post the order to the same environment as the redirect endpoint:
        // test.payu.in in test, secure.payu.in in production.
        let post_url = if self.api_endpoint.contains("test.payu.in") {
            "https://test.payu.in/merchant/postservice.php?form=1"
        } else {
            "https://secure.payu.in/merchant/postservice.php?form=1"
        };

        let client = gateway_http_client();
        let response = client
            .post(post_url)
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
        // PayU sends amount as a string in rupees (may include decimals, e.g. "1050.50")
        let amount_str = payload["amount"].as_str().unwrap_or("0");
        let amount_decimal = amount_str
            .parse::<sea_orm::prelude::Decimal>()
            .unwrap_or_else(|_| sea_orm::prelude::Decimal::new(0, 0));
        let payment_method = payload["payment_source"].as_str().map(|s| s.to_string());

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

        let client = gateway_http_client();
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
        let client = gateway_http_client();
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

        // Replay protection: reject signatures older than 5 minutes so a
        // captured/leaked webhook payload cannot be replayed later.
        let sent_ts: i64 = timestamp.parse().unwrap_or(0);
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0);
        if sent_ts <= 0 || (now - sent_ts).abs() > 300 {
            warn!(
                timestamp = %timestamp,
                "Stripe webhook signature is stale — rejecting replay"
            );
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
        // Stripe sends amount in smallest currency unit (cents/paise): scale 2 -> rupees
        let amount_decimal = sea_orm::prelude::Decimal::new(amount_raw, 2);

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

        let client = gateway_http_client();
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

#[cfg(test)]
mod tests {
    use super::*;
    use sea_orm::prelude::Decimal;

    fn assert_amount_eq(actual: Decimal, expected: &str) {
        let expected: Decimal = expected.parse().unwrap();
        assert_eq!(
            actual, expected,
            "amount {} != expected {}",
            actual, expected
        );
    }

    #[test]
    fn razorpay_webhook_amount_is_in_rupees_not_paise() {
        let adapter = RazorpayAdapter {
            key_id: "k".into(),
            key_secret: "s".into(),
            webhook_secret: "w".into(),
        };
        let payload = serde_json::json!({
            "event": "payment.captured",
            "payload": { "payment": { "entity": {
                "id": "pay_123", "order_id": "order_1", "amount": 1050,
                "status": "captured", "method": "upi"
            } } }
        });
        let parsed = adapter.parse_webhook(payload).unwrap();
        assert_amount_eq(parsed.amount, "10.50");
    }

    #[test]
    fn stripe_webhook_amount_is_in_cents_not_dollars() {
        let adapter = StripeAdapter {
            secret_key: "k".into(),
            webhook_signing_secret: "w".into(),
            api_endpoint: "https://api.stripe.com/v1".into(),
        };
        let payload = serde_json::json!({
            "type": "payment_intent.succeeded",
            "data": { "object": { "id": "pi_123", "amount": 1050, "status": "succeeded" } }
        });
        let parsed = adapter.parse_webhook(payload).unwrap();
        assert_amount_eq(parsed.amount, "10.50");
    }

    #[test]
    fn payu_webhook_amount_supports_decimal_string() {
        let adapter = PayuAdapter {
            merchant_key: "k".into(),
            merchant_salt: "s".into(),
            api_endpoint: "https://test.payu.in".into(),
        };
        let payload = serde_json::json!({
            "status": "success", "txnid": "pay_123", "order_id": "order_1",
            "amount": "1050.50", "payment_source": "netbanking"
        });
        let parsed = adapter.parse_webhook(payload).unwrap();
        assert_amount_eq(parsed.amount, "1050.50");
    }

    #[test]
    fn payu_webhook_amount_whole_rupees() {
        let adapter = PayuAdapter {
            merchant_key: "k".into(),
            merchant_salt: "s".into(),
            api_endpoint: "https://test.payu.in".into(),
        };
        let payload = serde_json::json!({
            "status": "success", "txnid": "pay_123", "order_id": "order_1",
            "amount": "500", "payment_source": "cc"
        });
        let parsed = adapter.parse_webhook(payload).unwrap();
        assert_amount_eq(parsed.amount, "500.00");
    }

    #[test]
    fn payu_hash_layout_uses_firstname_and_email_in_the_5th_and_6th_fields() {
        use sha2::Digest;

        // key|txnid|amount|productinfo|firstname|email|udf1..udf10|salt
        // = 17 pipe-separated fields, firstname/email at positions 5 and 6.
        let key = "gtKFFx";
        let txn = "txn_abc";
        let amount = "500.00";
        let productinfo = "inv-1";
        let firstname = "Rahul";
        let email = "rahul@example.com";
        let salt = "e5iIg1jwi8";
        let hash = payu_hash(key, txn, amount, productinfo, firstname, email, salt);

        // The hash must be computed over the exact documented layout.
        let expected_input =
            format!("{key}|{txn}|{amount}|{productinfo}|{firstname}|{email}|||||||||||{salt}");
        let mut hasher = sha2::Sha512::new();
        hasher.update(expected_input.as_bytes());
        let expected = hex::encode(hasher.finalize());
        assert_eq!(hash, expected);

        // A layout that swaps in `currency` (the historical bug) must differ,
        // proving the fix actually changes what is hashed.
        let wrong_input = format!("{key}|{txn}|{amount}|{productinfo}|INR|||||||||||{salt}");
        let mut hasher = sha2::Sha512::new();
        hasher.update(wrong_input.as_bytes());
        let wrong = hex::encode(hasher.finalize());
        assert_ne!(hash, wrong);
    }

    #[test]
    fn stripe_webhook_rejects_stale_signature() {
        let adapter = StripeAdapter {
            secret_key: "k".into(),
            webhook_signing_secret: "w".into(),
            api_endpoint: "https://api.stripe.com/v1".into(),
        };
        // A signature stamped 10 minutes ago (freshness window is 5 min) must
        // be rejected even before HMAC comparison.
        let stale_ts = (std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            - 600) as i64;
        let body = b"{}";
        let signature = format!("t={},v1=deadbeef", stale_ts);
        let ok = adapter
            .verify_webhook_signature(body, &signature, "w")
            .unwrap();
        assert!(!ok, "stale webhook signature must be rejected");
    }
}
