use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ApiKeyId(i64);

impl ApiKeyId {
    pub fn new(id: i64) -> Self { Self(id) }
    pub fn value(&self) -> i64 { self.0 }
}

impl fmt::Display for ApiKeyId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", self.0) }
}

impl From<i64> for ApiKeyId { fn from(id: i64) -> Self { Self(id) } }
