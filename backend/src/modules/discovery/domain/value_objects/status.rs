use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DiscoveryScanStatus {
    Idle,
    Running,
    Failed,
}

impl DiscoveryScanStatus {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "idle" => Some(Self::Idle), "running" => Some(Self::Running), "failed" => Some(Self::Failed), _ => None,
        }
    }
    pub fn as_str(&self) -> &'static str {
        match self { Self::Idle => "idle", Self::Running => "running", Self::Failed => "failed" }
    }
}

impl fmt::Display for DiscoveryScanStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", self.as_str()) }
}
