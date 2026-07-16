use serde::{Deserialize, Serialize};
use std::fmt;

/// BranchId value object
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BranchId(i64);

impl BranchId {
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

impl fmt::Display for BranchId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<i64> for BranchId {
    fn from(id: i64) -> Self {
        Self(id)
    }
}

impl From<BranchId> for i64 {
    fn from(id: BranchId) -> Self {
        id.0
    }
}
