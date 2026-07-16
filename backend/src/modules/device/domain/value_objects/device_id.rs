use serde::{Deserialize, Serialize};
use std::fmt;

/// DeviceId value object
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DeviceId(i64);

impl DeviceId {
    pub fn new(id: i64) -> Self { Self(id) }
    pub fn value(&self) -> i64 { self.0 }
    pub fn is_valid(&self) -> bool { self.0 > 0 }
}

impl fmt::Display for DeviceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", self.0) }
}

impl From<i64> for DeviceId {
    fn from(id: i64) -> Self { Self(id) }
}

impl From<DeviceId> for i64 {
    fn from(id: DeviceId) -> Self { id.0 }
}
