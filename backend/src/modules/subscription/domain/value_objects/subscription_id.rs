use serde::{Deserialize, Serialize};
use std::fmt;

/// SubscriptionId value object
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SubscriptionId(i64);

impl SubscriptionId {
    pub fn new(id: i64) -> Self {
        Self(id)
    }

    pub fn value(&self) -> i64 {
        self.0
    }

    pub fn is_valid(&self) -> bool {
        self.0 > 0
    }
}

impl fmt::Display for SubscriptionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
