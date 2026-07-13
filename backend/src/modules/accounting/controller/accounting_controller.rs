use axum::extract::{Json, Path, Query, State};
use validator::Validate;
use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::modules::accounting::request::accounting_request::*;
use crate::modules::accounting::response::accounting_response::*;
use crate::modules::accounting::service::accounting_service::AccountingService;

// ── Chart of Accounts ───────────────────────────────────────

#[utoipa::path(
    get,
    path = "/api/v1/accounting/accounts",
    tag = "Accounting",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "List of accounts", body = Vec<AccountResponse>),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn list_accounts(State(state): State<SharedState>) -> Result<Json<Vec<AccountResponse>>, AppError> {
    let svc = AccountingService::new(&state.db);
    Ok(Json(svc.list_accounts().await?))
}

#[utoipa::path(
    post,
    path = "/api/v1/accounting/accounts",
    tag = "Accounting",
    security(("bearer_auth" = [])),
    request_body = CreateAccountRequest,
    responses(
        (status = 200, description = "Account created", body = AccountResponse),
        (status = 422, description = "Validation error")
    )
)]
pub async fn create_account(State(state): State<SharedState>, Json(req): Json<CreateAccountRequest>) -> Result<Json<AccountResponse>, AppError> {
    req.validate()?;
    let svc = AccountingService::new(&state.db);
    Ok(Json(svc.create_account(req).await?))
}

// ── Journal Entries ─────────────────────────────────────────

#[utoipa::path(
    get,
    path = "/api/v1/accounting/journal",
    tag = "Accounting",
    security(("bearer_auth" = [])),
    params(
        ("page" = Option<i64>, Query, description = "Page number"),
        ("per_page" = Option<i64>, Query, description = "Items per page")
    ),
    responses(
        (status = 200, description = "List of journal entries"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn list_journal(State(state): State<SharedState>, Query(q): Query<AccountingQuery>) -> Result<Json<Vec<JournalEntryResponse>>, AppError> {
    let svc = AccountingService::new(&state.db);
    Ok(Json(svc.list_journal_entries(q.page.unwrap_or(1), q.per_page.unwrap_or(20)).await?))
}

#[utoipa::path(
    post,
    path = "/api/v1/accounting/journal",
    tag = "Accounting",
    security(("bearer_auth" = [])),
    request_body = CreateJournalEntryRequest,
    responses(
        (status = 200, description = "Journal entry created", body = JournalEntryDetailResponse),
        (status = 422, description = "Validation error")
    )
)]
pub async fn create_journal(State(state): State<SharedState>, Json(req): Json<CreateJournalEntryRequest>) -> Result<Json<JournalEntryDetailResponse>, AppError> {
    req.validate()?;
    let svc = AccountingService::new(&state.db);
    Ok(Json(svc.create_journal_entry(req).await?))
}

#[utoipa::path(
    get,
    path = "/api/v1/accounting/journal/{id}/lines",
    tag = "Accounting",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Journal entry ID")),
    responses(
        (status = 200, description = "List of journal entry lines", body = Vec<JournalEntryLineResponse>),
        (status = 404, description = "Entry not found")
    )
)]
pub async fn get_entry_lines(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<Vec<JournalEntryLineResponse>>, AppError> {
    let svc = AccountingService::new(&state.db);
    Ok(Json(svc.get_entry_lines(id).await?))
}

#[utoipa::path(
    post,
    path = "/api/v1/accounting/journal/{id}/post",
    tag = "Accounting",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Journal entry ID")),
    responses(
        (status = 200, description = "Entry posted"),
        (status = 404, description = "Entry not found")
    )
)]
pub async fn post_journal(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<MessageResponse>, AppError> {
    let svc = AccountingService::new(&state.db);
    Ok(Json(svc.post_entry(id).await?))
}

#[utoipa::path(
    post,
    path = "/api/v1/accounting/journal/{id}/void",
    tag = "Accounting",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Journal entry ID")),
    responses(
        (status = 200, description = "Entry voided"),
        (status = 404, description = "Entry not found")
    )
)]
pub async fn void_journal(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<MessageResponse>, AppError> {
    let svc = AccountingService::new(&state.db);
    Ok(Json(svc.void_entry(id).await?))
}

// ── Trial Balance ───────────────────────────────────────────

#[utoipa::path(
    get,
    path = "/api/v1/accounting/trial-balance",
    tag = "Accounting",
    security(("bearer_auth" = [])),
    params(
        ("period_start" = Option<String>, Query, description = "Period start date"),
        ("period_end" = Option<String>, Query, description = "Period end date")
    ),
    responses(
        (status = 200, description = "Trial balance", body = TrialBalanceResponse),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn trial_balance(State(state): State<SharedState>, Query(q): Query<TrialBalanceQuery>) -> Result<Json<TrialBalanceResponse>, AppError> {
    let svc = AccountingService::new(&state.db);
    Ok(Json(svc.trial_balance(q).await?))
}

// ── Financial Statements ────────────────────────────────────

#[utoipa::path(
    get,
    path = "/api/v1/accounting/statements/profit-loss",
    tag = "Accounting",
    security(("bearer_auth" = [])),
    params(
        ("period_start" = Option<String>, Query, description = "Period start date"),
        ("period_end" = Option<String>, Query, description = "Period end date")
    ),
    responses(
        (status = 200, description = "Profit & Loss statement", body = ProfitLossResponse),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn profit_loss(State(state): State<SharedState>, Query(q): Query<TrialBalanceQuery>) -> Result<Json<ProfitLossResponse>, AppError> {
    let svc = AccountingService::new(&state.db);
    Ok(Json(svc.profit_loss_statement(q).await?))
}

#[utoipa::path(
    get,
    path = "/api/v1/accounting/statements/balance-sheet",
    tag = "Accounting",
    security(("bearer_auth" = [])),
    params(
        ("period_start" = Option<String>, Query, description = "Period start date"),
        ("period_end" = Option<String>, Query, description = "Period end date")
    ),
    responses(
        (status = 200, description = "Balance sheet", body = BalanceSheetResponse),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn balance_sheet(State(state): State<SharedState>, Query(q): Query<TrialBalanceQuery>) -> Result<Json<BalanceSheetResponse>, AppError> {
    let svc = AccountingService::new(&state.db);
    Ok(Json(svc.balance_sheet(q).await?))
}

#[utoipa::path(
    get,
    path = "/api/v1/accounting/statements/cash-flow",
    tag = "Accounting",
    security(("bearer_auth" = [])),
    params(
        ("period_start" = Option<String>, Query, description = "Period start date"),
        ("period_end" = Option<String>, Query, description = "Period end date")
    ),
    responses(
        (status = 200, description = "Cash flow statement", body = CashFlowResponse),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn cash_flow(State(state): State<SharedState>, Query(q): Query<TrialBalanceQuery>) -> Result<Json<CashFlowResponse>, AppError> {
    let svc = AccountingService::new(&state.db);
    Ok(Json(svc.cash_flow_statement(q).await?))
}

// ── GST Returns ─────────────────────────────────────────────

#[utoipa::path(
    get,
    path = "/api/v1/accounting/gst/{return_type}",
    tag = "Accounting",
    security(("bearer_auth" = [])),
    params(
        ("return_type" = String, Path, description = "GST return type (GSTR1, GSTR3B)"),
        ("month" = Option<i32>, Query, description = "Month"),
        ("year" = Option<i32>, Query, description = "Year")
    ),
    responses(
        (status = 200, description = "GST return data", body = GstReturnResponse),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn gst_return_data(State(state): State<SharedState>, Path(return_type): Path<String>, Query(q): Query<GstQuery>) -> Result<Json<GstReturnResponse>, AppError> {
    let svc = AccountingService::new(&state.db);
    Ok(Json(svc.gst_return_data(&return_type, q.month, q.year).await?))
}
