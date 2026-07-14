//! Money value object.
//!
//! Represents a monetary amount with currency.

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Money value object.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Money {
    pub amount: Decimal,
    pub currency: String,
}

/// Money domain errors.
#[derive(Debug, Error)]
pub enum MoneyError {
    #[error("Invalid amount: {0}")]
    InvalidAmount(String),

    #[error("Currency mismatch: {0} != {1}")]
    CurrencyMismatch(String, String),
}

impl Money {
    /// Create a new Money value object.
    pub fn new(amount: Decimal, currency: &str) -> Result<Self, MoneyError> {
        if amount < Decimal::ZERO {
            return Err(MoneyError::InvalidAmount(
                "Amount cannot be negative".to_string(),
            ));
        }

        if currency.trim().is_empty() {
            return Err(MoneyError::InvalidAmount(
                "Currency cannot be empty".to_string(),
            ));
        }

        Ok(Self {
            amount,
            currency: currency.to_uppercase(),
        })
    }

    /// Add two Money values (must have same currency).
    pub fn add(&self, other: &Money) -> Result<Money, MoneyError> {
        if self.currency != other.currency {
            return Err(MoneyError::CurrencyMismatch(
                self.currency.clone(),
                other.currency.clone(),
            ));
        }

        Ok(Money {
            amount: self.amount + other.amount,
            currency: self.currency.clone(),
        })
    }

    /// Subtract two Money values (must have same currency).
    pub fn subtract(&self, other: &Money) -> Result<Money, MoneyError> {
        if self.currency != other.currency {
            return Err(MoneyError::CurrencyMismatch(
                self.currency.clone(),
                other.currency.clone(),
            ));
        }

        let result = self.amount - other.amount;
        if result < Decimal::ZERO {
            return Err(MoneyError::InvalidAmount(
                "Result cannot be negative".to_string(),
            ));
        }

        Ok(Money {
            amount: result,
            currency: self.currency.clone(),
        })
    }

    /// Check if amount is zero.
    pub fn is_zero(&self) -> bool {
        self.amount == Decimal::ZERO
    }
}

impl std::fmt::Display for Money {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.currency, self.amount)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_money() {
        let money = Money::new(Decimal::new(1000, 2), "INR").unwrap();
        assert_eq!(money.amount, Decimal::new(1000, 2));
        assert_eq!(money.currency, "INR");
    }

    #[test]
    fn test_add_money() {
        let m1 = Money::new(Decimal::new(100, 0), "INR").unwrap();
        let m2 = Money::new(Decimal::new(200, 0), "INR").unwrap();
        let result = m1.add(&m2).unwrap();
        assert_eq!(result.amount, Decimal::new(300, 0));
    }

    #[test]
    fn test_currency_mismatch() {
        let m1 = Money::new(Decimal::new(100, 0), "INR").unwrap();
        let m2 = Money::new(Decimal::new(100, 0), "USD").unwrap();
        assert!(m1.add(&m2).is_err());
    }

    #[test]
    fn test_is_zero() {
        let money = Money::new(Decimal::ZERO, "INR").unwrap();
        assert!(money.is_zero());
    }
}
