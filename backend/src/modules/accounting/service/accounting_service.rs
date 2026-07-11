//! SeaORM-based service for the Accounting domain.

use sea_orm::DatabaseConnection;

use crate::common::errors::app_error::AppError;
use crate::modules::accounting::repository::accounting_repository::AccountingRepository;
use crate::modules::accounting::request::accounting_request::*;
use crate::modules::accounting::response::accounting_response::*;

pub struct AccountingService<'a> {
    repo: AccountingRepository<'a>,
}

impl<'a> AccountingService<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { repo: AccountingRepository::new(db) }
    }

    pub async fn list_accounts(&self) -> Result<Vec<AccountResponse>, AppError> {
        let accounts = self.repo.list_accounts().await?;
        Ok(accounts.into_iter().map(|a| AccountResponse {
            id: a.id, code: a.code, name: a.name, account_type: a.account_type,
            is_active: a.is_active, created_at: a.created_at.into(),
        }).collect())
    }

    pub async fn create_account(&self, req: CreateAccountRequest) -> Result<AccountResponse, AppError> {
        let a = self.repo.create_account(&req.code, &req.name, &req.account_type, req.parent_id).await?;
        Ok(AccountResponse { id: a.id, code: a.code, name: a.name, account_type: a.account_type, is_active: a.is_active, created_at: a.created_at.into() })
    }

    pub async fn list_journal_entries(&self, page: i64, per_page: i64) -> Result<Vec<JournalEntryResponse>, AppError> {
        let (entries, _) = self.repo.list_journal_entries(page, per_page).await?;
        Ok(entries.into_iter().map(|e| JournalEntryResponse {
            id: e.id, entry_number: e.entry_number, entry_date: e.entry_date,
            description: e.description, total_debit: e.total_debit, total_credit: e.total_credit,
            status: e.status, created_at: e.created_at.into(),
        }).collect())
    }

    pub async fn create_journal_entry(&self, req: CreateJournalEntryRequest) -> Result<JournalEntryDetailResponse, AppError> {
        let total_debit: rust_decimal::Decimal = req.lines.iter().filter_map(|l| l.debit).sum();
        let total_credit: rust_decimal::Decimal = req.lines.iter().filter_map(|l| l.credit).sum();
        if total_debit != total_credit {
            return Err(AppError::Validation("Total debit must equal total credit".into()));
        }
        let entry_number = format!("JE-{}-{:04}", req.entry_date.format("%Y%m"), chrono::Utc::now().timestamp_micros() % 10000);
        let e = self.repo.create_journal_entry(&entry_number, req.entry_date, &req.description, total_debit, total_credit).await?;
        let mut lines = Vec::new();
        for line in &req.lines {
            let l = self.repo.create_entry_line(e.id, line.account_id, line.debit.unwrap_or(rust_decimal::Decimal::ZERO), line.credit.unwrap_or(rust_decimal::Decimal::ZERO), None).await?;
            lines.push(JournalEntryLineResponse { id: l.id, journal_entry_id: l.journal_entry_id, account_id: l.account_id, debit: l.debit, credit: l.credit, description: l.description, created_at: l.created_at.into() });
        }
        Ok(JournalEntryDetailResponse {
            entry: JournalEntryResponse { id: e.id, entry_number: e.entry_number, entry_date: e.entry_date, description: e.description, total_debit: e.total_debit, total_credit: e.total_credit, status: e.status, created_at: e.created_at.into() },
            lines,
        })
    }

    pub async fn post_entry(&self, id: i64) -> Result<MessageResponse, AppError> {
        if !self.repo.post_entry(id).await? { return Err(AppError::NotFound("Entry not found".into())); }
        Ok(MessageResponse { message: "Posted".into() })
    }

    pub async fn void_entry(&self, id: i64) -> Result<MessageResponse, AppError> {
        if !self.repo.void_entry(id).await? { return Err(AppError::NotFound("Entry not found".into())); }
        Ok(MessageResponse { message: "Voided".into() })
    }
}
