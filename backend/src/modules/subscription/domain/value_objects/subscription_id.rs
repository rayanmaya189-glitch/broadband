//! Subscription ID value object.

use serde::{Deserialize, Serialize};

/// Type-safe wrapper for subscription identifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SubscriptionId(i64);

impl SubscriptionId {
    pub fn new(id: i64) -> Self {
        Self(id)
    }

    pub fn inner(&self) -> i64 {
        self.0
    }
}

impl std::fmt::Display for SubscriptionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<i64> for SubscriptionId {
    fn from(id: i64) -> Self {
        Self(id)
    }
}

impl From<SubscriptionId> for i64 {
    fn from(id: SubscriptionId) -> Self {
        id.0
    }
}
