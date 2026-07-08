use sqlx::PgPool;
use crate::common::errors::app_error::AppError;
use crate::modules::accounting::repository::accounting_repository::AccountingRepository;
use crate::modules::accounting::request::accounting_request::*;
use crate::modules::accounting::response::accounting_response::*;

pub struct AccountingService<'a> { repo: AccountingRepository<'a> }
impl<'a> AccountingService<'a> {
    pub fn new(pool: &'a PgPool) -> Self { Self { repo: AccountingRepository::new(pool) } }
    pub async fn list_accounts(&self) -> Result<Vec<AccountResponse>, AppError> { let a = self.repo.list_accounts().await?; Ok(a.iter().map(|x| AccountResponse { id: x.id, code: x.code.clone(), name: x.name.clone(), account_type: x.account_type.clone(), is_active: x.is_active, created_at: x.created_at }).collect()) }
    pub async fn create_account(&self, req: CreateAccountRequest) -> Result<AccountResponse, AppError> { let a = self.repo.create_account(&req.code, &req.name, &req.account_type, req.parent_id).await?; Ok(AccountResponse { id: a.id, code: a.code, name: a.name, account_type: a.account_type, is_active: a.is_active, created_at: a.created_at }) }
    pub async fn list_journal_entries(&self, page: i64, per_page: i64) -> Result<Vec<JournalEntryResponse>, AppError> { let (e, _) = self.repo.list_journal_entries(page, per_page).await?; Ok(e.iter().map(|x| JournalEntryResponse { id: x.id, entry_number: x.entry_number.clone(), entry_date: x.entry_date, description: x.description.clone(), total_debit: x.total_debit, total_credit: x.total_credit, status: x.status.clone(), created_at: x.created_at }).collect()) }
    pub async fn create_journal_entry(&self, req: CreateJournalEntryRequest) -> Result<JournalEntryResponse, AppError> {
        let total_debit: rust_decimal::Decimal = req.lines.iter().filter_map(|l| l.debit).sum();
        let total_credit: rust_decimal::Decimal = req.lines.iter().filter_map(|l| l.credit).sum();
        let entry_number = format!("JE-{}-{:04}", req.entry_date.format("%Y%m"), chrono::Utc::now().timestamp_micros() % 10000);
        let e = self.repo.create_journal_entry(&entry_number, req.entry_date, &req.description, total_debit, total_credit).await?;
        Ok(JournalEntryResponse { id: e.id, entry_number: e.entry_number, entry_date: e.entry_date, description: e.description, total_debit: e.total_debit, total_credit: e.total_credit, status: e.status, created_at: e.created_at })
    }
    pub async fn post_entry(&self, id: i64) -> Result<MessageResponse, AppError> { if !self.repo.post_entry(id).await? { return Err(AppError::NotFound("Entry not found".into())); } Ok(MessageResponse { message: "Posted".into() }) }
    pub async fn void_entry(&self, id: i64) -> Result<MessageResponse, AppError> { if !self.repo.void_entry(id).await? { return Err(AppError::NotFound("Entry not found".into())); } Ok(MessageResponse { message: "Voided".into() }) }
}
