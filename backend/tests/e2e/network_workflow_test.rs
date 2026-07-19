//! End-to-end test: Network Management Workflow
//! Tests: VLAN creation → IP pool → PPPoE session → MAC binding

mod common;

use sea_orm::{ActiveModelTrait, EntityTrait, Set};
use crate::common::{TestDatabase, TestFixture};

/// Test VLAN lifecycle
#[tokio::test]
async fn test_vlan_lifecycle() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    let branch_id = TestFixture::create_branch(db).await;

    use crate::modules::network::domain::entities::vlan;

    let now = chrono::Utc::now();

    // Create VLAN
    let v = vlan::ActiveModel {
        branch_id: Set(branch_id),
        vlan_id: Set(200),
        name: Set("Customer Residential".to_string()),
        vlan_type: Set("customer_residential".to_string()),
        is_active: Set(true),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    };
    let v = v.insert(db).await.unwrap();
    assert!(v.id > 0);
    assert_eq!(v.vlan_id, 200);

    // Verify VLAN exists
    let found = vlan::Entity::find_by_id(v.id)
        .one(db)
        .await
        .unwrap();
    assert!(found.is_some());
    assert_eq!(found.unwrap().name, "Customer Residential");
}

/// Test IP pool creation
#[tokio::test]
async fn test_ip_pool_creation() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    let branch_id = TestFixture::create_branch(db).await;

    use crate::modules::network::domain::entities::ip_pool;

    let now = chrono::Utc::now();

    let pool = ip_pool::ActiveModel {
        branch_id: Set(branch_id),
        name: Set("Residential Pool".to_string()),
        cidr: Set("10.0.0.0/24".to_string()),
        gateway: Set("10.0.0.1".to_string()),
        vlan_id: Set(None),
        pool_type: Set("customer".to_string()),
        allocated_count: Set(0),
        total_count: Set(254),
        status: Set("healthy".to_string()),
        is_active: Set(true),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    };
    let pool = pool.insert(db).await.unwrap();
    assert!(pool.id > 0);
    assert_eq!(pool.total_count, 254);
}
