//! Integration test: Accounting Repository
//! Tests double-entry accounting operations with real PostgreSQL

mod common;

use sea_orm::{ActiveModelTrait, EntityTrait, Set};
use crate::common::TestDatabase;

/// Test chart of accounts CRUD
#[tokio::test]
async fn test_chart_of_accounts_crud() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    use crate::modules::accounting::domain::entities::chart_of_accounts;

    let now = chrono::Utc::now();

    // Create account
    let account = chart_of_accounts::ActiveModel {
        code: Set("1000".to_string()),
        name: Set("Cash".to_string()),
        account_type: Set("asset".to_string()),
        parent_id: Set(None),
        is_group: Set(false),
        is_active: Set(true),
        description: Set(Some("Cash on hand".to_string())),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    };
    let account = account.insert(db).await.unwrap();
    assert!(account.id > 0);
    assert_eq!(account.code, "1000");

    // Read account
    let found = chart_of_accounts::Entity::find_by_id(account.id)
        .one(db).await.unwrap();
    assert!(found.is_some());
    assert_eq!(found.unwrap().name, "Cash");

    // Update account
    let mut active: chart_of_accounts::ActiveModel = account.into();
    active.name = Set("Cash & Equivalents".to_string());
    active.updated_at = Set(chrono::Utc::now());
    let updated = active.update(db).await.unwrap();
    assert_eq!(updated.name, "Cash & Equivalents");
}

/// Test journal entry with balanced debits/credits
#[tokio::test]
async fn test_journal_entry_balance() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    use crate::modules::accounting::domain::entities::{
        chart_of_accounts, journal_entry, journal_entry_line,
    };

    let now = chrono::Utc::now();

    // Create two accounts
    let cash = chart_of_accounts::ActiveModel {
        code: Set("1000".to_string()),
        name: Set("Cash".to_string()),
        account_type: Set("asset".to_string()),
        is_active: Set(true),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    };
    let cash = cash.insert(db).await.unwrap();

    let revenue = chart_of_accounts::ActiveModel {
        code: Set("4000".to_string()),
        name: Set("Revenue".to_string()),
        account_type: Set("revenue".to_string()),
        is_active: Set(true),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    };
    let revenue = revenue.insert(db).await.unwrap();

    // Create journal entry
    let entry = journal_entry::ActiveModel {
        entry_number: Set("JE-20260719-00001".to_string()),
        entry_date: Set(now.date_naive()),
        description: Set("Test revenue entry".to_string()),
        reference_type: Set(None),
        reference_id: Set(None),
        total_debit: Set(rust_decimal::Decimal::from(1000)),
        total_credit: Set(rust_decimal::Decimal::from(1000)),
        status: Set("draft".to_string()),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    };
    let entry = entry.insert(db).await.unwrap();
    assert!(entry.id > 0);
    assert_eq!(entry.total_debit, entry.total_credit);

    // Add debit line (cash)
    let debit_line = journal_entry_line::ActiveModel {
        journal_entry_id: Set(entry.id),
        account_id: Set(cash.id),
        debit: Set(rust_decimal::Decimal::from(1000)),
        credit: Set(rust_decimal::Decimal::ZERO),
        description: Set(Some("Received payment".to_string())),
        created_at: Set(now),
        ..Default::default()
    };
    let dl = debit_line.insert(db).await.unwrap();
    assert!(dl.id > 0);

    // Add credit line (revenue)
    let credit_line = journal_entry_line::ActiveModel {
        journal_entry_id: Set(entry.id),
        account_id: Set(revenue.id),
        debit: Set(rust_decimal::Decimal::ZERO),
        credit: Set(rust_decimal::Decimal::from(1000)),
        description: Set(Some("Service revenue".to_string())),
        created_at: Set(now),
        ..Default::default()
    };
    let cl = credit_line.insert(db).await.unwrap();
    assert!(cl.id > 0);

    // Verify lines balance
    let lines = journal_entry_line::Entity::find()
        .filter(journal_entry_line::Column::JournalEntryId.eq(entry.id))
        .all(db).await.unwrap();
    assert_eq!(lines.len(), 2);

    let total_debit: rust_decimal::Decimal = lines.iter().map(|l| l.debit).sum();
    let total_credit: rust_decimal::Decimal = lines.iter().map(|l| l.credit).sum();
    assert_eq!(total_debit, total_credit);
}
