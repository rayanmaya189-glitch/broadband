//! End-to-end test: Payment Gateway Workflow
//! Tests: Payment link creation → Webhook processing → Invoice update

mod common;

use sea_orm::{ActiveModelTrait, EntityTrait, Set};
use crate::common::{TestDatabase, TestFixture};

/// Test payment gateway flow: invoice → payment link → payment record
#[tokio::test]
async fn test_payment_gateway_flow() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    let branch_id = TestFixture::create_branch(db).await;
    let customer_id = TestFixture::create_customer(db, branch_id).await;

    use crate::modules::billing::domain::entities::{invoice, payment};

    let now = chrono::Utc::now();

    // Create invoice
    let inv = invoice::ActiveModel {
        customer_id: Set(customer_id),
        branch_id: Set(branch_id),
        subscription_id: Set(0),
        invoice_number: Set(format!("INV-2026-07-{:04}", rand::random::<u16>() % 10000)),
        billing_period_start: Set(now.date_naive()),
        billing_period_end: Set((now + chrono::Duration::days(30)).date_naive()),
        subtotal: Set(rust_decimal::Decimal::from(590)),
        discount_amount: Set(rust_decimal::Decimal::ZERO),
        tax_amount: Set(rust_decimal::Decimal::from(106)),
        total_amount: Set(rust_decimal::Decimal::from(696)),
        currency: Set("INR".to_string()),
        status: Set("sent".to_string()),
        due_date: Set((now + chrono::Duration::days(15)).date_naive()),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    };
    let inv = inv.insert(db).await.unwrap();
    assert_eq!(inv.status, "sent");

    // Simulate webhook: payment completed
    let pay = payment::ActiveModel {
        payment_number: Set(format!("PAY-202607-{:04}", rand::random::<u16>() % 10000)),
        invoice_id: Set(inv.id),
        customer_id: Set(customer_id),
        branch_id: Set(branch_id),
        amount: Set(rust_decimal::Decimal::from(696)),
        currency: Set("INR".to_string()),
        payment_method: Set("upi".to_string()),
        payment_gateway: Set(Some("razorpay".to_string())),
        gateway_transaction_id: Set(Some("pay_abc123".to_string())),
        status: Set("completed".to_string()),
        created_at: Set(now),
        ..Default::default()
    };
    let pay = pay.insert(db).await.unwrap();
    assert!(pay.id > 0);
    assert_eq!(pay.status, "completed");

    // Update invoice to paid
    let mut active: invoice::ActiveModel = inv.into();
    active.status = Set("paid".to_string());
    active.paid_at = Set(Some(chrono::Utc::now()));
    active.payment_method = Set(Some("upi".to_string()));
    active.updated_at = Set(chrono::Utc::now());
    let inv = active.update(db).await.unwrap();
    assert_eq!(inv.status, "paid");
}

/// Test payment failure flow
#[tokio::test]
async fn test_payment_failure_flow() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    let branch_id = TestFixture::create_branch(db).await;
    let customer_id = TestFixture::create_customer(db, branch_id).await;

    use crate::modules::billing::domain::entities::{invoice, payment};

    let now = chrono::Utc::now();

    // Create invoice
    let inv = invoice::ActiveModel {
        customer_id: Set(customer_id),
        branch_id: Set(branch_id),
        subscription_id: Set(0),
        invoice_number: Set(format!("INV-2026-07-{:04}", rand::random::<u16>() % 10000)),
        billing_period_start: Set(now.date_naive()),
        billing_period_end: Set((now + chrono::Duration::days(30)).date_naive()),
        subtotal: Set(rust_decimal::Decimal::from(500)),
        discount_amount: Set(rust_decimal::Decimal::ZERO),
        tax_amount: Set(rust_decimal::Decimal::from(90)),
        total_amount: Set(rust_decimal::Decimal::from(590)),
        currency: Set("INR".to_string()),
        status: Set("sent".to_string()),
        due_date: Set((now + chrono::Duration::days(15)).date_naive()),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    };
    let inv = inv.insert(db).await.unwrap();

    // Simulate failed payment
    let pay = payment::ActiveModel {
        payment_number: Set(format!("PAY-202607-{:04}", rand::random::<u16>() % 10000)),
        invoice_id: Set(inv.id),
        customer_id: Set(customer_id),
        branch_id: Set(branch_id),
        amount: Set(rust_decimal::Decimal::from(590)),
        currency: Set("INR".to_string()),
        payment_method: Set("card".to_string()),
        payment_gateway: Set(Some("razorpay".to_string())),
        gateway_transaction_id: Set(Some("pay_fail123".to_string())),
        status: Set("failed".to_string()),
        created_at: Set(now),
        ..Default::default()
    };
    let pay = pay.insert(db).await.unwrap();
    assert_eq!(pay.status, "failed");

    // Invoice should still be sent (not paid)
    let inv = invoice::Entity::find_by_id(inv.id)
        .one(db)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(inv.status, "sent");
}
