//! End-to-end test: Customer Lifecycle
//! Tests the full flow: Create customer → KYC → Subscription → Invoice → Payment

mod common;

use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use crate::common::{TestDatabase, TestFixture};

/// Full customer lifecycle: registration → KYC → subscription → invoice → payment
#[tokio::test]
async fn test_customer_full_lifecycle() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    // Step 1: Create branch
    let branch_id = TestFixture::create_branch(db).await;
    assert!(branch_id > 0, "Branch created");

    // Step 2: Create customer (registered)
    let customer_id = TestFixture::create_customer(db, branch_id).await;
    assert!(customer_id > 0, "Customer created");

    // Step 3: Update customer to KYC pending
    use crate::modules::customer::domain::entities::customer;
    let cust = customer::Entity::find_by_id(customer_id)
        .one(db)
        .await
        .unwrap()
        .unwrap();
    let mut active: customer::ActiveModel = cust.into();
    active.status = Set("kyc_pending".to_string());
    active.updated_at = Set(chrono::Utc::now());
    let cust = active.update(db).await.unwrap();
    assert_eq!(cust.status, "kyc_pending");

    // Step 4: KYC verified
    let mut active: customer::ActiveModel = cust.into();
    active.status = Set("kyc_verified".to_string());
    active.updated_at = Set(chrono::Utc::now());
    let cust = active.update(db).await.unwrap();
    assert_eq!(cust.status, "kyc_verified");

    // Step 5: Create plan
    let plan_id = TestFixture::create_plan(db).await;
    assert!(plan_id > 0, "Plan created");

    // Step 6: Create subscription
    use crate::modules::subscription::domain::entities::subscription;
    let now = chrono::Utc::now();
    let sub_active = subscription::ActiveModel {
        customer_id: Set(customer_id),
        branch_id: Set(branch_id),
        plan_id: Set(plan_id),
        status: Set("active".to_string()),
        billing_period_months: Set(1),
        start_date: Set(now.date_naive()),
        auto_renew: Set(true),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    };
    let sub = sub_active.insert(db).await.unwrap();
    assert!(sub.id > 0, "Subscription created");
    assert_eq!(sub.status, "active");

    // Step 7: Create invoice
    use crate::modules::billing::domain::entities::invoice;
    let inv_active = invoice::ActiveModel {
        customer_id: Set(customer_id),
        branch_id: Set(branch_id),
        subscription_id: Set(sub.id),
        invoice_number: Set(format!("INV-2026-07-{:04}", rand::random::<u16>() % 10000)),
        billing_period_start: Set(now.date_naive()),
        billing_period_end: Set((now + chrono::Duration::days(30)).date_naive()),
        subtotal: Set(rust_decimal::Decimal::from(600)),
        discount_amount: Set(rust_decimal::Decimal::ZERO),
        tax_amount: Set(rust_decimal::Decimal::from(108)),
        total_amount: Set(rust_decimal::Decimal::from(708)),
        currency: Set("INR".to_string()),
        status: Set("sent".to_string()),
        due_date: Set((now + chrono::Duration::days(15)).date_naive()),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    };
    let inv = inv_active.insert(db).await.unwrap();
    assert!(inv.id > 0, "Invoice created");
    assert_eq!(inv.status, "sent");

    // Step 8: Record payment
    use crate::modules::billing::domain::entities::payment;
    let pay_active = payment::ActiveModel {
        payment_number: Set(format!("PAY-202607-{:04}", rand::random::<u16>() % 10000)),
        invoice_id: Set(inv.id),
        customer_id: Set(customer_id),
        branch_id: Set(branch_id),
        amount: Set(rust_decimal::Decimal::from(708)),
        currency: Set("INR".to_string()),
        payment_method: Set("upi".to_string()),
        status: Set("completed".to_string()),
        created_at: Set(now),
        ..Default::default()
    };
    let pay = pay_active.insert(db).await.unwrap();
    assert!(pay.id > 0, "Payment recorded");

    // Step 9: Update invoice to paid
    let inv = invoice::Entity::find_by_id(inv.id)
        .one(db)
        .await
        .unwrap()
        .unwrap();
    let mut active: invoice::ActiveModel = inv.into();
    active.status = Set("paid".to_string());
    active.updated_at = Set(chrono::Utc::now());
    let inv = active.update(db).await.unwrap();
    assert_eq!(inv.status, "paid");

    // Verify final state
    let cust = customer::Entity::find_by_id(customer_id)
        .one(db)
        .await
        .unwrap()
        .unwrap();
    let sub = subscription::Entity::find_by_id(sub.id)
        .one(db)
        .await
        .unwrap()
        .unwrap();

    assert_eq!(cust.status, "kyc_verified");
    assert_eq!(sub.status, "active");
    assert_eq!(inv.status, "paid");
}

/// Test customer suspension and reactivation
#[tokio::test]
async fn test_customer_suspension_reactivation() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    let branch_id = TestFixture::create_branch(db).await;
    let customer_id = TestFixture::create_customer(db, branch_id).await;

    use crate::modules::customer::domain::entities::customer;

    // Activate customer
    let cust = customer::Entity::find_by_id(customer_id)
        .one(db)
        .await
        .unwrap()
        .unwrap();
    let mut active: customer::ActiveModel = cust.into();
    active.status = Set("active".to_string());
    active.updated_at = Set(chrono::Utc::now());
    let cust = active.update(db).await.unwrap();
    assert_eq!(cust.status, "active");

    // Suspend customer
    let mut active: customer::ActiveModel = cust.into();
    active.status = Set("suspended".to_string());
    active.updated_at = Set(chrono::Utc::now());
    let cust = active.update(db).await.unwrap();
    assert_eq!(cust.status, "suspended");

    // Reactivate customer
    let mut active: customer::ActiveModel = cust.into();
    active.status = Set("active".to_string());
    active.updated_at = Set(chrono::Utc::now());
    let cust = active.update(db).await.unwrap();
    assert_eq!(cust.status, "active");
}
