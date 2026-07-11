//! SeaORM-based service for the Accounting domain.

use sea_orm::DatabaseConnection;

use crate::common::errors::app_error::AppError;
use crate::modules::accounting::repository::accounting_repository::AccountingRepository;
use crate::modules::accounting::request::accounting_request::*;
use crate::modules::accounting::response::accounting_response::*;

pub struct AccountingService<'a> {
    repo: AccountingRepository<'a>,
}

impl<'a> AccountingService<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { repo: AccountingRepository::new(db) }
    }

    pub async fn list_accounts(&self) -> Result<Vec<AccountResponse>, AppError> {
        let accounts = self.repo.list_accounts().await?;
        Ok(accounts.into_iter().map(|a| AccountResponse {
            id: a.id, code: a.code, name: a.name, account_type: a.account_type,
            is_active: a.is_active, created_at: a.created_at.into(),
        }).collect())
    }

    pub async fn create_account(&self, req: CreateAccountRequest) -> Result<AccountResponse, AppError> {
        let a = self.repo.create_account(&req.code, &req.name, &req.account_type, req.parent_id).await?;
        Ok(AccountResponse { id: a.id, code: a.code, name: a.name, account_type: a.account_type, is_active: a.is_active, created_at: a.created_at.into() })
    }

    pub async fn list_journal_entries(&self, page: i64, per_page: i64) -> Result<Vec<JournalEntryResponse>, AppError> {
        let (entries, _) = self.repo.list_journal_entries(page, per_page).await?;
        Ok(entries.into_iter().map(|e| JournalEntryResponse {
            id: e.id, entry_number: e.entry_number, entry_date: e.entry_date,
            description: e.description, total_debit: e.total_debit, total_credit: e.total_credit,
            status: e.status, created_at: e.created_at.into(),
        }).collect())
    }

    pub async fn create_journal_entry(&self, req: CreateJournalEntryRequest) -> Result<JournalEntryDetailResponse, AppError> {
        let total_debit: rust_decimal::Decimal = req.lines.iter().filter_map(|l| l.debit).sum();
        let total_credit: rust_decimal::Decimal = req.lines.iter().filter_map(|l| l.credit).sum();
        if total_debit != total_credit {
            return Err(AppError::Validation("Total debit must equal total credit".into()));
        }
        let entry_number = format!("JE-{}-{:04}", req.entry_date.format("%Y%m"), chrono::Utc::now().timestamp_micros() % 10000);
        let e = self.repo.create_journal_entry(&entry_number, req.entry_date, &req.description, total_debit, total_credit).await?;
        let mut lines = Vec::new();
        for line in &req.lines {
            let l = self.repo.create_entry_line(e.id, line.account_id, line.debit.unwrap_or(rust_decimal::Decimal::ZERO), line.credit.unwrap_or(rust_decimal::Decimal::ZERO), None).await?;
            lines.push(JournalEntryLineResponse { id: l.id, journal_entry_id: l.journal_entry_id, account_id: l.account_id, debit: l.debit, credit: l.credit, description: l.description, created_at: l.created_at.into() });
        }
        Ok(JournalEntryDetailResponse {
            entry: JournalEntryResponse { id: e.id, entry_number: e.entry_number, entry_date: e.entry_date, description: e.description, total_debit: e.total_debit, total_credit: e.total_credit, status: e.status, created_at: e.created_at.into() },
            lines,
        })
    }

    pub async fn post_entry(&self, id: i64) -> Result<MessageResponse, AppError> {
        if !self.repo.post_entry(id).await? { return Err(AppError::NotFound("Entry not found".into())); }
        Ok(MessageResponse { message: "Posted".into() })
    }

    pub async fn void_entry(&self, id: i64) -> Result<MessageResponse, AppError> {
        if !self.repo.void_entry(id).await? { return Err(AppError::NotFound("Entry not found".into())); }
        Ok(MessageResponse { message: "Voided".into() })
    }

    pub async fn get_entry_lines(&self, entry_id: i64) -> Result<Vec<JournalEntryLineResponse>, AppError> {
        let lines = self.repo.list_entry_lines(entry_id).await?;
        Ok(lines.into_iter().map(|l| JournalEntryLineResponse {
            id: l.id, journal_entry_id: l.journal_entry_id, account_id: l.account_id,
            debit: l.debit, credit: l.credit, description: l.description,
            created_at: l.created_at.into(),
        }).collect())
    }

    pub async fn trial_balance(&self, q: TrialBalanceQuery) -> Result<TrialBalanceResponse, AppError> {
        let balances = self.repo.get_account_balances_by_type(q.period_start, q.period_end).await?;
        let accounts: Vec<TrialBalanceAccount> = balances.into_iter().map(|b| TrialBalanceAccount {
            account_id: b.0, account_code: b.1, account_name: b.2, account_type: b.3,
            total_debit: b.4, total_credit: b.5, closing_balance: b.4 - b.5,
        }).collect();
        let total_debit: rust_decimal::Decimal = accounts.iter().map(|a| a.total_debit).sum();
        let total_credit: rust_decimal::Decimal = accounts.iter().map(|a| a.total_credit).sum();
        Ok(TrialBalanceResponse { period_start: q.period_start, period_end: q.period_end, accounts, total_debit, total_credit })
    }

    pub async fn profit_loss_statement(&self, q: TrialBalanceQuery) -> Result<ProfitLossResponse, AppError> {
        let balances = self.repo.get_account_balances_by_type(q.period_start, q.period_end).await?;
        let mut revenue = Vec::new();
        let mut expenses = Vec::new();
        for b in balances {
            let item = AccountLineItem { account_id: b.0, account_code: b.1, account_name: b.2.clone(), amount: b.4 - b.5 };
            match b.3.as_str() {
                "revenue" => revenue.push(item),
                "expense" => expenses.push(item),
                _ => {}
            }
        }
        let total_revenue: rust_decimal::Decimal = revenue.iter().map(|i| i.amount).sum();
        let total_expenses: rust_decimal::Decimal = expenses.iter().map(|i| i.amount).sum();
        Ok(ProfitLossResponse { period_start: q.period_start, period_end: q.period_end, revenue, total_revenue, expenses, total_expenses, net_income: total_revenue - total_expenses })
    }

    pub async fn balance_sheet(&self, q: TrialBalanceQuery) -> Result<BalanceSheetResponse, AppError> {
        let balances = self.repo.get_account_balances_by_type(q.period_start, q.period_end).await?;
        let mut assets = Vec::new();
        let mut liabilities = Vec::new();
        let mut equity = Vec::new();
        for b in balances {
            let item = AccountLineItem { account_id: b.0, account_code: b.1, account_name: b.2.clone(), amount: b.4 - b.5 };
            match b.3.as_str() {
                "asset" => assets.push(item),
                "liability" => liabilities.push(item),
                "equity" => equity.push(item),
                _ => {}
            }
        }
        let total_assets: rust_decimal::Decimal = assets.iter().map(|i| i.amount).sum();
        let total_liabilities: rust_decimal::Decimal = liabilities.iter().map(|i| i.amount).sum();
        let total_equity: rust_decimal::Decimal = equity.iter().map(|i| i.amount).sum();
        Ok(BalanceSheetResponse { as_of_date: q.period_end, assets, total_assets, liabilities, total_liabilities, equity, total_equity })
    }

    pub async fn cash_flow_statement(&self, q: TrialBalanceQuery) -> Result<CashFlowResponse, AppError> {
        let balances = self.repo.get_account_balances_by_type(q.period_start, q.period_end).await?;
        let operating: Vec<AccountLineItem> = balances.iter().filter(|b| b.3 == "revenue" || b.3 == "expense")
            .map(|b| AccountLineItem { account_id: b.0, account_code: b.1.clone(), account_name: b.2.clone(), amount: b.4 - b.5 }).collect();
        let net_cash_operating: rust_decimal::Decimal = operating.iter().map(|i| i.amount).sum();
        Ok(CashFlowResponse {
            period_start: q.period_start, period_end: q.period_end,
            operating_activities: operating, net_cash_operating,
            investing_activities: Vec::new(), net_cash_investing: rust_decimal::Decimal::ZERO,
            financing_activities: Vec::new(), net_cash_financing: rust_decimal::Decimal::ZERO,
            net_change_in_cash: net_cash_operating,
        })
    }

    pub async fn gst_return_data(&self, _return_type: &str, month: Option<i32>, year: Option<i32>) -> Result<GstReturnResponse, AppError> {
        use chrono::Datelike;
        let _m = month.unwrap_or(chrono::Utc::now().month() as i32);
        let _y = year.unwrap_or(chrono::Utc::now().year());
        Err(AppError::Internal(anyhow::anyhow!("GST invoice query not yet fully ported to SeaORM")))
    }
}
