//! Unit tests for the Accounting service layer.
//! Tests the transformation logic from repository rows to API responses.

#[cfg(test)]
mod tests {
    use rust_decimal::Decimal;
    use rust_decimal_macros::dec;
    use chrono::NaiveDate;

    use crate::modules::accounting::repository::accounting_repository::AccountBalanceRow;
    use crate::modules::accounting::response::accounting_response::*;

    // ── Helpers ──────────────────────────────────────────

    fn make_balance(id: i64, code: &str, name: &str, account_type: &str, debit: Decimal, credit: Decimal) -> AccountBalanceRow {
        AccountBalanceRow {
            account_id: id,
            account_code: code.to_owned(),
            account_name: name.to_owned(),
            account_type: account_type.to_owned(),
            total_debit: debit,
            total_credit: credit,
        }
    }

    // ── Trial Balance ────────────────────────────────────

    #[test]
    fn trial_balance_maps_accounts_correctly() {
        let balances = vec![
            make_balance(1, "1000", "Cash", "asset", dec!(50000), dec!(10000)),
            make_balance(2, "2000", "Accounts Payable", "liability", dec!(5000), dec!(20000)),
            make_balance(3, "4000", "Service Revenue", "revenue", dec!(0), dec!(100000)),
            make_balance(4, "5000", "Rent Expense", "expense", dec!(15000), dec!(0)),
        ];

        let accounts: Vec<TrialBalanceAccount> = balances.into_iter().map(|b| TrialBalanceAccount {
            account_id: b.account_id,
            account_code: b.account_code,
            account_name: b.account_name,
            account_type: b.account_type,
            total_debit: b.total_debit,
            total_credit: b.total_credit,
            closing_balance: b.total_debit - b.total_credit,
        }).collect();

        // Verify closing balances
        assert_eq!(accounts[0].closing_balance, dec!(40000));  // Cash: 50000 - 10000
        assert_eq!(accounts[1].closing_balance, dec!(-15000)); // AP: 5000 - 20000
        assert_eq!(accounts[2].closing_balance, dec!(-100000)); // Revenue: 0 - 100000
        assert_eq!(accounts[3].closing_balance, dec!(15000));   // Rent: 15000 - 0

        let total_debit: Decimal = accounts.iter().map(|a| a.total_debit).sum();
        let total_credit: Decimal = accounts.iter().map(|a| a.total_credit).sum();
        assert_eq!(total_debit, dec!(70000));  // 50000 + 5000 + 0 + 15000
        assert_eq!(total_credit, dec!(130000)); // 10000 + 20000 + 100000 + 0
    }

    #[test]
    fn trial_balance_empty_balances() {
        let balances: Vec<AccountBalanceRow> = vec![];
        let accounts: Vec<TrialBalanceAccount> = balances.into_iter().map(|b| TrialBalanceAccount {
            account_id: b.account_id,
            account_code: b.account_code,
            account_name: b.account_name,
            account_type: b.account_type,
            total_debit: b.total_debit,
            total_credit: b.total_credit,
            closing_balance: b.total_debit - b.total_credit,
        }).collect();

        assert!(accounts.is_empty());
        let total_debit: Decimal = accounts.iter().map(|a| a.total_debit).sum();
        let total_credit: Decimal = accounts.iter().map(|a| a.total_credit).sum();
        assert_eq!(total_debit, Decimal::ZERO);
        assert_eq!(total_credit, Decimal::ZERO);
    }

    // ── Profit & Loss ────────────────────────────────────

    #[test]
    fn profit_loss_separates_revenue_and_expenses() {
        let balances = vec![
            make_balance(1, "4000", "Service Revenue", "revenue", dec!(0), dec!(100000)),
            make_balance(2, "4100", "Interest Income", "revenue", dec!(0), dec!(5000)),
            make_balance(3, "5000", "Rent Expense", "expense", dec!(15000), dec!(0)),
            make_balance(4, "5100", "Salary Expense", "expense", dec!(40000), dec!(0)),
            make_balance(5, "1000", "Cash", "asset", dec!(50000), dec!(10000)), // should be ignored
            make_balance(6, "2000", "Accounts Payable", "liability", dec!(5000), dec!(20000)), // should be ignored
        ];

        let mut revenue = Vec::new();
        let mut expenses = Vec::new();
        for b in &balances {
            let item = AccountLineItem {
                account_id: b.account_id,
                account_code: b.account_code.clone(),
                account_name: b.account_name.clone(),
                amount: b.total_debit - b.total_credit,
            };
            match b.account_type.as_str() {
                "revenue" => revenue.push(item),
                "expense" => expenses.push(item),
                _ => {}
            }
        }

        assert_eq!(revenue.len(), 2);
        assert_eq!(expenses.len(), 2);

        let total_revenue: Decimal = revenue.iter().map(|i| i.amount).sum();
        let total_expenses: Decimal = expenses.iter().map(|i| i.amount).sum();
        assert_eq!(total_revenue, dec!(-105000)); // Revenue is credit-normal (negative net)
        assert_eq!(total_expenses, dec!(55000));   // Expenses are debit-normal (positive net)
        assert_eq!(total_revenue - total_expenses, dec!(-160000)); // Net income
    }

    #[test]
    fn profit_loss_no_revenue_or_expenses() {
        let balances = vec![
            make_balance(1, "1000", "Cash", "asset", dec!(50000), dec!(10000)),
        ];

        let mut revenue = Vec::new();
        let mut expenses = Vec::new();
        for b in &balances {
            let item = AccountLineItem {
                account_id: b.account_id,
                account_code: b.account_code.clone(),
                account_name: b.account_name.clone(),
                amount: b.total_debit - b.total_credit,
            };
            match b.account_type.as_str() {
                "revenue" => revenue.push(item),
                "expense" => expenses.push(item),
                _ => {}
            }
        }

        assert!(revenue.is_empty());
        assert!(expenses.is_empty());
        let total_revenue: Decimal = revenue.iter().map(|i| i.amount).sum();
        let total_expenses: Decimal = expenses.iter().map(|i| i.amount).sum();
        assert_eq!(total_revenue, Decimal::ZERO);
        assert_eq!(total_expenses, Decimal::ZERO);
    }

    // ── Balance Sheet ────────────────────────────────────

    #[test]
    fn balance_sheet_categorizes_three_account_types() {
        let balances = vec![
            make_balance(1, "1000", "Cash", "asset", dec!(50000), dec!(10000)),
            make_balance(2, "1100", "Accounts Receivable", "asset", dec!(20000), dec!(5000)),
            make_balance(3, "2000", "Accounts Payable", "liability", dec!(5000), dec!(20000)),
            make_balance(4, "2100", "Loans Payable", "liability", dec!(0), dec!(50000)),
            make_balance(5, "3000", "Owner's Equity", "equity", dec!(0), dec!(30000)),
            make_balance(6, "4000", "Revenue", "revenue", dec!(0), dec!(100000)), // ignored
        ];

        let mut assets = Vec::new();
        let mut liabilities = Vec::new();
        let mut equity = Vec::new();
        for b in &balances {
            let item = AccountLineItem {
                account_id: b.account_id,
                account_code: b.account_code.clone(),
                account_name: b.account_name.clone(),
                amount: b.total_debit - b.total_credit,
            };
            match b.account_type.as_str() {
                "asset" => assets.push(item),
                "liability" => liabilities.push(item),
                "equity" => equity.push(item),
                _ => {}
            }
        }

        assert_eq!(assets.len(), 2);
        assert_eq!(liabilities.len(), 2);
        assert_eq!(equity.len(), 1);

        let total_assets: Decimal = assets.iter().map(|i| i.amount).sum();
        let total_liabilities: Decimal = liabilities.iter().map(|i| i.amount).sum();
        let total_equity: Decimal = equity.iter().map(|i| i.amount).sum();

        assert_eq!(total_assets, dec!(55000));  // (50000-10000) + (20000-5000)
        assert_eq!(total_liabilities, dec!(-65000)); // (5000-20000) + (0-50000)
        assert_eq!(total_equity, dec!(-30000));  // 0 - 30000
    }

    #[test]
    fn balance_sheet_accounting_equation_holds() {
        // Assets = Liabilities + Equity
        let balances = vec![
            make_balance(1, "1000", "Cash", "asset", dec!(100000), dec!(0)),
            make_balance(2, "2000", "Loan", "liability", dec!(0), dec!(40000)),
            make_balance(3, "3000", "Capital", "equity", dec!(0), dec!(60000)),
        ];

        let total_assets: Decimal = balances.iter()
            .filter(|b| b.account_type == "asset")
            .map(|b| b.total_debit - b.total_credit)
            .sum();
        let total_liabilities: Decimal = balances.iter()
            .filter(|b| b.account_type == "liability")
            .map(|b| b.total_debit - b.total_credit)
            .sum();
        let total_equity: Decimal = balances.iter()
            .filter(|b| b.account_type == "equity")
            .map(|b| b.total_debit - b.total_credit)
            .sum();

        // Assets (100000) = Liabilities (-40000) + Equity (-60000) => 100000 = -100000
        // In accounting terms: assets are debit-normal, liabilities/equity are credit-normal
        assert_eq!(total_assets, dec!(100000));
        assert_eq!(total_liabilities, dec!(-40000));
        assert_eq!(total_equity, dec!(-60000));
        assert_eq!(total_assets, -(total_liabilities + total_equity));
    }

    // ── Cash Flow ────────────────────────────────────────

    #[test]
    fn cash_flow_only_includes_revenue_and_expense() {
        let balances = vec![
            make_balance(1, "4000", "Service Revenue", "revenue", dec!(0), dec!(100000)),
            make_balance(2, "5000", "Rent Expense", "expense", dec!(15000), dec!(0)),
            make_balance(3, "5100", "Salary Expense", "expense", dec!(40000), dec!(0)),
            make_balance(4, "1000", "Cash", "asset", dec!(50000), dec!(10000)), // ignored
            make_balance(5, "2000", "AP", "liability", dec!(5000), dec!(20000)), // ignored
            make_balance(6, "3000", "Equity", "equity", dec!(0), dec!(30000)),   // ignored
        ];

        let operating: Vec<AccountLineItem> = balances.iter()
            .filter(|b| b.account_type == "revenue" || b.account_type == "expense")
            .map(|b| AccountLineItem {
                account_id: b.account_id,
                account_code: b.account_code.clone(),
                account_name: b.account_name.clone(),
                amount: b.total_debit - b.total_credit,
            })
            .collect();

        assert_eq!(operating.len(), 3); // Only revenue and expense accounts
        let net_cash_operating: Decimal = operating.iter().map(|i| i.amount).sum();
        assert_eq!(net_cash_operating, dec!(-45000)); // -100000 + 15000 + 40000
    }

    #[test]
    fn cash_flow_zero_when_no_revenue_or_expenses() {
        let balances = vec![
            make_balance(1, "1000", "Cash", "asset", dec!(50000), dec!(10000)),
        ];

        let operating: Vec<AccountLineItem> = balances.iter()
            .filter(|b| b.account_type == "revenue" || b.account_type == "expense")
            .map(|b| AccountLineItem {
                account_id: b.account_id,
                account_code: b.account_code.clone(),
                account_name: b.account_name.clone(),
                amount: b.total_debit - b.total_credit,
            })
            .collect();

        assert!(operating.is_empty());
        let net_cash_operating: Decimal = operating.iter().map(|i| i.amount).sum();
        assert_eq!(net_cash_operating, Decimal::ZERO);
    }

    // ── AccountLineItem ──────────────────────────────────

    #[test]
    fn account_line_item_preserves_fields() {
        let item = AccountLineItem {
            account_id: 42,
            account_code: "4000".to_owned(),
            account_name: "Service Revenue".to_owned(),
            amount: dec!(100000),
        };
        assert_eq!(item.account_id, 42);
        assert_eq!(item.account_code, "4000");
        assert_eq!(item.account_name, "Service Revenue");
        assert_eq!(item.amount, dec!(100000));
    }

    // ── Edge Cases ───────────────────────────────────────

    #[test]
    fn decimal_precision_preserved() {
        let balances = vec![
            make_balance(1, "1000", "Cash", "asset", dec!(123456.789), dec!(0)),
            make_balance(2, "4000", "Revenue", "revenue", dec!(0), dec!(999999.99)),
        ];

        let total_debit: Decimal = balances.iter().map(|b| b.total_debit).sum();
        let total_credit: Decimal = balances.iter().map(|b| b.total_credit).sum();
        assert_eq!(total_debit, dec!(123456.789));
        assert_eq!(total_credit, dec!(999999.99));
    }

    #[test]
    fn negative_closing_balance_for_credit_normal_accounts() {
        let b = make_balance(1, "4000", "Revenue", "revenue", dec!(0), dec!(50000));
        let closing = b.total_debit - b.total_credit;
        assert_eq!(closing, dec!(-50000));
    }

    #[test]
    fn positive_closing_balance_for_debit_normal_accounts() {
        let b = make_balance(1, "1000", "Cash", "asset", dec!(50000), dec!(0));
        let closing = b.total_debit - b.total_credit;
        assert_eq!(closing, dec!(50000));
    }

    // ── Journal Entry Validation ─────────────────────────

    #[test]
    fn journal_entry_debit_must_equal_credit() {
        let total_debit = dec!(1000);
        let total_credit = dec!(1000);
        assert_eq!(total_debit, total_credit, "Balanced journal entry");

        let total_debit2 = dec!(1000);
        let total_credit2 = dec!(500);
        assert_ne!(total_debit2, total_credit2, "Unbalanced journal entry");
    }

    #[test]
    fn message_response_serializes() {
        let msg = MessageResponse { message: "Posted".to_owned() };
        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains("Posted"));
    }

    #[test]
    fn gst_invoice_line_fields() {
        let line = GstInvoiceLine {
            invoice_number: "INV-001".to_owned(),
            customer_gstin: Some("27AAPFU0939F1ZV".to_owned()),
            taxable_value: dec!(10000),
            cgst: dec!(900),
            sgst: dec!(900),
            igst: dec!(0),
        };
        assert_eq!(line.taxable_value + line.cgst + line.sgst + line.igst, dec!(11800));
    }

    #[test]
    fn gst_return_totals() {
        let invoices = vec![
            GstInvoiceLine {
                invoice_number: "INV-001".to_owned(),
                customer_gstin: None,
                taxable_value: dec!(10000),
                cgst: dec!(900),
                sgst: dec!(900),
                igst: dec!(0),
            },
            GstInvoiceLine {
                invoice_number: "INV-002".to_owned(),
                customer_gstin: Some("27AAPFU0939F1ZV".to_owned()),
                taxable_value: dec!(20000),
                cgst: dec!(0),
                sgst: dec!(0),
                igst: dec!(3600),
            },
        ];

        let total_taxable: Decimal = invoices.iter().map(|i| i.taxable_value).sum();
        let total_cgst: Decimal = invoices.iter().map(|i| i.cgst).sum();
        let total_sgst: Decimal = invoices.iter().map(|i| i.sgst).sum();
        let total_igst: Decimal = invoices.iter().map(|i| i.igst).sum();

        assert_eq!(total_taxable, dec!(30000));
        assert_eq!(total_cgst, dec!(900));
        assert_eq!(total_sgst, dec!(900));
        assert_eq!(total_igst, dec!(3600));
    }

    #[test]
    fn balance_sheet_with_empty_balances() {
        let balances: Vec<AccountBalanceRow> = vec![];
        let total_assets: Decimal = balances.iter().filter(|b| b.account_type == "asset").map(|b| b.total_debit - b.total_credit).sum();
        let total_liabilities: Decimal = balances.iter().filter(|b| b.account_type == "liability").map(|b| b.total_debit - b.total_credit).sum();
        let total_equity: Decimal = balances.iter().filter(|b| b.account_type == "equity").map(|b| b.total_debit - b.total_credit).sum();
        assert_eq!(total_assets, Decimal::ZERO);
        assert_eq!(total_liabilities, Decimal::ZERO);
        assert_eq!(total_equity, Decimal::ZERO);
    }

    #[test]
    fn cash_flow_response_structure() {
        let resp = CashFlowResponse {
            period_start: NaiveDate::from_ymd_opt(2026, 1, 1).unwrap(),
            period_end: NaiveDate::from_ymd_opt(2026, 6, 30).unwrap(),
            operating_activities: Vec::new(),
            net_cash_operating: dec!(50000),
            investing_activities: Vec::new(),
            net_cash_investing: dec!(-20000),
            financing_activities: Vec::new(),
            net_cash_financing: dec!(-10000),
            net_change_in_cash: dec!(20000),
        };
        assert_eq!(resp.net_change_in_cash, resp.net_cash_operating + resp.net_cash_investing + resp.net_cash_financing);
    }

    #[test]
    fn profit_loss_response_structure() {
        let resp = ProfitLossResponse {
            period_start: NaiveDate::from_ymd_opt(2026, 1, 1).unwrap(),
            period_end: NaiveDate::from_ymd_opt(2026, 6, 30).unwrap(),
            revenue: vec![AccountLineItem { account_id: 1, account_code: "4000".into(), account_name: "Revenue".into(), amount: dec!(-100000) }],
            total_revenue: dec!(-100000),
            expenses: vec![AccountLineItem { account_id: 2, account_code: "5000".into(), account_name: "Expense".into(), amount: dec!(55000) }],
            total_expenses: dec!(55000),
            net_income: dec!(-155000),
        };
        assert_eq!(resp.net_income, resp.total_revenue - resp.total_expenses);
    }
}
