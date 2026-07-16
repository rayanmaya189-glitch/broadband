use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TicketStatus {
    Open,
    Assigned,
    InProgress,
    WaitingCustomer,
    Escalated,
    Resolved,
    Closed,
    Reopened,
}

impl TicketStatus {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "open" => Some(Self::Open),
            "assigned" => Some(Self::Assigned),
            "in_progress" => Some(Self::InProgress),
            "waiting_customer" => Some(Self::WaitingCustomer),
            "escalated" => Some(Self::Escalated),
            "resolved" => Some(Self::Resolved),
            "closed" => Some(Self::Closed),
            "reopened" => Some(Self::Reopened),
            _ => None,
        }
    }
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Open => "open", Self::Assigned => "assigned", Self::InProgress => "in_progress",
            Self::WaitingCustomer => "waiting_customer", Self::Escalated => "escalated",
            Self::Resolved => "resolved", Self::Closed => "closed", Self::Reopened => "reopened",
        }
    }
}

impl fmt::Display for TicketStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", self.as_str()) }
}
