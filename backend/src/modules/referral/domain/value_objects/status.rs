use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ReferralStatus {
    Pending,
    Activated,
    Rewarded,
}

impl ReferralStatus {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "pending" => Some(Self::Pending), "activated" => Some(Self::Activated), "rewarded" => Some(Self::Rewarded), _ => None,
        }
    }
    pub fn as_str(&self) -> &'static str {
        match self { Self::Pending => "pending", Self::Activated => "activated", Self::Rewarded => "rewarded" }
    }
}

impl fmt::Display for ReferralStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", self.as_str()) }
}
