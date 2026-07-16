use serde::{Deserialize, Serialize};
use std::fmt;

/// PlanId value object
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PlanId(i64);

impl PlanId {
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

impl fmt::Display for PlanId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
