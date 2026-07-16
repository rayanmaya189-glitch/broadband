use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait};
use crate::shared::errors::AppError;

use crate::modules::accounting::domain::entities::{
    chart_of_accounts, journal_entry, journal_entry_line,
    ChartOfAccounts, JournalEntry, JournalEntryLine,
};

pub struct AccountingRepository;

impl AccountingRepository {
    pub async fn list_accounts(db: &DatabaseConnection) -> Result<Vec<chart_of_accounts::Model>, AppError> {
        Ok(ChartOfAccounts::find().all(db).await?)
    }

    pub async fn list_journal_entries(db: &DatabaseConnection, status: Option<String>) -> Result<Vec<journal_entry::Model>, AppError> {
        let mut query = JournalEntry::find();
        if let Some(s) = status {
            query = query.filter(journal_entry::Column::Status.eq(s));
        }
        Ok(query.all(db).await?)
    }

    pub async fn list_entry_lines(db: &DatabaseConnection, entry_id: i64) -> Result<Vec<journal_entry_line::Model>, AppError> {
        Ok(JournalEntryLine::find()
            .filter(journal_entry_line::Column::JournalEntryId.eq(entry_id))
            .all(db)
            .await?)
    }
}
