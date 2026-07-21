/// OpenAPI schemas and stub handlers for Accounting endpoints.
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

// ── Request Types ────────────────────────────────────────────────────

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateAccountRequest {
    /// Account code (e.g. "1001")
    pub code: String,
    /// Account name
    pub name: String,
    /// Account type (asset, liability, equity, revenue, expense)
    pub account_type: String,
    /// Parent group account ID
    #[serde(default)]
    pub parent_id: Option<i64>,
    /// Optional description
    #[serde(default)]
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateAccountRequest {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub is_active: Option<bool>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateJournalEntryRequest {
    /// Entry date in YYYY-MM-DD format
    pub entry_date: String,
    /// Description of the journal entry
    pub description: String,
    /// Optional reference type (e.g. "invoice", "payment")
    #[serde(default)]
    pub reference_type: Option<String>,
    /// Optional reference entity ID
    #[serde(default)]
    pub reference_id: Option<i64>,
    /// Journal lines (debit/credit pairs)
    pub lines: Vec<JournalLineRequest>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct JournalLineRequest {
    /// Account ID for this line
    pub account_id: i64,
    /// Debit amount as string
    pub debit: String,
    /// Credit amount as string
    pub credit: String,
    #[serde(default)]
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct TrialBalanceQuery {
    /// Period start date (YYYY-MM-DD)
    pub period_start: String,
    /// Period end date (YYYY-MM-DD)
    pub period_end: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct ProfitLossQuery {
    pub period_start: String,
    pub period_end: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct BalanceSheetQuery {
    /// As-of date for balance sheet (YYYY-MM-DD)
    pub as_of_date: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct GstQuery {
    /// GST period month (1-12)
    pub period_month: u32,
    /// GST period year
    pub period_year: i32,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct ReconciliationQuery {
    pub period_start: String,
    pub period_end: String,
}

// ── Response Types ───────────────────────────────────────────────────

#[derive(Debug, Serialize, ToSchema)]
pub struct AccountResponse {
    pub id: i64,
    pub code: String,
    pub name: String,
    pub account_type: String,
    pub parent_id: Option<i64>,
    pub is_group: bool,
    pub is_active: bool,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct JournalEntryResponse {
    pub id: i64,
    pub entry_number: String,
    pub entry_date: String,
    pub description: String,
    pub reference_type: Option<String>,
    pub reference_id: Option<i64>,
    pub total_debit: String,
    pub total_credit: String,
    pub status: String,
    pub created_by: Option<i64>,
    pub posted_at: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct JournalLineResponse {
    pub id: i64,
    pub account_id: i64,
    pub debit: String,
    pub credit: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TrialBalanceResponse {
    pub accounts: Vec<TrialBalanceLine>,
    pub total_debit: String,
    pub total_credit: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TrialBalanceLine {
    pub account_id: i64,
    pub account_code: String,
    pub account_name: String,
    pub debit: String,
    pub credit: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ProfitLossResponse {
    pub revenue: Vec<ProfitLossLine>,
    pub expenses: Vec<ProfitLossLine>,
    pub net_profit: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ProfitLossLine {
    pub account_id: i64,
    pub account_name: String,
    pub amount: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct BalanceSheetResponse {
    pub assets: Vec<BalanceSheetLine>,
    pub liabilities: Vec<BalanceSheetLine>,
    pub equity: Vec<BalanceSheetLine>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct BalanceSheetLine {
    pub account_id: i64,
    pub account_name: String,
    pub amount: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ReconciliationResponse {
    pub account_id: i64,
    pub period_start: String,
    pub period_end: String,
    pub opening_balance: String,
    pub closing_balance: String,
    pub reconciled: bool,
}

// ── Stub handler functions (for OpenAPI spec only) ───────────────────

/// List all chart-of-accounts entries
#[utoipa::path(
    get,
    path = "/api/v1/accounting/accounts",
    tag = "Accounting",
    responses(
        (status = 200, description = "List of accounts", body = Vec<AccountResponse>),
        (status = 401, description = "Unauthorized")
    ),
    security(("bearer_auth" = []))
)]
pub async fn list_accounts() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Create a new account in the chart of accounts
#[utoipa::path(
    post,
    path = "/api/v1/accounting/accounts",
    tag = "Accounting",
    request_body = CreateAccountRequest,
    responses(
        (status = 201, description = "Account created", body = AccountResponse),
        (status = 409, description = "Account code already exists"),
        (status = 422, description = "Validation error")
    ),
    security(("bearer_auth" = []))
)]
pub async fn create_account() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Update an existing account
#[utoipa::path(
    put,
    path = "/api/v1/accounting/accounts/{id}",
    tag = "Accounting",
    params(("id" = i64, Path, description = "Account ID")),
    request_body = UpdateAccountRequest,
    responses(
        (status = 200, description = "Account updated", body = AccountResponse),
        (status = 404, description = "Account not found")
    ),
    security(("bearer_auth" = []))
)]
pub async fn update_account() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// List journal entries
#[utoipa::path(
    get,
    path = "/api/v1/accounting/journal",
    tag = "Accounting",
    params(
        ("page" = Option<i64>, Query, description = "Page number"),
        ("limit" = Option<i64>, Query, description = "Items per page")
    ),
    responses(
        (status = 200, description = "List of journal entries"),
        (status = 401, description = "Unauthorized")
    ),
    security(("bearer_auth" = []))
)]
pub async fn list_journal_entries() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Create a new journal entry (draft)
#[utoipa::path(
    post,
    path = "/api/v1/accounting/journal",
    tag = "Accounting",
    request_body = CreateJournalEntryRequest,
    responses(
        (status = 201, description = "Journal entry created"),
        (status = 422, description = "Validation error or unbalanced entry")
    ),
    security(("bearer_auth" = []))
)]
pub async fn create_journal_entry() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Post a journal entry to the ledger
#[utoipa::path(
    post,
    path = "/api/v1/accounting/journal/{id}/post",
    tag = "Accounting",
    params(("id" = i64, Path, description = "Journal entry ID")),
    responses(
        (status = 200, description = "Journal entry posted", body = JournalEntryResponse),
        (status = 404, description = "Journal entry not found"),
        (status = 409, description = "Entry already posted or voided")
    ),
    security(("bearer_auth" = []))
)]
pub async fn post_journal_entry() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Void a journal entry
#[utoipa::path(
    post,
    path = "/api/v1/accounting/journal/{id}/void",
    tag = "Accounting",
    params(("id" = i64, Path, description = "Journal entry ID")),
    responses(
        (status = 200, description = "Journal entry voided", body = JournalEntryResponse),
        (status = 404, description = "Journal entry not found")
    ),
    security(("bearer_auth" = []))
)]
pub async fn void_journal_entry() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Generate trial balance for a period
#[utoipa::path(
    get,
    path = "/api/v1/accounting/trial-balance",
    tag = "Accounting",
    params(
        ("period_start" = String, Query, description = "Start date (YYYY-MM-DD)"),
        ("period_end" = String, Query, description = "End date (YYYY-MM-DD)")
    ),
    responses(
        (status = 200, description = "Trial balance", body = TrialBalanceResponse),
        (status = 422, description = "Invalid date range")
    ),
    security(("bearer_auth" = []))
)]
pub async fn generate_trial_balance() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Generate profit-and-loss statement
#[utoipa::path(
    get,
    path = "/api/v1/accounting/statements/profit-loss",
    tag = "Accounting",
    params(
        ("period_start" = String, Query, description = "Start date (YYYY-MM-DD)"),
        ("period_end" = String, Query, description = "End date (YYYY-MM-DD)")
    ),
    responses(
        (status = 200, description = "Profit and loss statement", body = ProfitLossResponse),
        (status = 422, description = "Invalid date range")
    ),
    security(("bearer_auth" = []))
)]
pub async fn profit_and_loss() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Generate balance sheet as of a date
#[utoipa::path(
    get,
    path = "/api/v1/accounting/statements/balance-sheet",
    tag = "Accounting",
    params(
        ("as_of_date" = String, Query, description = "As-of date (YYYY-MM-DD)")
    ),
    responses(
        (status = 200, description = "Balance sheet", body = BalanceSheetResponse),
        (status = 422, description = "Invalid date")
    ),
    security(("bearer_auth" = []))
)]
pub async fn balance_sheet() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Generate GST return data (GSTR1 or GSTR3B)
#[utoipa::path(
    get,
    path = "/api/v1/accounting/gst/{type}",
    tag = "Accounting",
    params(
        ("type" = String, Path, description = "GST return type: GSTR1 or GSTR3B"),
        ("period_month" = u32, Query, description = "Month (1-12)"),
        ("period_year" = i32, Query, description = "Year")
    ),
    responses(
        (status = 200, description = "GST return data"),
        (status = 422, description = "Invalid return type or date")
    ),
    security(("bearer_auth" = []))
)]
pub async fn gst_return() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Reconcile an account for a period
#[utoipa::path(
    get,
    path = "/api/v1/accounting/reconciliation/{account_id}",
    tag = "Accounting",
    params(
        ("account_id" = i64, Path, description = "Account ID to reconcile"),
        ("period_start" = String, Query, description = "Start date (YYYY-MM-DD)"),
        ("period_end" = String, Query, description = "End date (YYYY-MM-DD)")
    ),
    responses(
        (status = 200, description = "Reconciliation result", body = ReconciliationResponse),
        (status = 404, description = "Account not found")
    ),
    security(("bearer_auth" = []))
)]
pub async fn reconcile_account() -> axum::Json<serde_json::Value> {
    unimplemented!()
}
