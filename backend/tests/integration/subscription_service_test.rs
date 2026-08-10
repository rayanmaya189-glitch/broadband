//! Integration tests: SubscriptionService business flows
//! Exercises create/suspend/reactivate/cancel/upgrade/downgrade/renew against
//! PostgreSQL, verifying state transitions, outbox events, and invoice creation.

use crate::common::{TestDatabase, TestFixture};
use aeroxe_backend::modules::subscription::application::services::SubscriptionService;
use aeroxe_backend::shared::errors::AppError;
use rust_decimal_macros::dec;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};

/// Create a plan with active pricing; returns plan_id.
async fn create_priced_plan(db: &sea_orm::DatabaseConnection, price: rust_decimal::Decimal) -> i64 {
    let plan_id = TestFixture::create_plan(db).await;
    use aeroxe_backend::modules::plans::domain::entities::plan_pricing;
    let now = chrono::Utc::now();
    plan_pricing::ActiveModel {
        plan_id: Set(plan_id),
        billing_period_months: Set(1),
        price: Set(price),
        is_active: Set(true),
        created_at: Set(now),
        ..Default::default()
    }
    .insert(db)
    .await
    .expect("Failed to insert plan pricing");
    plan_id
}

/// Fetch outbox events for a subscription aggregate.
async fn outbox_events(
    db: &sea_orm::DatabaseConnection,
    aggregate_id: i64,
) -> Vec<aeroxe_backend::infrastructure::messaging::outbox::OutboxEvent> {
    use aeroxe_backend::infrastructure::messaging::outbox_entity;
    outbox_entity::Entity::find()
        .filter(outbox_entity::Column::AggregateType.eq("subscription"))
        .filter(outbox_entity::Column::AggregateId.eq(aggregate_id))
        .all(db)
        .await
        .expect("Failed to query outbox")
}

/// Creating a subscription sets status, next billing date, and emits an event.
#[ignore]
#[tokio::test]
async fn test_create_subscription_sets_lifecycle_and_outbox() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();
    let branch_id = TestFixture::create_branch(db).await;
    let customer_id = TestFixture::create_customer(db, branch_id).await;
    let plan_id = create_priced_plan(db, dec!(599.00)).await;

    let sub = SubscriptionService::create_subscription(db, customer_id, branch_id, plan_id, 1)
        .await
        .expect("Subscription creation failed");

    assert_eq!(sub.status, "active");
    assert_eq!(sub.billing_period_months, 1);
    let expected_next = sub.start_date + chrono::Duration::days(30);
    assert_eq!(sub.next_billing_date, Some(expected_next));

    let events = outbox_events(db, sub.id).await;
    assert!(
        events
            .iter()
            .any(|e| e.event_type == "subscription.created"),
        "expected subscription.created outbox event"
    );
}

/// Suspend → reactivate → cancel lifecycle emits the matching events.
#[ignore]
#[tokio::test]
async fn test_subscription_lifecycle_transitions() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();
    let branch_id = TestFixture::create_branch(db).await;
    let customer_id = TestFixture::create_customer(db, branch_id).await;
    let plan_id = create_priced_plan(db, dec!(599.00)).await;

    let sub = SubscriptionService::create_subscription(db, customer_id, branch_id, plan_id, 1)
        .await
        .unwrap();

    let suspended = SubscriptionService::suspend_subscription(db, sub.id, "non-payment")
        .await
        .unwrap();
    assert_eq!(suspended.status, "suspended");

    let reactivated = SubscriptionService::reactivate_subscription(db, sub.id)
        .await
        .expect("Reactivate failed");
    assert_eq!(reactivated.status, "active");
    assert!(reactivated.next_billing_date.is_some());

    let cancelled = SubscriptionService::cancel_subscription(db, sub.id, "customer request")
        .await
        .expect("Cancel failed");
    assert_eq!(cancelled.status, "cancelled");

    let events = outbox_events(db, sub.id).await;
    let types: Vec<&str> = events.iter().map(|e| e.event_type.as_str()).collect();
    for expected in [
        "subscription.created",
        "subscription.suspended",
        "subscription.reactivated",
        "subscription.cancelled",
    ] {
        assert!(types.contains(&expected), "missing outbox event {expected}");
    }
}

/// Reactivating an active subscription is rejected.
#[ignore]
#[tokio::test]
async fn test_reactivate_active_subscription_rejected() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();
    let branch_id = TestFixture::create_branch(db).await;
    let customer_id = TestFixture::create_customer(db, branch_id).await;
    let plan_id = create_priced_plan(db, dec!(599.00)).await;

    let sub = SubscriptionService::create_subscription(db, customer_id, branch_id, plan_id, 1)
        .await
        .unwrap();

    let err = SubscriptionService::reactivate_subscription(db, sub.id)
        .await
        .unwrap_err();
    assert!(
        matches!(err, AppError::Validation(_)),
        "active subscription must not be reactivated"
    );
}

/// Upgrading changes the plan, computes proration, and emits an event.
#[ignore]
#[tokio::test]
async fn test_upgrade_subscription_with_proration() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();
    let branch_id = TestFixture::create_branch(db).await;
    let customer_id = TestFixture::create_customer(db, branch_id).await;
    let old_plan = create_priced_plan(db, dec!(599.00)).await;
    let new_plan = create_priced_plan(db, dec!(999.00)).await;

    let sub = SubscriptionService::create_subscription(db, customer_id, branch_id, old_plan, 1)
        .await
        .unwrap();

    let (upgraded, proration) =
        SubscriptionService::upgrade_subscription(db, sub.id, new_plan, None)
            .await
            .expect("Upgrade failed");
    assert_eq!(upgraded.plan_id, new_plan);
    assert!(
        proration.is_some(),
        "proration should be computed when both plans are priced"
    );

    let events = outbox_events(db, sub.id).await;
    assert!(
        events
            .iter()
            .any(|e| e.event_type == "subscription.upgraded"),
        "expected subscription.upgraded outbox event"
    );

    // Upgrading to the same plan must be rejected.
    let err = SubscriptionService::upgrade_subscription(db, sub.id, new_plan, None)
        .await
        .unwrap_err();
    assert!(matches!(err, AppError::Validation(_)));
}

/// Downgrading soft-changes the plan and flags it for review.
#[ignore]
#[tokio::test]
async fn test_downgrade_subscription() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();
    let branch_id = TestFixture::create_branch(db).await;
    let customer_id = TestFixture::create_customer(db, branch_id).await;
    let old_plan = create_priced_plan(db, dec!(999.00)).await;
    let new_plan = create_priced_plan(db, dec!(599.00)).await;

    let sub = SubscriptionService::create_subscription(db, customer_id, branch_id, old_plan, 1)
        .await
        .unwrap();

    let downgraded = SubscriptionService::downgrade_subscription(db, sub.id, new_plan, None)
        .await
        .expect("Downgrade failed");
    assert_eq!(downgraded.plan_id, new_plan);
    assert_eq!(
        downgraded.review_status,
        Some("pending_downgrade".to_string())
    );

    let events = outbox_events(db, sub.id).await;
    assert!(
        events
            .iter()
            .any(|e| e.event_type == "subscription.downgraded"),
        "expected subscription.downgraded outbox event"
    );
}

/// Renewing advances the billing window and mints the next invoice.
#[ignore]
#[tokio::test]
async fn test_renew_subscription_creates_invoice() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();
    let branch_id = TestFixture::create_branch(db).await;
    let customer_id = TestFixture::create_customer(db, branch_id).await;
    let plan_id = create_priced_plan(db, dec!(599.00)).await;

    let sub = SubscriptionService::create_subscription(db, customer_id, branch_id, plan_id, 1)
        .await
        .unwrap();

    // Force the subscription due so renewal produces an invoice.
    use aeroxe_backend::modules::subscription::domain::entities::{subscription, Subscription};
    let model = Subscription::find_by_id(sub.id)
        .one(db)
        .await
        .unwrap()
        .unwrap();
    let mut active: subscription::ActiveModel = model.into();
    active.next_billing_date = Set(Some(chrono::Utc::now().date_naive()));
    active.updated_at = Set(chrono::Utc::now());
    active.update(db).await.unwrap();

    let renewed = SubscriptionService::renew_subscription(db, sub.id)
        .await
        .expect("Renewal failed");
    assert_eq!(renewed.status, "active");
    let today = chrono::Utc::now().date_naive();
    assert_eq!(
        renewed.next_billing_date,
        Some(today + chrono::Duration::days(30))
    );

    use aeroxe_backend::modules::billing::domain::entities::Invoice;
    use sea_orm::PaginatorTrait;
    let invoice_count = Invoice::find()
        .filter(
            aeroxe_backend::modules::billing::domain::entities::invoice::Column::SubscriptionId
                .eq(sub.id),
        )
        .count(db)
        .await
        .unwrap();
    assert_eq!(
        invoice_count, 1,
        "renewal should generate exactly one invoice"
    );

    let events = outbox_events(db, sub.id).await;
    assert!(
        events
            .iter()
            .any(|e| e.event_type == "subscription.renewed"),
        "expected subscription.renewed outbox event"
    );
}

/// Cancelled subscriptions cannot be renewed.
#[ignore]
#[tokio::test]
async fn test_renew_cancelled_subscription_rejected() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();
    let branch_id = TestFixture::create_branch(db).await;
    let customer_id = TestFixture::create_customer(db, branch_id).await;
    let plan_id = create_priced_plan(db, dec!(599.00)).await;

    let sub = SubscriptionService::create_subscription(db, customer_id, branch_id, plan_id, 1)
        .await
        .unwrap();
    SubscriptionService::cancel_subscription(db, sub.id, "churn")
        .await
        .unwrap();

    let err = SubscriptionService::renew_subscription(db, sub.id)
        .await
        .unwrap_err();
    assert!(matches!(err, AppError::Validation(_)));
}
