//! VLAN ID value object.

use serde::{Deserialize, Serialize};

/// Type-safe wrapper for VLAN identifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct VlanId(i64);

impl VlanId {
    pub fn new(id: i64) -> Self {
        Self(id)
    }

    pub fn inner(&self) -> i64 {
        self.0
    }
}

impl std::fmt::Display for VlanId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<i64> for VlanId {
    fn from(id: i64) -> Self {
        Self(id)
    }
}

impl From<VlanId> for i64 {
    fn from(id: VlanId) -> Self {
        id.0
    }
}
