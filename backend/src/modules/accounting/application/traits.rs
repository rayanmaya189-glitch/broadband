use async_trait::async_trait;
use sea_orm::{DatabaseConnection};
use crate::shared::errors::AppError;

pub type JournalEntryModel = crate::modules::accounting::domain::entities::journal_entry::Model;
pub type JournalEntryLineModel = crate::modules::accounting::domain::entities::journal_entry_line::Model;
pub type ChartOfAccountsModel = crate::modules::accounting::domain::entities::chart_of_accounts::Model;

#[async_trait]
pub trait AccountingServiceTrait: Send + Sync {
    async fn list_journal_entries(
        &self,
        db: &DatabaseConnection,
    ) -> Result<Vec<JournalEntryModel>, AppError>;

    async fn create_journal_entry(
        &self,
        db: &DatabaseConnection,
        entry_date: chrono::NaiveDate,
        description: String,
    ) -> Result<JournalEntryModel, AppError>;

    async fn post_journal_entry(
        &self,
        db: &DatabaseConnection,
        id: i64,
    ) -> Result<JournalEntryModel, AppError>;

    async fn list_accounts(
        &self,
        db: &DatabaseConnection,
    ) -> Result<Vec<ChartOfAccountsModel>, AppError>;

    async fn create_account(
        &self,
        db: &DatabaseConnection,
        code: String,
        name: String,
        account_type: String,
    ) -> Result<ChartOfAccountsModel, AppError>;
}
