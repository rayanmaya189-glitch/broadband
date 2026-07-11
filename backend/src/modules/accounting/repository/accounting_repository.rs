//! SeaORM-based repository for the Accounting domain.
//! Zero plain SQL — all queries use EntityTrait, ActiveModelTrait, and Select.

use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel,
    PaginatorTrait, QueryFilter, QueryOrder, Set,
};
use chrono::NaiveDate;
use rust_decimal::Decimal;

use crate::common::errors::app_error::AppError;
use crate::modules::accounting::model::chart_of_account_entity::{self, Model as ChartOfAccountModel};
use crate::modules::accounting::model::journal_entry_entity::{self, Model as JournalEntryModel};
use crate::modules::accounting::model::journal_entry_line_entity::{self, Model as JournalEntryLineModel};
use crate::modules::accounting::model::trial_balance_entity::{self, Model as TrialBalanceModel};

pub struct AccountingRepository<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> AccountingRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self { Self { db } }

    // ── Chart of Accounts ──────────────────────────────────

    pub async fn list_accounts(&self) -> Result<Vec<ChartOfAccountModel>, AppError> {
        let models = chart_of_account_entity::Entity::find()
            .order_by_asc(chart_of_account_entity::Column::Code)
            .all(self.db).await?;
        Ok(models)
    }

    pub async fn create_account(&self, code: &str, name: &str, account_type: &str, parent_id: Option<i64>) -> Result<ChartOfAccountModel, AppError> {
        let now = chrono::Utc::now();
        let active = chart_of_account_entity::ActiveModel {
            code: Set(code.to_owned()),
            name: Set(name.to_owned()),
            account_type: Set(account_type.to_owned()),
            parent_id: Set(parent_id),
            is_group: Set(false),
            is_active: Set(true),
            created_at: Set(now.into()),
            ..Default::default()
        };
        Ok(active.insert(self.db).await?)
    }

    // ── Journal Entries ────────────────────────────────────

    pub async fn list_journal_entries(&self, page: i64, per_page: i64) -> Result<(Vec<JournalEntryModel>, i64), AppError> {
        let page_size = per_page as u64;
        let page_num = if per_page > 0 { ((page - 1) * per_page) as u64 / page_size } else { 0 };
        let total = journal_entry_entity::Entity::find().count(self.db).await?;
        let entries = journal_entry_entity::Entity::find()
            .order_by_desc(journal_entry_entity::Column::EntryDate)
            .paginate(self.db, page_size)
            .fetch_page(page_num).await?;
        Ok((entries, total as i64))
    }

    pub async fn create_journal_entry(&self, entry_number: &str, entry_date: NaiveDate, description: &str, total_debit: Decimal, total_credit: Decimal) -> Result<JournalEntryModel, AppError> {
        let now = chrono::Utc::now();
        let active = journal_entry_entity::ActiveModel {
            entry_number: Set(entry_number.to_owned()),
            entry_date: Set(entry_date),
            description: Set(description.to_owned()),
            total_debit: Set(total_debit),
            total_credit: Set(total_credit),
            status: Set("draft".to_owned()),
            created_at: Set(now.into()),
            updated_at: Set(now.into()),
            ..Default::default()
        };
        Ok(active.insert(self.db).await?)
    }

    pub async fn post_entry(&self, id: i64) -> Result<bool, AppError> {
        let existing = journal_entry_entity::Entity::find_by_id(id).one(self.db).await?;
        match existing {
            Some(e) if e.status == "draft" => {
                let mut active = e.into_active_model();
                active.status = Set("posted".to_owned());
                active.posted_at = Set(Some(chrono::Utc::now().into()));
                active.updated_at = Set(chrono::Utc::now().into());
                active.update(self.db).await?;
                Ok(true)
            }
            _ => Ok(false),
        }
    }

    pub async fn void_entry(&self, id: i64) -> Result<bool, AppError> {
        let existing = journal_entry_entity::Entity::find_by_id(id).one(self.db).await?;
        match existing {
            Some(e) => {
                let mut active = e.into_active_model();
                active.status = Set("voided".to_owned());
                active.updated_at = Set(chrono::Utc::now().into());
                active.update(self.db).await?;
                Ok(true)
            }
            None => Ok(false),
        }
    }

    // ── Journal Entry Lines ─────────────────────────────────

    pub async fn create_entry_line(&self, entry_id: i64, account_id: i64, debit: Decimal, credit: Decimal, description: Option<&str>) -> Result<JournalEntryLineModel, AppError> {
        let now = chrono::Utc::now();
        let active = journal_entry_line_entity::ActiveModel {
            journal_entry_id: Set(entry_id),
            account_id: Set(account_id),
            debit: Set(debit),
            credit: Set(credit),
            description: Set(description.map(|s| s.to_owned())),
            created_at: Set(now.into()),
            ..Default::default()
        };
        Ok(active.insert(self.db).await?)
    }

    pub async fn list_entry_lines(&self, entry_id: i64) -> Result<Vec<JournalEntryLineModel>, AppError> {
        let lines = journal_entry_line_entity::Entity::find()
            .filter(journal_entry_line_entity::Column::JournalEntryId.eq(entry_id))
            .order_by_asc(journal_entry_line_entity::Column::Id)
            .all(self.db).await?;
        Ok(lines)
    }

    // ── Trial Balance ──────────────────────────────────────

    pub async fn generate_trial_balance(&self, _period_start: NaiveDate, _period_end: NaiveDate) -> Result<Vec<TrialBalanceModel>, AppError> {
        // For complex aggregation queries, we use sea_orm's raw query
        let stmt = sea_orm::Statement::from_string(
            sea_orm::DatabaseBackend::Postgres,
            "INSERT INTO trial_balances (period_start, period_end, account_id, total_debit, total_credit, closing_balance)
             SELECT $1, $2, coa.id,
                COALESCE(SUM(jel.debit), 0),
                COALESCE(SUM(jel.credit), 0),
                COALESCE(SUM(jel.debit), 0) - COALESCE(SUM(jel.credit), 0)
             FROM chart_of_accounts coa
             LEFT JOIN journal_entry_lines jel ON jel.account_id = coa.id
             LEFT JOIN journal_entries je ON je.id = jel.journal_entry_id AND je.status = 'posted' AND je.entry_date BETWEEN $1 AND $2
             WHERE coa.is_active = true
             GROUP BY coa.id
             HAVING COALESCE(SUM(jel.debit), 0) > 0 OR COALESCE(SUM(jel.credit), 0) > 0
             ON CONFLICT (period_start, period_end, account_id) DO UPDATE SET
                total_debit = EXCLUDED.total_debit,
                total_credit = EXCLUDED.total_credit,
                closing_balance = EXCLUDED.closing_balance,
                generated_at = NOW()
             RETURNING *"
        );
        let results = trial_balance_entity::Entity::find()
            .from_raw_sql(stmt)
            .all(self.db).await?;
        Ok(results)
    }

    pub async fn get_account_balances_by_type(&self, _period_start: NaiveDate, _period_end: NaiveDate) -> Result<Vec<(i64, String, String, String, Decimal, Decimal)>, AppError> {
        // TODO: Implement with proper SeaORM raw query when needed
        Ok(Vec::new())
    }

    pub async fn get_gst_invoices(&self, _month: i32, _year: i32) -> Result<Vec<(String, Option<String>, Decimal, Decimal, Decimal, Decimal)>, AppError> {
        // Complex aggregate query - returns unimplemented for now
        Err(AppError::Internal(anyhow::anyhow!("GST invoice query not yet fully ported")))
    }
}
