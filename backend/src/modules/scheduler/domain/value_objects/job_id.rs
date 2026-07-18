use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct JobDefinitionId(i64);

impl JobDefinitionId {
    pub fn new(id: i64) -> Self {
        Self(id)
    }
    pub fn value(&self) -> i64 {
        self.0
    }
}

impl fmt::Display for JobDefinitionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<i64> for JobDefinitionId {
    fn from(id: i64) -> Self {
        Self(id)
    }
}
