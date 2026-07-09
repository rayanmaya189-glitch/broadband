use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct ChartOfAccount {
    pub id: i64,
    pub code: String,
    pub name: String,
    pub account_type: String,
    pub parent_id: Option<i64>,
    pub is_group: bool,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow)]
pub struct JournalEntry {
    pub id: i64,
    pub entry_number: String,
    pub entry_date: NaiveDate,
    pub description: String,
    pub reference_type: Option<String>,
    pub reference_id: Option<i64>,
    pub total_debit: Decimal,
    pub total_credit: Decimal,
    pub status: String,
    pub created_by: Option<i64>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow)]
pub struct JournalEntryLine {
    pub id: i64,
    pub journal_entry_id: i64,
    pub account_id: i64,
    pub debit: Decimal,
    pub credit: Decimal,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow)]
pub struct TrialBalance {
    pub id: i64,
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
    pub account_id: i64,
    pub opening_balance: Decimal,
    pub total_debit: Decimal,
    pub total_credit: Decimal,
    pub closing_balance: Decimal,
    pub generated_at: DateTime<Utc>,
}
