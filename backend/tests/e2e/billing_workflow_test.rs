//! End-to-end test: Billing & Payment Workflow
//! Tests: Invoice creation → Payment → Refund → Dunning

mod common;

use sea_orm::{ActiveModelTrait, EntityTrait, Set};
use crate::common::{TestDatabase, TestFixture};

/// Test invoice lifecycle: create → send → pay → refund
#[tokio::test]
async fn test_invoice_lifecycle() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    let branch_id = TestFixture::create_branch(db).await;
    let customer_id = TestFixture::create_customer(db, branch_id).await;

    use crate::modules::billing::domain::entities::{invoice, payment, refund};

    let now = chrono::Utc::now();

    // Create draft invoice
    let inv = invoice::ActiveModel {
        customer_id: Set(customer_id),
        branch_id: Set(branch_id),
        subscription_id: Set(0), // No subscription
        invoice_number: Set(format!("INV-2026-07-{:04}", rand::random::<u16>() % 10000)),
        billing_period_start: Set(now.date_naive()),
        billing_period_end: Set((now + chrono::Duration::days(30)).date_naive()),
        subtotal: Set(rust_decimal::Decimal::from(1000)),
        discount_amount: Set(rust_decimal::Decimal::ZERO),
        tax_amount: Set(rust_decimal::Decimal::from(180)),
        total_amount: Set(rust_decimal::Decimal::from(1180)),
        currency: Set("INR".to_string()),
        status: Set("draft".to_string()),
        due_date: Set((now + chrono::Duration::days(15)).date_naive()),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    };
    let inv = inv.insert(db).await.unwrap();
    assert_eq!(inv.status, "draft");

    // Send invoice
    let mut active: invoice::ActiveModel = inv.into();
    active.status = Set("sent".to_string());
    active.updated_at = Set(chrono::Utc::now());
    let inv = active.update(db).await.unwrap();
    assert_eq!(inv.status, "sent");

    // Record payment
    let pay = payment::ActiveModel {
        payment_number: Set(format!("PAY-202607-{:04}", rand::random::<u16>() % 10000)),
        invoice_id: Set(inv.id),
        customer_id: Set(customer_id),
        branch_id: Set(branch_id),
        amount: Set(rust_decimal::Decimal::from(1180)),
        currency: Set("INR".to_string()),
        payment_method: Set("upi".to_string()),
        status: Set("completed".to_string()),
        created_at: Set(chrono::Utc::now()),
        ..Default::default()
    };
    let pay = pay.insert(db).await.unwrap();
    assert!(pay.id > 0);

    // Mark invoice as paid
    let mut active: invoice::ActiveModel = inv.into();
    active.status = Set("paid".to_string());
    active.updated_at = Set(chrono::Utc::now());
    let inv = active.update(db).await.unwrap();
    assert_eq!(inv.status, "paid");

    // Create refund
    let refnd = refund::ActiveModel {
        refund_number: Set(format!("REF-202607-{:04}", rand::random::<u16>() % 10000)),
        payment_id: Set(pay.id),
        invoice_id: Set(inv.id),
        customer_id: Set(customer_id),
        amount: Set(rust_decimal::Decimal::from(100)),
        reason: Set("Service disruption".to_string()),
        status: Set("approved".to_string()),
        created_at: Set(chrono::Utc::now()),
        ..Default::default()
    };
    let refnd = refnd.insert(db).await.unwrap();
    assert!(refnd.id > 0);
}

/// Test overdue invoice dunning flow
#[tokio::test]
async fn test_overdue_dunning_flow() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    let branch_id = TestFixture::create_branch(db).await;
    let customer_id = TestFixture::create_customer(db, branch_id).await;

    use crate::modules::billing::domain::entities::invoice;

    let now = chrono::Utc::now();

    // Create overdue invoice (due date in the past)
    let inv = invoice::ActiveModel {
        customer_id: Set(customer_id),
        branch_id: Set(branch_id),
        subscription_id: Set(0),
        invoice_number: Set(format!("INV-2026-06-{:04}", rand::random::<u16>() % 10000)),
        billing_period_start: Set((now - chrono::Duration::days(30)).date_naive()),
        billing_period_end: Set(now.date_naive()),
        subtotal: Set(rust_decimal::Decimal::from(500)),
        discount_amount: Set(rust_decimal::Decimal::ZERO),
        tax_amount: Set(rust_decimal::Decimal::from(90)),
        total_amount: Set(rust_decimal::Decimal::from(590)),
        currency: Set("INR".to_string()),
        status: Set("overdue".to_string()),
        due_date: Set((now - chrono::Duration::days(10)).date_naive()),
        created_at: Set(now - chrono::Duration::days(30)),
        updated_at: Set(now),
        ..Default::default()
    };
    let inv = inv.insert(db).await.unwrap();
    assert_eq!(inv.status, "overdue");

    // Verify invoice can transition to voided
    let mut active: invoice::ActiveModel = inv.into();
    active.status = Set("voided".to_string());
    active.updated_at = Set(chrono::Utc::now());
    let inv = active.update(db).await.unwrap();
    assert_eq!(inv.status, "voided");
}
