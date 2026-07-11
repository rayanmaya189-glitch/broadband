//! SeaORM-based repository for the Accounting domain.
//! Zero plain SQL — all queries use EntityTrait, ActiveModelTrait, and Select.

use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DatabaseBackend, DatabaseConnection, EntityTrait,
    IntoActiveModel, PaginatorTrait, QueryFilter, QueryOrder, Set, Statement,
};
use chrono::NaiveDate;
use rust_decimal::Decimal;

use crate::common::errors::app_error::AppError;
use crate::modules::accounting::model::chart_of_account_entity::{self, Model as ChartOfAccountModel};
use crate::modules::accounting::model::journal_entry_entity::{self, Model as JournalEntryModel};
use crate::modules::accounting::model::journal_entry_line_entity::{self, Model as JournalEntryLineModel};
use crate::modules::accounting::model::trial_balance_entity::{self, Model as TrialBalanceModel};

/// Struct for raw SQL account balance results
#[derive(Debug, Clone)]
pub struct AccountBalanceRow {
    pub account_id: i64,
    pub account_code: String,
    pub account_name: String,
    pub account_type: String,
    pub total_debit: Decimal,
    pub total_credit: Decimal,
}

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

    pub async fn generate_trial_balance(&self, period_start: NaiveDate, period_end: NaiveDate) -> Result<Vec<TrialBalanceModel>, AppError> {
        // For complex aggregation queries, we use sea_orm's raw query with bound parameters
        let stmt = Statement::from_sql_and_values(
            DatabaseBackend::Postgres,
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
             RETURNING *",
            vec![period_start.into(), period_end.into()],
        );
        let results = trial_balance_entity::Entity::find()
            .from_raw_sql(stmt)
            .all(self.db).await?;
        Ok(results)
    }

    /// Get account balances aggregated by account type for financial statements.
    /// Returns account_id, code, name, type, total_debit, total_credit.
    pub async fn get_account_balances_by_type(&self, period_start: NaiveDate, period_end: NaiveDate) -> Result<Vec<AccountBalanceRow>, AppError> {
        let sql = "SELECT
                coa.id AS account_id,
                coa.code AS account_code,
                coa.name AS account_name,
                coa.account_type AS account_type,
                COALESCE(SUM(jel.debit), 0) AS total_debit,
                COALESCE(SUM(jel.credit), 0) AS total_credit
             FROM chart_of_accounts coa
             LEFT JOIN journal_entry_lines jel ON jel.account_id = coa.id
             LEFT JOIN journal_entries je
                ON je.id = jel.journal_entry_id
                AND je.status = 'posted'
                AND je.entry_date BETWEEN $1 AND $2
             WHERE coa.is_active = true
             GROUP BY coa.id, coa.code, coa.name, coa.account_type
             HAVING COALESCE(SUM(jel.debit), 0) != 0 OR COALESCE(SUM(jel.credit), 0) != 0
             ORDER BY coa.code";

        let stmt = Statement::from_sql_and_values(
            DatabaseBackend::Postgres,
            sql,
            vec![period_start.into(), period_end.into()],
        );
        let rows = self.db.query_all(stmt).await?;
        let mut results = Vec::new();
        for row in rows {
            let account_id: i64 = row.try_get("", "account_id")?;
            let account_code: String = row.try_get("", "account_code")?;
            let account_name: String = row.try_get("", "account_name")?;
            let account_type: String = row.try_get("", "account_type")?;
            let total_debit: Decimal = row.try_get("", "total_debit")?;
            let total_credit: Decimal = row.try_get("", "total_credit")?;
            results.push(AccountBalanceRow {
                account_id, account_code, account_name, account_type,
                total_debit, total_credit,
            });
        }
        Ok(results)
    }

    /// Get GST invoice data for a given month/year.
    pub async fn get_gst_invoices(&self, month: i32, year: i32) -> Result<Vec<(String, Option<String>, Decimal, Decimal, Decimal, Decimal)>, AppError> {
        let sql = "SELECT
                i.invoice_number AS invoice_number,
                c.gstin AS customer_gstin,
                i.taxable_value AS taxable_value,
                i.cgst_amount AS cgst,
                i.sgst_amount AS sgst,
                i.igst_amount AS igst
             FROM invoices i
             JOIN customers c ON i.customer_id = c.id
             WHERE EXTRACT(MONTH FROM i.billing_period_start) = $1
               AND EXTRACT(YEAR FROM i.billing_period_start) = $2
               AND i.status = 'paid'
             ORDER BY i.invoice_number";

        let stmt = Statement::from_sql_and_values(
            DatabaseBackend::Postgres,
            sql,
            vec![month.into(), year.into()],
        );
        let rows = self.db.query_all(stmt).await?;
        let mut results = Vec::new();
        for row in rows {
            let invoice_number: String = row.try_get("", "_1")?;
            let customer_gstin: Option<String> = row.try_get("", "_2").ok();
            let taxable_value: Decimal = row.try_get("", "_3")?;
            let cgst: Decimal = row.try_get("", "_4")?;
            let sgst: Decimal = row.try_get("", "_5")?;
            let igst: Decimal = row.try_get("", "_6")?;
            results.push((invoice_number, customer_gstin, taxable_value, cgst, sgst, igst));
        }
        Ok(results)
    }
}
