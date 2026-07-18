//! MSG91 SMS Adapter
//!
//! MSG91 is a popular Indian SMS gateway for:
//! - OTP generation and verification
//! - Transactional SMS (invoices, alerts, notifications)
//! - DLT (TRAI) compliant templates
//!
//! API Reference: https://msg91.com/help/api

use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::{debug, info, warn};

use super::{SmsDeliveryStatus, SmsProvider};
use crate::shared::errors::AppError;

// ============================================================================
// Configuration
// ============================================================================

/// MSG91 adapter configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Msg91Config {
    pub auth_key: String,
    pub sender_id: String,
    pub route: String,
    pub country_code: String,
    pub api_url: String,
}

impl Default for Msg91Config {
    fn default() -> Self {
        Self {
            auth_key: std::env::var("MSG91_AUTH_KEY").unwrap_or_default(),
            sender_id: std::env::var("MSG91_SENDER_ID").unwrap_or_else(|_| "AEROXE".to_string()),
            route: std::env::var("MSG91_ROUTE").unwrap_or_else(|_| "4".to_string()), // 4 = Transactional
            country_code: std::env::var("MSG91_COUNTRY_CODE").unwrap_or_else(|_| "91".to_string()),
            api_url: std::env::var("MSG91_API_URL")
                .unwrap_or_else(|_| "https://api.msg91.com/api/v5/otp".to_string()),
        }
    }
}

// ============================================================================
// Data Types
// ============================================================================

/// OTP send request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OtpSendRequest {
    #[serde(rename = "mobile")]
    pub phone: String,
    #[serde(rename = "otp")]
    pub otp: Option<String>,
    #[serde(rename = "otp_expiry")]
    pub expiry_minutes: Option<u32>,
    #[serde(rename = "template_id")]
    pub template_id: Option<String>,
    #[serde(rename = "sender")]
    pub sender_id: Option<String>,
}

/// OTP verification request
#[derive(Debug, Clone, Serialize)]
pub struct OtpVerifyRequest {
    #[serde(rename = "mobile")]
    pub phone: String,
    #[serde(rename = "otp")]
    pub otp: String,
}

/// SMS send request
#[derive(Debug, Clone, Serialize)]
pub struct SmsSendRequest {
    #[serde(rename = "sender")]
    pub sender_id: String,
    #[serde(rename = "route")]
    pub route: String,
    #[serde(rename = "country")]
    pub country_code: String,
    #[serde(rename = "sms")]
    pub messages: Vec<SmsMessage>,
}

/// Individual SMS message
#[derive(Debug, Clone, Serialize)]
pub struct SmsMessage {
    #[serde(rename = "message")]
    pub text: String,
    #[serde(rename = "to")]
    pub recipients: Vec<String>,
    #[serde(rename = "template_id")]
    pub template_id: Option<String>,
}

/// API response
#[derive(Debug, Clone, Deserialize)]
pub struct Msg91Response {
    pub type_: Option<String>,
    pub message: Option<String>,
    pub request_id: Option<String>,
}

// ============================================================================
// MSG91 Adapter
// ============================================================================

/// MSG91 SMS adapter
pub struct Msg91Adapter {
    config: Msg91Config,
    client: Client,
}

impl Msg91Adapter {
    /// Create a new MSG91 adapter
    pub fn new(config: Msg91Config) -> Self {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .unwrap_or_default();

        Self { config, client }
    }

    /// Create adapter from environment variables
    pub fn from_env() -> Self {
        Self::new(Msg91Config::default())
    }

    /// Check if the adapter is configured (has an auth key)
    pub fn is_configured(&self) -> bool {
        !self.config.auth_key.is_empty()
    }

    /// Generate a 6-digit OTP
    fn generate_otp() -> String {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        format!("{:06}", rng.gen_range(100000..999999))
    }
}

#[async_trait]
impl SmsProvider for Msg91Adapter {
    async fn send_otp(&self, phone: &str, template_id: Option<&str>) -> Result<String, AppError> {
        let otp = Self::generate_otp();

        let body = serde_json::json!({
            "mobile": format!("{}{}", self.config.country_code, phone),
            "otp": otp,
            "otp_expiry": "5",
            "template_id": template_id.unwrap_or(""),
            "sender": self.config.sender_id,
        });

        debug!(phone = %phone, "Sending OTP via MSG91");

        let response = self
            .client
            .post(&self.config.api_url)
            .header("authkey", &self.config.auth_key)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| AppError::External(format!("MSG91 API request failed: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            warn!(status = %status, body = %body, "MSG91 OTP send failed");
            return Err(AppError::External(format!(
                "MSG91 API error ({}): {}",
                status, body
            )));
        }

        let result: serde_json::Value = response
            .json()
            .await
            .map_err(|e| AppError::External(format!("Failed to parse MSG91 response: {}", e)))?;

        let request_id = result["request_id"].as_str().unwrap_or("").to_string();

        info!(phone = %phone, request_id = %request_id, "Sent OTP via MSG91");
        Ok(otp)
    }

    async fn verify_otp(&self, phone: &str, otp: &str) -> Result<bool, AppError> {
        let mobile = format!("{}{}", self.config.country_code, phone);
        let verify_url = format!(
            "https://api.msg91.com/api/v5/otp/verify?mobile={}&otp={}",
            mobile, otp
        );

        debug!(phone = %phone, "Verifying OTP via MSG91");

        let response = self
            .client
            .get(&verify_url)
            .header("authkey", &self.config.auth_key)
            .send()
            .await
            .map_err(|e| AppError::External(format!("MSG91 API request failed: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            warn!(status = %status, body = %body, "MSG91 OTP verification failed");
            return Err(AppError::External(format!(
                "MSG91 API error ({}): {}",
                status, body
            )));
        }

        let result: serde_json::Value = response
            .json()
            .await
            .map_err(|e| AppError::External(format!("Failed to parse MSG91 response: {}", e)))?;

        let verified = result["type"].as_str() == Some("success");
        info!(phone = %phone, verified = verified, "OTP verification result from MSG91");
        Ok(verified)
    }

    async fn send_sms(
        &self,
        phone: &str,
        message: &str,
        template_id: Option<&str>,
    ) -> Result<String, AppError> {
        let body = serde_json::json!({
            "sender": self.config.sender_id,
            "route": self.config.route,
            "country": self.config.country_code,
            "sms": [{
                "message": message,
                "to": [format!("{}{}", self.config.country_code, phone)],
                "template_id": template_id,
            }],
        });

        debug!(phone = %phone, "Sending SMS via MSG91");

        let url = "https://api.msg91.com/api/v5/sms/send";
        let response = self
            .client
            .post(url)
            .header("authkey", &self.config.auth_key)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| AppError::External(format!("MSG91 API request failed: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            warn!(status = %status, body = %body, "MSG91 SMS send failed");
            return Err(AppError::External(format!(
                "MSG91 API error ({}): {}",
                status, body
            )));
        }

        let result: serde_json::Value = response
            .json()
            .await
            .map_err(|e| AppError::External(format!("Failed to parse MSG91 response: {}", e)))?;

        let request_id = result["request_id"].as_str().unwrap_or("").to_string();

        info!(phone = %phone, request_id = %request_id, "Sent SMS via MSG91");
        Ok(request_id)
    }

    async fn send_bulk_sms(
        &self,
        phones: &[String],
        message: &str,
        template_id: Option<&str>,
    ) -> Result<String, AppError> {
        let recipients: Vec<String> = phones
            .iter()
            .map(|p| format!("{}{}", self.config.country_code, p))
            .collect();

        let body = serde_json::json!({
            "sender": self.config.sender_id,
            "route": self.config.route,
            "country": self.config.country_code,
            "sms": [{
                "message": message,
                "to": recipients,
                "template_id": template_id,
            }],
        });

        debug!(count = phones.len(), "Sending bulk SMS via MSG91");

        let url = "https://api.msg91.com/api/v5/sms/send";
        let response = self
            .client
            .post(url)
            .header("authkey", &self.config.auth_key)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| AppError::External(format!("MSG91 API request failed: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            warn!(status = %status, body = %body, "MSG91 bulk SMS send failed");
            return Err(AppError::External(format!(
                "MSG91 API error ({}): {}",
                status, body
            )));
        }

        let result: serde_json::Value = response
            .json()
            .await
            .map_err(|e| AppError::External(format!("Failed to parse MSG91 response: {}", e)))?;

        let request_id = result["request_id"].as_str().unwrap_or("").to_string();

        info!(count = phones.len(), request_id = %request_id, "Sent bulk SMS via MSG91");
        Ok(request_id)
    }

    async fn check_delivery(&self, request_id: &str) -> Result<SmsDeliveryStatus, AppError> {
        let url = format!(
            "https://api.msg91.com/api/v5/otp/retry?request_id={}",
            request_id
        );

        let response = self
            .client
            .get(&url)
            .header("authkey", &self.config.auth_key)
            .send()
            .await
            .map_err(|e| AppError::External(format!("MSG91 API request failed: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            warn!(status = %status, body = %body, "MSG91 delivery check failed");
            return Err(AppError::External(format!(
                "MSG91 API error ({}): {}",
                status, body
            )));
        }

        let result: serde_json::Value = response
            .json()
            .await
            .map_err(|e| AppError::External(format!("Failed to parse MSG91 response: {}", e)))?;

        Ok(SmsDeliveryStatus {
            request_id: request_id.to_string(),
            status: result["type"].as_str().unwrap_or("unknown").to_string(),
            phone: String::new(),
            delivered: result["type"].as_str() == Some("success"),
            delivered_at: None,
            error: result["message"].as_str().map(|s| s.to_string()),
        })
    }
}
