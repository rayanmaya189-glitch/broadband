//! SeaORM-based controller for the Accounting domain.

use axum::extract::{Json, Path, Query, State};
use validator::Validate;

use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::modules::accounting::request::accounting_request::*;
use crate::modules::accounting::response::accounting_response::*;
use crate::modules::accounting::service::accounting_service_seaorm::AccountingServiceSeaorm;

pub async fn list_accounts(State(state): State<SharedState>) -> Result<Json<Vec<AccountResponse>>, AppError> {
    let svc = AccountingServiceSeaorm::new(&state.db_seaorm);
    Ok(Json(svc.list_accounts().await?))
}

pub async fn create_account(State(state): State<SharedState>, Json(req): Json<CreateAccountRequest>) -> Result<Json<AccountResponse>, AppError> {
    req.validate().map_err(|e| AppError::Validation(e.to_string()))?;
    let svc = AccountingServiceSeaorm::new(&state.db_seaorm);
    Ok(Json(svc.create_account(req).await?))
}

pub async fn list_journal(State(state): State<SharedState>, Query(q): Query<AccountingQuery>) -> Result<Json<Vec<JournalEntryResponse>>, AppError> {
    let svc = AccountingServiceSeaorm::new(&state.db_seaorm);
    Ok(Json(svc.list_journal_entries(q.page.unwrap_or(1), q.per_page.unwrap_or(20)).await?))
}

pub async fn create_journal(State(state): State<SharedState>, Json(req): Json<CreateJournalEntryRequest>) -> Result<Json<JournalEntryDetailResponse>, AppError> {
    req.validate().map_err(|e| AppError::Validation(e.to_string()))?;
    let svc = AccountingServiceSeaorm::new(&state.db_seaorm);
    Ok(Json(svc.create_journal_entry(req).await?))
}

pub async fn post_journal(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<MessageResponse>, AppError> {
    let svc = AccountingServiceSeaorm::new(&state.db_seaorm);
    Ok(Json(svc.post_entry(id).await?))
}

pub async fn void_journal(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<MessageResponse>, AppError> {
    let svc = AccountingServiceSeaorm::new(&state.db_seaorm);
    Ok(Json(svc.void_entry(id).await?))
}
