use sqlx::PgPool;
use crate::modules::accounting::model::accounting::{ChartOfAccount, JournalEntry, JournalEntryLine, TrialBalance};

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct AccountBalance {
    pub account_id: i64,
    pub account_code: String,
    pub account_name: String,
    pub account_type: String,
    pub total_debit: rust_decimal::Decimal,
    pub total_credit: rust_decimal::Decimal,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct GstInvoiceRow {
    pub invoice_number: String,
    pub customer_gstin: Option<String>,
    pub taxable_value: rust_decimal::Decimal,
    pub cgst: rust_decimal::Decimal,
    pub sgst: rust_decimal::Decimal,
    pub igst: rust_decimal::Decimal,
}

pub struct AccountingRepository<'a> { pool: &'a PgPool }
impl<'a> AccountingRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self { Self { pool } }

    // ── Chart of Accounts ──────────────────────────────────

    pub async fn list_accounts(&self) -> Result<Vec<ChartOfAccount>, sqlx::Error> {
        sqlx::query_as::<_, ChartOfAccount>("SELECT * FROM chart_of_accounts ORDER BY code")
            .fetch_all(self.pool).await
    }

    pub async fn create_account(&self, code: &str, name: &str, account_type: &str, parent_id: Option<i64>) -> Result<ChartOfAccount, sqlx::Error> {
        sqlx::query_as::<_, ChartOfAccount>("INSERT INTO chart_of_accounts (code, name, account_type, parent_id) VALUES ($1,$2,$3,$4) RETURNING *")
            .bind(code).bind(name).bind(account_type).bind(parent_id).fetch_one(self.pool).await
    }

    // ── Journal Entries ────────────────────────────────────

    pub async fn list_journal_entries(&self, page: i64, per_page: i64) -> Result<(Vec<JournalEntry>, i64), sqlx::Error> {
        let offset = (page - 1) * per_page;
        let count_row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM journal_entries")
            .fetch_one(self.pool).await?;
        let entries: Vec<JournalEntry> = sqlx::query_as("SELECT * FROM journal_entries ORDER BY entry_date DESC LIMIT $1 OFFSET $2")
            .bind(per_page).bind(offset).fetch_all(self.pool).await?;
        Ok((entries, count_row.0))
    }

    pub async fn create_journal_entry(&self, entry_number: &str, entry_date: chrono::NaiveDate, description: &str, total_debit: rust_decimal::Decimal, total_credit: rust_decimal::Decimal) -> Result<JournalEntry, sqlx::Error> {
        sqlx::query_as::<_, JournalEntry>("INSERT INTO journal_entries (entry_number, entry_date, description, total_debit, total_credit) VALUES ($1,$2,$3,$4,$5) RETURNING *")
            .bind(entry_number).bind(entry_date).bind(description).bind(total_debit).bind(total_credit).fetch_one(self.pool).await
    }

    pub async fn post_entry(&self, id: i64) -> Result<bool, sqlx::Error> {
        let r = sqlx::query("UPDATE journal_entries SET status = 'posted', posted_at = NOW() WHERE id = $1 AND status = 'draft'")
            .bind(id).execute(self.pool).await?;
        Ok(r.rows_affected() > 0)
    }

    pub async fn void_entry(&self, id: i64) -> Result<bool, sqlx::Error> {
        let r = sqlx::query("UPDATE journal_entries SET status = 'voided' WHERE id = $1")
            .bind(id).execute(self.pool).await?;
        Ok(r.rows_affected() > 0)
    }

    // ── Journal Entry Lines ─────────────────────────────────

    pub async fn create_entry_line(&self, entry_id: i64, account_id: i64, debit: rust_decimal::Decimal, credit: rust_decimal::Decimal, description: Option<&str>) -> Result<JournalEntryLine, sqlx::Error> {
        sqlx::query_as::<_, JournalEntryLine>("INSERT INTO journal_entry_lines (journal_entry_id, account_id, debit, credit, description) VALUES ($1,$2,$3,$4,$5) RETURNING *")
            .bind(entry_id).bind(account_id).bind(debit).bind(credit).bind(description).fetch_one(self.pool).await
    }

    pub async fn list_entry_lines(&self, entry_id: i64) -> Result<Vec<JournalEntryLine>, sqlx::Error> {
        sqlx::query_as::<_, JournalEntryLine>("SELECT * FROM journal_entry_lines WHERE journal_entry_id = $1 ORDER BY id")
            .bind(entry_id).fetch_all(self.pool).await
    }

    // ── Trial Balance ──────────────────────────────────────

    pub async fn generate_trial_balance(&self, period_start: chrono::NaiveDate, period_end: chrono::NaiveDate) -> Result<Vec<TrialBalance>, sqlx::Error> {
        // Get all accounts with their posted journal entry line sums
        sqlx::query_as::<_, TrialBalance>(
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
        ).bind(period_start).bind(period_end).fetch_all(self.pool).await
    }

    // ── Account Balances by Type ─────────────────────────────

    pub async fn get_account_balances_by_type(&self, period_start: chrono::NaiveDate, period_end: chrono::NaiveDate) -> Result<Vec<AccountBalance>, sqlx::Error> {
        sqlx::query_as::<_, AccountBalance>(
            "SELECT coa.id as account_id, coa.code as account_code, coa.name as account_name, coa.account_type,
                COALESCE(SUM(jel.debit), 0) as total_debit, COALESCE(SUM(jel.credit), 0) as total_credit
             FROM chart_of_accounts coa
             LEFT JOIN journal_entry_lines jel ON jel.account_id = coa.id
             LEFT JOIN journal_entries je ON je.id = jel.journal_entry_id AND je.status = 'posted' AND je.entry_date BETWEEN $1 AND $2
             WHERE coa.is_active = true
             GROUP BY coa.id, coa.code, coa.name, coa.account_type
             HAVING COALESCE(SUM(jel.debit), 0) > 0 OR COALESCE(SUM(jel.credit), 0) > 0
             ORDER BY coa.code"
        ).bind(period_start).bind(period_end).fetch_all(self.pool).await
    }

    // ── GST Invoices ─────────────────────────────────────────

    pub async fn get_gst_invoices(&self, month: i32, year: i32) -> Result<Vec<GstInvoiceRow>, sqlx::Error> {
        sqlx::query_as::<_, GstInvoiceRow>(
            "SELECT i.invoice_number, c.gstin as customer_gstin,
                i.taxable_value, i.cgst_amount as cgst, i.sgst_amount as sgst, i.igst_amount as igst
             FROM invoices i
             JOIN customers c ON i.customer_id = c.id
             WHERE EXTRACT(MONTH FROM i.billing_period_start) = $1
               AND EXTRACT(YEAR FROM i.billing_period_start) = $2
               AND i.status = 'paid'
             ORDER BY i.invoice_number"
        ).bind(month).bind(year).fetch_all(self.pool).await
    }
}
