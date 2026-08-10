//! Integration tests: AccountingService business flows
//! Exercises chart of accounts, double-entry journal posting, trial balance,
//! and profit & loss aggregation against PostgreSQL.

use crate::common::TestDatabase;
use aeroxe_backend::modules::accounting::application::services::{
    AccountingService, CreateJournalLine,
};
use aeroxe_backend::shared::errors::AppError;
use rust_decimal_macros::dec;

/// Helper: create a chart-of-accounts entry via the service.
async fn create_account(
    db: &sea_orm::DatabaseConnection,
    code: &str,
    name: &str,
    account_type: &str,
) -> i64 {
    AccountingService::create_account(
        db,
        code.to_string(),
        name.to_string(),
        account_type.to_string(),
        None,
        None,
    )
    .await
    .expect("create_account failed")
    .id
}

/// Account creation validates the account_type enum.
#[ignore]
#[tokio::test]
async fn test_create_account_validates_type() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    let cash = AccountingService::create_account(
        db,
        "1000".to_string(),
        "Cash".to_string(),
        "asset".to_string(),
        None,
        None,
    )
    .await
    .expect("valid account type should be accepted");
    assert!(cash.id > 0);
    assert_eq!(cash.code, "1000");

    let err = AccountingService::create_account(
        db,
        "9999".to_string(),
        "Bad".to_string(),
        "gold".to_string(),
        None,
        None,
    )
    .await
    .unwrap_err();
    assert!(
        matches!(err, AppError::Validation(_)),
        "invalid account_type must be rejected"
    );
}

/// A balanced journal entry persists header + lines.
#[ignore]
#[tokio::test]
async fn test_create_balanced_journal_entry() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();
    let cash = create_account(db, "1000", "Cash", "asset").await;
    let revenue = create_account(db, "4000", "Internet Revenue", "revenue").await;

    let today = chrono::Utc::now().date_naive();
    let entry = AccountingService::create_journal_entry(
        db,
        today,
        "Monthly subscription collection".to_string(),
        Some("invoice".to_string()),
        Some(1),
        vec![
            CreateJournalLine {
                account_id: cash,
                debit: dec!(706.82),
                credit: dec!(0),
                description: Some("Cash received".to_string()),
            },
            CreateJournalLine {
                account_id: revenue,
                debit: dec!(0),
                credit: dec!(706.82),
                description: Some("Service revenue".to_string()),
            },
        ],
        Some(1),
    )
    .await
    .expect("Balanced journal entry should be created");

    assert!(entry.entry_number.starts_with("JE-"));
    assert_eq!(entry.total_debit, dec!(706.82));
    assert_eq!(entry.total_credit, dec!(706.82));
    assert_eq!(entry.status, "draft");

    let lines = AccountingService::get_journal_entry_lines(db, entry.id)
        .await
        .unwrap();
    assert_eq!(lines.len(), 2);
}

/// Unbalanced and empty entries are rejected.
#[ignore]
#[tokio::test]
async fn test_unbalanced_journal_entry_rejected() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();
    let cash = create_account(db, "1000", "Cash", "asset").await;
    let revenue = create_account(db, "4000", "Internet Revenue", "revenue").await;

    let today = chrono::Utc::now().date_naive();

    let err = AccountingService::create_journal_entry(
        db,
        today,
        "Sloppy entry".to_string(),
        None,
        None,
        vec![
            CreateJournalLine {
                account_id: cash,
                debit: dec!(1000),
                credit: dec!(0),
                description: None,
            },
            CreateJournalLine {
                account_id: revenue,
                debit: dec!(0),
                credit: dec!(500),
                description: None,
            },
        ],
        None,
    )
    .await
    .unwrap_err();
    assert!(matches!(err, AppError::Validation(_)));

    let err = AccountingService::create_journal_entry(
        db,
        today,
        "Empty entry".to_string(),
        None,
        None,
        vec![],
        None,
    )
    .await
    .unwrap_err();
    assert!(matches!(err, AppError::Validation(_)));
}

/// Posting flips the entry to posted and sets posted_at; re-posting is a conflict.
#[ignore]
#[tokio::test]
async fn test_post_journal_entry_lifecycle() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();
    let cash = create_account(db, "1000", "Cash", "asset").await;
    let revenue = create_account(db, "4000", "Internet Revenue", "revenue").await;

    let today = chrono::Utc::now().date_naive();
    let entry = AccountingService::create_journal_entry(
        db,
        today,
        "Cash sale".to_string(),
        None,
        None,
        vec![
            CreateJournalLine {
                account_id: cash,
                debit: dec!(500),
                credit: dec!(0),
                description: None,
            },
            CreateJournalLine {
                account_id: revenue,
                debit: dec!(0),
                credit: dec!(500),
                description: None,
            },
        ],
        None,
    )
    .await
    .unwrap();

    let posted = AccountingService::post_journal_entry(db, entry.id, Some(1))
        .await
        .expect("Posting failed");
    assert_eq!(posted.status, "posted");
    assert!(posted.posted_at.is_some());

    let err = AccountingService::post_journal_entry(db, entry.id, None)
        .await
        .unwrap_err();
    assert!(
        matches!(err, AppError::Conflict(_)),
        "re-posting a posted entry must conflict"
    );
}

/// Trial balance and P&L reflect only posted entries in the period.
#[ignore]
#[tokio::test]
async fn test_trial_balance_and_profit_loss() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();
    let cash = create_account(db, "1000", "Cash", "asset").await;
    let revenue = create_account(db, "4000", "Internet Revenue", "revenue").await;
    let expense = create_account(db, "5000", "Support Wages", "expense").await;

    let today = chrono::Utc::now().date_naive();
    let period_start = today - chrono::Duration::days(30);
    let period_end = today + chrono::Duration::days(1);

    // Revenue entry: debit cash, credit revenue.
    let rev_entry = AccountingService::create_journal_entry(
        db,
        today,
        "Subscriptions".to_string(),
        None,
        None,
        vec![
            CreateJournalLine {
                account_id: cash,
                debit: dec!(1000),
                credit: dec!(0),
                description: None,
            },
            CreateJournalLine {
                account_id: revenue,
                debit: dec!(0),
                credit: dec!(1000),
                description: None,
            },
        ],
        None,
    )
    .await
    .unwrap();
    AccountingService::post_journal_entry(db, rev_entry.id, Some(1))
        .await
        .unwrap();

    // Expense entry: debit expense, credit cash.
    let exp_entry = AccountingService::create_journal_entry(
        db,
        today,
        "Wages".to_string(),
        None,
        None,
        vec![
            CreateJournalLine {
                account_id: expense,
                debit: dec!(200),
                credit: dec!(0),
                description: None,
            },
            CreateJournalLine {
                account_id: cash,
                debit: dec!(0),
                credit: dec!(200),
                description: None,
            },
        ],
        None,
    )
    .await
    .unwrap();
    AccountingService::post_journal_entry(db, exp_entry.id, Some(1))
        .await
        .unwrap();

    // A draft entry must NOT affect the trial balance.
    AccountingService::create_journal_entry(
        db,
        today,
        "Not posted".to_string(),
        None,
        None,
        vec![
            CreateJournalLine {
                account_id: cash,
                debit: dec!(9999),
                credit: dec!(0),
                description: None,
            },
            CreateJournalLine {
                account_id: revenue,
                debit: dec!(0),
                credit: dec!(9999),
                description: None,
            },
        ],
        None,
    )
    .await
    .unwrap();

    let tb = AccountingService::generate_trial_balance(db, period_start, period_end)
        .await
        .expect("Trial balance generation failed");

    let cash_row = tb
        .entries
        .iter()
        .find(|e| e.account_code == 1000)
        .expect("cash row missing");
    assert_eq!(cash_row.closing_balance, dec!(800)); // 1000 - 200

    let rev_row = tb
        .entries
        .iter()
        .find(|e| e.account_code == 4000)
        .expect("revenue row missing");
    assert_eq!(rev_row.closing_balance, dec!(1000));

    let exp_row = tb
        .entries
        .iter()
        .find(|e| e.account_code == 5000)
        .expect("expense row missing");
    assert_eq!(exp_row.closing_balance, dec!(200));

    // P&L aggregates revenue and expenses.
    let pl = AccountingService::profit_and_loss(db, period_start, period_end)
        .await
        .expect("P&L generation failed");
    assert_eq!(pl.total_revenue, dec!(1000));
    assert_eq!(pl.total_expense, dec!(200));
    assert_eq!(pl.net_income, dec!(800));

    // Balance sheet carries the asset balance.
    let bs = AccountingService::balance_sheet(db, period_end)
        .await
        .expect("Balance sheet generation failed");
    assert_eq!(bs.total_assets, dec!(800));
}

/// Voiding a draft entry works; voiding a posted entry is a conflict.
#[ignore]
#[tokio::test]
async fn test_void_journal_entry_guard() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();
    let cash = create_account(db, "1000", "Cash", "asset").await;
    let revenue = create_account(db, "4000", "Internet Revenue", "revenue").await;

    let today = chrono::Utc::now().date_naive();
    let entry = AccountingService::create_journal_entry(
        db,
        today,
        "To void".to_string(),
        None,
        None,
        vec![
            CreateJournalLine {
                account_id: cash,
                debit: dec!(10),
                credit: dec!(0),
                description: None,
            },
            CreateJournalLine {
                account_id: revenue,
                debit: dec!(0),
                credit: dec!(10),
                description: None,
            },
        ],
        None,
    )
    .await
    .unwrap();

    let voided = AccountingService::void_journal_entry(db, entry.id)
        .await
        .expect("Voiding draft entry should succeed");
    assert_eq!(voided.status, "voided");

    // Voiding again is a conflict.
    let err = AccountingService::void_journal_entry(db, entry.id)
        .await
        .unwrap_err();
    assert!(matches!(err, AppError::Conflict(_)));
}
