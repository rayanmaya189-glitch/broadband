use utoipa::ToSchema;
use serde::Deserialize;
use validator::Validate;
use chrono::NaiveDate;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateAccountRequest { pub code: String, pub name: String, pub account_type: String, pub parent_id: Option<i64> }
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateJournalEntryRequest { pub entry_date: NaiveDate, pub description: String, pub lines: Vec<JournalLineRequest> }
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct JournalLineRequest { pub account_id: i64, pub debit: Option<rust_decimal::Decimal>, pub credit: Option<rust_decimal::Decimal> }
#[derive(Debug, Deserialize, ToSchema)]
pub struct AccountingQuery { pub page: Option<i64>, pub per_page: Option<i64> }
