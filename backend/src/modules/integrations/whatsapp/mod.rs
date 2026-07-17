//! WhatsApp Business API Adapter
//!
//! Integrates with the WhatsApp Business Platform Cloud API for:
//! - Template-based message sending (pre-approved templates)
//! - Interactive messages (buttons, lists)
//! - Media messages (images, documents)
//! - Delivery status tracking
//!
//! API Reference: https://developers.facebook.com/docs/whatsapp/cloud-api

use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::{debug, info, warn};

use crate::shared::errors::AppError;

// ============================================================================
// Configuration
// ============================================================================

/// WhatsApp Business API configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhatsAppConfig {
    /// Meta Business API access token
    pub access_token: String,
    /// WhatsApp Business phone number ID
    pub phone_number_id: String,
    /// WhatsApp Business Account ID
    pub business_account_id: String,
    /// API version
    pub api_version: String,
    /// API base URL
    pub api_url: String,
}

impl Default for WhatsAppConfig {
    fn default() -> Self {
        Self {
            access_token: std::env::var("WHATSAPP_ACCESS_TOKEN").unwrap_or_default(),
            phone_number_id: std::env::var("WHATSAPP_PHONE_NUMBER_ID").unwrap_or_default(),
            business_account_id: std::env::var("WHATSAPP_BUSINESS_ACCOUNT_ID").unwrap_or_default(),
            api_version: std::env::var("WHATSAPP_API_VERSION")
                .unwrap_or_else(|_| "v18.0".to_string()),
            api_url: std::env::var("WHATSAPP_API_URL")
                .unwrap_or_else(|_| "https://graph.facebook.com".to_string()),
        }
    }
}

// ============================================================================
// Data Types
// ============================================================================

/// WhatsApp message request
#[derive(Debug, Clone, Serialize)]
pub struct WhatsAppMessageRequest {
    pub messaging_product: String,
    pub to: String,
    #[serde(rename = "type")]
    pub message_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<TemplatePayload>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<TextPayload>,
}

/// Template message payload
#[derive(Debug, Clone, Serialize)]
pub struct TemplatePayload {
    pub name: String,
    pub language: LanguagePayload,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<TemplateComponent>>,
}

/// Language specification for template
#[derive(Debug, Clone, Serialize)]
pub struct LanguagePayload {
    pub code: String,
}

/// Template component (header, body params, buttons)
#[derive(Debug, Clone, Serialize)]
pub struct TemplateComponent {
    #[serde(rename = "type")]
    pub component_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<ComponentParameter>>,
}

/// Component parameter
#[derive(Debug, Clone, Serialize)]
pub struct ComponentParameter {
    #[serde(rename = "type")]
    pub param_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

/// Plain text message payload
#[derive(Debug, Clone, Serialize)]
pub struct TextPayload {
    pub preview_url: bool,
    pub body: String,
}

/// WhatsApp API response
#[derive(Debug, Clone, Deserialize)]
pub struct WhatsAppResponse {
    pub messaging_product: Option<String>,
    pub contacts: Option<Vec<ContactInfo>>,
    pub messages: Option<Vec<MessageInfo>>,
    pub error: Option<WhatsAppError>,
}

/// Contact info in response
#[derive(Debug, Clone, Deserialize)]
pub struct ContactInfo {
    pub input: Option<String>,
    pub wa_id: Option<String>,
}

/// Message info in response
#[derive(Debug, Clone, Deserialize)]
pub struct MessageInfo {
    pub id: Option<String>,
}

/// WhatsApp API error
#[derive(Debug, Clone, Deserialize)]
pub struct WhatsAppError {
    pub message: Option<String>,
    #[serde(rename = "type")]
    pub error_type: Option<String>,
    pub code: Option<i32>,
    pub error_subcode: Option<i32>,
}

/// Delivery status of a WhatsApp message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhatsAppDeliveryStatus {
    pub message_id: String,
    pub status: String,
    pub recipient: String,
    pub delivered: bool,
    pub delivered_at: Option<String>,
    pub read_at: Option<String>,
    pub error: Option<String>,
}

/// WhatsApp message status webhook payload
#[derive(Debug, Clone, Deserialize)]
pub struct WhatsAppStatusWebhook {
    pub entry: Vec<WhatsAppEntry>,
}

/// Webhook entry
#[derive(Debug, Clone, Deserialize)]
pub struct WhatsAppEntry {
    pub changes: Vec<WhatsAppChange>,
}

/// Webhook change
#[derive(Debug, Clone, Deserialize)]
pub struct WhatsAppChange {
    pub value: WhatsAppChangeValue,
}

/// Change value containing statuses
#[derive(Debug, Clone, Deserialize)]
pub struct WhatsAppChangeValue {
    pub statuses: Option<Vec<WhatsAppStatusUpdate>>,
}

/// Status update from webhook
#[derive(Debug, Clone, Deserialize)]
pub struct WhatsAppStatusUpdate {
    pub id: Option<String>,
    pub status: Option<String>,
    pub timestamp: Option<String>,
    pub recipient_id: Option<String>,
    pub errors: Option<Vec<WhatsAppStatusError>>,
}

/// Status error from webhook
#[derive(Debug, Clone, Deserialize)]
pub struct WhatsAppStatusError {
    pub code: Option<i32>,
    pub title: Option<String>,
    pub message: Option<String>,
}

// ============================================================================
// WhatsApp Business API Adapter
// ============================================================================

/// WhatsApp Business API adapter
pub struct WhatsAppAdapter {
    config: WhatsAppConfig,
    client: Client,
}

impl WhatsAppAdapter {
    /// Create a new WhatsApp adapter
    pub fn new(config: WhatsAppConfig) -> Self {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .unwrap_or_default();

        Self { config, client }
    }

    /// Create adapter from environment variables
    pub fn from_env() -> Self {
        Self::new(WhatsAppConfig::default())
    }

    /// Check if the adapter is configured
    pub fn is_configured(&self) -> bool {
        !self.config.access_token.is_empty() && !self.config.phone_number_id.is_empty()
    }

    /// Send a template-based message (most common for notifications)
    pub async fn send_template_message(
        &self,
        phone: &str,
        template_name: &str,
        language_code: &str,
        params: Option<Vec<ComponentParameter>>,
    ) -> Result<WhatsAppDeliveryStatus, AppError> {
        let components = params.map(|p| {
            vec![TemplateComponent {
                component_type: "body".to_string(),
                sub_type: None,
                index: None,
                parameters: Some(p),
            }]
        });

        let request = WhatsAppMessageRequest {
            messaging_product: "whatsapp".to_string(),
            to: normalize_phone(phone),
            message_type: "template".to_string(),
            template: Some(TemplatePayload {
                name: template_name.to_string(),
                language: LanguagePayload {
                    code: language_code.to_string(),
                },
                components,
            }),
            text: None,
        };

        self.send_message(request, phone).await
    }

    /// Send a free-form text message
    pub async fn send_text_message(
        &self,
        phone: &str,
        message: &str,
    ) -> Result<WhatsAppDeliveryStatus, AppError> {
        let request = WhatsAppMessageRequest {
            messaging_product: "whatsapp".to_string(),
            to: normalize_phone(phone),
            message_type: "text".to_string(),
            template: None,
            text: Some(TextPayload {
                preview_url: false,
                body: message.to_string(),
            }),
        };

        self.send_message(request, phone).await
    }

    /// Internal method to send a message via the WhatsApp API
    async fn send_message(
        &self,
        request: WhatsAppMessageRequest,
        phone: &str,
    ) -> Result<WhatsAppDeliveryStatus, AppError> {
        let url = format!(
            "{}/{}/messages",
            self.config.api_url, self.config.api_version
        );

        debug!(phone = %phone, message_type = %request.message_type, "Sending WhatsApp message");

        let response = self
            .client
            .post(&url)
            .bearer_auth(&self.config.access_token)
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| AppError::External(format!("WhatsApp API request failed: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            warn!(status = %status, body = %body, "WhatsApp message send failed");
            return Err(AppError::External(format!(
                "WhatsApp API error ({}): {}",
                status, body
            )));
        }

        let result: WhatsAppResponse = response
            .json()
            .await
            .map_err(|e| AppError::External(format!("Failed to parse WhatsApp response: {}", e)))?;

        if let Some(err) = result.error {
            let msg = err.message.unwrap_or_else(|| "Unknown error".to_string());
            warn!(error = %msg, code = ?err.code, "WhatsApp API returned error");
            return Err(AppError::External(format!("WhatsApp error: {}", msg)));
        }

        let message_id = result
            .messages
            .as_ref()
            .and_then(|msgs| msgs.first())
            .and_then(|m| m.id.clone())
            .unwrap_or_default();

        let recipient = result
            .contacts
            .as_ref()
            .and_then(|c| c.first())
            .and_then(|c| c.wa_id.clone())
            .unwrap_or_else(|| phone.to_string());

        info!(phone = %phone, message_id = %message_id, "Sent WhatsApp message");

        Ok(WhatsAppDeliveryStatus {
            message_id,
            status: "sent".to_string(),
            recipient,
            delivered: false,
            delivered_at: None,
            read_at: None,
            error: None,
        })
    }

    /// Send a notification using a pre-approved template
    /// This is the primary method used by the notification worker
    pub async fn send_notification(
        &self,
        phone: &str,
        template_name: &str,
        body_params: Vec<String>,
    ) -> Result<WhatsAppDeliveryStatus, AppError> {
        let parameters: Vec<ComponentParameter> = body_params
            .into_iter()
            .map(|text| ComponentParameter {
                param_type: "text".to_string(),
                text: Some(text),
            })
            .collect();

        self.send_template_message(phone, template_name, "en", Some(parameters))
            .await
    }

    /// Check delivery status of a message (via API polling)
    pub async fn check_delivery(&self, message_id: &str) -> Result<WhatsAppDeliveryStatus, AppError> {
        let url = format!(
            "{}/{}/{}",
            self.config.api_url, self.config.api_version, message_id
        );

        let response = self
            .client
            .get(&url)
            .bearer_auth(&self.config.access_token)
            .send()
            .await
            .map_err(|e| AppError::External(format!("WhatsApp API request failed: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(AppError::External(format!(
                "WhatsApp API error ({}): {}",
                status, body
            )));
        }

        let result: serde_json::Value = response
            .json()
            .await
            .map_err(|e| AppError::External(format!("Failed to parse WhatsApp response: {}", e)))?;

        let status = result["status"].as_str().unwrap_or("unknown").to_string();

        Ok(WhatsAppDeliveryStatus {
            message_id: message_id.to_string(),
            status: status.clone(),
            recipient: String::new(),
            delivered: status == "delivered",
            delivered_at: None,
            read_at: None,
            error: None,
        })
    }

    /// Parse a status webhook update from WhatsApp
    pub fn parse_status_webhook(payload: &str) -> Option<WhatsAppStatusUpdate> {
        let webhook: WhatsAppStatusWebhook = serde_json::from_str(payload).ok()?;
        webhook
            .entry
            .first()?
            .changes
            .first()?
            .value
            .statuses
            .as_ref()?
            .first()
            .cloned()
    }
}

/// Normalize an Indian phone number for WhatsApp
/// WhatsApp expects numbers without + or leading zeros
fn normalize_phone(phone: &str) -> String {
    let cleaned: String = phone
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect();

    // If starts with 0, assume India and prepend 91
    if cleaned.starts_with('0') && cleaned.len() <= 11 {
        return format!("91{}", &cleaned[1..]);
    }

    // If already has country code (10+ digits), use as is
    if cleaned.len() >= 12 {
        return cleaned;
    }

    // Default: assume 10-digit Indian number, prepend 91
    if cleaned.len() == 10 {
        return format!("91{}", cleaned);
    }

    cleaned
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_phone() {
        assert_eq!(normalize_phone("+919876543210"), "919876543210");
        assert_eq!(normalize_phone("9876543210"), "919876543210");
        assert_eq!(normalize_phone("09876543210"), "919876543210");
        assert_eq!(normalize_phone("+11234567890"), "11234567890");
    }

    #[test]
    fn test_is_configured() {
        let adapter = WhatsAppAdapter::from_env();
        // Should not be configured without env vars
        assert!(!adapter.is_configured());
    }
}
