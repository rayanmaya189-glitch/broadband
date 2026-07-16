use crate::modules::accounting::domain::value_objects::{JournalEntryId, JournalEntryStatus};

/// JournalEntry aggregate root - represents a double-entry accounting journal entry
#[derive(Debug, Clone)]
pub struct JournalEntry {
    pub id: JournalEntryId,
    pub entry_number: String,
    pub entry_date: chrono::NaiveDate,
    pub description: String,
    pub reference_type: Option<String>,
    pub reference_id: Option<i64>,
    pub total_debit: rust_decimal::Decimal,
    pub total_credit: rust_decimal::Decimal,
    pub status: JournalEntryStatus,
    pub created_by: Option<i64>,
}

/// Domain errors for JournalEntry aggregate
#[derive(Debug, Clone, PartialEq)]
pub enum AccountingDomainError {
    EntryNotFound(i64),
    DebitCreditMismatch,
    AlreadyPosted,
    AlreadyVoided,
    CannotModifyPostedEntry,
    InvalidAmount,
}

impl std::fmt::Display for AccountingDomainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EntryNotFound(id) => write!(f, "Journal entry {} not found", id),
            Self::DebitCreditMismatch => write!(f, "Total debits must equal total credits"),
            Self::AlreadyPosted => write!(f, "Journal entry is already posted"),
            Self::AlreadyVoided => write!(f, "Journal entry is already voided"),
            Self::CannotModifyPostedEntry => write!(f, "Cannot modify a posted journal entry"),
            Self::InvalidAmount => write!(f, "Amount must be positive"),
        }
    }
}

impl std::error::Error for AccountingDomainError {}

impl JournalEntry {
    pub fn new(
        entry_number: String,
        entry_date: chrono::NaiveDate,
        description: String,
        created_by: Option<i64>,
    ) -> Self {
        Self {
            id: JournalEntryId::new(0),
            entry_number,
            entry_date,
            description,
            reference_type: None,
            reference_id: None,
            total_debit: rust_decimal::Decimal::ZERO,
            total_credit: rust_decimal::Decimal::ZERO,
            status: JournalEntryStatus::Draft,
            created_by,
        }
    }

    pub fn set_totals(&mut self, debit: rust_decimal::Decimal, credit: rust_decimal::Decimal) -> Result<(), AccountingDomainError> {
        if debit != credit {
            return Err(AccountingDomainError::DebitCreditMismatch);
        }
        if debit <= rust_decimal::Decimal::ZERO {
            return Err(AccountingDomainError::InvalidAmount);
        }
        self.total_debit = debit;
        self.total_credit = credit;
        Ok(())
    }

    pub fn post(&mut self) -> Result<(), AccountingDomainError> {
        if self.status == JournalEntryStatus::Posted {
            return Err(AccountingDomainError::AlreadyPosted);
        }
        if self.total_debit != self.total_credit {
            return Err(AccountingDomainError::DebitCreditMismatch);
        }
        self.status = JournalEntryStatus::Posted;
        Ok(())
    }

    pub fn void(&mut self) -> Result<(), AccountingDomainError> {
        if self.status == JournalEntryStatus::Voided {
            return Err(AccountingDomainError::AlreadyVoided);
        }
        self.status = JournalEntryStatus::Voided;
        Ok(())
    }

    pub fn is_balanced(&self) -> bool {
        self.total_debit == self.total_credit
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_journal_entry() {
        let entry = JournalEntry::new(
            "JE-001".to_string(), chrono::Utc::now().date_naive(),
            "Test entry".to_string(), Some(1),
        );
        assert_eq!(entry.status, JournalEntryStatus::Draft);
        assert!(entry.is_balanced());
    }

    #[test]
    fn test_post_entry() {
        let mut entry = JournalEntry::new(
            "JE-001".to_string(), chrono::Utc::now().date_naive(),
            "Test".to_string(), Some(1),
        );
        entry.set_totals(rust_decimal_macros::dec!(100), rust_decimal_macros::dec!(100)).unwrap();
        entry.post().unwrap();
        assert_eq!(entry.status, JournalEntryStatus::Posted);
    }

    #[test]
    fn test_debit_credit_mismatch() {
        let mut entry = JournalEntry::new(
            "JE-001".to_string(), chrono::Utc::now().date_naive(),
            "Test".to_string(), Some(1),
        );
        assert_eq!(
            entry.set_totals(rust_decimal_macros::dec!(100), rust_decimal_macros::dec!(50)),
            Err(AccountingDomainError::DebitCreditMismatch)
        );
    }
}
