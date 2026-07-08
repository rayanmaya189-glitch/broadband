use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct AccountResponse { pub id: i64, pub code: String, pub name: String, pub account_type: String, pub is_active: bool, pub created_at: DateTime<Utc> }
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct JournalEntryResponse { pub id: i64, pub entry_number: String, pub entry_date: NaiveDate, pub description: String, pub total_debit: Decimal, pub total_credit: Decimal, pub status: String, pub created_at: DateTime<Utc> }
#[derive(Debug, Serialize, Deserialize)]
pub struct MessageResponse { pub message: String }
