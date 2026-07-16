use serde::{Deserialize, Serialize};
use std::fmt;

/// Network entity status value object
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VlanStatus {
    Active,
    Inactive,
}

impl VlanStatus {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "active" => Some(Self::Active),
            "inactive" => Some(Self::Inactive),
            _ => None,
        }
    }
    pub fn as_str(&self) -> &'static str {
        match self { Self::Active => "active", Self::Inactive => "inactive" }
    }
}

impl fmt::Display for VlanStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", self.as_str()) }
}

/// PPPoE session status
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PppoeSessionStatus {
    Active,
    Inactive,
    Terminated,
}

impl PppoeSessionStatus {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "active" => Some(Self::Active),
            "inactive" => Some(Self::Inactive),
            "terminated" => Some(Self::Terminated),
            _ => None,
        }
    }
    pub fn as_str(&self) -> &'static str {
        match self { Self::Active => "active", Self::Inactive => "inactive", Self::Terminated => "terminated" }
    }
}

impl fmt::Display for PppoeSessionStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", self.as_str()) }
}
