//! Integration tests for DB-backed payment/webhook idempotency:
//! concurrent gateway webhook replays must not double-record a payment or a
//! webhook log, and the auto-billing job must not double-generate an invoice.

use crate::common::{TestDatabase, TestFixture};
use aeroxe_backend::modules::billing::domain::entities::invoice::Column as InvoiceColumn;
use aeroxe_backend::modules::billing::domain::entities::invoice::Entity as InvoiceEntity;
use aeroxe_backend::modules::payment::application::services::PaymentService;
use sea_orm::prelude::Decimal;
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, PaginatorTrait, QueryFilter, Statement};

async fn create_payment_link(
    db: &sea_orm::DatabaseConnection,
    branch_id: i64,
    customer_id: i64,
    order_id: &str,
) {
    let now = chrono::Utc::now();
    db.execute(Statement::from_string(
        db.get_database_backend(),
        format!(
            "INSERT INTO payment.payment_links
                (link_id, invoice_id, customer_id, branch_id, amount, currency, gateway_id,
                 gateway_order_id, status, idempotency_key, created_at, updated_at)
             VALUES ('link-{order_id}', NULL, {customer_id}, {branch_id}, 100, 'INR', 'razorpay',
                     '{order_id}', 'pending', 'idem-{order_id}', '{}', '{}')",
            now.format("%Y-%m-%d %H:%M:%S"),
            now.format("%Y-%m-%d %H:%M:%S")
        ),
    ))
    .await
    .expect("Failed to create test payment link");
}

async fn count_payments(db: &sea_orm::DatabaseConnection, tx_id: &str) -> u64 {
    use aeroxe_backend::modules::billing::domain::entities::payment::Column as PaymentColumn;
    use aeroxe_backend::modules::billing::domain::entities::payment::Entity as PaymentEntity;
    PaymentEntity::find()
        .filter(PaymentColumn::GatewayTransactionId.eq(tx_id))
        .count(db)
        .await
        .expect("count payments")
}

/// Replaying the same successful webhook must not double-record the payment.
#[ignore]
#[tokio::test]
async fn test_payment_replay_is_idempotent() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    let branch_id = TestFixture::create_branch(db).await;
    let customer_id = TestFixture::create_customer(db, branch_id).await;

    let order_id = format!("order-{}", rand::random::<u64>());
    let tx_id = format!("pay_tx_{}", rand::random::<u64>());
    create_payment_link(db, branch_id, customer_id, &order_id).await;

    let amount = Decimal::from(100);
    PaymentService::process_successful_payment(
        db,
        "razorpay",
        &tx_id,
        Some(&order_id),
        amount,
        Some("upi".to_string()),
    )
    .await
    .expect("first processing should succeed");

    assert_eq!(count_payments(db, &tx_id).await, 1);

    // Replay the exact same webhook payload (simulating a retry/race).
    PaymentService::process_successful_payment(
        db,
        "razorpay",
        &tx_id,
        Some(&order_id),
        amount,
        Some("upi".to_string()),
    )
    .await
    .expect("replay should be treated as idempotent, not an error");

    assert_eq!(
        count_payments(db, &tx_id).await,
        1,
        "a replayed webhook must not create a second payment record"
    );
}

/// Replaying the same gateway event must not double-log the webhook.
#[ignore]
#[tokio::test]
async fn test_webhook_log_replay_is_idempotent() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    let gateway_id = "razorpay";
    let event_id = format!("evt_{}", rand::random::<u64>());

    let first = PaymentService::log_webhook(
        db,
        gateway_id,
        &event_id,
        "payment.captured",
        serde_json::json!({ "x": 1 }),
    )
    .await
    .expect("log webhook");
    assert!(!first, "first log should report a new webhook");

    let second = PaymentService::log_webhook(
        db,
        gateway_id,
        &event_id,
        "payment.captured",
        serde_json::json!({ "x": 1 }),
    )
    .await
    .expect("log webhook replay");
    assert!(second, "replay should report an already-processed webhook");

    use aeroxe_backend::modules::payment::domain::entities::webhook_log::Column as WebhookLogColumn;
    use aeroxe_backend::modules::payment::domain::entities::webhook_log::Entity as WebhookLogEntity;
    let count = WebhookLogEntity::find()
        .filter(WebhookLogColumn::GatewayId.eq(gateway_id))
        .filter(WebhookLogColumn::EventId.eq(&event_id))
        .count(db)
        .await
        .expect("count webhook logs");
    assert_eq!(count, 1, "a replayed webhook must not be logged twice");
}

/// Auto-billing must not generate a second invoice for the same period when
/// run twice (the DB unique constraint backs up the service-level check).
#[ignore]
#[tokio::test]
async fn test_auto_invoice_generation_is_idempotent() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    let branch_id = TestFixture::create_branch(db).await;
    let customer_id = TestFixture::create_customer(db, branch_id).await;
    let subscription_id = TestFixture::create_subscription(db, customer_id, branch_id).await;

    // The fixture subscription is created without next_billing_date; set it to
    // today so the auto-billing job picks it up as due.
    let today = chrono::Utc::now().date_naive();
    db.execute(Statement::from_string(
        db.get_database_backend(),
        format!(
            "UPDATE subscription.subscriptions SET next_billing_date = '{}' WHERE id = {}",
            today.format("%Y-%m-%d"),
            subscription_id
        ),
    ))
    .await
    .expect("set next_billing_date");

    let count = aeroxe_backend::modules::billing::application::services::BillingService::auto_generate_invoices(
        db,
    )
    .await
    .expect("auto-generate invoices");

    // The fixture subscription is active with next_billing_date <= today.
    assert!(count >= 1, "expected at least one invoice");

    // Run again — the unique (subscription_id, billing_period_start) constraint
    // must prevent a duplicate even though the service re-checks first.
    aeroxe_backend::modules::billing::application::services::BillingService::auto_generate_invoices(
        db,
    )
    .await
    .expect("second run should be idempotent");

    let invoices = InvoiceEntity::find()
        .filter(InvoiceColumn::SubscriptionId.eq(subscription_id))
        .all(db)
        .await
        .expect("load invoices");
    assert_eq!(
        invoices.len(),
        1,
        "the same subscription/period must not be invoiced twice"
    );
}
