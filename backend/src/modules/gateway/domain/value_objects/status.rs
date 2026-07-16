use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ApiKeyStatus {
    Active,
    Revoked,
    Expired,
}

impl ApiKeyStatus {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "active" => Some(Self::Active), "revoked" => Some(Self::Revoked), "expired" => Some(Self::Expired), _ => None,
        }
    }
    pub fn as_str(&self) -> &'static str {
        match self { Self::Active => "active", Self::Revoked => "revoked", Self::Expired => "expired" }
    }
}

impl fmt::Display for ApiKeyStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", self.as_str()) }
}
