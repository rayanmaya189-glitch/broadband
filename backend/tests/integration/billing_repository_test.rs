//! Integration tests for billing repository using testcontainers

mod common;

use sea_orm::{DatabaseConnection, EntityTrait, Set, ActiveModelTrait};
use crate::common::{TestDatabase, TestFixture};

/// Test invoice creation
#[tokio::test]
async fn test_create_invoice() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();
    
    // Create prerequisite data
    let branch_id = TestFixture::create_branch(db).await;
    let customer_id = TestFixture::create_customer(db, branch_id).await;
    let plan_id = TestFixture::create_plan(db).await;
    
    // Create subscription first
    use crate::modules::subscription::domain::entities::subscription;
    
    let now = chrono::Utc::now();
    let sub_model = subscription::ActiveModel {
        customer_id: Set(customer_id),
        branch_id: Set(branch_id),
        plan_id: Set(plan_id),
        status: Set("active".to_string()),
        billing_period_months: Set(1),
        start_date: Set(chrono::NaiveDate::from_ymd_opt(2026, 7, 1).unwrap()),
        auto_renew: Set(true),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    };
    
    let subscription = sub_model.insert(db).await.expect("Failed to create subscription");
    
    // Create invoice
    use crate::modules::billing::domain::entities::invoice;
    
    let invoice_model = invoice::ActiveModel {
        invoice_number: Set("INV-2026-07-0001".to_string()),
        customer_id: Set(customer_id),
        branch_id: Set(branch_id),
        subscription_id: Set(subscription.id),
        billing_period_start: Set(chrono::NaiveDate::from_ymd_opt(2026, 7, 1).unwrap()),
        billing_period_end: Set(chrono::NaiveDate::from_ymd_opt(2026, 7, 31).unwrap()),
        subtotal: Set(rust_decimal::Decimal::new(600, 2)),
        total_amount: Set(rust_decimal::Decimal::new(708, 2)),
        currency: Set("INR".to_string()),
        status: Set("pending".to_string()),
        due_date: Set(chrono::NaiveDate::from_ymd_opt(2026, 7, 10).unwrap()),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    };
    
    let invoice = invoice_model.insert(db).await.expect("Failed to create invoice");
    assert!(invoice.id > 0, "Invoice should be created");
    assert_eq!(invoice.invoice_number, "INV-2026-07-0001");
}

/// Test invoice payment flow
#[tokio::test]
async fn test_invoice_payment_flow() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();
    
    let branch_id = TestFixture::create_branch(db).await;
    let customer_id = TestFixture::create_customer(db, branch_id).await;
    let plan_id = TestFixture::create_plan(db).await;
    
    // Create subscription
    use crate::modules::subscription::domain::entities::subscription;
    
    let now = chrono::Utc::now();
    let sub_model = subscription::ActiveModel {
        customer_id: Set(customer_id),
        branch_id: Set(branch_id),
        plan_id: Set(plan_id),
        status: Set("active".to_string()),
        billing_period_months: Set(1),
        start_date: Set(chrono::NaiveDate::from_ymd_opt(2026, 7, 1).unwrap()),
        auto_renew: Set(true),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    };
    
    let subscription = sub_model.insert(db).await.expect("Failed to create subscription");
    
    // Create invoice
    use crate::modules::billing::domain::entities::invoice;
    
    let invoice_model = invoice::ActiveModel {
        invoice_number: Set("INV-2026-07-0002".to_string()),
        customer_id: Set(customer_id),
        branch_id: Set(branch_id),
        subscription_id: Set(subscription.id),
        billing_period_start: Set(chrono::NaiveDate::from_ymd_opt(2026, 7, 1).unwrap()),
        billing_period_end: Set(chrono::NaiveDate::from_ymd_opt(2026, 7, 31).unwrap()),
        subtotal: Set(rust_decimal::Decimal::new(600, 2)),
        total_amount: Set(rust_decimal::Decimal::new(708, 2)),
        currency: Set("INR".to_string()),
        status: Set("pending".to_string()),
        due_date: Set(chrono::NaiveDate::from_ymd_opt(2026, 7, 10).unwrap()),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    };
    
    let invoice = invoice_model.insert(db).await.expect("Failed to create invoice");
    
    // Mark invoice as paid
    let mut active_model: invoice::ActiveModel = invoice.into();
    active_model.status = Set("paid".to_string());
    active_model.paid_at = Set(Some(chrono::Utc::now()));
    active_model.updated_at = Set(chrono::Utc::now());
    
    let paid_invoice = active_model.update(db).await.expect("Failed to mark invoice as paid");
    assert_eq!(paid_invoice.status, "paid");
    assert!(paid_invoice.paid_at.is_some(), "Paid_at should be set");
}

/// Test invoice voiding
#[tokio::test]
async fn test_invoice_voiding() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();
    
    let branch_id = TestFixture::create_branch(db).await;
    let customer_id = TestFixture::create_customer(db, branch_id).await;
    let plan_id = TestFixture::create_plan(db).await;
    
    // Create subscription
    use crate::modules::subscription::domain::entities::subscription;
    
    let now = chrono::Utc::now();
    let sub_model = subscription::ActiveModel {
        customer_id: Set(customer_id),
        branch_id: Set(branch_id),
        plan_id: Set(plan_id),
        status: Set("active".to_string()),
        billing_period_months: Set(1),
        start_date: Set(chrono::NaiveDate::from_ymd_opt(2026, 7, 1).unwrap()),
        auto_renew: Set(true),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    };
    
    let subscription = sub_model.insert(db).await.expect("Failed to create subscription");
    
    // Create invoice
    use crate::modules::billing::domain::entities::invoice;
    
    let invoice_model = invoice::ActiveModel {
        invoice_number: Set("INV-2026-07-0003".to_string()),
        customer_id: Set(customer_id),
        branch_id: Set(branch_id),
        subscription_id: Set(subscription.id),
        billing_period_start: Set(chrono::NaiveDate::from_ymd_opt(2026, 7, 1).unwrap()),
        billing_period_end: Set(chrono::NaiveDate::from_ymd_opt(2026, 7, 31).unwrap()),
        subtotal: Set(rust_decimal::Decimal::new(600, 2)),
        total_amount: Set(rust_decimal::Decimal::new(708, 2)),
        currency: Set("INR".to_string()),
        status: Set("pending".to_string()),
        due_date: Set(chrono::NaiveDate::from_ymd_opt(2026, 7, 10).unwrap()),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    };
    
    let invoice = invoice_model.insert(db).await.expect("Failed to create invoice");
    
    // Void invoice
    let mut active_model: invoice::ActiveModel = invoice.into();
    active_model.status = Set("void".to_string());
    active_model.updated_at = Set(chrono::Utc::now());
    
    let voided_invoice = active_model.update(db).await.expect("Failed to void invoice");
    assert_eq!(voided_invoice.status, "void");
}
