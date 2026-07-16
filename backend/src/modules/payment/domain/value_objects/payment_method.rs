use serde::{Deserialize, Serialize};
use std::fmt;

/// Payment method value object
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PaymentMethod {
    Upi,
    Card,
    NetBanking,
    Wallet,
    Manual,
    BankTransfer,
}

impl PaymentMethod {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "upi" => Some(Self::Upi),
            "card" | "credit_card" | "debit_card" => Some(Self::Card),
            "netbanking" | "net_banking" => Some(Self::NetBanking),
            "wallet" => Some(Self::Wallet),
            "manual" => Some(Self::Manual),
            "bank_transfer" | "banktransfer" => Some(Self::BankTransfer),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Upi => "upi",
            Self::Card => "card",
            Self::NetBanking => "netbanking",
            Self::Wallet => "wallet",
            Self::Manual => "manual",
            Self::BankTransfer => "bank_transfer",
        }
    }
}

impl fmt::Display for PaymentMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
