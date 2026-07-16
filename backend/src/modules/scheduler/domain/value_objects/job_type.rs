use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum JobType {
    Billing,
    Notification,
    DeviceSync,
    Bandwidth,
    Cleanup,
    Custom(String),
}

impl JobType {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "billing" => Self::Billing, "notification" => Self::Notification,
            "device_sync" => Self::DeviceSync, "bandwidth" => Self::Bandwidth,
            "cleanup" => Self::Cleanup, _ => Self::Custom(s.to_string()),
        }
    }
    pub fn as_str(&self) -> &str {
        match self {
            Self::Billing => "billing", Self::Notification => "notification", Self::DeviceSync => "device_sync",
            Self::Bandwidth => "bandwidth", Self::Cleanup => "cleanup", Self::Custom(s) => s,
        }
    }
}

impl fmt::Display for JobType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", self.as_str()) }
}
