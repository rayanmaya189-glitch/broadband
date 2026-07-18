use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BandwidthProfileId(i64);

impl BandwidthProfileId {
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

impl fmt::Display for BandwidthProfileId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<i64> for BandwidthProfileId {
    fn from(id: i64) -> Self {
        Self(id)
    }
}
impl From<BandwidthProfileId> for i64 {
    fn from(id: BandwidthProfileId) -> Self {
        id.0
    }
}
