//! WhatsApp Business API v23 delivery service.
//!
//! Implements the WhatsApp Cloud API v23 (https://developers.facebook.com/docs/whatsapp/cloud-api).
//! Config stored in `notification_channels.config` as:
//! ```json
//! {
//!   "access_token": "EAAG...",
//!   "phone_number_id": "1234567890",
//!   "business_account_id": "WABA-..."
//! }
//! ```

use reqwest::Client;
use tracing::{info, warn};

use crate::common::errors::app_error::AppError;

const WHATSAPP_API_VERSION: &str = "v21.0";
const WHATSAPP_API_BASE: &str = "https://graph.facebook.com";

/// WhatsApp-specific configuration extracted from the channel config JSON.
#[derive(Debug, Clone)]
pub struct WhatsAppConfig {
    pub access_token: String,
    pub phone_number_id: String,
    pub business_account_id: Option<String>,
}

impl WhatsAppConfig {
    /// Parse from the channel config JSON stored in the database.
    pub    fn from_json(config: &serde_json::Value) -> Result<Self, AppError> {
        let access_token = config
            .get("access_token")
            .and_then(|v| v.as_str())
            .ok_or_else(|| AppError::Validation("WhatsApp config missing 'access_token'".into()))?
            .to_string();

        let phone_number_id = config
            .get("phone_number_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                AppError::Validation("WhatsApp config missing 'phone_number_id'".into())
            })?
            .to_string();

        let business_account_id = config
            .get("business_account_id")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        if access_token.is_empty() {
            return Err(AppError::Validation("WhatsApp access_token is empty".into()));
        }
        if phone_number_id.is_empty() {
            return Err(AppError::Validation(
                "WhatsApp phone_number_id is empty".into(),
            ));
        }

        Ok(Self {
            access_token,
            phone_number_id,
            business_account_id,
        })
    }

}

/// Result of sending a WhatsApp message.
#[derive(Debug)]
pub struct SendResult {
    pub success: bool,
    pub message_id: Option<String>,
    pub error: Option<String>,
}

/// Template message for WhatsApp Business API v23.
#[derive(Debug, Clone)]
pub struct TemplateMessage {
    pub name: String,
    pub language_code: String,
    pub parameters: Vec<TemplateParam>,
}

#[derive(Debug, Clone)]
pub struct TemplateParam {
    pub r#type: String,
    pub text: String,
}

/// Send a text message via WhatsApp Cloud API v23.
///
/// `to` must be a phone number in E.164 format (e.g., "+919876543210").
pub async fn send_text(
    client: &Client,
    config: &WhatsAppConfig,
    to: &str,
    text: &str,
) -> Result<SendResult, AppError> {
    let url = format!("{}/{}/messages", WHATSAPP_API_BASE, WHATSAPP_API_VERSION);

    let body = serde_json::json!({
        "messaging_product": "whatsapp",
        "recipient_type": "individual",
        "to": to,
        "type": "text",
        "text": {
            "preview_url": false,
            "body": text
        }
    });

    let response = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", config.access_token))
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await
        .map_err(|e| AppError::External(format!("WhatsApp API request failed: {e}")))?;

    let status = response.status();
    let payload: serde_json::Value = response
        .json()
        .await
        .map_err(|e| AppError::External(format!("WhatsApp API response parse error: {e}")))?;

    if status.is_success() {
        let msg_id = payload["messages"][0]["id"].as_str().map(|s| s.to_string());
        info!(to = to, message_id = ?msg_id, "WhatsApp text message sent");
        Ok(SendResult {
            success: true,
            message_id: msg_id,
            error: None,
        })
    } else {
        let error_msg = payload["error"]["message"]
            .as_str()
            .unwrap_or("unknown error")
            .to_string();
        let error_code = payload["error"]["code"].as_i64().unwrap_or(0);
        warn!(
            to = to,
            error_code = error_code,
            error = %error_msg,
            "WhatsApp text message failed"
        );
        Ok(SendResult {
            success: false,
            message_id: None,
            error: Some(format!("WhatsApp API error {error_code}: {error_msg}")),
        })
    }
}

/// Send a template message via WhatsApp Cloud API v23.
///
/// Template messages are used for transactional notifications (invoices,
/// installation updates, OTP) and require pre-approved templates in the
/// WhatsApp Business Account.
pub async fn send_template(
    client: &Client,
    config: &WhatsAppConfig,
    to: &str,
    template: &TemplateMessage,
) -> Result<SendResult, AppError> {
    let url = format!("{}/{}/messages", WHATSAPP_API_BASE, WHATSAPP_API_VERSION);

    let params: Vec<serde_json::Value> = template
        .parameters
        .iter()
        .map(|p| {
            serde_json::json!({
                "type": p.r#type,
                "text": p.text
            })
        })
        .collect();

    let body = serde_json::json!({
        "messaging_product": "whatsapp",
        "recipient_type": "individual",
        "to": to,
        "type": "template",
        "template": {
            "name": template.name,
            "language": {
                "code": template.language_code
            },
            "components": [],
            "params": params
        }
    });

    let response = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", config.access_token))
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await
        .map_err(|e| AppError::External(format!("WhatsApp API request failed: {e}")))?;

    let status = response.status();
    let payload: serde_json::Value = response
        .json()
        .await
        .map_err(|e| AppError::External(format!("WhatsApp API response parse error: {e}")))?;

    if status.is_success() {
        let msg_id = payload["messages"][0]["id"].as_str().map(|s| s.to_string());
        info!(to = to, template = %template.name, message_id = ?msg_id, "WhatsApp template message sent");
        Ok(SendResult {
            success: true,
            message_id: msg_id,
            error: None,
        })
    } else {
        let error_msg = payload["error"]["message"]
            .as_str()
            .unwrap_or("unknown error")
            .to_string();
        let error_code = payload["error"]["code"].as_i64().unwrap_or(0);
        warn!(
            to = to,
            template = %template.name,
            error_code = error_code,
            error = %error_msg,
            "WhatsApp template message failed"
        );
        Ok(SendResult {
            success: false,
            message_id: None,
            error: Some(format!("WhatsApp API error {error_code}: {error_msg}")),
        })
    }
}

/// Send an OTP via WhatsApp using a pre-approved template.
///
/// The template name should be something like "otp_message" with a single
/// parameter for the OTP code.
pub async fn send_otp(
    client: &Client,
    config: &WhatsAppConfig,
    to: &str,
    otp_code: &str,
    template_name: Option<&str>,
    language_code: Option<&str>,
) -> Result<SendResult, AppError> {
    let tmpl = TemplateMessage {
        name: template_name.unwrap_or("otp_message").to_string(),
        language_code: language_code.unwrap_or("en").to_string(),
        parameters: vec![TemplateParam {
            r#type: "text".to_string(),
            text: otp_code.to_string(),
        }],
    };
    send_template(client, config, to, &tmpl).await
}

/// Send a free-form notification message via WhatsApp.
///
/// Uses the text message type for immediate delivery without template approval.
pub async fn send_notification(
    client: &Client,
    config: &WhatsAppConfig,
    to: &str,
    subject: &str,
    body: &str,
) -> Result<SendResult, AppError> {
    let text = format!("*{}*\n\n{}", escape_md(subject), escape_md(body));
    send_text(client, config, to, &text).await
}

/// Escape MarkdownV2 special characters for WhatsApp bold/italic formatting.
fn escape_md(s: &str) -> String {
    let specials = ['_', '*', '[', ']', '(', ')', '~', '`', '>', '#', '+', '-', '=', '|', '{', '}', '.', '!'];
    let mut result = String::with_capacity(s.len() * 2);
    for c in s.chars() {
        if c == '\\' {
            result.push('\\');
            result.push('\\');
        } else if specials.contains(&c) {
            result.push('\\');
            result.push(c);
        } else {
            result.push(c);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_from_json() {
        let json = serde_json::json!({
            "access_token": "EAAG...",
            "phone_number_id": "1234567890"
        });
        let config = WhatsAppConfig::from_json(&json).unwrap();
        assert_eq!(config.access_token, "EAAG...");
        assert_eq!(config.phone_number_id, "1234567890");
        assert!(config.business_account_id.is_none());
    }

    #[test]
    fn test_config_missing_fields() {
        let json = serde_json::json!({});
        assert!(WhatsAppConfig::from_json(&json).is_err());
    }

    #[test]
    fn test_escape_md() {
        assert_eq!(escape_md("a.b"), "a\\.b");
        assert_eq!(escape_md("hello!"), "hello\\!");
        assert_eq!(escape_md("no_specials"), "no_specials");
    }
}
