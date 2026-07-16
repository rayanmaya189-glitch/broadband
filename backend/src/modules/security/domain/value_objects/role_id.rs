use serde::{Deserialize, Serialize};
use std::fmt;

/// RoleId value object
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RoleId(i64);

impl RoleId {
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

impl fmt::Display for RoleId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<i64> for RoleId {
    fn from(id: i64) -> Self {
        Self(id)
    }
}

impl From<RoleId> for i64 {
    fn from(id: RoleId) -> Self {
        id.0
    }
}
