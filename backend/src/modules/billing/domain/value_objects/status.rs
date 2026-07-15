use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InvoiceStatus {
    Pending,
    Paid,
    Overdue,
    Cancelled,
    Void,
    PartiallyPaid,
}

impl InvoiceStatus {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "pending" => Some(Self::Pending),
            "paid" => Some(Self::Paid),
            "overdue" => Some(Self::Overdue),
            "cancelled" => Some(Self::Cancelled),
            "void" => Some(Self::Void),
            "partially_paid" => Some(Self::PartiallyPaid),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Paid => "paid",
            Self::Overdue => "overdue",
            Self::Cancelled => "cancelled",
            Self::Void => "void",
            Self::PartiallyPaid => "partially_paid",
        }
    }

    pub fn is_terminal(&self) -> bool {
        matches!(self, Self::Paid | Self::Cancelled | Self::Void)
    }
}

impl fmt::Display for InvoiceStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl From<InvoiceStatus> for String {
    fn from(status: InvoiceStatus) -> Self {
        status.as_str().to_string()
    }
}
