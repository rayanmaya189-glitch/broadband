use sqlx::PgPool;
use crate::common::errors::app_error::AppError;
use crate::modules::accounting::repository::accounting_repository::AccountingRepository;
use crate::modules::accounting::request::accounting_request::*;
use crate::modules::accounting::response::accounting_response::*;

pub struct AccountingService<'a> { repo: AccountingRepository<'a> }
impl<'a> AccountingService<'a> {
    pub fn new(pool: &'a PgPool) -> Self { Self { repo: AccountingRepository::new(pool) } }

    pub async fn list_accounts(&self) -> Result<Vec<AccountResponse>, AppError> {
        let a = self.repo.list_accounts().await?;
        Ok(a.iter().map(|x| AccountResponse { id: x.id, code: x.code.clone(), name: x.name.clone(), account_type: x.account_type.clone(), is_active: x.is_active, created_at: x.created_at }).collect())
    }

    pub async fn create_account(&self, req: CreateAccountRequest) -> Result<AccountResponse, AppError> {
        let a = self.repo.create_account(&req.code, &req.name, &req.account_type, req.parent_id).await?;
        Ok(AccountResponse { id: a.id, code: a.code, name: a.name, account_type: a.account_type, is_active: a.is_active, created_at: a.created_at })
    }

    pub async fn list_journal_entries(&self, page: i64, per_page: i64) -> Result<Vec<JournalEntryResponse>, AppError> {
        let (e, _) = self.repo.list_journal_entries(page, per_page).await?;
        Ok(e.iter().map(|x| JournalEntryResponse { id: x.id, entry_number: x.entry_number.clone(), entry_date: x.entry_date, description: x.description.clone(), total_debit: x.total_debit, total_credit: x.total_credit, status: x.status.clone(), created_at: x.created_at }).collect())
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
            lines.push(JournalEntryLineResponse { id: l.id, journal_entry_id: l.journal_entry_id, account_id: l.account_id, debit: l.debit, credit: l.credit, description: l.description, created_at: l.created_at });
        }
        Ok(JournalEntryDetailResponse {
            entry: JournalEntryResponse { id: e.id, entry_number: e.entry_number, entry_date: e.entry_date, description: e.description, total_debit: e.total_debit, total_credit: e.total_credit, status: e.status, created_at: e.created_at },
            lines,
        })
    }

    pub async fn get_entry_lines(&self, entry_id: i64) -> Result<Vec<JournalEntryLineResponse>, AppError> {
        let lines = self.repo.list_entry_lines(entry_id).await?;
        Ok(lines.iter().map(|l| JournalEntryLineResponse { id: l.id, journal_entry_id: l.journal_entry_id, account_id: l.account_id, debit: l.debit, credit: l.credit, description: l.description.clone(), created_at: l.created_at }).collect())
    }

    pub async fn post_entry(&self, id: i64) -> Result<MessageResponse, AppError> {
        if !self.repo.post_entry(id).await? { return Err(AppError::NotFound("Entry not found".into())); }
        Ok(MessageResponse { message: "Posted".into() })
    }

    pub async fn void_entry(&self, id: i64) -> Result<MessageResponse, AppError> {
        if !self.repo.void_entry(id).await? { return Err(AppError::NotFound("Entry not found".into())); }
        Ok(MessageResponse { message: "Voided".into() })
    }

    pub async fn trial_balance(&self, query: TrialBalanceQuery) -> Result<TrialBalanceResponse, AppError> {
        let tb = self.repo.generate_trial_balance(query.period_start, query.period_end).await?;
        // Fetch account details to populate the response
        let accounts_list = self.repo.list_accounts().await?;
        let account_map: std::collections::HashMap<i64, _> = accounts_list.into_iter().map(|a| (a.id, a)).collect();
        let accounts: Vec<TrialBalanceAccount> = tb.iter().map(|t| {
            let acc = account_map.get(&t.account_id);
            TrialBalanceAccount {
                account_id: t.account_id,
                account_code: acc.map(|a| a.code.clone()).unwrap_or_default(),
                account_name: acc.map(|a| a.name.clone()).unwrap_or_default(),
                account_type: acc.map(|a| a.account_type.clone()).unwrap_or_default(),
                total_debit: t.total_debit,
                total_credit: t.total_credit,
                closing_balance: t.closing_balance,
            }
        }).collect();
        let total_debit: rust_decimal::Decimal = tb.iter().map(|t| t.total_debit).sum();
        let total_credit: rust_decimal::Decimal = tb.iter().map(|t| t.total_credit).sum();
        Ok(TrialBalanceResponse { period_start: query.period_start, period_end: query.period_end, accounts, total_debit, total_credit })
    }

    // ── Profit & Loss Statement ────────────────────────────────

    pub async fn profit_loss_statement(&self, query: TrialBalanceQuery) -> Result<ProfitLossResponse, AppError> {
        let accounts = self.repo.get_account_balances_by_type(query.period_start, query.period_end).await?;
        let revenue: Vec<AccountLineItem> = accounts.iter()
            .filter(|a| a.account_type == "revenue")
            .map(|a| AccountLineItem { account_id: a.account_id, account_code: a.account_code.clone(), account_name: a.account_name.clone(), amount: a.total_credit - a.total_debit })
            .collect();
        let expenses: Vec<AccountLineItem> = accounts.iter()
            .filter(|a| a.account_type == "expense")
            .map(|a| AccountLineItem { account_id: a.account_id, account_code: a.account_code.clone(), account_name: a.account_name.clone(), amount: a.total_debit - a.total_credit })
            .collect();
        let total_revenue: rust_decimal::Decimal = revenue.iter().map(|r| r.amount).sum();
        let total_expenses: rust_decimal::Decimal = expenses.iter().map(|e| e.amount).sum();
        Ok(ProfitLossResponse { period_start: query.period_start, period_end: query.period_end, revenue, total_revenue, expenses, total_expenses, net_income: total_revenue - total_expenses })
    }

    // ── Balance Sheet ──────────────────────────────────────────

    pub async fn balance_sheet(&self, query: TrialBalanceQuery) -> Result<BalanceSheetResponse, AppError> {
        let accounts = self.repo.get_account_balances_by_type(query.period_start, query.period_end).await?;
        let assets: Vec<AccountLineItem> = accounts.iter()
            .filter(|a| a.account_type == "asset")
            .map(|a| AccountLineItem { account_id: a.account_id, account_code: a.account_code.clone(), account_name: a.account_name.clone(), amount: a.total_debit - a.total_credit })
            .collect();
        let liabilities: Vec<AccountLineItem> = accounts.iter()
            .filter(|a| a.account_type == "liability")
            .map(|a| AccountLineItem { account_id: a.account_id, account_code: a.account_code.clone(), account_name: a.account_name.clone(), amount: a.total_credit - a.total_debit })
            .collect();
        let equity: Vec<AccountLineItem> = accounts.iter()
            .filter(|a| a.account_type == "equity")
            .map(|a| AccountLineItem { account_id: a.account_id, account_code: a.account_code.clone(), account_name: a.account_name.clone(), amount: a.total_credit - a.total_debit })
            .collect();
        let total_assets: rust_decimal::Decimal = assets.iter().map(|a| a.amount).sum();
        let total_liabilities: rust_decimal::Decimal = liabilities.iter().map(|l| l.amount).sum();
        let total_equity: rust_decimal::Decimal = equity.iter().map(|e| e.amount).sum();
        Ok(BalanceSheetResponse { as_of_date: query.period_end, assets, total_assets, liabilities, total_liabilities, equity, total_equity })
    }

    // ── Cash Flow Statement ────────────────────────────────────

    pub async fn cash_flow_statement(&self, query: TrialBalanceQuery) -> Result<CashFlowResponse, AppError> {
        let accounts = self.repo.get_account_balances_by_type(query.period_start, query.period_end).await?;
        let operating: Vec<AccountLineItem> = accounts.iter()
            .filter(|a| matches!(a.account_code.as_str(), "1200" | "4000" | "4100" | "4200" | "4300" | "4400" | "5200" | "5300" | "5400" | "5500" | "5700"))
            .map(|a| {
                let amount = if a.account_type == "revenue" { a.total_credit - a.total_debit }
                    else if a.account_type == "expense" { a.total_debit - a.total_credit }
                    else { a.total_debit - a.total_credit };
                AccountLineItem { account_id: a.account_id, account_code: a.account_code.clone(), account_name: a.account_name.clone(), amount }
            })
            .collect();
        let investing: Vec<AccountLineItem> = accounts.iter()
            .filter(|a| matches!(a.account_code.as_str(), "1400" | "1500"))
            .map(|a| AccountLineItem { account_id: a.account_id, account_code: a.account_code.clone(), account_name: a.account_name.clone(), amount: a.total_debit - a.total_credit })
            .collect();
        let financing: Vec<AccountLineItem> = accounts.iter()
            .filter(|a| matches!(a.account_code.as_str(), "2300" | "2400" | "3000" | "3100"))
            .map(|a| AccountLineItem { account_id: a.account_id, account_code: a.account_code.clone(), account_name: a.account_name.clone(), amount: a.total_credit - a.total_debit })
            .collect();
        let net_operating: rust_decimal::Decimal = operating.iter().map(|o| o.amount).sum();
        let net_investing: rust_decimal::Decimal = investing.iter().map(|i| i.amount).sum();
        let net_financing: rust_decimal::Decimal = financing.iter().map(|f| f.amount).sum();
        Ok(CashFlowResponse { period_start: query.period_start, period_end: query.period_end, operating_activities: operating, net_cash_operating: net_operating, investing_activities: investing, net_cash_investing: net_investing, financing_activities: financing, net_cash_financing: net_financing, net_change_in_cash: net_operating + net_investing + net_financing })
    }

    // ── GST Returns ────────────────────────────────────────────

    pub async fn gst_return_data(&self, return_type: &str, month: i32, year: i32) -> Result<GstReturnResponse, AppError> {
        if !matches!(return_type, "GSTR1" | "GSTR3B") {
            return Err(AppError::Validation("return_type must be 'GSTR1' or 'GSTR3B'".into()));
        }
        let invoices = self.repo.get_gst_invoices(month, year).await?;
        let total_taxable: rust_decimal::Decimal = invoices.iter().map(|i| i.taxable_value).sum();
        let total_cgst: rust_decimal::Decimal = invoices.iter().map(|i| i.cgst).sum();
        let total_sgst: rust_decimal::Decimal = invoices.iter().map(|i| i.sgst).sum();
        let total_igst: rust_decimal::Decimal = invoices.iter().map(|i| i.igst).sum();
        Ok(GstReturnResponse {
            return_type: return_type.to_string(),
            period_month: month,
            period_year: year,
            total_taxable_value: total_taxable,
            total_cgst,
            total_sgst,
            total_igst,
            invoices: invoices.into_iter().map(|i| GstInvoiceLine { invoice_number: i.invoice_number, customer_gstin: i.customer_gstin, taxable_value: i.taxable_value, cgst: i.cgst, sgst: i.sgst, igst: i.igst }).collect(),
        })
    }
}
