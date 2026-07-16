use serde::{Deserialize, Serialize};
use std::fmt;

/// Device status value object
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DeviceStatus {
    Online,
    Offline,
    Degraded,
    Maintenance,
    Decommissioned,
}

impl DeviceStatus {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "online" => Some(Self::Online),
            "offline" => Some(Self::Offline),
            "degraded" => Some(Self::Degraded),
            "maintenance" => Some(Self::Maintenance),
            "decommissioned" => Some(Self::Decommissioned),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Online => "online",
            Self::Offline => "offline",
            Self::Degraded => "degraded",
            Self::Maintenance => "maintenance",
            Self::Decommissioned => "decommissioned",
        }
    }

    pub fn is_operational(&self) -> bool {
        matches!(self, Self::Online | Self::Degraded)
    }
}

impl fmt::Display for DeviceStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", self.as_str()) }
}
