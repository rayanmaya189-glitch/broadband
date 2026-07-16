use serde::{Deserialize, Serialize};
use std::fmt;

/// Plan status value object
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PlanStatus {
    Draft,
    Published,
    Unpublished,
    Archived,
}

impl PlanStatus {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "draft" => Some(Self::Draft),
            "published" => Some(Self::Published),
            "unpublished" | "pending" => Some(Self::Unpublished),
            "archived" => Some(Self::Archived),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Draft => "draft",
            Self::Published => "published",
            Self::Unpublished => "unpublished",
            Self::Archived => "archived",
        }
    }

    pub fn is_available(&self) -> bool {
        matches!(self, Self::Published)
    }
}

impl fmt::Display for PlanStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl From<PlanStatus> for String {
    fn from(status: PlanStatus) -> Self {
        status.as_str().to_string()
    }
}
