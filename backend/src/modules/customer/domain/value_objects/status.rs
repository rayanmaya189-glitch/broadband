use serde::{Deserialize, Serialize};
use std::fmt;

/// Customer status value object
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CustomerStatus {
    /// Customer registered but not yet verified
    Pending,
    /// Customer is active and can use services
    Active,
    /// Customer is suspended due to non-payment or violation
    Suspended,
    /// Customer account is locked due to security reasons
    Locked,
    /// Customer has been soft-deleted
    Deleted,
}

impl CustomerStatus {
    /// Parse status from string
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "pending" => Some(Self::Pending),
            "active" => Some(Self::Active),
            "suspended" => Some(Self::Suspended),
            "locked" => Some(Self::Locked),
            "deleted" => Some(Self::Deleted),
            _ => None,
        }
    }

    /// Convert to string
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Active => "active",
            Self::Suspended => "suspended",
            Self::Locked => "locked",
            Self::Deleted => "deleted",
        }
    }

    /// Check if customer can use services
    pub fn can_use_services(&self) -> bool {
        matches!(self, Self::Active)
    }

    /// Check if customer can be modified
    pub fn can_be_modified(&self) -> bool {
        !matches!(self, Self::Deleted)
    }

    /// Check if status transition is valid
    pub fn can_transition_to(&self, new_status: &Self) -> bool {
        match (self, new_status) {
            // Pending can go to Active, Locked, or Deleted
            (Self::Pending, Self::Active | Self::Locked | Self::Deleted) => true,
            // Active can go to Suspended, Locked, or Deleted
            (Self::Active, Self::Suspended | Self::Locked | Self::Deleted) => true,
            // Suspended can go to Active, Locked, or Deleted
            (Self::Suspended, Self::Active | Self::Locked | Self::Deleted) => true,
            // Locked can go to Active, Suspended, or Deleted
            (Self::Locked, Self::Active | Self::Suspended | Self::Deleted) => true,
            // Any status can stay the same (no-op)
            (s, n) if s == n => true,
            // All other transitions are invalid
            _ => false,
        }
    }
}

impl fmt::Display for CustomerStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl From<CustomerStatus> for String {
    fn from(status: CustomerStatus) -> Self {
        status.as_str().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_from_str() {
        assert_eq!(
            CustomerStatus::from_str("pending"),
            Some(CustomerStatus::Pending)
        );
        assert_eq!(
            CustomerStatus::from_str("active"),
            Some(CustomerStatus::Active)
        );
        assert_eq!(
            CustomerStatus::from_str("ACTIVE"),
            Some(CustomerStatus::Active)
        );
        assert_eq!(CustomerStatus::from_str("invalid"), None);
    }

    #[test]
    fn test_status_as_str() {
        assert_eq!(CustomerStatus::Pending.as_str(), "pending");
        assert_eq!(CustomerStatus::Active.as_str(), "active");
    }

    #[test]
    fn test_can_use_services() {
        assert!(CustomerStatus::Active.can_use_services());
        assert!(!CustomerStatus::Pending.can_use_services());
        assert!(!CustomerStatus::Suspended.can_use_services());
    }

    #[test]
    fn test_valid_transitions() {
        assert!(CustomerStatus::Pending.can_transition_to(&CustomerStatus::Active));
        assert!(CustomerStatus::Active.can_transition_to(&CustomerStatus::Suspended));
        assert!(CustomerStatus::Suspended.can_transition_to(&CustomerStatus::Active));
        assert!(!CustomerStatus::Pending.can_transition_to(&CustomerStatus::Suspended));
        assert!(!CustomerStatus::Active.can_transition_to(&CustomerStatus::Pending));
    }
}
