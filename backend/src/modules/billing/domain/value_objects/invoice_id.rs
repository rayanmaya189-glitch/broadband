//! Invoice ID value object.

use serde::{Deserialize, Serialize};

/// Type-safe wrapper for invoice identifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct InvoiceId(i64);

impl InvoiceId {
    pub fn new(id: i64) -> Self {
        Self(id)
    }

    pub fn inner(&self) -> i64 {
        self.0
    }
}

impl std::fmt::Display for InvoiceId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<i64> for InvoiceId {
    fn from(id: i64) -> Self {
        Self(id)
    }
}

impl From<InvoiceId> for i64 {
    fn from(id: InvoiceId) -> Self {
        id.0
    }
}
