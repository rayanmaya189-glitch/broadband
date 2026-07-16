use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BandwidthStatus {
    Active,
    Inactive,
    Applying,
    Failed,
}

impl BandwidthStatus {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "active" => Some(Self::Active),
            "inactive" => Some(Self::Inactive),
            "applying" => Some(Self::Applying),
            "failed" => Some(Self::Failed),
            _ => None,
        }
    }
    pub fn as_str(&self) -> &'static str {
        match self { Self::Active => "active", Self::Inactive => "inactive", Self::Applying => "applying", Self::Failed => "failed" }
    }
}

impl fmt::Display for BandwidthStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", self.as_str()) }
}
