use serde::{Deserialize, Serialize};
use std::fmt;

/// VlanId value object
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct VlanId(i64);

impl VlanId {
    pub fn new(id: i64) -> Self { Self(id) }
    pub fn value(&self) -> i64 { self.0 }
    pub fn is_valid(&self) -> bool { self.0 > 0 }
}

impl fmt::Display for VlanId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", self.0) }
}

impl From<i64> for VlanId {
    fn from(id: i64) -> Self { Self(id) }
}

impl From<VlanId> for i64 {
    fn from(id: VlanId) -> Self { id.0 }
}
