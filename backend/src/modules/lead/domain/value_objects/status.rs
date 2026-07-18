use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LeadStatus {
    New,
    Contacted,
    Interested,
    Surveyed,
    Quoted,
    Converted,
    Lost,
}

impl LeadStatus {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "new" => Some(Self::New),
            "contacted" => Some(Self::Contacted),
            "interested" => Some(Self::Interested),
            "surveyed" => Some(Self::Surveyed),
            "quoted" => Some(Self::Quoted),
            "converted" => Some(Self::Converted),
            "lost" => Some(Self::Lost),
            _ => None,
        }
    }
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::New => "new",
            Self::Contacted => "contacted",
            Self::Interested => "interested",
            Self::Surveyed => "surveyed",
            Self::Quoted => "quoted",
            Self::Converted => "converted",
            Self::Lost => "lost",
        }
    }
    pub fn is_terminal(&self) -> bool {
        matches!(self, Self::Converted | Self::Lost)
    }
}

impl fmt::Display for LeadStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
