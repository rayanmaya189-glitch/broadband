use crate::modules::accounting::domain::value_objects::AccountType;
use rust_decimal::Decimal;

/// JournalEntryLine entity — a single debit or credit line in a journal entry.
#[derive(Debug, Clone, PartialEq)]
pub struct JournalEntryLine {
    pub id: i64,
    pub journal_entry_id: i64,
    pub account_id: i64,
    pub account_code: i32,
    pub account_name: String,
    pub account_type: AccountType,
    pub debit: Decimal,
    pub credit: Decimal,
    pub description: Option<String>,
}

/// Domain errors for JournalEntryLine
#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum JournalEntryLineError {
    /// A line must have either debit or credit, not both
    #[error("A line cannot have both debit and credit")]
    BothDebitAndCredit,
    /// A line must have either debit or credit, not neither
    #[error("A line must have either debit or credit")]
    NeitherDebitNorCredit,
    /// Debit/credit amounts must be positive
    #[error("Amount must be positive")]
    InvalidAmount,
    /// Journal entry lines are not balanced (total debit != total credit)
    #[error(
        "Journal entry is unbalanced: debit={debit}, credit={credit}, difference={difference}"
    )]
    Unbalanced {
        debit: rust_decimal::Decimal,
        credit: rust_decimal::Decimal,
        difference: rust_decimal::Decimal,
    },
}

impl JournalEntryLine {
    /// Create a debit line
    pub fn debit(
        account_id: i64,
        account_code: i32,
        account_name: String,
        account_type: AccountType,
        amount: Decimal,
        description: Option<String>,
    ) -> Result<Self, JournalEntryLineError> {
        if amount <= Decimal::ZERO {
            return Err(JournalEntryLineError::InvalidAmount);
        }
        Ok(Self {
            id: 0,
            journal_entry_id: 0,
            account_id,
            account_code,
            account_name,
            account_type,
            debit: amount,
            credit: Decimal::ZERO,
            description,
        })
    }

    /// Create a credit line
    pub fn credit(
        account_id: i64,
        account_code: i32,
        account_name: String,
        account_type: AccountType,
        amount: Decimal,
        description: Option<String>,
    ) -> Result<Self, JournalEntryLineError> {
        if amount <= Decimal::ZERO {
            return Err(JournalEntryLineError::InvalidAmount);
        }
        Ok(Self {
            id: 0,
            journal_entry_id: 0,
            account_id,
            account_code,
            account_name,
            account_type,
            debit: Decimal::ZERO,
            credit: amount,
            description,
        })
    }

    /// Check if this is a debit line
    pub fn is_debit(&self) -> bool {
        self.debit > Decimal::ZERO
    }

    /// Check if this is a credit line
    pub fn is_credit(&self) -> bool {
        self.credit > Decimal::ZERO
    }

    /// Get the absolute amount
    pub fn amount(&self) -> Decimal {
        if self.is_debit() {
            self.debit
        } else {
            self.credit
        }
    }
}

/// Validate that a set of journal entry lines is balanced
pub fn validate_lines_balanced(lines: &[JournalEntryLine]) -> Result<(), JournalEntryLineError> {
    let total_debit: Decimal = lines.iter().map(|l| l.debit).sum();
    let total_credit: Decimal = lines.iter().map(|l| l.credit).sum();

    if total_debit != total_credit {
        return Err(JournalEntryLineError::Unbalanced {
            debit: total_debit,
            credit: total_credit,
            difference: total_debit - total_credit,
        });
    }

    if total_debit <= Decimal::ZERO {
        return Err(JournalEntryLineError::NeitherDebitNorCredit);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_debit_line() {
        let line = JournalEntryLine::debit(
            1,
            1000,
            "Cash".to_string(),
            AccountType::Asset,
            rust_decimal_macros::dec!(1000),
            Some("Payment received".to_string()),
        )
        .unwrap();
        assert!(line.is_debit());
        assert!(!line.is_credit());
        assert_eq!(line.amount(), rust_decimal_macros::dec!(1000));
    }

    #[test]
    fn test_credit_line() {
        let line = JournalEntryLine::credit(
            2,
            4000,
            "Revenue".to_string(),
            AccountType::Revenue,
            rust_decimal_macros::dec!(1000),
            Some("Service revenue".to_string()),
        )
        .unwrap();
        assert!(!line.is_debit());
        assert!(line.is_credit());
        assert_eq!(line.amount(), rust_decimal_macros::dec!(1000));
    }

    #[test]
    fn test_invalid_amount_zero() {
        let result = JournalEntryLine::debit(
            1,
            1000,
            "Cash".to_string(),
            AccountType::Asset,
            rust_decimal_macros::dec!(0),
            None,
        );
        assert_eq!(result, Err(JournalEntryLineError::InvalidAmount));
    }

    #[test]
    fn test_invalid_amount_negative() {
        let result = JournalEntryLine::credit(
            1,
            1000,
            "Cash".to_string(),
            AccountType::Asset,
            rust_decimal_macros::dec!(-100),
            None,
        );
        assert_eq!(result, Err(JournalEntryLineError::InvalidAmount));
    }

    #[test]
    fn test_validate_lines_balanced() {
        let lines = vec![
            JournalEntryLine::debit(
                1,
                1000,
                "Cash".to_string(),
                AccountType::Asset,
                rust_decimal_macros::dec!(1000),
                None,
            )
            .unwrap(),
            JournalEntryLine::credit(
                2,
                4000,
                "Revenue".to_string(),
                AccountType::Revenue,
                rust_decimal_macros::dec!(1000),
                None,
            )
            .unwrap(),
        ];
        assert!(validate_lines_balanced(&lines).is_ok());
    }

    #[test]
    fn test_validate_lines_unbalanced() {
        let lines = vec![
            JournalEntryLine::debit(
                1,
                1000,
                "Cash".to_string(),
                AccountType::Asset,
                rust_decimal_macros::dec!(1000),
                None,
            )
            .unwrap(),
            JournalEntryLine::credit(
                2,
                4000,
                "Revenue".to_string(),
                AccountType::Revenue,
                rust_decimal_macros::dec!(500),
                None,
            )
            .unwrap(),
        ];
        assert!(validate_lines_balanced(&lines).is_err());
    }
}
