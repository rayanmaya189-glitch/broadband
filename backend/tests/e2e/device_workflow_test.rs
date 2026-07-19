//! End-to-end test: Device Management Workflow
//! Tests: Device registration → Status update → Port management

mod common;

use sea_orm::{ActiveModelTrait, EntityTrait, Set};
use crate::common::{TestDatabase, TestFixture};

/// Test device registration and status lifecycle
#[tokio::test]
async fn test_device_lifecycle() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    let branch_id = TestFixture::create_branch(db).await;

    use crate::modules::device::domain::entities::network_device;

    let now = chrono::Utc::now();

    // Register device
    let device = network_device::ActiveModel {
        branch_id: Set(branch_id),
        name: Set("OLT-HUAW-01".to_string()),
        device_model_id: Set(1),
        serial_number: Set(format!("SN-{:08}", rand::random::<u32>())),
        management_ip: Set("10.0.0.10".to_string()),
        management_port: Set(22),
        firmware_version: Set(Some("V800R013C10".to_string())),
        status: Set("offline".to_string()),
        health_score: Set(0),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    };
    let device = device.insert(db).await.unwrap();
    assert!(device.id > 0);
    assert_eq!(device.status, "offline");

    // Update status to online
    let mut active: network_device::ActiveModel = device.into();
    active.status = Set("online".to_string());
    active.health_score = Set(85);
    active.updated_at = Set(chrono::Utc::now());
    let device = active.update(db).await.unwrap();
    assert_eq!(device.status, "online");
    assert_eq!(device.health_score, 85);

    // Verify device exists
    let found = network_device::Entity::find_by_id(device.id)
        .one(db)
        .await
        .unwrap();
    assert!(found.is_some());
    assert_eq!(found.unwrap().status, "online");
}

/// Test device maintenance workflow
#[tokio::test]
async fn test_device_maintenance() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    let branch_id = TestFixture::create_branch(db).await;

    use crate::modules::device::domain::entities::network_device;

    let now = chrono::Utc::now();

    // Register device
    let device = network_device::ActiveModel {
        branch_id: Set(branch_id),
        name: Set("SWITCH-MIK-01".to_string()),
        device_model_id: Set(2),
        serial_number: Set(format!("SN-{:08}", rand::random::<u32>())),
        management_ip: Set("10.0.0.20".to_string()),
        management_port: Set(22),
        status: Set("online".to_string()),
        health_score: Set(90),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    };
    let device = device.insert(db).await.unwrap();

    // Move to maintenance
    let mut active: network_device::ActiveModel = device.into();
    active.status = Set("maintenance".to_string());
    active.health_score = Set(0);
    active.updated_at = Set(chrono::Utc::now());
    let device = active.update(db).await.unwrap();
    assert_eq!(device.status, "maintenance");

    // Back to online
    let mut active: network_device::ActiveModel = device.into();
    active.status = Set("online".to_string());
    active.health_score = Set(95);
    active.updated_at = Set(chrono::Utc::now());
    let device = active.update(db).await.unwrap();
    assert_eq!(device.status, "online");
    assert_eq!(device.health_score, 95);
}
