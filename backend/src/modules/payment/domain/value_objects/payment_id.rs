use serde::{Deserialize, Serialize};
use std::fmt;

/// PaymentId value object
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PaymentId(i64);

impl PaymentId {
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

impl fmt::Display for PaymentId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<i64> for PaymentId {
    fn from(id: i64) -> Self {
        Self(id)
    }
}

impl From<PaymentId> for i64 {
    fn from(id: PaymentId) -> Self {
        id.0
    }
}
