use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NotificationChannel {
    Email,
    Sms,
    WhatsApp,
    Push,
    InApp,
}

impl NotificationChannel {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "email" => Some(Self::Email),
            "sms" => Some(Self::Sms),
            "whatsapp" => Some(Self::WhatsApp),
            "push" => Some(Self::Push),
            "in_app" | "inapp" => Some(Self::InApp),
            _ => None,
        }
    }
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Email => "email", Self::Sms => "sms", Self::WhatsApp => "whatsapp",
            Self::Push => "push", Self::InApp => "in_app",
        }
    }
}

impl fmt::Display for NotificationChannel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", self.as_str()) }
}
