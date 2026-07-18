use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NotificationStatus {
    Queued,
    Sent,
    Delivered,
    Failed,
    Retrying,
}

impl NotificationStatus {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "queued" => Some(Self::Queued),
            "sent" => Some(Self::Sent),
            "delivered" => Some(Self::Delivered),
            "failed" => Some(Self::Failed),
            "retrying" => Some(Self::Retrying),
            _ => None,
        }
    }
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Queued => "queued",
            Self::Sent => "sent",
            Self::Delivered => "delivered",
            Self::Failed => "failed",
            Self::Retrying => "retrying",
        }
    }
    pub fn is_terminal(&self) -> bool {
        matches!(self, Self::Delivered | Self::Failed)
    }
}

impl fmt::Display for NotificationStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
