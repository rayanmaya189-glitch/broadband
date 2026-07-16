use crate::modules::gateway::domain::value_objects::{ApiKeyId, ApiKeyStatus};

/// ApiKey aggregate root - represents an API key for gateway access
#[derive(Debug, Clone)]
pub struct ApiKey {
    pub id: ApiKeyId,
    pub name: String,
    pub key_hash: String,
    pub key_prefix: String,
    pub permissions: String,
    pub rate_limit_override: Option<i32>,
    pub is_active: bool,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    pub status: ApiKeyStatus,
}

/// Domain errors for ApiKey aggregate
#[derive(Debug, Clone, PartialEq)]
pub enum GatewayDomainError {
    ApiKeyNotFound(i64),
    ApiKeyExpired,
    ApiKeyRevoked,
    InsufficientPermissions,
}

impl std::fmt::Display for GatewayDomainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ApiKeyNotFound(id) => write!(f, "API key {} not found", id),
            Self::ApiKeyExpired => write!(f, "API key has expired"),
            Self::ApiKeyRevoked => write!(f, "API key has been revoked"),
            Self::InsufficientPermissions => write!(f, "Insufficient permissions"),
        }
    }
}

impl std::error::Error for GatewayDomainError {}

impl ApiKey {
    pub fn new(name: String, key_hash: String, key_prefix: String, permissions: String) -> Self {
        Self {
            id: ApiKeyId::new(0),
            name,
            key_hash,
            key_prefix,
            permissions,
            rate_limit_override: None,
            is_active: true,
            expires_at: None,
            status: ApiKeyStatus::Active,
        }
    }

    pub fn revoke(&mut self) {
        self.status = ApiKeyStatus::Revoked;
        self.is_active = false;
    }

    pub fn is_valid(&self) -> bool {
        self.is_active
            && self.status == ApiKeyStatus::Active
            && self.expires_at.map_or(true, |t| t > chrono::Utc::now())
    }

    pub fn has_permission(&self, permission: &str) -> bool {
        self.permissions.contains(permission) || self.permissions.contains("*")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_api_key() {
        let key = ApiKey::new("Test".to_string(), "hash".to_string(), "ak_".to_string(), "device.*".to_string());
        assert!(key.is_valid());
    }

    #[test]
    fn test_revoke_key() {
        let mut key = ApiKey::new("Test".to_string(), "hash".to_string(), "ak_".to_string(), "*".to_string());
        key.revoke();
        assert!(!key.is_valid());
    }
}
