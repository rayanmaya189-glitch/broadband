use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TicketPriority {
    Low,
    Medium,
    High,
    Critical,
}

impl TicketPriority {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "low" => Some(Self::Low),
            "medium" => Some(Self::Medium),
            "high" => Some(Self::High),
            "critical" => Some(Self::Critical),
            _ => None,
        }
    }
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Low => "low",
            Self::Medium => "medium",
            Self::High => "high",
            Self::Critical => "critical",
        }
    }

    /// Response SLA in minutes
    pub fn response_sla_minutes(&self) -> i64 {
        match self {
            Self::Critical => 15,
            Self::High => 30,
            Self::Medium => 120,
            Self::Low => 480,
        }
    }

    /// Resolution SLA in minutes
    pub fn resolution_sla_minutes(&self) -> i64 {
        match self {
            Self::Critical => 120,
            Self::High => 240,
            Self::Medium => 1440,
            Self::Low => 4320,
        }
    }
}

impl fmt::Display for TicketPriority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
