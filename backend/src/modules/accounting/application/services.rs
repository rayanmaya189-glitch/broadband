use sea_orm::{DatabaseConnection, EntityTrait, ActiveModelTrait, Set, QueryFilter, ColumnTrait, QueryOrder, PaginatorTrait};
use chrono::Utc;
use crate::shared::errors::AppError;
use crate::modules::accounting::domain::entities::{
    chart_of_accounts, journal_entry, journal_entry_line,
    ChartOfAccounts, ChartOfAccountsActiveModel,
    JournalEntry, JournalEntryActiveModel,
    JournalEntryLine, JournalEntryLineActiveModel,
};

/// Minimum date for balance sheet queries (epoch)
const MIN_DATE: chrono::NaiveDate = chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap();

pub struct AccountingService;

impl AccountingService {
    // ── Chart of Accounts ──

    pub async fn list_accounts(db: &DatabaseConnection) -> Result<Vec<chart_of_accounts::Model>, AppError> {
        Ok(ChartOfAccounts::find()
            .order_by_asc(chart_of_accounts::Column::Code)
            .all(db)
            .await?)
    }

    pub async fn get_account(db: &DatabaseConnection, id: i64) -> Result<chart_of_accounts::Model, AppError> {
        ChartOfAccounts::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Account {} not found", id)))
    }

    pub async fn create_account(
        db: &DatabaseConnection,
        code: String,
        name: String,
        account_type: String,
        parent_id: Option<i64>,
        description: Option<String>,
    ) -> Result<chart_of_accounts::Model, AppError> {
        let valid_types = ["asset", "liability", "equity", "revenue", "expense"];
        if !valid_types.contains(&account_type.as_str()) {
            return Err(AppError::Validation(format!(
                "Invalid account_type: {}. Must be one of: asset, liability, equity, revenue, expense",
                account_type
            )));
        }

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

    pub async fn update_account(
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
        if let Some(n) = name { active.name = Set(n); }
        if let Some(d) = description { active.description = Set(Some(d)); }
        if let Some(a) = is_active { active.is_active = Set(a); }
        active.updated_at = Set(Utc::now());
        Ok(active.update(db).await?)
    }

    // ── Journal Entries ──

    pub async fn list_journal_entries(
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

    pub async fn get_journal_entry(db: &DatabaseConnection, id: i64) -> Result<journal_entry::Model, AppError> {
        JournalEntry::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Journal entry {} not found", id)))
    }

    pub async fn get_journal_entry_lines(
        db: &DatabaseConnection,
        entry_id: i64,
    ) -> Result<Vec<journal_entry_line::Model>, AppError> {
        Ok(JournalEntryLine::find()
            .filter(journal_entry_line::Column::JournalEntryId.eq(entry_id))
            .all(db)
            .await?)
    }

    pub async fn create_journal_entry(
        db: &DatabaseConnection,
        entry_date: chrono::NaiveDate,
        description: String,
        reference_type: Option<String>,
        reference_id: Option<i64>,
        lines: Vec<CreateJournalLine>,
        created_by: Option<i64>,
    ) -> Result<journal_entry::Model, AppError> {
        if lines.is_empty() {
            return Err(AppError::Validation("Journal entry must have at least one line".into()));
        }

        // Validate debits == credits
        let total_debit: sea_orm::prelude::Decimal = lines.iter().map(|l| l.debit).sum();
        let total_credit: sea_orm::prelude::Decimal = lines.iter().map(|l| l.credit).sum();
        if total_debit != total_credit {
            return Err(AppError::Validation(format!(
                "Debit/credit mismatch: debit={}, credit={}",
                total_debit, total_credit
            )));
        }
        if total_debit <= sea_orm::prelude::Decimal::ZERO {
            return Err(AppError::Validation("Total amount must be positive".into()));
        }

        // Validate each line has either debit or credit (not both, not neither)
        for (i, line) in lines.iter().enumerate() {
            if line.debit < sea_orm::prelude::Decimal::ZERO || line.credit < sea_orm::prelude::Decimal::ZERO {
                return Err(AppError::Validation(format!("Line {}: amounts must be non-negative", i + 1)));
            }
            if line.debit == sea_orm::prelude::Decimal::ZERO && line.credit == sea_orm::prelude::Decimal::ZERO {
                return Err(AppError::Validation(format!("Line {}: must have either debit or credit amount", i + 1)));
            }
        }

        // Generate entry number
        let now = Utc::now();
        let entry_count = JournalEntry::find().count(db).await.unwrap_or(0);
        let entry_number = format!("JE-{}-{:05}", now.format("%Y%m%d"), entry_count + 1);

        let entry = JournalEntryActiveModel {
            entry_number: Set(entry_number),
            entry_date: Set(entry_date),
            description: Set(description),
            reference_type: Set(reference_type),
            reference_id: Set(reference_id),
            total_debit: Set(total_debit),
            total_credit: Set(total_credit),
            status: Set("draft".to_string()),
            created_by: Set(created_by),
            posted_at: Set(None),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };
        let saved_entry = entry.insert(db).await?;

        // Insert lines
        for line in lines {
            let line_model = JournalEntryLineActiveModel {
                journal_entry_id: Set(saved_entry.id),
                account_id: Set(line.account_id),
                debit: Set(line.debit),
                credit: Set(line.credit),
                description: Set(line.description),
                created_at: Set(Utc::now()),
                ..Default::default()
            };
            line_model.insert(db).await?;
        }

        Ok(saved_entry)
    }

    pub async fn post_journal_entry(
        db: &DatabaseConnection,
        id: i64,
        reviewer_id: Option<i64>,
    ) -> Result<journal_entry::Model, AppError> {
        let existing = JournalEntry::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Journal entry {} not found", id)))?;

        if existing.status == "posted" {
            return Err(AppError::Conflict("Journal entry is already posted".into()));
        }
        if existing.status == "voided" {
            return Err(AppError::Conflict("Cannot post a voided journal entry".into()));
        }
        if existing.total_debit != existing.total_credit {
            return Err(AppError::Validation("Cannot post unbalanced entry".into()));
        }

        let now = Utc::now();
        let mut active: JournalEntryActiveModel = existing.into();
        active.status = Set("posted".to_string());
        active.posted_at = Set(Some(now));
        active.updated_at = Set(now);
        if reviewer_id.is_some() {
            active.reviewed_by = Set(reviewer_id);
            active.reviewed_at = Set(Some(now));
        }
        Ok(active.update(db).await?)
    }

    pub async fn void_journal_entry(
        db: &DatabaseConnection,
        id: i64,
    ) -> Result<journal_entry::Model, AppError> {
        let existing = JournalEntry::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Journal entry {} not found", id)))?;

        if existing.status == "voided" {
            return Err(AppError::Conflict("Journal entry is already voided".into()));
        }
        if existing.status == "posted" {
            return Err(AppError::Conflict("Cannot void a posted journal entry".into()));
        }

        let now = Utc::now();
        let mut active: JournalEntryActiveModel = existing.into();
        active.status = Set("voided".to_string());
        active.updated_at = Set(now);
        Ok(active.update(db).await?)
    }

    // ── Trial Balance ──

    pub async fn generate_trial_balance(
        db: &DatabaseConnection,
        period_start: chrono::NaiveDate,
        period_end: chrono::NaiveDate,
    ) -> Result<Vec<TrialBalanceRow>, AppError> {
        let accounts = ChartOfAccounts::find()
            .filter(chart_of_accounts::Column::IsActive.eq(true))
            .order_by_asc(chart_of_accounts::Column::Code)
            .all(db)
            .await?;

        let mut results = Vec::new();

        for account in accounts {
            // Query journal entry lines for this account in posted entries within the period
            let posted_entries: Vec<i64> = JournalEntry::find()
                .filter(journal_entry::Column::Status.eq("posted"))
                .filter(journal_entry::Column::EntryDate.gte(period_start))
                .filter(journal_entry::Column::EntryDate.lte(period_end))
                .all(db)
                .await?
                .into_iter()
                .map(|e| e.id)
                .collect();

            let lines = if posted_entries.is_empty() {
                Vec::new()
            } else {
                JournalEntryLine::find()
                    .filter(journal_entry_line::Column::AccountId.eq(account.id))
                    .filter(journal_entry_line::Column::JournalEntryId.is_in(posted_entries))
                    .all(db)
                    .await?
            };

            let total_debit: sea_orm::prelude::Decimal = lines.iter().map(|l| l.debit).sum();
            let total_credit: sea_orm::prelude::Decimal = lines.iter().map(|l| l.credit).sum();

            // For assets and expenses: opening + debit - credit
            // For liabilities, equity, revenue: opening + credit - debit
            let is_debit_type = matches!(account.account_type.as_str(), "asset" | "expense");
            let opening = sea_orm::prelude::Decimal::ZERO;
            let closing = if is_debit_type {
                opening + total_debit - total_credit
            } else {
                opening + total_credit - total_debit
            };

            results.push(TrialBalanceRow {
                account_id: account.id,
                account_code: account.code,
                account_name: account.name,
                account_type: account.account_type,
                opening_balance: opening,
                total_debit,
                total_credit,
                closing_balance: closing,
            });
        }

        Ok(results)
    }

    // ── Financial Statements ──

    pub async fn profit_and_loss(
        db: &DatabaseConnection,
        period_start: chrono::NaiveDate,
        period_end: chrono::NaiveDate,
    ) -> Result<ProfitLossStatement, AppError> {
        let trial_balance = Self::generate_trial_balance(db, period_start, period_end).await?;

        let mut total_revenue = sea_orm::prelude::Decimal::ZERO;
        let mut total_expense = sea_orm::prelude::Decimal::ZERO;
        let mut revenue_lines = Vec::new();
        let mut expense_lines = Vec::new();

        for row in &trial_balance {
            match row.account_type.as_str() {
                "revenue" => {
                    total_revenue += row.closing_balance.abs();
                    revenue_lines.push(StatementLine {
                        account_code: row.account_code.clone(),
                        account_name: row.account_name.clone(),
                        amount: row.closing_balance.abs(),
                    });
                }
                "expense" => {
                    total_expense += row.closing_balance.abs();
                    expense_lines.push(StatementLine {
                        account_code: row.account_code.clone(),
                        account_name: row.account_name.clone(),
                        amount: row.closing_balance.abs(),
                    });
                }
                _ => {}
            }
        }

        Ok(ProfitLossStatement {
            period_start,
            period_end,
            revenue_lines,
            expense_lines,
            total_revenue,
            total_expense,
            net_income: total_revenue - total_expense,
        })
    }

    pub async fn balance_sheet(
        db: &DatabaseConnection,
        as_of_date: chrono::NaiveDate,
    ) -> Result<BalanceSheet, AppError> {
        // Balance sheet accumulates ALL posted entries from beginning up to as_of_date
        let trial_balance = Self::generate_trial_balance(db, MIN_DATE, as_of_date).await?;

        let mut total_assets = sea_orm::prelude::Decimal::ZERO;
        let mut total_liabilities = sea_orm::prelude::Decimal::ZERO;
        let mut total_equity = sea_orm::prelude::Decimal::ZERO;
        let mut asset_lines = Vec::new();
        let mut liability_lines = Vec::new();
        let mut equity_lines = Vec::new();

        for row in &trial_balance {
            match row.account_type.as_str() {
                "asset" => {
                    total_assets += row.closing_balance.abs();
                    asset_lines.push(StatementLine {
                        account_code: row.account_code.clone(),
                        account_name: row.account_name.clone(),
                        amount: row.closing_balance.abs(),
                    });
                }
                "liability" => {
                    total_liabilities += row.closing_balance.abs();
                    liability_lines.push(StatementLine {
                        account_code: row.account_code.clone(),
                        account_name: row.account_name.clone(),
                        amount: row.closing_balance.abs(),
                    });
                }
                "equity" => {
                    total_equity += row.closing_balance.abs();
                    equity_lines.push(StatementLine {
                        account_code: row.account_code.clone(),
                        account_name: row.account_name.clone(),
                        amount: row.closing_balance.abs(),
                    });
                }
                _ => {}
            }
        }

        Ok(BalanceSheet {
            as_of_date,
            asset_lines,
            liability_lines,
            equity_lines,
            total_assets,
            total_liabilities,
            total_equity,
        })
    }

    // ── GST Returns ──

    pub async fn generate_gst_return(
        db: &DatabaseConnection,
        return_type: String,
        period_month: u32,
        period_year: i32,
    ) -> Result<GstReturnData, AppError> {
        let period_start = chrono::NaiveDate::from_ymd_opt(period_year, period_month, 1)
            .ok_or_else(|| AppError::Validation("Invalid date".into()))?;

        let (next_month, next_year) = if period_month == 12 { (1, period_year + 1) } else { (period_month + 1, period_year) };
        let period_end = chrono::NaiveDate::from_ymd_opt(next_year, next_month, 1)
            .ok_or_else(|| AppError::Validation("Invalid date".into()))?
            - chrono::Duration::days(1);

        // Query invoices in period to compute GST
        let invoices = crate::modules::billing::domain::entities::Invoice::find()
            .filter(crate::modules::billing::domain::entities::invoice::Column::Status.eq("paid"))
            .filter(crate::modules::billing::domain::entities::invoice::Column::BillingPeriodStart.gte(period_start))
            .filter(crate::modules::billing::domain::entities::invoice::Column::BillingPeriodEnd.lte(period_end))
            .all(db)
            .await?;

        let total_taxable: sea_orm::prelude::Decimal = invoices.iter().map(|i| i.subtotal).sum();
        let total_tax: sea_orm::prelude::Decimal = invoices.iter().map(|i| i.tax_amount).sum();
        let cgst = total_tax / sea_orm::prelude::Decimal::from(2);
        let sgst = total_tax / sea_orm::prelude::Decimal::from(2);
        let igst = sea_orm::prelude::Decimal::ZERO;

        Ok(GstReturnData {
            return_type,
            period_month: period_month as i32,
            period_year,
            total_taxable_value: total_taxable,
            total_cgst: cgst,
            total_sgst: sgst,
            total_igst: igst,
            invoice_count: invoices.len() as i64,
        })
    }
}

// ── Request / Response Types ──

#[derive(Debug, Clone)]
pub struct CreateJournalLine {
    pub account_id: i64,
    pub debit: sea_orm::prelude::Decimal,
    pub credit: sea_orm::prelude::Decimal,
    pub description: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct TrialBalanceRow {
    pub account_id: i64,
    pub account_code: String,
    pub account_name: String,
    pub account_type: String,
    pub opening_balance: sea_orm::prelude::Decimal,
    pub total_debit: sea_orm::prelude::Decimal,
    pub total_credit: sea_orm::prelude::Decimal,
    pub closing_balance: sea_orm::prelude::Decimal,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct StatementLine {
    pub account_code: String,
    pub account_name: String,
    pub amount: sea_orm::prelude::Decimal,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ProfitLossStatement {
    pub period_start: chrono::NaiveDate,
    pub period_end: chrono::NaiveDate,
    pub revenue_lines: Vec<StatementLine>,
    pub expense_lines: Vec<StatementLine>,
    pub total_revenue: sea_orm::prelude::Decimal,
    pub total_expense: sea_orm::prelude::Decimal,
    pub net_income: sea_orm::prelude::Decimal,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct BalanceSheet {
    pub as_of_date: chrono::NaiveDate,
    pub asset_lines: Vec<StatementLine>,
    pub liability_lines: Vec<StatementLine>,
    pub equity_lines: Vec<StatementLine>,
    pub total_assets: sea_orm::prelude::Decimal,
    pub total_liabilities: sea_orm::prelude::Decimal,
    pub total_equity: sea_orm::prelude::Decimal,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct GstReturnData {
    pub return_type: String,
    pub period_month: i32,
    pub period_year: i32,
    pub total_taxable_value: sea_orm::prelude::Decimal,
    pub total_cgst: sea_orm::prelude::Decimal,
    pub total_sgst: sea_orm::prelude::Decimal,
    pub total_igst: sea_orm::prelude::Decimal,
    pub invoice_count: i64,
}
