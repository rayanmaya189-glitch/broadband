use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NotificationId(i64);

impl NotificationId {
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

impl fmt::Display for NotificationId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<i64> for NotificationId {
    fn from(id: i64) -> Self {
        Self(id)
    }
}
impl From<NotificationId> for i64 {
    fn from(id: NotificationId) -> Self {
        id.0
    }
}
