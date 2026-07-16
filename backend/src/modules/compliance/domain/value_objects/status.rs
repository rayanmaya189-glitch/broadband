use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum KycStatus {
    Pending,
    Submitted,
    Verified,
    Rejected,
}

impl KycStatus {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "pending" => Some(Self::Pending),
            "submitted" => Some(Self::Submitted),
            "verified" => Some(Self::Verified),
            "rejected" => Some(Self::Rejected),
            _ => None,
        }
    }
    pub fn as_str(&self) -> &'static str {
        match self { Self::Pending => "pending", Self::Submitted => "submitted", Self::Verified => "verified", Self::Rejected => "rejected" }
    }
    pub fn is_verified(&self) -> bool { matches!(self, Self::Verified) }
}

impl fmt::Display for KycStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", self.as_str()) }
}
