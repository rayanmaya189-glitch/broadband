use serde::{Deserialize, Serialize};
use std::fmt;
use std::ops::{Add, Sub};

/// Money value object for representing currency amounts
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Money {
    amount: i64, // Amount in smallest currency unit (paise for INR)
    currency: String,
}

impl Money {
    /// Create a new Money instance (amount in smallest currency unit, e.g. paise)
    pub fn new(amount: i64) -> Self {
        Self {
            amount,
            currency: "INR".to_string(),
        }
    }

    /// Create Money from major units (e.g., 100.50 INR)
    pub fn from_major(amount: i64, decimals: u32) -> Self {
        let multiplier = 10i64.pow(decimals);
        Self {
            amount: amount * multiplier,
            currency: "INR".to_string(),
        }
    }

    /// Get the raw amount (in smallest currency unit)
    pub fn amount(&self) -> i64 {
        self.amount
    }

    /// Get amount as decimal (major units)
    pub fn to_major(&self) -> f64 {
        self.amount as f64 / 100.0
    }

    /// Get currency
    pub fn currency(&self) -> &str {
        &self.currency
    }

    /// Check if amount is positive
    pub fn is_positive(&self) -> bool {
        self.amount > 0
    }

    /// Check if amount is zero
    pub fn is_zero(&self) -> bool {
        self.amount == 0
    }

    /// Check if amount is negative
    pub fn is_negative(&self) -> bool {
        self.amount < 0
    }

    /// Get absolute value
    pub fn abs(&self) -> Self {
        Self {
            amount: self.amount.abs(),
            currency: self.currency.clone(),
        }
    }

    /// Zero money
    pub fn zero() -> Self {
        Self {
            amount: 0,
            currency: "INR".to_string(),
        }
    }
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let major = self.amount as f64 / 100.0;
        write!(f, "{} {:.2}", self.currency, major)
    }
}

impl Add for Money {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        assert_eq!(
            self.currency, other.currency,
            "Cannot add different currencies"
        );
        Self {
            amount: self.amount + other.amount,
            currency: self.currency,
        }
    }
}

impl Sub for Money {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        assert_eq!(
            self.currency, other.currency,
            "Cannot subtract different currencies"
        );
        Self {
            amount: self.amount - other.amount,
            currency: self.currency,
        }
    }
}

impl From<i64> for Money {
    fn from(amount: i64) -> Self {
        Self::new(amount)
    }
}

impl From<f64> for Money {
    fn from(amount: f64) -> Self {
        Self::new((amount * 100.0) as i64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_money_creation() {
        let money = Money::new(1000);
        assert_eq!(money.amount(), 1000);
        assert_eq!(money.currency(), "INR");
    }

    #[test]
    fn test_money_from_major() {
        let money = Money::from_major(100, 2);
        assert_eq!(money.amount(), 10000);
    }

    #[test]
    fn test_money_to_major() {
        let money = Money::new(10000);
        assert_eq!(money.to_major(), 100.0);
    }

    #[test]
    fn test_money_addition() {
        let a = Money::new(1000);
        let b = Money::new(2000);
        let c = a + b;
        assert_eq!(c.amount(), 3000);
    }

    #[test]
    fn test_money_subtraction() {
        let a = Money::new(3000);
        let b = Money::new(1000);
        let c = a - b;
        assert_eq!(c.amount(), 2000);
    }

    #[test]
    fn test_money_display() {
        let money = Money::new(10000);
        assert_eq!(format!("{}", money), "INR 100.00");
    }
}
