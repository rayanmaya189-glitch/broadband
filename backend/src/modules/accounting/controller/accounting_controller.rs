//! SeaORM-based controller for the Accounting domain.

use axum::extract::{Json, Path, Query, State};
use validator::Validate;

use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::modules::accounting::request::accounting_request::*;
use crate::modules::accounting::response::accounting_response::*;
use crate::modules::accounting::service::accounting_service::AccountingService;

pub async fn list_accounts(State(state): State<SharedState>) -> Result<Json<Vec<AccountResponse>>, AppError> {
    let svc = AccountingService::new(&state.db_seaorm);
    Ok(Json(svc.list_accounts().await?))
}

pub async fn create_account(State(state): State<SharedState>, Json(req): Json<CreateAccountRequest>) -> Result<Json<AccountResponse>, AppError> {
    req.validate().map_err(|e| AppError::Validation(e.to_string()))?;
    let svc = AccountingService::new(&state.db_seaorm);
    Ok(Json(svc.create_account(req).await?))
}

pub async fn list_journal(State(state): State<SharedState>, Query(q): Query<AccountingQuery>) -> Result<Json<Vec<JournalEntryResponse>>, AppError> {
    let svc = AccountingService::new(&state.db_seaorm);
    Ok(Json(svc.list_journal_entries(q.page.unwrap_or(1), q.per_page.unwrap_or(20)).await?))
}

pub async fn create_journal(State(state): State<SharedState>, Json(req): Json<CreateJournalEntryRequest>) -> Result<Json<JournalEntryDetailResponse>, AppError> {
    req.validate().map_err(|e| AppError::Validation(e.to_string()))?;
    let svc = AccountingService::new(&state.db_seaorm);
    Ok(Json(svc.create_journal_entry(req).await?))
}

pub async fn post_journal(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<MessageResponse>, AppError> {
    let svc = AccountingService::new(&state.db_seaorm);
    Ok(Json(svc.post_entry(id).await?))
}

pub async fn void_journal(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<MessageResponse>, AppError> {
    let svc = AccountingService::new(&state.db_seaorm);
    Ok(Json(svc.void_entry(id).await?))
}

pub async fn get_entry_lines(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<Vec<JournalEntryLineResponse>>, AppError> {
    let svc = AccountingService::new(&state.db_seaorm);
    Ok(Json(svc.get_entry_lines(id).await?))
}

pub async fn trial_balance(State(state): State<SharedState>, Query(q): Query<TrialBalanceQuery>) -> Result<Json<TrialBalanceResponse>, AppError> {
    let svc = AccountingService::new(&state.db_seaorm);
    Ok(Json(svc.trial_balance(q).await?))
}

pub async fn profit_loss(State(state): State<SharedState>, Query(q): Query<TrialBalanceQuery>) -> Result<Json<ProfitLossResponse>, AppError> {
    let svc = AccountingService::new(&state.db_seaorm);
    Ok(Json(svc.profit_loss_statement(q).await?))
}

pub async fn balance_sheet(State(state): State<SharedState>, Query(q): Query<TrialBalanceQuery>) -> Result<Json<BalanceSheetResponse>, AppError> {
    let svc = AccountingService::new(&state.db_seaorm);
    Ok(Json(svc.balance_sheet(q).await?))
}

pub async fn cash_flow(State(state): State<SharedState>, Query(q): Query<TrialBalanceQuery>) -> Result<Json<CashFlowResponse>, AppError> {
    let svc = AccountingService::new(&state.db_seaorm);
    Ok(Json(svc.cash_flow_statement(q).await?))
}

pub async fn gst_return_data(State(state): State<SharedState>, Path(return_type): Path<String>, Query(q): Query<GstQuery>) -> Result<Json<GstReturnResponse>, AppError> {
    let svc = AccountingService::new(&state.db_seaorm);
    Ok(Json(svc.gst_return_data(&return_type, Some(q.month), Some(q.year)).await?))
}

pub async fn gstr1(State(state): State<SharedState>, Query(q): Query<GstQuery>) -> Result<Json<Gstr1Response>, AppError> {
    let svc = AccountingService::new(&state.db_seaorm);
    Ok(Json(svc.gstr1(q.month, q.year).await?))
}

pub async fn gstr3b(State(state): State<SharedState>, Query(q): Query<GstQuery>) -> Result<Json<Gstr3bResponse>, AppError> {
    let svc = AccountingService::new(&state.db_seaorm);
    Ok(Json(svc.gstr3b(q.month, q.year).await?))
}
