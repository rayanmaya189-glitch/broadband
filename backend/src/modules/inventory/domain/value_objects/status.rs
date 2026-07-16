use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InventoryStatus {
    InStock,
    Assigned,
    Installed,
    Returned,
    Damaged,
    Scrapped,
    InTransit,
}

impl InventoryStatus {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "in_stock" => Some(Self::InStock), "assigned" => Some(Self::Assigned),
            "installed" => Some(Self::Installed), "returned" => Some(Self::Returned),
            "damaged" => Some(Self::Damaged), "scrapped" => Some(Self::Scrapped),
            "in_transit" => Some(Self::InTransit), _ => None,
        }
    }
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::InStock => "in_stock", Self::Assigned => "assigned", Self::Installed => "installed",
            Self::Returned => "returned", Self::Damaged => "damaged", Self::Scrapped => "scrapped",
            Self::InTransit => "in_transit",
        }
    }
}

impl fmt::Display for InventoryStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", self.as_str()) }
}
