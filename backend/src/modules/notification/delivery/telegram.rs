//! Telegram Bot API delivery service.
//!
//! Sends messages via the Telegram Bot API (https://core.telegram.org/bots/api).
//! Config stored in `notification_channels.config` as:
//! ```json
//! { "bot_token": "123456:ABC-DEF..." }
//! ```

use reqwest::Client;
use tracing::{info, warn};

use crate::common::errors::app_error::AppError;

/// Telegram-specific configuration extracted from the channel config JSON.
#[derive(Debug, Clone)]
pub struct TelegramConfig {
    pub bot_token: String,
}

impl TelegramConfig {
    /// Parse from the channel config JSON stored in the database.
    pub fn from_json(config: &serde_json::Value) -> Result<Self, AppError> {
        let bot_token = config
            .get("bot_token")
            .and_then(|v| v.as_str())
            .ok_or_else(|| AppError::Validation("Telegram config missing 'bot_token'".into()))?
            .to_string();

        if bot_token.is_empty() {
            return Err(AppError::Validation("Telegram bot_token is empty".into()));
        }

        Ok(Self { bot_token })
    }

    fn api_base(&self) -> String {
        format!("https://api.telegram.org/bot{}", self.bot_token)
    }
}

/// Result of sending a Telegram message.
#[derive(Debug)]
pub struct SendResult {
    pub success: bool,
    pub message_id: Option<i64>,
    pub error: Option<String>,
}

/// Send a text message via Telegram Bot API.
///
/// `chat_id` can be a numeric Telegram user/group ID or a `@channelusername`.
pub async fn send_message(
    client: &Client,
    config: &TelegramConfig,
    chat_id: &str,
    text: &str,
    parse_mode: Option<&str>,
) -> Result<SendResult, AppError> {
    let url = format!("{}/sendMessage", config.api_base());

    let mut body = serde_json::json!({
        "chat_id": chat_id,
        "text": text,
    });

    if let Some(mode) = parse_mode {
        body["parse_mode"] = serde_json::json!(mode);
    }

    let response = client
        .post(&url)
        .json(&body)
        .send()
        .await
        .map_err(|e| AppError::External(format!("Telegram API request failed: {e}")))?;

    let status = response.status();
    let payload: serde_json::Value = response
        .json()
        .await
        .map_err(|e| AppError::External(format!("Telegram API response parse error: {e}")))?;

    if status.is_success() && payload["ok"].as_bool() == Some(true) {
        let msg_id = payload["result"]["message_id"].as_i64();
        info!(
            chat_id = chat_id,
            message_id = msg_id,
            "Telegram message sent successfully"
        );
        Ok(SendResult {
            success: true,
            message_id: msg_id,
            error: None,
        })
    } else {
        let error_code = payload["error_code"].as_i64().unwrap_or(0);
        let description = payload["description"]
            .as_str()
            .unwrap_or("unknown error")
            .to_string();
        warn!(
            chat_id = chat_id,
            error_code = error_code,
            description = %description,
            "Telegram message failed"
        );
        Ok(SendResult {
            success: false,
            message_id: None,
            error: Some(format!("Telegram API error {error_code}: {description}")),
        })
    }
}

/// Send an OTP code via Telegram as a plain text message.
///
/// This is a convenience wrapper around `send_message` that formats
/// the OTP message in a user-friendly way.
pub async fn send_otp(
    client: &Client,
    config: &TelegramConfig,
    chat_id: &str,
    otp_code: &str,
    app_name: Option<&str>,
) -> Result<SendResult, AppError> {
    let app = app_name.unwrap_or("AeroXe");
    let text = format!(
        "🔐 Your {} verification code is:\n\n{}\n\nThis code expires in 5 minutes. Do not share it with anyone.",
        app, otp_code
    );
    send_message(client, config, chat_id, &text, None).await
}

/// Send a rich notification with optional HTML formatting via Telegram.
///
/// Supports HTML parse mode for bold, italic, links, and code blocks.
pub async fn send_rich_notification(
    client: &Client,
    config: &TelegramConfig,
    chat_id: &str,
    subject: &str,
    body: &str,
) -> Result<SendResult, AppError> {
    let text = format!("<b>{}</b>\n\n{}", escape_html(subject), escape_html(body));
    send_message(client, config, chat_id, &text, Some("HTML")).await
}

/// Send a plain text message via Telegram Bot API.
///
/// Convenience wrapper around `send_message` with no parse mode.
pub async fn send_text(
    client: &Client,
    config: &TelegramConfig,
    chat_id: &str,
    text: &str,
) -> Result<SendResult, AppError> {
    send_message(client, config, chat_id, text, None).await
}

/// Escape special HTML characters for Telegram HTML parse mode.
fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escape_html() {
        assert_eq!(escape_html("a < b & c > d"), "a &lt; b &amp; c &gt; d");
        assert_eq!(escape_html("no special chars"), "no special chars");
    }

    #[test]
    fn test_config_from_json() {
        let json = serde_json::json!({ "bot_token": "123456:ABC" });
        let config = TelegramConfig::from_json(&json).unwrap();
        assert_eq!(config.bot_token, "123456:ABC");
    }

    #[test]
    fn test_config_missing_token() {
        let json = serde_json::json!({});
        assert!(TelegramConfig::from_json(&json).is_err());
    }
}
