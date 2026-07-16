use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AuditAction {
    Login,
    Logout,
    Create,
    Update,
    Delete,
    View,
    Export,
    Approve,
    Reject,
    Custom(String),
}

impl AuditAction {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "login" => Self::Login,
            "logout" => Self::Logout,
            "create" => Self::Create,
            "update" => Self::Update,
            "delete" => Self::Delete,
            "view" => Self::View,
            "export" => Self::Export,
            "approve" => Self::Approve,
            "reject" => Self::Reject,
            _ => Self::Custom(s.to_string()),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::Login => "login", Self::Logout => "logout", Self::Create => "create",
            Self::Update => "update", Self::Delete => "delete", Self::View => "view",
            Self::Export => "export", Self::Approve => "approve", Self::Reject => "reject",
            Self::Custom(s) => s,
        }
    }
}

impl fmt::Display for AuditAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", self.as_str()) }
}
