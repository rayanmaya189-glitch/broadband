//! Integration tests for subscription repository using testcontainers

mod common;

use sea_orm::{DatabaseConnection, EntityTrait, Set, ActiveModelTrait};
use crate::common::{TestDatabase, TestFixture};

/// Test subscription creation with dependencies
#[tokio::test]
async fn test_create_subscription() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();
    
    // Create prerequisite data
    let branch_id = TestFixture::create_branch(db).await;
    let customer_id = TestFixture::create_customer(db, branch_id).await;
    let plan_id = TestFixture::create_plan(db).await;
    
    // Create subscription
    use crate::modules::subscription::domain::entities::subscription;
    
    let now = chrono::Utc::now();
    let active_model = subscription::ActiveModel {
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
    
    let result = active_model.insert(db).await.expect("Failed to create subscription");
    assert!(result.id > 0, "Subscription should be created with positive ID");
}

/// Test subscription lifecycle
#[tokio::test]
async fn test_subscription_lifecycle() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();
    
    let branch_id = TestFixture::create_branch(db).await;
    let customer_id = TestFixture::create_customer(db, branch_id).await;
    let plan_id = TestFixture::create_plan(db).await;
    
    use crate::modules::subscription::domain::entities::subscription;
    
    let now = chrono::Utc::now();
    let active_model = subscription::ActiveModel {
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
    
    let sub = active_model.insert(db).await.expect("Failed to create subscription");
    
    // Suspend subscription
    let mut active_model: subscription::ActiveModel = sub.into();
    active_model.status = Set("suspended".to_string());
    active_model.updated_at = Set(chrono::Utc::now());
    
    let suspended = active_model.update(db).await.expect("Failed to suspend subscription");
    assert_eq!(suspended.status, "suspended");
    
    // Reactivate subscription
    let mut active_model: subscription::ActiveModel = suspended.into();
    active_model.status = Set("active".to_string());
    active_model.updated_at = Set(chrono::Utc::now());
    
    let reactivated = active_model.update(db).await.expect("Failed to reactivate subscription");
    assert_eq!(reactivated.status, "active");
}

/// Test subscription cancellation
#[tokio::test]
async fn test_subscription_cancellation() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();
    
    let branch_id = TestFixture::create_branch(db).await;
    let customer_id = TestFixture::create_customer(db, branch_id).await;
    let plan_id = TestFixture::create_plan(db).await;
    
    use crate::modules::subscription::domain::entities::subscription;
    
    let now = chrono::Utc::now();
    let active_model = subscription::ActiveModel {
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
    
    let sub = active_model.insert(db).await.expect("Failed to create subscription");
    
    // Cancel subscription
    let mut active_model: subscription::ActiveModel = sub.into();
    active_model.status = Set("cancelled".to_string());
    active_model.end_date = Set(Some(chrono::NaiveDate::from_ymd_opt(2026, 7, 15).unwrap()));
    active_model.updated_at = Set(chrono::Utc::now());
    
    let cancelled = active_model.update(db).await.expect("Failed to cancel subscription");
    assert_eq!(cancelled.status, "cancelled");
    assert!(cancelled.end_date.is_some(), "End date should be set after cancellation");
}
