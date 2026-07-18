use serde::{Deserialize, Serialize};
use std::fmt;

/// Device type value object
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DeviceType {
    Olt,
    Ont,
    Router,
    Switch,
    AccessPoint,
}

impl DeviceType {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "olt" => Some(Self::Olt),
            "ont" => Some(Self::Ont),
            "router" => Some(Self::Router),
            "switch" => Some(Self::Switch),
            "access_point" | "ap" => Some(Self::AccessPoint),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Olt => "olt",
            Self::Ont => "ont",
            Self::Router => "router",
            Self::Switch => "switch",
            Self::AccessPoint => "access_point",
        }
    }
}

impl fmt::Display for DeviceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
