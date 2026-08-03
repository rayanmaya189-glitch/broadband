//! Telegram Bot Adapter
//!
//! Sends messages through a Telegram bot for OTP login and notifications:
//! - Direct messages to a bound chat_id (sendMessage)
//! - Webhook registration so the bot can receive /start /login <phone> binds
//!
//! API Reference: https://core.telegram.org/bots/api

use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::{debug, info, warn};

use crate::shared::errors::AppError;

// ============================================================================
// Configuration
// ============================================================================

/// Telegram bot configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelegramConfig {
    /// Bot token from BotFather
    pub bot_token: String,
    /// Telegram API base URL
    pub api_url: String,
}

impl Default for TelegramConfig {
    fn default() -> Self {
        Self {
            bot_token: std::env::var("TELEGRAM_BOT_TOKEN").unwrap_or_default(),
            api_url: std::env::var("TELEGRAM_API_URL")
                .unwrap_or_else(|_| "https://api.telegram.org".to_string()),
        }
    }
}

// ============================================================================
// Data Types
// ============================================================================

/// Outbound message request
#[derive(Debug, Clone, Serialize)]
pub struct TelegramSendRequest {
    pub chat_id: i64,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
}

/// Telegram API response envelope
#[derive(Debug, Clone, Deserialize)]
pub struct TelegramApiResponse<T> {
    pub ok: bool,
    pub result: Option<T>,
    pub description: Option<String>,
    pub error_code: Option<i32>,
}

/// Result of a successful sendMessage call
#[derive(Debug, Clone, Deserialize)]
pub struct TelegramSentMessage {
    pub message_id: i64,
    pub chat: Option<TelegramChat>,
    pub text: Option<String>,
}

/// Chat info
#[derive(Debug, Clone, Deserialize)]
pub struct TelegramChat {
    pub id: i64,
    #[serde(rename = "type")]
    pub chat_type: Option<String>,
}

/// Incoming webhook Update payload
#[derive(Debug, Clone, Deserialize)]
pub struct TelegramUpdate {
    pub update_id: i64,
    pub message: Option<TelegramInboundMessage>,
}

/// Inbound message from a user
#[derive(Debug, Clone, Deserialize)]
pub struct TelegramInboundMessage {
    pub message_id: i64,
    pub chat: TelegramChat,
    pub text: Option<String>,
    pub from: Option<TelegramUser>,
}

/// Telegram user info
#[derive(Debug, Clone, Deserialize)]
pub struct TelegramUser {
    pub id: i64,
    pub first_name: Option<String>,
    pub username: Option<String>,
}

// ============================================================================
// Telegram Bot Adapter
// ============================================================================

/// Telegram bot adapter
pub struct TelegramBotAdapter {
    config: TelegramConfig,
    client: Client,
}

impl TelegramBotAdapter {
    /// Create a new adapter
    pub fn new(config: TelegramConfig) -> Self {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .unwrap_or_default();

        Self { config, client }
    }

    /// Create adapter from environment variables
    pub fn from_env() -> Self {
        Self::new(TelegramConfig::default())
    }

    /// Check if the adapter is configured (has a bot token)
    pub fn is_configured(&self) -> bool {
        !self.config.bot_token.is_empty()
    }

    /// Send a text message to a chat
    pub async fn send_message(
        &self,
        chat_id: i64,
        text: &str,
    ) -> Result<TelegramSentMessage, AppError> {
        let url = format!(
            "{}/bot{}/sendMessage",
            self.config.api_url, self.config.bot_token
        );
        let request = TelegramSendRequest {
            chat_id,
            text: text.to_string(),
            parse_mode: Some("HTML".to_string()),
        };

        debug!(chat_id = chat_id, "Sending Telegram message");

        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| AppError::External(format!("Telegram API request failed: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            warn!(status = %status, body = %body, "Telegram message send failed");
            return Err(AppError::External(format!(
                "Telegram API error ({}): {}",
                status, body
            )));
        }

        let result: TelegramApiResponse<TelegramSentMessage> = response
            .json()
            .await
            .map_err(|e| AppError::External(format!("Failed to parse Telegram response: {}", e)))?;

        if !result.ok {
            let msg = result
                .description
                .unwrap_or_else(|| "Unknown Telegram error".to_string());
            warn!(error = %msg, code = ?result.error_code, "Telegram API returned error");
            return Err(AppError::External(format!("Telegram error: {}", msg)));
        }

        let message = result
            .result
            .ok_or_else(|| AppError::External("Telegram returned no result".to_string()))?;

        info!(
            chat_id = chat_id,
            message_id = message.message_id,
            "Sent Telegram message"
        );
        Ok(message)
    }

    /// Register a webhook URL for incoming bot updates
    pub async fn set_webhook(&self, url: &str) -> Result<bool, AppError> {
        let endpoint = format!(
            "{}/bot{}/setWebhook",
            self.config.api_url, self.config.bot_token
        );

        let response = self
            .client
            .post(&endpoint)
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({ "url": url }))
            .send()
            .await
            .map_err(|e| AppError::External(format!("Telegram API request failed: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(AppError::External(format!(
                "Telegram webhook error ({}): {}",
                status, body
            )));
        }

        let result: TelegramApiResponse<serde_json::Value> = response
            .json()
            .await
            .map_err(|e| AppError::External(format!("Failed to parse Telegram response: {}", e)))?;

        if !result.ok {
            return Err(AppError::External(
                result
                    .description
                    .unwrap_or_else(|| "Unknown Telegram error".to_string()),
            ));
        }

        Ok(true)
    }

    /// Remove the registered webhook
    pub async fn delete_webhook(&self) -> Result<bool, AppError> {
        let endpoint = format!(
            "{}/bot{}/deleteWebhook",
            self.config.api_url, self.config.bot_token
        );

        let response = self
            .client
            .post(&endpoint)
            .send()
            .await
            .map_err(|e| AppError::External(format!("Telegram API request failed: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(AppError::External(format!(
                "Telegram webhook error ({}): {}",
                status, body
            )));
        }

        let result: TelegramApiResponse<serde_json::Value> = response
            .json()
            .await
            .map_err(|e| AppError::External(format!("Failed to parse Telegram response: {}", e)))?;

        Ok(result.ok)
    }
}

/// Parse an incoming webhook update from raw JSON
pub fn parse_update(payload: &str) -> Option<TelegramUpdate> {
    serde_json::from_str(payload).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_configured() {
        let adapter = TelegramBotAdapter::from_env();
        assert!(!adapter.is_configured());
    }

    #[test]
    fn test_parse_update() {
        let payload = r#"{"update_id":1,"message":{"message_id":10,"chat":{"id":42,"type":"private"},"text":"/login 9876543210","from":{"id":42,"first_name":"Test"}}}"#;
        let update = parse_update(payload).expect("update should parse");
        assert_eq!(update.update_id, 1);
        let msg = update.message.expect("message present");
        assert_eq!(msg.chat.id, 42);
        assert_eq!(msg.text.as_deref(), Some("/login 9876543210"));
    }
}
