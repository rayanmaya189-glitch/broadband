use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Trial balance entry — one row per account in the trial balance report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrialBalanceEntry {
    pub account_code: i32,
    pub account_name: String,
    pub account_type: String,
    pub opening_balance: Decimal,
    pub total_debit: Decimal,
    pub total_credit: Decimal,
    pub closing_balance: Decimal,
}

/// Trial balance report — aggregate of all account balances for a period.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrialBalance {
    pub id: i64,
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
    pub entries: Vec<TrialBalanceEntry>,
    pub total_debit: Decimal,
    pub total_credit: Decimal,
    pub is_balanced: bool,
    pub generated_at: chrono::DateTime<chrono::Utc>,
}

impl TrialBalance {
    /// Create a new trial balance from entries.
    ///
    /// # Arguments
    /// * `id` - Unique identifier for this trial balance
    /// * `period_start` - Start of the accounting period
    /// * `period_end` - End of the accounting period
    /// * `entries` - Account balance entries
    pub fn new(
        id: i64,
        period_start: NaiveDate,
        period_end: NaiveDate,
        entries: Vec<TrialBalanceEntry>,
    ) -> Self {
        // Sum directly from total_debit and total_credit fields on entries,
        // NOT from closing_balance (which is already net: debit - credit).
        let total_debit: Decimal = entries.iter().map(|e| e.total_debit).sum();
        let total_credit: Decimal = entries.iter().map(|e| e.total_credit).sum();

        Self {
            id,
            period_start,
            period_end,
            total_debit,
            total_credit,
            is_balanced: total_debit == total_credit,
            entries,
            generated_at: chrono::Utc::now(),
        }
    }

    /// Check if the trial balance is balanced
    pub fn verify_balanced(&self) -> Result<(), TrialBalanceError> {
        if !self.is_balanced {
            return Err(TrialBalanceError::Unbalanced {
                debit: self.total_debit,
                credit: self.total_credit,
                difference: self.total_debit - self.total_credit,
            });
        }
        Ok(())
    }

    /// Get entries filtered by account type
    pub fn entries_by_type(&self, account_type: &str) -> Vec<&TrialBalanceEntry> {
        self.entries
            .iter()
            .filter(|e| e.account_type == account_type)
            .collect()
    }
}

/// Errors for trial balance
#[derive(Debug, Clone, Error)]
pub enum TrialBalanceError {
    #[error(
        "Trial balance is unbalanced: debit={debit}, credit={credit}, difference={difference}"
    )]
    Unbalanced {
        debit: Decimal,
        credit: Decimal,
        difference: Decimal,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_balanced_trial_balance() {
        let entries = vec![
            TrialBalanceEntry {
                account_code: 1000,
                account_name: "Cash".to_string(),
                account_type: "asset".to_string(),
                opening_balance: Decimal::ZERO,
                total_debit: rust_decimal_macros::dec!(1000),
                total_credit: Decimal::ZERO,
                closing_balance: rust_decimal_macros::dec!(1000),
            },
            TrialBalanceEntry {
                account_code: 4000,
                account_name: "Revenue".to_string(),
                account_type: "revenue".to_string(),
                opening_balance: Decimal::ZERO,
                total_debit: Decimal::ZERO,
                total_credit: rust_decimal_macros::dec!(1000),
                closing_balance: rust_decimal_macros::dec!(-1000),
            },
        ];
        let tb = TrialBalance::new(
            1,
            NaiveDate::from_ymd_opt(2026, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2026, 1, 31).unwrap(),
            entries,
        );
        assert!(tb.is_balanced);
        assert!(tb.verify_balanced().is_ok());
    }

    #[test]
    fn test_unbalanced_trial_balance() {
        let entries = vec![
            TrialBalanceEntry {
                account_code: 1000,
                account_name: "Cash".to_string(),
                account_type: "asset".to_string(),
                opening_balance: Decimal::ZERO,
                total_debit: rust_decimal_macros::dec!(1000),
                total_credit: Decimal::ZERO,
                closing_balance: rust_decimal_macros::dec!(1000),
            },
            TrialBalanceEntry {
                account_code: 4000,
                account_name: "Revenue".to_string(),
                account_type: "revenue".to_string(),
                opening_balance: Decimal::ZERO,
                total_debit: Decimal::ZERO,
                total_credit: rust_decimal_macros::dec!(500),
                closing_balance: rust_decimal_macros::dec!(-500),
            },
        ];
        let tb = TrialBalance::new(
            2,
            NaiveDate::from_ymd_opt(2026, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2026, 1, 31).unwrap(),
            entries,
        );
        assert!(!tb.is_balanced);
        assert!(tb.verify_balanced().is_err());
    }
}
