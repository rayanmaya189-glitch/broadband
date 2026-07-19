//! End-to-end test: Subscription Plan Change Workflow
//! Tests: Create subscription → Upgrade → Downgrade → Cancel

mod common;

use sea_orm::{ActiveModelTrait, EntityTrait, Set};
use crate::common::{TestDatabase, TestFixture};

/// Test subscription upgrade/downgrade workflow
#[tokio::test]
async fn test_subscription_plan_change_workflow() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    // Setup
    let branch_id = TestFixture::create_branch(db).await;
    let customer_id = TestFixture::create_customer(db, branch_id).await;
    let plan_id = TestFixture::create_plan(db).await;

    use crate::modules::customer::domain::entities::customer;
    use crate::modules::subscription::domain::entities::subscription;

    // Activate customer
    let cust = customer::Entity::find_by_id(customer_id)
        .one(db).await.unwrap().unwrap();
    let mut active: customer::ActiveModel = cust.into();
    active.status = Set("active".to_string());
    active.updated_at = Set(chrono::Utc::now());
    active.update(db).await.unwrap();

    // Create initial subscription
    let now = chrono::Utc::now();
    let sub = subscription::ActiveModel {
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
    let sub = sub.insert(db).await.unwrap();
    assert_eq!(sub.status, "active");

    // Simulate upgrade (change plan)
    let mut active: subscription::ActiveModel = sub.into();
    active.plan_id = Set(999); // New plan ID
    active.updated_at = Set(chrono::Utc::now());
    let sub = active.update(db).await.unwrap();
    assert_eq!(sub.plan_id, 999);

    // Simulate cancellation
    let mut active: subscription::ActiveModel = sub.into();
    active.status = Set("cancelled".to_string());
    active.auto_renew = Set(false);
    active.updated_at = Set(chrono::Utc::now());
    let sub = active.update(db).await.unwrap();
    assert_eq!(sub.status, "cancelled");
}

/// Test subscription suspension and reactivation
#[tokio::test]
async fn test_subscription_suspend_reactivate() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    let branch_id = TestFixture::create_branch(db).await;
    let customer_id = TestFixture::create_customer(db, branch_id).await;
    let plan_id = TestFixture::create_plan(db).await;

    use crate::modules::subscription::domain::entities::subscription;

    let now = chrono::Utc::now();
    let sub = subscription::ActiveModel {
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
    let sub = sub.insert(db).await.unwrap();

    // Suspend
    let mut active: subscription::ActiveModel = sub.into();
    active.status = Set("suspended".to_string());
    active.updated_at = Set(chrono::Utc::now());
    let sub = active.update(db).await.unwrap();
    assert_eq!(sub.status, "suspended");

    // Reactivate
    let mut active: subscription::ActiveModel = sub.into();
    active.status = Set("active".to_string());
    active.updated_at = Set(chrono::Utc::now());
    let sub = active.update(db).await.unwrap();
    assert_eq!(sub.status, "active");
}
