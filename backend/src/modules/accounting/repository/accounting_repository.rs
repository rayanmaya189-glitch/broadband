//! SeaORM-based repository for the Accounting domain.
//! Zero plain SQL — all queries use EntityTrait, ActiveModelTrait, and Select.

use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait,
    IntoActiveModel, PaginatorTrait, QueryFilter, QueryOrder, Set,
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
        use std::collections::HashMap;

        // 1. Fetch all active accounts
        let accounts = chart_of_account_entity::Entity::find()
            .filter(chart_of_account_entity::Column::IsActive.eq(true))
            .all(self.db).await?;

        // 2. Fetch all posted journal entries in the period
        let posted_entries = journal_entry_entity::Entity::find()
            .filter(journal_entry_entity::Column::Status.eq("posted"))
            .filter(journal_entry_entity::Column::EntryDate.gte(period_start))
            .filter(journal_entry_entity::Column::EntryDate.lte(period_end))
            .all(self.db).await?;
        let posted_entry_ids: Vec<i64> = posted_entries.iter().map(|e| e.id).collect();

        // 3. Fetch journal entry lines for those entries
        let lines = if posted_entry_ids.is_empty() {
            Vec::new()
        } else {
            journal_entry_line_entity::Entity::find()
                .filter(journal_entry_line_entity::Column::JournalEntryId.is_in(posted_entry_ids))
                .all(self.db).await?
        };

        // 4. Aggregate debit/credit per account
        let mut balances: HashMap<i64, (Decimal, Decimal)> = HashMap::new();
        for line in &lines {
            let entry = balances.entry(line.account_id).or_insert((Decimal::ZERO, Decimal::ZERO));
            entry.0 += line.debit;
            entry.1 += line.credit;
        }

        // 5. Insert or update trial balance rows (only accounts with activity)
        let mut results = Vec::new();
        let now = chrono::Utc::now();
        for account in &accounts {
            if let Some((total_debit, total_credit)) = balances.get(&account.id) {
                let closing_balance = total_debit - total_credit;
                // Upsert: check if row exists
                let existing = trial_balance_entity::Entity::find()
                    .filter(trial_balance_entity::Column::PeriodStart.eq(period_start))
                    .filter(trial_balance_entity::Column::PeriodEnd.eq(period_end))
                    .filter(trial_balance_entity::Column::AccountId.eq(account.id))
                    .one(self.db).await?;

                match existing {
                    Some(tb) => {
                        let mut active = tb.into_active_model();
                        active.total_debit = Set(*total_debit);
                        active.total_credit = Set(*total_credit);
                        active.closing_balance = Set(closing_balance);
                        active.generated_at = Set(now.into());
                        let updated = active.update(self.db).await?;
                        results.push(updated);
                    }
                    None => {
                        let active = trial_balance_entity::ActiveModel {
                            period_start: Set(period_start),
                            period_end: Set(period_end),
                            account_id: Set(account.id),
                            opening_balance: Set(Decimal::ZERO),
                            total_debit: Set(*total_debit),
                            total_credit: Set(*total_credit),
                            closing_balance: Set(closing_balance),
                            generated_at: Set(now.into()),
                            ..Default::default()
                        };
                        let inserted = active.insert(self.db).await?;
                        results.push(inserted);
                    }
                }
            }
        }
        Ok(results)
    }

    /// Get account balances aggregated by account type for financial statements.
    /// Uses pure SeaORM queries with application-level aggregation.
    pub async fn get_account_balances_by_type(&self, period_start: NaiveDate, period_end: NaiveDate) -> Result<Vec<AccountBalanceRow>, AppError> {
        use std::collections::HashMap;

        // 1. Fetch all active accounts
        let accounts = chart_of_account_entity::Entity::find()
            .filter(chart_of_account_entity::Column::IsActive.eq(true))
            .all(self.db).await?;

        // 2. Fetch posted journal entry IDs in the date range
        let posted_entries = journal_entry_entity::Entity::find()
            .filter(journal_entry_entity::Column::Status.eq("posted"))
            .filter(journal_entry_entity::Column::EntryDate.gte(period_start))
            .filter(journal_entry_entity::Column::EntryDate.lte(period_end))
            .all(self.db).await?;
        let posted_entry_ids: Vec<i64> = posted_entries.iter().map(|e| e.id).collect();

        // 3. Fetch journal entry lines for those entries
        let lines = if posted_entry_ids.is_empty() {
            Vec::new()
        } else {
            journal_entry_line_entity::Entity::find()
                .filter(journal_entry_line_entity::Column::JournalEntryId.is_in(posted_entry_ids))
                .all(self.db).await?
        };

        // 4. Aggregate debit/credit per account
        let mut balances: HashMap<i64, (Decimal, Decimal)> = HashMap::new();
        for line in &lines {
            let entry = balances.entry(line.account_id).or_insert((Decimal::ZERO, Decimal::ZERO));
            entry.0 += line.debit;
            entry.1 += line.credit;
        }

        // 5. Build results — only accounts with non-zero activity
        let mut results = Vec::new();
        for account in &accounts {
            if let Some((total_debit, total_credit)) = balances.get(&account.id) {
                if *total_debit != Decimal::ZERO || *total_credit != Decimal::ZERO {
                    results.push(AccountBalanceRow {
                        account_id: account.id,
                        account_code: account.code.clone(),
                        account_name: account.name.clone(),
                        account_type: account.account_type.clone(),
                        total_debit: *total_debit,
                        total_credit: *total_credit,
                    });
                }
            }
        }
        results.sort_by(|a, b| a.account_code.cmp(&b.account_code));
        Ok(results)
    }

    /// Get GST invoice data for a given month/year using pure SeaORM.
    // ── Branch & Customer Helpers ─────────────────────────

    pub async fn get_branch_state(&self, branch_id: i64) -> Result<Option<String>, AppError> {
        use crate::modules::branch::model::branch_entity;
        let branch = branch_entity::Entity::find_by_id(branch_id)
            .one(self.db).await?;
        Ok(branch.and_then(|b| b.state))
    }

    pub async fn get_branch_gstin(&self, branch_id: i64) -> Option<String> {
        use crate::modules::branch::model::branch_entity;
        branch_entity::Entity::find_by_id(branch_id)
            .one(self.db).await
            .ok()
            .flatten()
            .and_then(|b| b.gstin)
    }

    /// Get customer name by ID.
    pub async fn get_customer_name(&self, customer_id: i64) -> Result<String, AppError> {
        use crate::modules::customer::model::customer_entity;
        let c = customer_entity::Entity::find_by_id(customer_id)
            .one(self.db).await?;
        Ok(c.map(|c| format!("{} {}", c.first_name, c.last_name.unwrap_or_default()))
            .unwrap_or_else(|| "Unknown".to_string()))
    }

    /// Get customer GSTIN from profile.
    pub async fn get_customer_gstin(&self, customer_id: i64) -> Result<Option<String>, AppError> {
        use crate::modules::customer::model::customer_profile_entity;
        let p = customer_profile_entity::Entity::find()
            .filter(customer_profile_entity::Column::CustomerId.eq(customer_id))
            .one(self.db).await?;
        Ok(p.and_then(|p| p.gstin))
    }

    /// Fetch paid invoices for a month/year range.
    pub async fn get_paid_invoices_for_period(&self, month: i32, year: i32) -> Result<Vec<crate::modules::billing::model::invoice_entity::Model>, AppError> {
        use crate::modules::billing::model::invoice_entity;
        use sea_orm::{ColumnTrait, QueryFilter, QueryOrder};
        let start_date = chrono::NaiveDate::from_ymd_opt(year, month as u32, 1)
            .ok_or_else(|| AppError::Validation("Invalid month/year".into()))?;
        let end_date = if month == 12 {
            chrono::NaiveDate::from_ymd_opt(year + 1, 1, 1)
        } else {
            chrono::NaiveDate::from_ymd_opt(year, (month + 1) as u32, 1)
        }
        .ok_or_else(|| AppError::Validation("Invalid month/year".into()))?;
        let invoices = invoice_entity::Entity::find()
            .filter(invoice_entity::Column::Status.eq("paid"))
            .filter(invoice_entity::Column::BillingPeriodStart.gte(start_date))
            .filter(invoice_entity::Column::BillingPeriodStart.lt(end_date))
            .order_by_asc(invoice_entity::Column::InvoiceNumber)
            .all(self.db).await?;
        Ok(invoices)
    }

    /// Get GST invoice data for a given month/year using pure SeaORM.
    pub async fn get_gst_invoices(&self, month: i32, year: i32) -> Result<Vec<(String, Option<String>, Decimal, Decimal, Decimal, Decimal)>, AppError> {
        use crate::modules::billing::model::invoice_entity;

        // Calculate date range for the month
        let start_date = chrono::NaiveDate::from_ymd_opt(year, month as u32, 1)
            .ok_or_else(|| AppError::Validation("Invalid month/year".into()))?;
        let end_date = if month == 12 {
            chrono::NaiveDate::from_ymd_opt(year + 1, 1, 1)
        } else {
            chrono::NaiveDate::from_ymd_opt(year, (month + 1) as u32, 1)
        }
        .ok_or_else(|| AppError::Validation("Invalid month/year".into()))?;

        // Query paid invoices in the date range
        let invoices = invoice_entity::Entity::find()
            .filter(invoice_entity::Column::Status.eq("paid"))
            .filter(invoice_entity::Column::BillingPeriodStart.gte(start_date))
            .filter(invoice_entity::Column::BillingPeriodStart.lt(end_date))
            .order_by_asc(invoice_entity::Column::InvoiceNumber)
            .all(self.db).await?;

        let mut results = Vec::new();
        for inv in invoices {
            // Fetch customer profile for GSTIN
            let profile = crate::modules::customer::model::customer_profile_entity::Entity::find()
                .filter(crate::modules::customer::model::customer_profile_entity::Column::CustomerId.eq(inv.customer_id))
                .one(self.db).await?;
            let customer_gstin = profile.and_then(|p| p.gstin);

            // Use actual CGST/SGST/IGST columns from invoice
            results.push((
                inv.invoice_number,
                customer_gstin,
                inv.subtotal,
                inv.cgst_amount,
                inv.sgst_amount,
                inv.igst_amount,
            ));
        }
        Ok(results)
    }
}
