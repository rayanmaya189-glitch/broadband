//! Customer status value object.
//!
//! Represents the lifecycle status of a customer.

use serde::{Deserialize, Serialize};

/// Customer status value object.
///
/// # Status Transitions
/// ```text
/// lead → prospect → active → suspended → active
///                    ↓
///                 deactivated
///                    ↓
///                 blacklist
/// ```
///
/// # Invariants
/// - A customer cannot be activated without KYC verification
/// - A customer cannot be deleted while active subscriptions exist
/// - Any status change should be recorded via domain event
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CustomerStatus {
    /// Initial status when customer first inquires
    Lead,
    /// Customer has shown interest and is being pursued
    Prospect,
    /// Active customer with valid subscription
    Active,
    /// Temporarily suspended (e.g., non-payment)
    Suspended,
    /// Permanently deactivated
    Deactivated,
    /// Blacklisted due to fraud or policy violation
    Blacklist,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StatusError {
    InvalidTransition(String),
    KycRequired,
    ActiveSubscriptionsExist,
}

impl std::fmt::Display for StatusError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StatusError::InvalidTransition(msg) => write!(f, "Invalid status transition: {}", msg),
            StatusError::KycRequired => write!(f, "KYC verification required before activation"),
            StatusError::ActiveSubscriptionsExist => write!(f, "Cannot deactivate customer with active subscriptions"),
        }
    }
}

impl std::error::Error for StatusError {}

impl CustomerStatus {
    /// Parse a status string into a CustomerStatus.
    pub fn from_str(status: &str) -> Result<Self, StatusError> {
        match status.to_lowercase().as_str() {
            "lead" => Ok(Self::Lead),
            "prospect" => Ok(Self::Prospect),
            "active" => Ok(Self::Active),
            "suspended" => Ok(Self::Suspended),
            "deactivated" => Ok(Self::Deactivated),
            "blacklist" | "blacklisted" => Ok(Self::Blacklist),
            _ => Err(StatusError::InvalidTransition(format!("Unknown status: {}", status))),
        }
    }

    /// Get the string representation.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Lead => "lead",
            Self::Prospect => "prospect",
            Self::Active => "active",
            Self::Suspended => "suspended",
            Self::Deactivated => "deactivated",
            Self::Blacklist => "blacklist",
        }
    }

    /// Check if transition from this status to the target status is valid.
    pub fn can_transition_to(&self, target: &CustomerStatus) -> bool {
        matches!(
            (self, target),
            (Self::Lead, Self::Prospect)
                | (Self::Lead, Self::Active)
                | (Self::Prospect, Self::Active)
                | (Self::Prospect, Self::Lead)
                | (Self::Active, Self::Suspended)
                | (Self::Active, Self::Deactivated)
                | (Self::Active, Self::Blacklist)
                | (Self::Suspended, Self::Active)
                | (Self::Suspended, Self::Deactivated)
        )
    }

    /// Check if customer is in an active state.
    pub fn is_active(&self) -> bool {
        matches!(self, Self::Active)
    }

    /// Check if customer can receive services.
    pub fn can_receive_services(&self) -> bool {
        matches!(self, Self::Active | Self::Suspended)
    }
}

impl std::fmt::Display for CustomerStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl From<CustomerStatus> for String {
    fn from(status: CustomerStatus) -> Self {
        status.as_str().to_string()
    }
}

impl TryFrom<&str> for CustomerStatus {
    type Error = StatusError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_from_str() {
        assert_eq!(CustomerStatus::from_str("lead").unwrap(), CustomerStatus::Lead);
        assert_eq!(CustomerStatus::from_str("ACTIVE").unwrap(), CustomerStatus::Active);
        assert_eq!(CustomerStatus::from_str("blacklisted").unwrap(), CustomerStatus::Blacklist);
    }

    #[test]
    fn test_invalid_status() {
        assert!(matches!(
            CustomerStatus::from_str("unknown"),
            Err(StatusError::InvalidTransition(_))
        ));
    }

    #[test]
    fn test_valid_transitions() {
        assert!(CustomerStatus::Lead.can_transition_to(&CustomerStatus::Prospect));
        assert!(CustomerStatus::Prospect.can_transition_to(&CustomerStatus::Active));
        assert!(CustomerStatus::Active.can_transition_to(&CustomerStatus::Suspended));
        assert!(CustomerStatus::Suspended.can_transition_to(&CustomerStatus::Active));
    }

    #[test]
    fn test_invalid_transitions() {
        assert!(!CustomerStatus::Lead.can_transition_to(&CustomerStatus::Active));
        assert!(!CustomerStatus::Suspended.can_transition_to(&CustomerStatus::Prospect));
        assert!(!CustomerStatus::Deactivated.can_transition_to(&CustomerStatus::Active));
    }

    #[test]
    fn test_is_active() {
        assert!(CustomerStatus::Active.is_active());
        assert!(!CustomerStatus::Suspended.is_active());
    }

    #[test]
    fn test_can_receive_services() {
        assert!(CustomerStatus::Active.can_receive_services());
        assert!(CustomerStatus::Suspended.can_receive_services());
        assert!(!CustomerStatus::Deactivated.can_receive_services());
    }
}
