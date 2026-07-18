//! Twilio SMS Adapter
//!
//! Twilio is a global SMS provider with excellent India support for:
//! - OTP generation and verification (Verify API)
//! - Transactional SMS (invoices, alerts, notifications)
//! - Two-way messaging
//!
//! API Reference: https://www.twilio.com/docs/messaging/api

use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::{debug, info, warn};

use crate::shared::errors::AppError;

use super::{SmsDeliveryStatus, SmsProvider};

// ============================================================================
// Configuration
// ============================================================================

/// Twilio adapter configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TwilioConfig {
    pub account_sid: String,
    pub auth_token: String,
    pub from_number: String,
    pub verify_service_sid: Option<String>,
    pub api_url: String,
}

impl Default for TwilioConfig {
    fn default() -> Self {
        Self {
            account_sid: std::env::var("TWILIO_ACCOUNT_SID").unwrap_or_default(),
            auth_token: std::env::var("TWILIO_AUTH_TOKEN").unwrap_or_default(),
            from_number: std::env::var("TWILIO_FROM_NUMBER").unwrap_or_default(),
            verify_service_sid: std::env::var("TWILIO_VERIFY_SERVICE_SID").ok(),
            api_url: std::env::var("TWILIO_API_URL")
                .unwrap_or_else(|_| "https://api.twilio.com".to_string()),
        }
    }
}

// ============================================================================
// Data Types
// ============================================================================

/// Twilio message response
#[derive(Debug, Clone, Deserialize)]
pub struct TwilioMessageResponse {
    pub sid: String,
    pub status: String,
    pub to: String,
    pub from: String,
    pub date_sent: Option<String>,
    pub error_code: Option<i32>,
    pub error_message: Option<String>,
}

/// Twilio verification check response
#[derive(Debug, Clone, Deserialize)]
pub struct TwilioVerifyResponse {
    pub sid: String,
    pub status: String,
    pub to: String,
    pub valid: bool,
}

// ============================================================================
// Twilio Adapter
// ============================================================================

/// Twilio SMS adapter
pub struct TwilioSmsAdapter {
    config: TwilioConfig,
    client: Client,
}

impl TwilioSmsAdapter {
    /// Create a new Twilio adapter
    pub fn new(config: TwilioConfig) -> Self {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .unwrap_or_default();

        Self { config, client }
    }

    /// Create adapter from environment variables
    pub fn from_env() -> Self {
        Self::new(TwilioConfig::default())
    }

    /// Get basic auth header for Twilio API
    fn auth_header(&self) -> String {
        use base64::Engine;
        let credentials = format!("{}:{}", self.config.account_sid, self.config.auth_token);
        format!(
            "Basic {}",
            base64::engine::general_purpose::STANDARD.encode(credentials)
        )
    }

    /// Send SMS via Twilio Messaging API
    async fn send_message(&self, to: &str, body: &str) -> Result<TwilioMessageResponse, AppError> {
        let url = format!(
            "{}/2010-04-01/Accounts/{}/Messages.json",
            self.config.api_url, self.config.account_sid
        );

        let form = [
            ("To", to),
            ("From", &self.config.from_number),
            ("Body", body),
        ];

        debug!(to = %to, "Sending SMS via Twilio");

        let response = self
            .client
            .post(&url)
            .header("Authorization", self.auth_header())
            .form(&form)
            .send()
            .await
            .map_err(|e| AppError::External(format!("Twilio API request failed: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            warn!(status = %status, body = %body, "Twilio SMS send failed");
            return Err(AppError::External(format!(
                "Twilio API error ({}): {}",
                status, body
            )));
        }

        let result: TwilioMessageResponse = response
            .json()
            .await
            .map_err(|e| AppError::External(format!("Failed to parse Twilio response: {}", e)))?;

        info!(sid = %result.sid, to = %to, status = %result.status, "Sent SMS via Twilio");
        Ok(result)
    }
}

#[async_trait]
impl SmsProvider for TwilioSmsAdapter {
    async fn send_otp(&self, phone: &str, _template_id: Option<&str>) -> Result<String, AppError> {
        // Use Twilio Verify API if service SID is configured
        if let Some(ref service_sid) = self.config.verify_service_sid {
            let url = format!(
                "{}/v2/Services/{}/Verifications",
                self.config.api_url, service_sid
            );

            let form = [
                ("To", &format!("+91{}", phone)),
                ("Channel", &"sms".to_string()),
            ];

            debug!(phone = %phone, "Sending OTP via Twilio Verify");

            let response = self
                .client
                .post(&url)
                .header("Authorization", self.auth_header())
                .form(&form)
                .send()
                .await
                .map_err(|e| {
                    AppError::External(format!("Twilio Verify API request failed: {}", e))
                })?;

            if !response.status().is_success() {
                let status = response.status();
                let body = response.text().await.unwrap_or_default();
                warn!(status = %status, body = %body, "Twilio Verify send failed");
                return Err(AppError::External(format!(
                    "Twilio Verify API error ({}): {}",
                    status, body
                )));
            }

            let result: serde_json::Value = response.json().await.map_err(|e| {
                AppError::External(format!("Failed to parse Twilio response: {}", e))
            })?;

            let sid = result["sid"].as_str().unwrap_or("").to_string();
            info!(phone = %phone, sid = %sid, "Sent OTP via Twilio Verify");
            return Ok(sid);
        }

        // Fallback: Generate OTP and send via regular SMS
        let otp = {
            use rand::Rng;
            format!("{:06}", rand::thread_rng().gen_range(100000..999999))
        };

        let message = format!(
            "Your AeroXe verification code is: {}. Valid for 5 minutes. Do not share this code.",
            otp
        );

        let result = self
            .send_message(&format!("+91{}", phone), &message)
            .await?;
        Ok(result.sid)
    }

    async fn verify_otp(&self, phone: &str, otp: &str) -> Result<bool, AppError> {
        // Use Twilio Verify API if service SID is configured
        if let Some(ref service_sid) = self.config.verify_service_sid {
            let url = format!(
                "{}/v2/Services/{}/VerificationCheck",
                self.config.api_url, service_sid
            );

            let form = [("To", &format!("+91{}", phone)), ("Code", &otp.to_string())];

            debug!(phone = %phone, "Verifying OTP via Twilio Verify");

            let response = self
                .client
                .post(&url)
                .header("Authorization", self.auth_header())
                .form(&form)
                .send()
                .await
                .map_err(|e| {
                    AppError::External(format!("Twilio Verify API request failed: {}", e))
                })?;

            if !response.status().is_success() {
                let status = response.status();
                let body = response.text().await.unwrap_or_default();
                warn!(status = %status, body = %body, "Twilio Verify check failed");
                return Err(AppError::External(format!(
                    "Twilio Verify API error ({}): {}",
                    status, body
                )));
            }

            let result: TwilioVerifyResponse = response.json().await.map_err(|e| {
                AppError::External(format!("Failed to parse Twilio response: {}", e))
            })?;

            info!(phone = %phone, valid = result.valid, "OTP verification result from Twilio");
            return Ok(result.valid);
        }

        // Without Verify API, we can't verify (would need to store OTP in DB)
        warn!("Twilio Verify Service SID not configured, cannot verify OTP");
        Ok(false)
    }

    async fn send_sms(
        &self,
        phone: &str,
        message: &str,
        _template_id: Option<&str>,
    ) -> Result<String, AppError> {
        let result = self.send_message(&format!("+91{}", phone), message).await?;
        Ok(result.sid)
    }

    async fn send_bulk_sms(
        &self,
        phones: &[String],
        message: &str,
        template_id: Option<&str>,
    ) -> Result<String, AppError> {
        let mut last_sid = String::new();
        for phone in phones {
            match self.send_sms(phone, message, template_id).await {
                Ok(sid) => last_sid = sid,
                Err(e) => {
                    warn!(phone = %phone, error = %e, "Failed to send bulk SMS to number");
                    continue;
                }
            }
        }
        Ok(last_sid)
    }

    async fn check_delivery(&self, request_id: &str) -> Result<SmsDeliveryStatus, AppError> {
        let url = format!(
            "{}/2010-04-01/Accounts/{}/Messages/{}.json",
            self.config.api_url, self.config.account_sid, request_id
        );

        let response = self
            .client
            .get(&url)
            .header("Authorization", self.auth_header())
            .send()
            .await
            .map_err(|e| AppError::External(format!("Twilio API request failed: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            warn!(status = %status, body = %body, "Twilio delivery check failed");
            return Err(AppError::External(format!(
                "Twilio API error ({}): {}",
                status, body
            )));
        }

        let result: TwilioMessageResponse = response
            .json()
            .await
            .map_err(|e| AppError::External(format!("Failed to parse Twilio response: {}", e)))?;

        Ok(SmsDeliveryStatus {
            request_id: result.sid.clone(),
            status: result.status.clone(),
            phone: result.to,
            delivered: result.status == "delivered",
            delivered_at: result.date_sent,
            error: result.error_message,
        })
    }
}
