use crate::modules::accounting::domain::entities::{
    chart_of_accounts, journal_entry, journal_entry_line, JournalEntryActiveModel,
    JournalEntryLineActiveModel,
};
use crate::shared::errors::AppError;
use async_trait::async_trait;
use sea_orm::DatabaseConnection;

/// Repository trait for accounting data access (DDD compliance).
/// Application layer defines the contract; infrastructure implements it.
#[async_trait]
pub trait AccountingRepositoryTrait: Send + Sync {
    async fn list_accounts(
        &self,
        db: &DatabaseConnection,
    ) -> Result<Vec<chart_of_accounts::Model>, AppError>;
    async fn get_account(
        &self,
        db: &DatabaseConnection,
        id: i64,
    ) -> Result<chart_of_accounts::Model, AppError>;
    async fn create_account(
        &self,
        db: &DatabaseConnection,
        code: String,
        name: String,
        account_type: String,
        parent_id: Option<i64>,
        description: Option<String>,
    ) -> Result<chart_of_accounts::Model, AppError>;
    async fn update_account(
        &self,
        db: &DatabaseConnection,
        id: i64,
        name: Option<String>,
        description: Option<String>,
        is_active: Option<bool>,
    ) -> Result<chart_of_accounts::Model, AppError>;

    async fn list_journal_entries(
        &self,
        db: &DatabaseConnection,
        status: Option<String>,
    ) -> Result<Vec<journal_entry::Model>, AppError>;
    async fn get_journal_entry(
        &self,
        db: &DatabaseConnection,
        id: i64,
    ) -> Result<journal_entry::Model, AppError>;
    async fn get_journal_entry_lines(
        &self,
        db: &DatabaseConnection,
        entry_id: i64,
    ) -> Result<Vec<journal_entry_line::Model>, AppError>;
    async fn count_journal_entries(&self, db: &DatabaseConnection) -> Result<u64, AppError>;
    async fn create_journal_entry(
        &self,
        db: &DatabaseConnection,
        entry: JournalEntryActiveModel,
    ) -> Result<journal_entry::Model, AppError>;
    async fn create_journal_entry_line(
        &self,
        db: &DatabaseConnection,
        line: JournalEntryLineActiveModel,
    ) -> Result<journal_entry_line::Model, AppError>;
    async fn update_journal_entry(
        &self,
        db: &DatabaseConnection,
        entry: JournalEntryActiveModel,
    ) -> Result<journal_entry::Model, AppError>;

    async fn get_posted_entries_in_period(
        &self,
        db: &DatabaseConnection,
        start: chrono::NaiveDate,
        end: chrono::NaiveDate,
    ) -> Result<Vec<journal_entry::Model>, AppError>;
    async fn get_lines_for_account_in_entries(
        &self,
        db: &DatabaseConnection,
        account_id: i64,
        entry_ids: &[i64],
    ) -> Result<Vec<journal_entry_line::Model>, AppError>;
}
