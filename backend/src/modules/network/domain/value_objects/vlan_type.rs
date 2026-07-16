use serde::{Deserialize, Serialize};
use std::fmt;

/// VLAN type value object
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VlanType {
    Management,
    CustomerResidential,
    CustomerBusiness,
    Iptv,
    Voip,
    Monitoring,
}

impl VlanType {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "management" => Some(Self::Management),
            "customer_residential" => Some(Self::CustomerResidential),
            "customer_business" => Some(Self::CustomerBusiness),
            "iptv" => Some(Self::Iptv),
            "voip" => Some(Self::Voip),
            "monitoring" => Some(Self::Monitoring),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Management => "management",
            Self::CustomerResidential => "customer_residential",
            Self::CustomerBusiness => "customer_business",
            Self::Iptv => "iptv",
            Self::Voip => "voip",
            Self::Monitoring => "monitoring",
        }
    }
}

impl fmt::Display for VlanType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", self.as_str()) }
}
