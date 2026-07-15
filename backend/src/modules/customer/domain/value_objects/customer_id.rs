use serde::{Deserialize, Serialize};
use std::fmt;

/// CustomerId value object
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CustomerId(i64);

impl CustomerId {
    /// Create a new CustomerId
    pub fn new(id: i64) -> Self {
        Self(id)
    }

    /// Get the raw id value
    pub fn value(&self) -> i64 {
        self.0
    }

    /// Check if this is a valid (non-zero) id
    pub fn is_valid(&self) -> bool {
        self.0 > 0
    }
}

impl fmt::Display for CustomerId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
    fn test_customer_id() {
        let id = CustomerId::new(123);
        assert_eq!(id.value(), 123);
        assert!(id.is_valid());
    }

    #[test]
    fn test_invalid_id() {
        let id = CustomerId::new(0);
        assert!(!id.is_valid());
    }

    #[test]
    fn test_conversions() {
        let id = CustomerId::new(456);
        let raw: i64 = id.into();
        assert_eq!(raw, 456);

        let id2: CustomerId = 789.into();
        assert_eq!(id2.value(), 789);
    }
}
