use sqlx::PgPool;
use crate::modules::accounting::model::accounting::{ChartOfAccount, JournalEntry};

pub struct AccountingRepository<'a> { pool: &'a PgPool }
impl<'a> AccountingRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self { Self { pool } }
    pub async fn list_accounts(&self) -> Result<Vec<ChartOfAccount>, sqlx::Error> { sqlx::query_as::<_, ChartOfAccount>("SELECT * FROM chart_of_accounts ORDER BY code").fetch_all(self.pool).await }
    pub async fn create_account(&self, code: &str, name: &str, account_type: &str, parent_id: Option<i64>) -> Result<ChartOfAccount, sqlx::Error> {
        sqlx::query_as::<_, ChartOfAccount>("INSERT INTO chart_of_accounts (code, name, account_type, parent_id) VALUES ($1,$2,$3,$4) RETURNING *").bind(code).bind(name).bind(account_type).bind(parent_id).fetch_one(self.pool).await
    }
    pub async fn list_journal_entries(&self, page: i64, per_page: i64) -> Result<(Vec<JournalEntry>, i64), sqlx::Error> {
        let offset = (page - 1) * per_page;
        let count_row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM journal_entries").fetch_one(self.pool).await?;
        let entries: Vec<JournalEntry> = sqlx::query_as("SELECT * FROM journal_entries ORDER BY entry_date DESC LIMIT $1 OFFSET $2").bind(per_page).bind(offset).fetch_all(self.pool).await?;
        Ok((entries, count_row.0))
    }
    pub async fn create_journal_entry(&self, entry_number: &str, entry_date: chrono::NaiveDate, description: &str, total_debit: rust_decimal::Decimal, total_credit: rust_decimal::Decimal) -> Result<JournalEntry, sqlx::Error> {
        sqlx::query_as::<_, JournalEntry>("INSERT INTO journal_entries (entry_number, entry_date, description, total_debit, total_credit) VALUES ($1,$2,$3,$4,$5) RETURNING *").bind(entry_number).bind(entry_date).bind(description).bind(total_debit).bind(total_credit).fetch_one(self.pool).await
    }
    pub async fn post_entry(&self, id: i64) -> Result<bool, sqlx::Error> { let r = sqlx::query("UPDATE journal_entries SET status = 'posted', posted_at = NOW() WHERE id = $1 AND status = 'draft'").bind(id).execute(self.pool).await?; Ok(r.rows_affected() > 0) }
    pub async fn void_entry(&self, id: i64) -> Result<bool, sqlx::Error> { let r = sqlx::query("UPDATE journal_entries SET status = 'voided' WHERE id = $1").bind(id).execute(self.pool).await?; Ok(r.rows_affected() > 0) }
}
