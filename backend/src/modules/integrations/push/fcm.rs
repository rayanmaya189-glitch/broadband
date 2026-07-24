//! Firebase Cloud Messaging (FCM) Push Adapter
//!
//! Sends push notifications to mobile apps via FCM HTTP v1 API:
//! - Single device messaging
//! - Topic-based messaging
//! - Priority and TTL configuration
//! - Delivery status tracking
//!
//! API Reference: https://firebase.google.com/docs/reference/fcm/v1

use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::{debug, info, warn};

use crate::shared::errors::AppError;

// ============================================================================
// Configuration
// ============================================================================

/// FCM adapter configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FcmConfig {
    /// Firebase service account key (JSON)
    pub service_account_key: String,
    /// FCM project ID
    pub project_id: String,
    /// FCM API URL
    pub api_url: String,
}

impl Default for FcmConfig {
    fn default() -> Self {
        Self {
            service_account_key: std::env::var("FCM_SERVICE_ACCOUNT_KEY").unwrap_or_default(),
            project_id: std::env::var("FCM_PROJECT_ID").unwrap_or_default(),
            api_url: std::env::var("FCM_API_URL")
                .unwrap_or_else(|_| "https://fcm.googleapis.com/v1/projects".to_string()),
        }
    }
}

impl FcmConfig {
    /// Create config from environment variables
    pub fn from_env() -> Self {
        Self::default()
    }
}

// ============================================================================
// Data Types
// ============================================================================

/// FCM message request (HTTP v1 format)
#[derive(Debug, Clone, Serialize)]
pub struct FcmMessageRequest {
    pub message: FcmMessage,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validate_only: Option<bool>,
}

/// FCM message body
#[derive(Debug, Clone, Serialize)]
pub struct FcmMessage {
    /// Device registration token or topic
    pub token: String,
    /// Notification payload (displayed when app is in foreground)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification: Option<FcmNotification>,
    /// Data payload (always delivered to app)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<std::collections::HashMap<String, String>>,
    /// Android-specific configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub android: Option<FcmAndroidConfig>,
    /// iOS-specific configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apns: Option<FcmApnsConfig>,
    /// Web-specific configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webpush: Option<FcmWebpushConfig>,
    /// TTL in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<String>,
    /// Collapse key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collapse_key: Option<String>,
}

/// Notification payload
#[derive(Debug, Clone, Serialize)]
pub struct FcmNotification {
    pub title: String,
    pub body: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
}

/// Android-specific config
#[derive(Debug, Clone, Serialize)]
pub struct FcmAndroidConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification: Option<FcmAndroidNotification>,
}

/// Android notification config
#[derive(Debug, Clone, Serialize)]
pub struct FcmAndroidNotification {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub click_action: Option<String>,
}

/// APNs (iOS) config
#[derive(Debug, Clone, Serialize)]
pub struct FcmApnsConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<serde_json::Value>,
}

/// Web push config
#[derive(Debug, Clone, Serialize)]
pub struct FcmWebpushConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<std::collections::HashMap<String, String>>,
}

/// FCM API response
#[derive(Debug, Clone, Deserialize)]
pub struct FcmResponse {
    #[serde(rename = "name")]
    pub message_id: Option<String>,
    #[serde(rename = "multicast_id")]
    pub multicast_id: Option<i64>,
    pub success: Option<i32>,
    pub failure: Option<i32>,
    #[serde(rename = "canonical_ids")]
    pub canonical_ids: Option<i32>,
    pub results: Option<Vec<FcmResult>>,
    pub error: Option<FcmError>,
}

/// Individual result per device
#[derive(Debug, Clone, Deserialize)]
pub struct FcmResult {
    pub message_id: Option<String>,
    pub registration_id: Option<String>,
    pub error: Option<String>,
}

/// FCM error
#[derive(Debug, Clone, Deserialize)]
pub struct FcmError {
    pub code: Option<i32>,
    pub message: Option<String>,
    #[serde(rename = "status")]
    pub status_str: Option<String>,
}

/// Push notification delivery status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PushDeliveryStatus {
    pub message_id: String,
    pub status: String,
    pub device_token: String,
    pub delivered: bool,
    pub delivered_at: Option<String>,
    pub error: Option<String>,
}

// ============================================================================
// FCM Adapter
// ============================================================================

/// Firebase Cloud Messaging adapter
pub struct FcmAdapter {
    config: FcmConfig,
    client: Client,
    /// Cached access token
    access_token: Option<String>,
}

impl FcmAdapter {
    /// Create a new FCM adapter
    pub fn new(config: FcmConfig) -> Self {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .unwrap_or_default();

        Self {
            config,
            client,
            access_token: None,
        }
    }

    /// Create adapter from environment variables
    pub fn from_env() -> Self {
        Self::new(FcmConfig::default())
    }

    /// Check if the adapter is configured
    pub fn is_configured(&self) -> bool {
        !self.config.service_account_key.is_empty() && !self.config.project_id.is_empty()
    }

    /// Get OAuth2 access token from service account key
    ///
    /// Uses the service account's private key to sign a JWT assertion,
    /// then exchanges it for a Google OAuth2 access token.
    ///
    /// NOTE: For production, add the `jsonwebtoken` crate and replace
    /// the signing logic with proper RS256 JWT creation.
    async fn get_access_token(&mut self) -> Result<String, AppError> {
        if let Some(ref token) = self.access_token {
            return Ok(token.clone());
        }

        // Parse service account key
        let sa_key: serde_json::Value = serde_json::from_str(&self.config.service_account_key)
            .map_err(|e| {
                AppError::Internal(anyhow::anyhow!("Invalid service account key: {}", e))
            })?;

        let client_email = sa_key["client_email"].as_str().ok_or_else(|| {
            AppError::Internal(anyhow::anyhow!("Missing client_email in service account"))
        })?;

        // Build JWT claims for OAuth2 token exchange
        let now = chrono::Utc::now().timestamp();
        let exp = now + 3600;
        let scope = "https://www.googleapis.com/auth/firebase.messaging";

        // Create JWT header and claims
        let header = serde_json::json!({
            "alg": "RS256",
            "typ": "JWT"
        });
        let claims = serde_json::json!({
            "iss": client_email,
            "scope": scope,
            "aud": "https://oauth2.googleapis.com/token",
            "iat": now,
            "exp": exp
        });

        // Base64url encode header and claims
        let header_b64 = base64url_encode(&serde_json::to_vec(&header).unwrap());
        let claims_b64 = base64url_encode(&serde_json::to_vec(&claims).unwrap());
        let signing_input = format!("{}.{}", header_b64, claims_b64);

        // Sign with private key
        let signature = create_jwt_signature(&signing_input, &self.config.service_account_key)?;
        let jwt = format!("{}.{}", signing_input, signature);

        // Exchange JWT for access token
        let response = self
            .client
            .post("https://oauth2.googleapis.com/token")
            .form(&[
                ("grant_type", "urn:ietf:params:oauth:grant-type:jwt-bearer"),
                ("assertion", &jwt),
            ])
            .send()
            .await
            .map_err(|e| AppError::External(format!("Token exchange failed: {}", e)))?;

        if !response.status().is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(AppError::External(format!(
                "Token exchange error: {}",
                body
            )));
        }

        let token_response: serde_json::Value = response
            .json()
            .await
            .map_err(|e| AppError::External(format!("Failed to parse token response: {}", e)))?;

        let access_token = token_response["access_token"]
            .as_str()
            .ok_or_else(|| AppError::Internal(anyhow::anyhow!("Missing access_token in response")))?
            .to_string();

        self.access_token = Some(access_token.clone());
        Ok(access_token)
    }

    /// Invalidate the cached access token (call on 401 errors)
    pub fn invalidate_token(&mut self) {
        self.access_token = None;
    }

    /// Send a push notification to a single device
    pub async fn send_push(
        &mut self,
        device_token: &str,
        title: &str,
        body: &str,
        data: Option<std::collections::HashMap<String, String>>,
    ) -> Result<PushDeliveryStatus, AppError> {
        let access_token = self.get_access_token().await?;

        let request = FcmMessageRequest {
            message: FcmMessage {
                token: device_token.to_string(),
                notification: Some(FcmNotification {
                    title: title.to_string(),
                    body: body.to_string(),
                    image: None,
                }),
                data,
                android: Some(FcmAndroidConfig {
                    priority: Some("high".to_string()),
                    ttl: Some("86400s".to_string()),
                    notification: Some(FcmAndroidNotification {
                        channel_id: Some("aeroxe_notifications".to_string()),
                        icon: Some("ic_notification".to_string()),
                        color: Some("#4CAF50".to_string()),
                        click_action: None,
                    }),
                }),
                apns: None,
                webpush: None,
                ttl: Some("86400".to_string()),
                collapse_key: None,
            },
            validate_only: None,
        };

        let url = format!(
            "{}/{}/messages:send",
            self.config.api_url, self.config.project_id
        );

        debug!(device_token = %device_token, "Sending FCM push notification");

        let response = self
            .client
            .post(&url)
            .bearer_auth(&access_token)
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| AppError::External(format!("FCM API request failed: {}", e)))?;

        if response.status().as_u16() == 401 {
            // Token expired, invalidate and retry once
            self.invalidate_token();
            let access_token = self.get_access_token().await?;
            let response = self
                .client
                .post(&url)
                .bearer_auth(&access_token)
                .header("Content-Type", "application/json")
                .json(&request)
                .send()
                .await
                .map_err(|e| AppError::External(format!("FCM API retry failed: {}", e)))?;

            if !response.status().is_success() {
                let status = response.status();
                let body = response.text().await.unwrap_or_default();
                return Err(AppError::External(format!(
                    "FCM API error ({}): {}",
                    status, body
                )));
            }

            let result: FcmResponse = response
                .json()
                .await
                .map_err(|e| AppError::External(format!("Failed to parse FCM response: {}", e)))?;

            let message_id = result.message_id.unwrap_or_default();
            info!(device_token = %device_token, message_id = %message_id, "Sent FCM push notification (after token refresh)");

            return Ok(PushDeliveryStatus {
                message_id,
                status: "sent".to_string(),
                device_token: device_token.to_string(),
                delivered: true,
                delivered_at: Some(chrono::Utc::now().to_rfc3339()),
                error: None,
            });
        }

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            warn!(status = %status, body = %body, "FCM push notification failed");
            return Err(AppError::External(format!(
                "FCM API error ({}): {}",
                status, body
            )));
        }

        let result: FcmResponse = response
            .json()
            .await
            .map_err(|e| AppError::External(format!("Failed to parse FCM response: {}", e)))?;

        if let Some(err) = result.error {
            let msg = err.message.unwrap_or_else(|| "Unknown error".to_string());
            return Err(AppError::External(format!("FCM error: {}", msg)));
        }

        let message_id = result.message_id.unwrap_or_default();

        info!(device_token = %device_token, message_id = %message_id, "Sent FCM push notification");

        Ok(PushDeliveryStatus {
            message_id,
            status: "sent".to_string(),
            device_token: device_token.to_string(),
            delivered: true,
            delivered_at: Some(chrono::Utc::now().to_rfc3339()),
            error: None,
        })
    }

    /// Send a push notification to a topic
    pub async fn send_to_topic(
        &mut self,
        topic: &str,
        title: &str,
        body: &str,
        data: Option<std::collections::HashMap<String, String>>,
    ) -> Result<PushDeliveryStatus, AppError> {
        let access_token = self.get_access_token().await?;

        let request = FcmMessageRequest {
            message: FcmMessage {
                token: format!("/topics/{}", topic),
                notification: Some(FcmNotification {
                    title: title.to_string(),
                    body: body.to_string(),
                    image: None,
                }),
                data,
                android: Some(FcmAndroidConfig {
                    priority: Some("high".to_string()),
                    ttl: Some("86400s".to_string()),
                    notification: Some(FcmAndroidNotification {
                        channel_id: Some("aeroxe_notifications".to_string()),
                        icon: Some("ic_notification".to_string()),
                        color: Some("#4CAF50".to_string()),
                        click_action: None,
                    }),
                }),
                apns: None,
                webpush: None,
                ttl: Some("86400".to_string()),
                collapse_key: None,
            },
            validate_only: None,
        };

        let url = format!(
            "{}/{}/messages:send",
            self.config.api_url, self.config.project_id
        );

        debug!(topic = %topic, "Sending FCM push notification to topic");

        let response = self
            .client
            .post(&url)
            .bearer_auth(&access_token)
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| AppError::External(format!("FCM API request failed: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            warn!(status = %status, body = %body, "FCM topic notification failed");
            return Err(AppError::External(format!(
                "FCM API error ({}): {}",
                status, body
            )));
        }

        let result: FcmResponse = response
            .json()
            .await
            .map_err(|e| AppError::External(format!("Failed to parse FCM response: {}", e)))?;

        let message_id = result.message_id.unwrap_or_default();

        info!(topic = %topic, message_id = %message_id, "Sent FCM notification to topic");

        Ok(PushDeliveryStatus {
            message_id,
            status: "sent".to_string(),
            device_token: format!("/topics/{}", topic),
            delivered: true,
            delivered_at: Some(chrono::Utc::now().to_rfc3339()),
            error: None,
        })
    }
}

/// Base64url encode bytes (RFC 4648 §5)
fn base64url_encode(data: &[u8]) -> String {
    use base64::Engine;
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(data)
}

/// Create a JWT RS256 signature for OAuth2 token exchange
///
/// Uses the `jsonwebtoken` crate to sign the JWT with the service account's RSA private key.
fn create_jwt_signature(signing_input: &str, service_account_key: &str) -> Result<String, AppError> {
    use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};

    let sa_key: serde_json::Value = serde_json::from_str(service_account_key)
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Invalid service account key for signing: {}", e)))?;

    let private_key_pem = sa_key["private_key"]
        .as_str()
        .ok_or_else(|| AppError::Internal(anyhow::anyhow!("Missing private_key in service account")))?;

    // Decode the PEM to DER if it's PEM-wrapped
    let der_key = if private_key_pem.contains("BEGIN") {
        let pem_lines: String = private_key_pem
            .lines()
            .filter(|l| !l.starts_with("-----"))
            .collect();
        use base64::Engine;
        base64::engine::general_purpose::STANDARD
            .decode(&pem_lines)
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to decode PEM key: {}", e)))?
    } else {
        use base64::Engine;
        base64::engine::general_purpose::STANDARD
            .decode(private_key_pem)
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to decode base64 key: {}", e)))?
    };

    let header = Header::new(Algorithm::RS256);

    // Parse the signing_input (header_b64.claims_b64) to extract claims
    let parts: Vec<&str> = signing_input.split('.').collect();
    let claims_b64 = parts.get(1).unwrap_or(&"");

    let claims_value: serde_json::Value = {
        use base64::Engine;
        let decoded = base64::engine::general_purpose::URL_SAFE_NO_PAD
            .decode(*claims_b64)
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to decode claims: {}", e)))?;
        serde_json::from_slice(&decoded)
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to parse claims: {}", e)))?
    };

    let key = EncodingKey::from_rsa_der(&der_key);
    let token = encode(&header, &claims_value, &key)
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to sign JWT: {}", e)))?;

    // Return only the signature part (third segment)
    let signature = token.split('.').nth(2).unwrap_or("").to_string();
    Ok(signature)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_configured() {
        let adapter = FcmAdapter::from_env();
        assert!(!adapter.is_configured());
    }

    #[test]
    fn test_base64url_encode() {
        let encoded = base64url_encode(b"hello");
        assert_eq!(encoded, "aGVsbG8");
    }

    #[test]
    fn test_base64url_encode_binary() {
        // Ensure padding-free encoding
        let data = vec![0u8; 32];
        let encoded = base64url_encode(&data);
        assert!(!encoded.contains('='));
    }
}
