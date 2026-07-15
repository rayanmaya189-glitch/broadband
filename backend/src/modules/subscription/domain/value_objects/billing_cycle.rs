use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BillingCycle {
    Monthly,
    Quarterly,
    HalfYearly,
    Annual,
}

impl BillingCycle {
    pub fn from_months(months: i32) -> Option<Self> {
        match months {
            1 => Some(Self::Monthly),
            3 => Some(Self::Quarterly),
            6 => Some(Self::HalfYearly),
            12 => Some(Self::Annual),
            _ => None,
        }
    }

    pub fn to_months(&self) -> i32 {
        match self {
            Self::Monthly => 1,
            Self::Quarterly => 3,
            Self::HalfYearly => 6,
            Self::Annual => 12,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Monthly => "monthly",
            Self::Quarterly => "quarterly",
            Self::HalfYearly => "half_yearly",
            Self::Annual => "annual",
        }
    }
}

impl fmt::Display for BillingCycle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
