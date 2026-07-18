use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::modules::accounting::application::services::AccountingService;
use crate::shared::app_state::AppState;
use crate::shared::errors::AppError;
use crate::shared::middleware::auth::{require_permission, UserContext};
use crate::shared::primitives::PaginationParams;

// ── Request Types ──

#[derive(Debug, Deserialize)]
pub struct CreateAccountRequest {
    pub code: String,
    pub name: String,
    pub account_type: String,
    pub parent_id: Option<i64>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateAccountRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct CreateJournalEntryRequest {
    pub entry_date: String,
    pub description: String,
    pub reference_type: Option<String>,
    pub reference_id: Option<i64>,
    pub lines: Vec<JournalLineRequest>,
}

#[derive(Debug, Deserialize)]
pub struct JournalLineRequest {
    pub account_id: i64,
    pub debit: String,
    pub credit: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TrialBalanceQuery {
    pub period_start: String,
    pub period_end: String,
}

#[derive(Debug, Deserialize)]
pub struct ProfitLossQuery {
    pub period_start: String,
    pub period_end: String,
}

#[derive(Debug, Deserialize)]
pub struct BalanceSheetQuery {
    pub as_of_date: String,
}

#[derive(Debug, Deserialize)]
pub struct GstQuery {
    pub period_month: u32,
    pub period_year: i32,
}

// ── Response Types ──

#[derive(Debug, Serialize)]
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

impl From<crate::modules::accounting::domain::entities::chart_of_accounts::Model>
    for AccountResponse
{
    fn from(m: crate::modules::accounting::domain::entities::chart_of_accounts::Model) -> Self {
        Self {
            id: m.id,
            code: m.code,
            name: m.name,
            account_type: m.account_type,
            parent_id: m.parent_id,
            is_group: m.is_group,
            is_active: m.is_active,
            description: m.description,
        }
    }
}

#[derive(Debug, Serialize)]
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

impl From<crate::modules::accounting::domain::entities::journal_entry::Model>
    for JournalEntryResponse
{
    fn from(m: crate::modules::accounting::domain::entities::journal_entry::Model) -> Self {
        Self {
            id: m.id,
            entry_number: m.entry_number,
            entry_date: m.entry_date.to_string(),
            description: m.description,
            reference_type: m.reference_type,
            reference_id: m.reference_id,
            total_debit: m.total_debit.to_string(),
            total_credit: m.total_credit.to_string(),
            status: m.status,
            created_by: m.created_by,
            posted_at: m.posted_at.map(|d| d.to_rfc3339()),
            created_at: m.created_at.to_rfc3339(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct JournalLineResponse {
    pub id: i64,
    pub account_id: i64,
    pub debit: String,
    pub credit: String,
    pub description: Option<String>,
}

impl From<crate::modules::accounting::domain::entities::journal_entry_line::Model>
    for JournalLineResponse
{
    fn from(m: crate::modules::accounting::domain::entities::journal_entry_line::Model) -> Self {
        Self {
            id: m.id,
            account_id: m.account_id,
            debit: m.debit.to_string(),
            credit: m.credit.to_string(),
            description: m.description,
        }
    }
}

// ── Handlers ──

/// GET /api/v1/accounting/accounts
pub async fn list_accounts(
    State(state): State<Arc<AppState>>,
    _user: UserContext,
) -> Result<Json<serde_json::Value>, AppError> {
    let accounts = AccountingService::list_accounts(&state.db).await?;
    let resp: Vec<AccountResponse> = accounts.into_iter().map(AccountResponse::from).collect();
    Ok(Json(
        serde_json::json!({ "items": resp, "total": resp.len() }),
    ))
}

/// POST /api/v1/accounting/accounts
pub async fn create_account(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Json(req): Json<CreateAccountRequest>,
) -> Result<(StatusCode, Json<AccountResponse>), AppError> {
    require_permission(&user, "accounting.accounts.create")
        .map_err(|e| AppError::Forbidden(e.1))?;
    let account = AccountingService::create_account(
        &state.db,
        req.code,
        req.name,
        req.account_type,
        req.parent_id,
        req.description,
    )
    .await?;
    Ok((StatusCode::CREATED, Json(AccountResponse::from(account))))
}

/// PUT /api/v1/accounting/accounts/:id
pub async fn update_account(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Path(id): Path<i64>,
    Json(req): Json<UpdateAccountRequest>,
) -> Result<Json<AccountResponse>, AppError> {
    require_permission(&user, "accounting.accounts.update")
        .map_err(|e| AppError::Forbidden(e.1))?;
    let account =
        AccountingService::update_account(&state.db, id, req.name, req.description, req.is_active)
            .await?;
    Ok(Json(AccountResponse::from(account)))
}

/// GET /api/v1/accounting/journal
pub async fn list_journal_entries(
    State(state): State<Arc<AppState>>,
    _user: UserContext,
    Query(params): Query<PaginationParams>,
) -> Result<Json<serde_json::Value>, AppError> {
    let entries = AccountingService::list_journal_entries(&state.db, None).await?;
    let total = entries.len() as u64;
    let resp: Vec<JournalEntryResponse> = entries
        .into_iter()
        .skip(((params.page() - 1) * params.limit()) as usize)
        .take(params.limit() as usize)
        .map(JournalEntryResponse::from)
        .collect();
    Ok(Json(serde_json::json!({ "items": resp, "total": total })))
}

/// POST /api/v1/accounting/journal
pub async fn create_journal_entry(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Json(req): Json<CreateJournalEntryRequest>,
) -> Result<(StatusCode, Json<serde_json::Value>), AppError> {
    require_permission(&user, "accounting.journal.create").map_err(|e| AppError::Forbidden(e.1))?;

    let entry_date: chrono::NaiveDate = req.entry_date.parse().map_err(|_| {
        AppError::Validation("Invalid entry_date format, expected YYYY-MM-DD".into())
    })?;

    let lines: Vec<crate::modules::accounting::application::services::CreateJournalLine> = req
        .lines
        .into_iter()
        .map(|l| {
            Ok(
                crate::modules::accounting::application::services::CreateJournalLine {
                    account_id: l.account_id,
                    debit: l
                        .debit
                        .parse()
                        .map_err(|_| AppError::Validation("Invalid debit amount".into()))?,
                    credit: l
                        .credit
                        .parse()
                        .map_err(|_| AppError::Validation("Invalid credit amount".into()))?,
                    description: l.description,
                },
            )
        })
        .collect::<Result<Vec<_>, AppError>>()?;

    let entry = AccountingService::create_journal_entry(
        &state.db,
        entry_date,
        req.description,
        req.reference_type,
        req.reference_id,
        lines,
        Some(user.user_id),
    )
    .await?;

    Ok((
        StatusCode::CREATED,
        Json(serde_json::json!({
            "id": entry.id,
            "entry_number": entry.entry_number,
            "total_debit": entry.total_debit.to_string(),
            "total_credit": entry.total_credit.to_string(),
            "status": entry.status,
        })),
    ))
}

/// POST /api/v1/accounting/journal/:id/post
pub async fn post_journal_entry(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Path(id): Path<i64>,
) -> Result<Json<JournalEntryResponse>, AppError> {
    require_permission(&user, "accounting.journal.post").map_err(|e| AppError::Forbidden(e.1))?;
    let entry = AccountingService::post_journal_entry(&state.db, id, Some(user.user_id)).await?;
    Ok(Json(JournalEntryResponse::from(entry)))
}

/// POST /api/v1/accounting/journal/:id/void
pub async fn void_journal_entry(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Path(id): Path<i64>,
) -> Result<Json<JournalEntryResponse>, AppError> {
    require_permission(&user, "accounting.journal.void").map_err(|e| AppError::Forbidden(e.1))?;
    let entry = AccountingService::void_journal_entry(&state.db, id).await?;
    Ok(Json(JournalEntryResponse::from(entry)))
}

/// GET /api/v1/accounting/trial-balance
pub async fn generate_trial_balance(
    State(state): State<Arc<AppState>>,
    _user: UserContext,
    Query(q): Query<TrialBalanceQuery>,
) -> Result<Json<serde_json::Value>, AppError> {
    let start: chrono::NaiveDate = q
        .period_start
        .parse()
        .map_err(|_| AppError::Validation("Invalid period_start".into()))?;
    let end: chrono::NaiveDate = q
        .period_end
        .parse()
        .map_err(|_| AppError::Validation("Invalid period_end".into()))?;
    let rows = AccountingService::generate_trial_balance(&state.db, start, end).await?;
    Ok(Json(
        serde_json::json!({ "period_start": start, "period_end": end, "rows": rows }),
    ))
}

/// GET /api/v1/accounting/statements/profit-loss
pub async fn profit_and_loss(
    State(state): State<Arc<AppState>>,
    _user: UserContext,
    Query(q): Query<ProfitLossQuery>,
) -> Result<Json<serde_json::Value>, AppError> {
    let start: chrono::NaiveDate = q
        .period_start
        .parse()
        .map_err(|_| AppError::Validation("Invalid period_start".into()))?;
    let end: chrono::NaiveDate = q
        .period_end
        .parse()
        .map_err(|_| AppError::Validation("Invalid period_end".into()))?;
    let stmt = AccountingService::profit_and_loss(&state.db, start, end).await?;
    Ok(Json(serde_json::to_value(stmt).unwrap_or_default()))
}

/// GET /api/v1/accounting/statements/balance-sheet
pub async fn balance_sheet(
    State(state): State<Arc<AppState>>,
    _user: UserContext,
    Query(q): Query<BalanceSheetQuery>,
) -> Result<Json<serde_json::Value>, AppError> {
    let date: chrono::NaiveDate = q
        .as_of_date
        .parse()
        .map_err(|_| AppError::Validation("Invalid as_of_date".into()))?;
    let stmt = AccountingService::balance_sheet(&state.db, date).await?;
    Ok(Json(serde_json::to_value(stmt).unwrap_or_default()))
}

/// GET /api/v1/accounting/gst/:type
pub async fn gst_return(
    State(state): State<Arc<AppState>>,
    _user: UserContext,
    Path(return_type): Path<String>,
    Query(q): Query<GstQuery>,
) -> Result<Json<serde_json::Value>, AppError> {
    if !["GSTR1", "GSTR3B"].contains(&return_type.as_str()) {
        return Err(AppError::Validation(
            "Invalid GST return type, must be GSTR1 or GSTR3B".into(),
        ));
    }
    let data = AccountingService::generate_gst_return(
        &state.db,
        return_type,
        q.period_month,
        q.period_year,
    )
    .await?;
    Ok(Json(serde_json::to_value(data).unwrap_or_default()))
}
