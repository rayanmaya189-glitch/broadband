//! Customer ID value object.
//!
//! Type-safe wrapper for customer identifiers.

use serde::{Deserialize, Serialize};

/// Customer ID value object.
///
/// Provides type safety to prevent accidentally mixing customer IDs
/// with other entity IDs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CustomerId(i64);

impl CustomerId {
    /// Create a new CustomerId.
    pub fn new(id: i64) -> Self {
        Self(id)
    }

    /// Get the inner i64 value.
    pub fn inner(&self) -> i64 {
        self.0
    }
}

impl std::fmt::Display for CustomerId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<i64> for CustomerId {
    fn from(id: i64) -> Self {
        Self(id)
    }
}

impl From<CustomerId> for i64 {
    fn from(id: CustomerId) -> Self {
        id.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_customer_id_creation() {
        let id = CustomerId::new(123);
        assert_eq!(id.inner(), 123);
    }

    #[test]
    fn test_customer_id_from_i64() {
        let id: CustomerId = 456.into();
        assert_eq!(id.inner(), 456);
    }

    #[test]
    fn test_customer_id_to_i64() {
        let id = CustomerId::new(789);
        let num: i64 = id.into();
        assert_eq!(num, 789);
    }

    #[test]
    fn test_customer_id_display() {
        let id = CustomerId::new(100);
        assert_eq!(format!("{}", id), "100");
    }

    #[test]
    fn test_customer_id_equality() {
        let id1 = CustomerId::new(1);
        let id2 = CustomerId::new(1);
        let id3 = CustomerId::new(2);

        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
    }
}
