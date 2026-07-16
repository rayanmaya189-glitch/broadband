use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LeadSource {
    LandingPage,
    WhatsApp,
    Referral,
    WalkIn,
    ColdCall,
    SocialMedia,
    FieldVisit,
}

impl LeadSource {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "landing_page" | "landingpage" => Some(Self::LandingPage),
            "whatsapp" => Some(Self::WhatsApp),
            "referral" => Some(Self::Referral),
            "walk_in" | "walkin" => Some(Self::WalkIn),
            "cold_call" | "coldcall" => Some(Self::ColdCall),
            "social_media" | "socialmedia" => Some(Self::SocialMedia),
            "field_visit" | "fieldvisit" => Some(Self::FieldVisit),
            _ => None,
        }
    }
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::LandingPage => "landing_page", Self::WhatsApp => "whatsapp", Self::Referral => "referral",
            Self::WalkIn => "walk_in", Self::ColdCall => "cold_call", Self::SocialMedia => "social_media",
            Self::FieldVisit => "field_visit",
        }
    }
}

impl fmt::Display for LeadSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", self.as_str()) }
}
