use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, Set,
};

use crate::modules::accounting::application::traits::AccountingRepositoryTrait;
use crate::modules::accounting::domain::entities::{
    chart_of_accounts, journal_entry, journal_entry_line, ChartOfAccounts,
    ChartOfAccountsActiveModel, JournalEntry, JournalEntryActiveModel, JournalEntryLine,
    JournalEntryLineActiveModel,
};
use crate::shared::errors::AppError;
use async_trait::async_trait;
use chrono::Utc;

pub struct AccountingRepository;

impl Default for AccountingRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl AccountingRepository {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl AccountingRepositoryTrait for AccountingRepository {
    async fn list_accounts(
        &self,
        db: &DatabaseConnection,
    ) -> Result<Vec<chart_of_accounts::Model>, AppError> {
        Ok(ChartOfAccounts::find()
            .order_by_asc(chart_of_accounts::Column::Code)
            .all(db)
            .await?)
    }

    async fn get_account(
        &self,
        db: &DatabaseConnection,
        id: i64,
    ) -> Result<chart_of_accounts::Model, AppError> {
        ChartOfAccounts::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Account {} not found", id)))
    }

    async fn create_account(
        &self,
        db: &DatabaseConnection,
        code: String,
        name: String,
        account_type: String,
        parent_id: Option<i64>,
        description: Option<String>,
    ) -> Result<chart_of_accounts::Model, AppError> {
        let now = Utc::now();
        let account = ChartOfAccountsActiveModel {
            code: Set(code),
            name: Set(name),
            account_type: Set(account_type),
            parent_id: Set(parent_id),
            is_group: Set(false),
            is_active: Set(true),
            description: Set(description),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };
        Ok(account.insert(db).await?)
    }

    async fn update_account(
        &self,
        db: &DatabaseConnection,
        id: i64,
        name: Option<String>,
        description: Option<String>,
        is_active: Option<bool>,
    ) -> Result<chart_of_accounts::Model, AppError> {
        let existing = ChartOfAccounts::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Account {} not found", id)))?;
        let mut active: ChartOfAccountsActiveModel = existing.into();
        if let Some(n) = name {
            active.name = Set(n);
        }
        if let Some(d) = description {
            active.description = Set(Some(d));
        }
        if let Some(a) = is_active {
            active.is_active = Set(a);
        }
        active.updated_at = Set(Utc::now());
        Ok(active.update(db).await?)
    }

    async fn list_journal_entries(
        &self,
        db: &DatabaseConnection,
        status: Option<String>,
    ) -> Result<Vec<journal_entry::Model>, AppError> {
        let mut query = JournalEntry::find();
        if let Some(s) = status {
            query = query.filter(journal_entry::Column::Status.eq(s));
        }
        Ok(query
            .order_by_desc(journal_entry::Column::CreatedAt)
            .all(db)
            .await?)
    }

    async fn get_journal_entry(
        &self,
        db: &DatabaseConnection,
        id: i64,
    ) -> Result<journal_entry::Model, AppError> {
        JournalEntry::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Journal entry {} not found", id)))
    }

    async fn get_journal_entry_lines(
        &self,
        db: &DatabaseConnection,
        entry_id: i64,
    ) -> Result<Vec<journal_entry_line::Model>, AppError> {
        Ok(JournalEntryLine::find()
            .filter(journal_entry_line::Column::JournalEntryId.eq(entry_id))
            .all(db)
            .await?)
    }

    async fn count_journal_entries(&self, db: &DatabaseConnection) -> Result<u64, AppError> {
        Ok(JournalEntry::find().count(db).await?)
    }

    async fn create_journal_entry(
        &self,
        db: &DatabaseConnection,
        entry: JournalEntryActiveModel,
    ) -> Result<journal_entry::Model, AppError> {
        Ok(entry.insert(db).await?)
    }

    async fn create_journal_entry_line(
        &self,
        db: &DatabaseConnection,
        line: JournalEntryLineActiveModel,
    ) -> Result<journal_entry_line::Model, AppError> {
        Ok(line.insert(db).await?)
    }

    async fn update_journal_entry(
        &self,
        db: &DatabaseConnection,
        entry: JournalEntryActiveModel,
    ) -> Result<journal_entry::Model, AppError> {
        Ok(entry.update(db).await?)
    }

    async fn get_posted_entries_in_period(
        &self,
        db: &DatabaseConnection,
        start: chrono::NaiveDate,
        end: chrono::NaiveDate,
    ) -> Result<Vec<journal_entry::Model>, AppError> {
        Ok(JournalEntry::find()
            .filter(journal_entry::Column::Status.eq("posted"))
            .filter(journal_entry::Column::EntryDate.gte(start))
            .filter(journal_entry::Column::EntryDate.lte(end))
            .all(db)
            .await?)
    }

    async fn get_lines_for_account_in_entries(
        &self,
        db: &DatabaseConnection,
        account_id: i64,
        entry_ids: &[i64],
    ) -> Result<Vec<journal_entry_line::Model>, AppError> {
        if entry_ids.is_empty() {
            return Ok(Vec::new());
        }
        Ok(JournalEntryLine::find()
            .filter(journal_entry_line::Column::AccountId.eq(account_id))
            .filter(journal_entry_line::Column::JournalEntryId.is_in(entry_ids.to_vec()))
            .all(db)
            .await?)
    }
}
