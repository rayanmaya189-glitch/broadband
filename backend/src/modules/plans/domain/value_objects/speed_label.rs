use serde::{Deserialize, Serialize};
use std::fmt;

/// Speed label value object representing plan speed category
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SpeedLabel(String);

impl SpeedLabel {
    pub fn new(label: &str) -> Result<Self, String> {
        if label.is_empty() || label.len() > 20 {
            return Err("Invalid speed label".to_string());
        }
        Ok(Self(label.to_string()))
    }

    pub fn from_mbps(download_mbps: i32) -> Self {
        Self(format!("{} Mbps", download_mbps))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn parse_mbps(&self) -> Option<i32> {
        self.0
            .split_whitespace()
            .next()
            .and_then(|s| s.parse::<i32>().ok())
    }
}

impl fmt::Display for SpeedLabel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
