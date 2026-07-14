//! Payment ID value object.

use serde::{Deserialize, Serialize};

/// Type-safe wrapper for payment identifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PaymentId(i64);

impl PaymentId {
    pub fn new(id: i64) -> Self {
        Self(id)
    }

    pub fn inner(&self) -> i64 {
        self.0
    }
}

impl std::fmt::Display for PaymentId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<i64> for PaymentId {
    fn from(id: i64) -> Self {
        Self(id)
    }
}

impl From<PaymentId> for i64 {
    fn from(id: PaymentId) -> Self {
        id.0
    }
}
