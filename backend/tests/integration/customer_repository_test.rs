//! Integration tests for customer repository using testcontainers

mod common;

use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait, Set, ActiveModelTrait};
use crate::common::{TestDatabase, TestFixture};

/// Test that we can connect to a test database
#[tokio::test]
async fn test_database_connection() {
    let test_db = TestDatabase::new().await;
    let _db = test_db.connection();
    
    // Just verify we can connect - no tables yet
    assert!(true, "Database connection successful");
}

/// Test creating and retrieving a branch
#[tokio::test]
async fn test_create_branch() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();
    
    // Note: This test assumes the branches table exists
    // In a real scenario, we'd run migrations first
    let branch_id = TestFixture::create_branch(db).await;
    assert!(branch_id > 0, "Branch should be created with positive ID");
}

/// Test creating a customer with branch reference
#[tokio::test]
async fn test_create_customer() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();
    
    let branch_id = TestFixture::create_branch(db).await;
    let customer_id = TestFixture::create_customer(db, branch_id).await;
    
    assert!(customer_id > 0, "Customer should be created with positive ID");
}

/// Test customer status transitions
#[tokio::test]
async fn test_customer_status_transitions() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();
    
    let branch_id = TestFixture::create_branch(db).await;
    let customer_id = TestFixture::create_customer(db, branch_id).await;
    
    // Update customer status
    use crate::modules::customer::domain::entities::customer;
    
    let customer = customer::Entity::find_by_id(customer_id)
        .one(db)
        .await
        .expect("Failed to find customer")
        .expect("Customer not found");
    
    let mut active_model: customer::ActiveModel = customer.into();
    active_model.status = Set("kyc_pending".to_string());
    active_model.updated_at = Set(chrono::Utc::now());
    
    let updated = active_model.update(db).await.expect("Failed to update customer");
    assert_eq!(updated.status, "kyc_pending");
}

/// Test plan creation and retrieval
#[tokio::test]
async fn test_plan_crud() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();
    
    let plan_id = TestFixture::create_plan(db).await;
    assert!(plan_id > 0, "Plan should be created");
    
    // Retrieve the plan
    use crate::modules::plans::domain::entities::plan;
    
    let plan = plan::Entity::find_by_id(plan_id)
        .one(db)
        .await
        .expect("Failed to find plan")
        .expect("Plan not found");
    
    assert_eq!(plan.slug, "test-plan-100");
    assert_eq!(plan.download_mbps, 100);
}

/// Test error handling for invalid data
#[tokio::test]
async fn test_invalid_customer_data() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();
    
    // Try to create customer with invalid branch_id
    use crate::modules::customer::domain::entities::customer;
    use sea_orm::{ActiveModelTrait, Set};
    
    let now = chrono::Utc::now();
    let active_model = customer::ActiveModel {
        customer_code: Set("AX-TST-202607-9999".to_string()),
        branch_id: Set(999999), // Non-existent branch
        name: Set("Invalid Customer".to_string()),
        phone: Set("+919876543210".to_string()),
        status: Set("registered".to_string()),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    };
    
    // This should fail due to foreign key constraint
    let result = active_model.insert(db).await;
    assert!(result.is_err(), "Should fail with invalid branch_id");
}

/// Test concurrent operations
#[tokio::test]
async fn test_concurrent_operations() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();
    
    let branch_id = TestFixture::create_branch(db).await;
    
    // Create multiple customers concurrently
    let mut handles = Vec::new();
    
    for i in 0..5 {
        let db_clone = db.clone();
        let branch_id = branch_id;
        let handle = tokio::spawn(async move {
            use crate::modules::customer::domain::entities::customer;
            use sea_orm::{ActiveModelTrait, Set};
            
            let now = chrono::Utc::now();
            let active_model = customer::ActiveModel {
                customer_code: Set(format!("AX-TST-202607-{:04}", i)),
                branch_id: Set(branch_id),
                name: Set(format!("Customer {}", i)),
                phone: Set(format!("+91987654321{}", i)),
                status: Set("registered".to_string()),
                created_at: Set(now),
                updated_at: Set(now),
                ..Default::default()
            };
            
            active_model.insert(&db_clone).await.expect("Failed to create customer")
        });
        handles.push(handle);
    }
    
    // Wait for all to complete
    let results: Vec<_> = futures::future::join_all(handles)
        .await
        .into_iter()
        .collect::<Result<_, _>>()
        .expect("One or more tasks failed");
    
    assert_eq!(results.len(), 5, "All customers should be created");
}
