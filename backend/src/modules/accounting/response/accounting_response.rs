use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;


#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AccountResponse {
    pub id: i64,
    pub code: String,
    pub name: String,
    pub account_type: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct JournalEntryResponse {
    pub id: i64,
    pub entry_number: String,
    pub entry_date: NaiveDate,
    pub description: String,
    pub total_debit: Decimal,
    pub total_credit: Decimal,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct JournalEntryLineResponse {
    pub id: i64,
    pub journal_entry_id: i64,
    pub account_id: i64,
    pub debit: Decimal,
    pub credit: Decimal,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct JournalEntryDetailResponse {
    pub entry: JournalEntryResponse,
    pub lines: Vec<JournalEntryLineResponse>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TrialBalanceResponse {
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
    pub accounts: Vec<TrialBalanceAccount>,
    pub total_debit: Decimal,
    pub total_credit: Decimal,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TrialBalanceAccount {
    pub account_id: i64,
    pub account_code: String,
    pub account_name: String,
    pub account_type: String,
    pub total_debit: Decimal,
    pub total_credit: Decimal,
    pub closing_balance: Decimal,
}

// ── Profit & Loss Statement ──────────────────────────────

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ProfitLossResponse {
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
    pub revenue: Vec<AccountLineItem>,
    pub total_revenue: Decimal,
    pub expenses: Vec<AccountLineItem>,
    pub total_expenses: Decimal,
    pub net_income: Decimal,
}

// ── Balance Sheet ──────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct BalanceSheetResponse {
    pub as_of_date: NaiveDate,
    pub assets: Vec<AccountLineItem>,
    pub total_assets: Decimal,
    pub liabilities: Vec<AccountLineItem>,
    pub total_liabilities: Decimal,
    pub equity: Vec<AccountLineItem>,
    pub total_equity: Decimal,
}

// ── Cash Flow Statement ───────────────────────────────────

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CashFlowResponse {
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
    pub operating_activities: Vec<AccountLineItem>,
    pub net_cash_operating: Decimal,
    pub investing_activities: Vec<AccountLineItem>,
    pub net_cash_investing: Decimal,
    pub financing_activities: Vec<AccountLineItem>,
    pub net_cash_financing: Decimal,
    pub net_change_in_cash: Decimal,
}

// ── GST Return ─────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct GstReturnResponse {
    pub return_type: String,
    pub period_month: i32,
    pub period_year: i32,
    pub total_taxable_value: Decimal,
    pub total_cgst: Decimal,
    pub total_sgst: Decimal,
    pub total_igst: Decimal,
    pub invoices: Vec<GstInvoiceLine>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct GstInvoiceLine {
    pub invoice_number: String,
    pub customer_gstin: Option<String>,
    pub taxable_value: Decimal,
    pub cgst: Decimal,
    pub sgst: Decimal,
    pub igst: Decimal,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AccountLineItem {
    pub account_id: i64,
    pub account_code: String,
    pub account_name: String,
    pub amount: Decimal,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct MessageResponse {
    pub message: String,
}

// ── GSTR-1: Outward Supplies (Invoice-level detail) ──────

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Gstr1Response {
    pub period_month: i32,
    pub period_year: i32,
    pub supplier_gstin: Option<String>,
    pub total_taxable_value: Decimal,
    pub total_cgst: Decimal,
    pub total_sgst: Decimal,
    pub total_igst: Decimal,
    pub total_invoices: i64,
    pub invoices: Vec<Gstr1Invoice>,
    pub b2c_summary: Gstr1B2cSummary,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Gstr1Invoice {
    pub invoice_number: String,
    pub invoice_date: NaiveDate,
    pub customer_gstin: Option<String>,
    pub customer_name: String,
    pub place_of_supply: String,
    pub supply_type: String, // "Regular" or "Reverse Charge"
    pub taxable_value: Decimal,
    pub cgst: Decimal,
    pub sgst: Decimal,
    pub igst: Decimal,
    pub invoice_value: Decimal,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Gstr1B2cSummary {
    pub total_taxable_value: Decimal,
    pub total_cgst: Decimal,
    pub total_sgst: Decimal,
    pub total_igst: Decimal,
    pub invoice_count: i64,
}

// ── GSTR-3B: Monthly Summary Return ─────────────────────

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Gstr3bResponse {
    pub period_month: i32,
    pub period_year: i32,
    pub supplier_gstin: Option<String>,
    pub supplier_state: Option<String>,
    pub outward: Gstr3bOutward,
    pub interstate: Gstr3bInterstate,
    pub tax_payable: Gstr3bTaxPayable,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Gstr3bOutward {
    pub taxable_value: Decimal,
    pub cgst: Decimal,
    pub sgst: Decimal,
    pub igst: Decimal,
    pub total: Decimal,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Gstr3bInterstate {
    pub taxable_value: Decimal,
    pub igst: Decimal,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Gstr3bTaxPayable {
    pub total_cgst: Decimal,
    pub total_sgst: Decimal,
    pub total_igst: Decimal,
    pub total_tax: Decimal,
}
