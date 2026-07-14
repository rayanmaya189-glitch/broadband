//! Plan ID value object.

use serde::{Deserialize, Serialize};

/// Type-safe wrapper for plan identifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PlanId(i64);

impl PlanId {
    pub fn new(id: i64) -> Self {
        Self(id)
    }

    pub fn inner(&self) -> i64 {
        self.0
    }
}

impl std::fmt::Display for PlanId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<i64> for PlanId {
    fn from(id: i64) -> Self {
        Self(id)
    }
}

impl From<PlanId> for i64 {
    fn from(id: PlanId) -> Self {
        id.0
    }
}
