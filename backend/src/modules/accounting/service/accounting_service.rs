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
            account_id: b.account_id, account_code: b.account_code, account_name: b.account_name, account_type: b.account_type,
            total_debit: b.total_debit, total_credit: b.total_credit, closing_balance: b.total_debit - b.total_credit,
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
            let item = AccountLineItem { account_id: b.account_id, account_code: b.account_code, account_name: b.account_name.clone(), amount: b.total_debit - b.total_credit };
            match b.account_type.as_str() {
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
            let item = AccountLineItem { account_id: b.account_id, account_code: b.account_code, account_name: b.account_name.clone(), amount: b.total_debit - b.total_credit };
            match b.account_type.as_str() {
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
        let operating: Vec<AccountLineItem> = balances.iter().filter(|b| b.account_type == "revenue" || b.account_type == "expense")
            .map(|b| AccountLineItem { account_id: b.account_id, account_code: b.account_code.clone(), account_name: b.account_name.clone(), amount: b.total_debit - b.total_credit }).collect();
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
        let m = month.unwrap_or(chrono::Utc::now().month() as i32);
        let y = year.unwrap_or(chrono::Utc::now().year());
        let invoices = self.repo.get_gst_invoices(m, y).await?;
        let mut total_taxable = rust_decimal::Decimal::ZERO;
        let mut total_cgst = rust_decimal::Decimal::ZERO;
        let mut total_sgst = rust_decimal::Decimal::ZERO;
        let mut total_igst = rust_decimal::Decimal::ZERO;
        let items: Vec<GstInvoiceLine> = invoices.into_iter().map(|(num, gstin, taxable, cgst, sgst, igst)| {
            total_taxable += taxable;
            total_cgst += cgst;
            total_sgst += sgst;
            total_igst += igst;
            GstInvoiceLine {
                invoice_number: num,
                customer_gstin: gstin,
                taxable_value: taxable,
                cgst,
                sgst,
                igst,
            }
        }).collect();
        Ok(GstReturnResponse {
            return_type: _return_type.to_string(),
            period_month: m,
            period_year: y,
            total_taxable_value: total_taxable,
            total_cgst: total_cgst,
            total_sgst: total_sgst,
            total_igst: total_igst,
            invoices: items,
        })
    }

    /// Generate GSTR-1 report — outward supplies with invoice-level detail.
    /// Separates B2B (with GSTIN) from B2C (without GSTIN) invoices.
    pub async fn gstr1(&self, month: i32, year: i32) -> Result<Gstr1Response, AppError> {
        let invoices = self.repo.get_paid_invoices_for_period(month, year).await?;
        let supplier_gstin = self.repo.get_branch_gstin(invoices.first().map(|i| i.branch_id).unwrap_or(0)).await;

        let mut gstr1_invoices = Vec::new();
        let mut b2c = Gstr1B2cSummary {
            total_taxable_value: rust_decimal::Decimal::ZERO,
            total_cgst: rust_decimal::Decimal::ZERO,
            total_sgst: rust_decimal::Decimal::ZERO,
            total_igst: rust_decimal::Decimal::ZERO,
            invoice_count: 0,
        };
        let mut total_taxable = rust_decimal::Decimal::ZERO;
        let mut total_cgst = rust_decimal::Decimal::ZERO;
        let mut total_sgst = rust_decimal::Decimal::ZERO;
        let mut total_igst = rust_decimal::Decimal::ZERO;

        for inv in &invoices {
            let customer_name = self.repo.get_customer_name(inv.customer_id).await?;
            let customer_gstin = self.repo.get_customer_gstin(inv.customer_id).await?;
            let branch_state = self.repo.get_branch_state(inv.branch_id).await?
                .unwrap_or_else(|| "Maharashtra".to_string());

            let invoice_line = Gstr1Invoice {
                invoice_number: inv.invoice_number.clone(),
                invoice_date: inv.billing_period_start,
                customer_gstin: customer_gstin.clone(),
                customer_name,
                place_of_supply: branch_state,
                supply_type: "Regular".to_string(),
                taxable_value: inv.subtotal,
                cgst: inv.cgst_amount,
                sgst: inv.sgst_amount,
                igst: inv.igst_amount,
                invoice_value: inv.total_amount,
            };

            total_taxable += inv.subtotal;
            total_cgst += inv.cgst_amount;
            total_sgst += inv.sgst_amount;
            total_igst += inv.igst_amount;

            if customer_gstin.is_some() {
                gstr1_invoices.push(invoice_line);
            } else {
                b2c.total_taxable_value += inv.subtotal;
                b2c.total_cgst += inv.cgst_amount;
                b2c.total_sgst += inv.sgst_amount;
                b2c.total_igst += inv.igst_amount;
                b2c.invoice_count += 1;
            }
        }

        Ok(Gstr1Response {
            period_month: month,
            period_year: year,
            supplier_gstin,
            total_taxable_value: total_taxable,
            total_cgst,
            total_sgst,
            total_igst,
            total_invoices: invoices.len() as i64,
            invoices: gstr1_invoices,
            b2c_summary: b2c,
        })
    }

    /// Generate GSTR-3B report — monthly summary return with tax liability.
    pub async fn gstr3b(&self, month: i32, year: i32) -> Result<Gstr3bResponse, AppError> {
        let invoices = self.repo.get_paid_invoices_for_period(month, year).await?;
        let first_branch_id = invoices.first().map(|i| i.branch_id).unwrap_or(0);
        let supplier_gstin = self.repo.get_branch_gstin(first_branch_id).await;
        let supplier_state = self.repo.get_branch_state(first_branch_id).await?;

        let mut outward_taxable = rust_decimal::Decimal::ZERO;
        let mut outward_cgst = rust_decimal::Decimal::ZERO;
        let mut outward_sgst = rust_decimal::Decimal::ZERO;
        let mut outward_igst = rust_decimal::Decimal::ZERO;
        let mut interstate_taxable = rust_decimal::Decimal::ZERO;
        let mut interstate_igst = rust_decimal::Decimal::ZERO;

        for inv in &invoices {
            outward_taxable += inv.subtotal;
            outward_cgst += inv.cgst_amount;
            outward_sgst += inv.sgst_amount;
            outward_igst += inv.igst_amount;

            if inv.igst_amount > rust_decimal::Decimal::ZERO {
                interstate_taxable += inv.subtotal;
                interstate_igst += inv.igst_amount;
            }
        }

        let outward_total = outward_cgst + outward_sgst + outward_igst;

        Ok(Gstr3bResponse {
            period_month: month,
            period_year: year,
            supplier_gstin,
            supplier_state,
            outward: Gstr3bOutward {
                taxable_value: outward_taxable,
                cgst: outward_cgst,
                sgst: outward_sgst,
                igst: outward_igst,
                total: outward_total,
            },
            interstate: Gstr3bInterstate {
                taxable_value: interstate_taxable,
                igst: interstate_igst,
            },
            tax_payable: Gstr3bTaxPayable {
                total_cgst: outward_cgst,
                total_sgst: outward_sgst,
                total_igst: outward_igst,
                total_tax: outward_total,
            },
        })
    }
}
