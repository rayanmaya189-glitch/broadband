//! API key validator.
//!
//! Validates API keys from external integrations.

use serde::{Deserialize, Serialize};

/// API key validation errors.
#[derive(Debug, Clone)]
pub enum ApiKeyError {
    InvalidKey,
    ExpiredKey,
    RevokedKey,
    InsufficientPermissions,
}

impl std::fmt::Display for ApiKeyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiKeyError::InvalidKey => write!(f, "Invalid API key"),
            ApiKeyError::ExpiredKey => write!(f, "API key has expired"),
            ApiKeyError::RevokedKey => write!(f, "API key has been revoked"),
            ApiKeyError::InsufficientPermissions => write!(f, "Insufficient permissions for this API key"),
        }
    }
}

impl std::error::Error for ApiKeyError {}

/// API key metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyInfo {
    pub key_id: String,
    pub name: String,
    pub scopes: Vec<String>,
    pub rate_limit: u32,
}

/// API key validator.
pub struct ApiKeyValidator {
    // In production, this would connect to a database or cache
}

impl ApiKeyValidator {
    /// Create a new API key validator.
    pub fn new() -> Self {
        Self {}
    }

    /// Validate an API key.
    pub async fn validate(&self, key: &str) -> Result<ApiKeyInfo, ApiKeyError> {
        // In production, this would:
        // 1. Look up the key in Redis/database
        // 2. Check if it's revoked
        // 3. Check if it's expired
        // 4. Return the key metadata
        
        // Placeholder validation
        if key.is_empty() {
            return Err(ApiKeyError::InvalidKey);
        }

        // Demo key for testing
        if key == "demo-api-key-12345" {
            return Ok(ApiKeyInfo {
                key_id: "key_001".to_string(),
                name: "Demo API Key".to_string(),
                scopes: vec!["read".to_string(), "write".to_string()],
                rate_limit: 1000,
            });
        }

        Err(ApiKeyError::InvalidKey)
    }
}

impl Default for ApiKeyValidator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_valid_api_key() {
        let validator = ApiKeyValidator::new();
        let result = validator.validate("demo-api-key-12345").await;
        assert!(result.is_ok());
        let info = result.unwrap();
        assert_eq!(info.key_id, "key_001");
    }

    #[tokio::test]
    async fn test_invalid_api_key() {
        let validator = ApiKeyValidator::new();
        let result = validator.validate("invalid-key").await;
        assert!(matches!(result, Err(ApiKeyError::InvalidKey)));
    }

    #[tokio::test]
    async fn test_empty_api_key() {
        let validator = ApiKeyValidator::new();
        let result = validator.validate("").await;
        assert!(matches!(result, Err(ApiKeyError::InvalidKey)));
    }
}
