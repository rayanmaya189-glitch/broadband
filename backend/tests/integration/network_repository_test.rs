//! Integration tests for network module using testcontainers
//!
//! Covers VLAN CRUD, IP pool management, MAC binding, PPPoE session lifecycle,
//! and IP address allocation tracking.

mod common;

use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use crate::common::{TestDatabase, TestFixture};

// ===========================================================================
// VLAN tests
// ===========================================================================

#[tokio::test]
async fn test_vlan_crud() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    let branch_id = TestFixture::create_branch(db).await;

    use crate::modules::network::domain::entities::vlan;
    let now = chrono::Utc::now();

    let active = vlan::ActiveModel {
        branch_id: Set(branch_id),
        vlan_id: Set(200),
        name: Set("Customer Data".to_string()),
        vlan_type: Set("customer_data".to_string()),
        is_active: Set(true),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    };

    let vlan = active.insert(db).await.expect("Failed to create VLAN");
    assert!(vlan.id > 0);
    assert_eq!(vlan.vlan_id, 200);
    assert_eq!(vlan.name, "Customer Data");
    assert_eq!(vlan.vlan_type, "customer_data");
    assert!(vlan.is_active);
}

#[tokio::test]
async fn test_vlan_update() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    let branch_id = TestFixture::create_branch(db).await;

    use crate::modules::network::domain::entities::vlan;
    let now = chrono::Utc::now();

    let created = vlan::ActiveModel {
        branch_id: Set(branch_id),
        vlan_id: Set(300),
        name: Set("Management".to_string()),
        vlan_type: Set("management".to_string()),
        is_active: Set(true),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    }
    .insert(db)
    .await
    .unwrap();

    let mut active: vlan::ActiveModel = created.into();
    active.name = Set("MGMT VLAN".to_string());
    active.is_active = Set(false);
    active.updated_at = Set(chrono::Utc::now());

    let updated = active.update(db).await.unwrap();
    assert_eq!(updated.name, "MGMT VLAN");
    assert!(!updated.is_active);
}

#[tokio::test]
async fn test_vlan_filter_by_branch() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    let branch_a = TestFixture::create_branch(db).await;
    let branch_b = TestFixture::create_branch(db).await;

    use crate::modules::network::domain::entities::vlan;
    let now = chrono::Utc::now();

    for (branch, vid) in [(branch_a, 100), (branch_a, 101), (branch_b, 200)] {
        vlan::ActiveModel {
            branch_id: Set(branch),
            vlan_id: Set(vid),
            name: Set(format!("VLAN {}", vid)),
            vlan_type: Set("customer_data".to_string()),
            is_active: Set(true),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        }
        .insert(db)
        .await
        .unwrap();
    }

    let branch_a_vlans = vlan::Entity::find()
        .filter(vlan::Column::BranchId.eq(branch_a))
        .all(db)
        .await
        .unwrap();
    assert_eq!(branch_a_vlans.len(), 2);

    let branch_b_vlans = vlan::Entity::find()
        .filter(vlan::Column::BranchId.eq(branch_b))
        .all(db)
        .await
        .unwrap();
    assert_eq!(branch_b_vlans.len(), 1);
}

#[tokio::test]
async fn test_vlan_deactivation() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    let branch_id = TestFixture::create_branch(db).await;

    use crate::modules::network::domain::entities::vlan;
    let now = chrono::Utc::now();

    let created = vlan::ActiveModel {
        branch_id: Set(branch_id),
        vlan_id: Set(500),
        name: Set("Temp VLAN".to_string()),
        vlan_type: Set("customer_data".to_string()),
        is_active: Set(true),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    }
    .insert(db)
    .await
    .unwrap();

    let mut active: vlan::ActiveModel = created.into();
    active.is_active = Set(false);
    active.updated_at = Set(chrono::Utc::now());

    let updated = active.update(db).await.unwrap();
    assert!(!updated.is_active);

    // Verify can still retrieve it
    let found = vlan::Entity::find_by_id(updated.id)
        .one(db)
        .await
        .unwrap()
        .expect("VLAN should still exist");
    assert!(!found.is_active);
}

#[tokio::test]
async fn test_vlan_with_approval() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    let branch_id = TestFixture::create_branch(db).await;

    use crate::modules::network::domain::entities::vlan;
    let now = chrono::Utc::now();

    let created = vlan::ActiveModel {
        branch_id: Set(branch_id),
        vlan_id: Set(600),
        name: Set("Needs approval".to_string()),
        vlan_type: Set("customer_data".to_string()),
        is_active: Set(false),
        created_by: Set(Some(10)),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    }
    .insert(db)
    .await
    .unwrap();

    assert!(created.approved_by.is_none());

    // Approve
    let mut active: vlan::ActiveModel = created.into();
    active.approved_by = Set(Some(1));
    active.approved_at = Set(Some(chrono::Utc::now()));
    active.is_active = Set(true);
    active.updated_at = Set(chrono::Utc::now());

    let approved = active.update(db).await.unwrap();
    assert!(approved.approved_by.is_some());
    assert!(approved.approved_at.is_some());
    assert!(approved.is_active);
}

// ===========================================================================
// IP Pool tests
// ===========================================================================

#[tokio::test]
async fn test_ip_pool_crud() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    let branch_id = TestFixture::create_branch(db).await;

    use crate::modules::network::domain::entities::ip_pool;
    let now = chrono::Utc::now();

    let active = ip_pool::ActiveModel {
        branch_id: Set(branch_id),
        name: Set("Residential Pool".to_string()),
        cidr: Set("10.0.200.0/24".to_string()),
        gateway: Set("10.0.200.1".to_string()),
        dns_primary: Set("8.8.8.8".to_string()),
        dns_secondary: Set(Some("8.8.4.4".to_string())),
        vlan_id: Set(Some(200)),
        pool_type: Set("residential".to_string()),
        allocated_count: Set(0),
        total_count: Set(254),
        status: Set("healthy".to_string()),
        is_active: Set(true),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    };

    let pool = active.insert(db).await.expect("Failed to create IP pool");
    assert!(pool.id > 0);
    assert_eq!(pool.cidr, "10.0.200.0/24");
    assert_eq!(pool.gateway, "10.0.200.1");
    assert_eq!(pool.total_count, 254);
    assert_eq!(pool.allocated_count, 0);
    assert_eq!(pool.status, "healthy");
}

#[tokio::test]
async fn test_ip_pool_utilization_tracking() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    let branch_id = TestFixture::create_branch(db).await;

    use crate::modules::network::domain::entities::ip_pool;
    let now = chrono::Utc::now();

    let pool = ip_pool::ActiveModel {
        branch_id: Set(branch_id),
        name: Set("Pool with utilization".to_string()),
        cidr: Set("10.0.201.0/24".to_string()),
        gateway: Set("10.0.201.1".to_string()),
        pool_type: Set("residential".to_string()),
        allocated_count: Set(0),
        total_count: Set(254),
        status: Set("healthy".to_string()),
        is_active: Set(true),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    }
    .insert(db)
    .await
    .unwrap();

    // Allocate some IPs
    let mut active: ip_pool::ActiveModel = pool.into();
    active.allocated_count = Set(50);
    active.updated_at = Set(chrono::Utc::now());
    let updated = active.update(db).await.unwrap();
    assert_eq!(updated.allocated_count, 50);

    // Allocate more
    let mut active: ip_pool::ActiveModel = updated.into();
    active.allocated_count = Set(200);
    active.status = Set("warning".to_string());
    active.updated_at = Set(chrono::Utc::now());
    let updated = active.update(db).await.unwrap();
    assert_eq!(updated.allocated_count, 200);
    assert_eq!(updated.status, "warning");

    // Exhaust the pool
    let mut active: ip_pool::ActiveModel = updated.into();
    active.allocated_count = Set(254);
    active.status = Set("exhausted".to_string());
    active.updated_at = Set(chrono::Utc::now());
    let updated = active.update(db).await.unwrap();
    assert_eq!(updated.allocated_count, 254);
    assert_eq!(updated.status, "exhausted");
}

#[tokio::test]
async fn test_ip_pool_filter_by_branch() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    let branch_a = TestFixture::create_branch(db).await;
    let branch_b = TestFixture::create_branch(db).await;

    use crate::modules::network::domain::entities::ip_pool;
    let now = chrono::Utc::now();

    for (branch, name, cidr) in [
        (branch_a, "Pool A1", "10.0.10.0/24"),
        (branch_a, "Pool A2", "10.0.11.0/24"),
        (branch_b, "Pool B1", "10.0.20.0/24"),
    ] {
        ip_pool::ActiveModel {
            branch_id: Set(branch),
            name: Set(name.to_string()),
            cidr: Set(cidr.to_string()),
            gateway: Set("10.0.0.1".to_string()),
            pool_type: Set("residential".to_string()),
            allocated_count: Set(0),
            total_count: Set(254),
            status: Set("healthy".to_string()),
            is_active: Set(true),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        }
        .insert(db)
        .await
        .unwrap();
    }

    let branch_a_pools = ip_pool::Entity::find()
        .filter(ip_pool::Column::BranchId.eq(branch_a))
        .all(db)
        .await
        .unwrap();
    assert_eq!(branch_a_pools.len(), 2);
}

#[tokio::test]
async fn test_ip_pool_update_dns() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    let branch_id = TestFixture::create_branch(db).await;

    use crate::modules::network::domain::entities::ip_pool;
    let now = chrono::Utc::now();

    let pool = ip_pool::ActiveModel {
        branch_id: Set(branch_id),
        name: Set("DNS update pool".to_string()),
        cidr: Set("10.0.202.0/24".to_string()),
        gateway: Set("10.0.202.1".to_string()),
        dns_primary: Set("8.8.8.8".to_string()),
        pool_type: Set("residential".to_string()),
        allocated_count: Set(0),
        total_count: Set(254),
        status: Set("healthy".to_string()),
        is_active: Set(true),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    }
    .insert(db)
    .await
    .unwrap();

    let mut active: ip_pool::ActiveModel = pool.into();
    active.dns_primary = Set(Some("1.1.1.1".to_string()));
    active.dns_secondary = Set(Some("1.0.0.1".to_string()));
    active.updated_at = Set(chrono::Utc::now());

    let updated = active.update(db).await.unwrap();
    assert_eq!(updated.dns_primary.as_deref(), Some("1.1.1.1"));
    assert_eq!(updated.dns_secondary.as_deref(), Some("1.0.0.1"));
}

// ===========================================================================
// MAC Binding tests
// ===========================================================================

#[tokio::test]
async fn test_mac_binding_crud() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    let branch_id = TestFixture::create_branch(db).await;
    let customer_id = TestFixture::create_customer(db, branch_id).await;

    use crate::modules::network::domain::entities::mac_binding;
    let now = chrono::Utc::now();

    let active = mac_binding::ActiveModel {
        branch_id: Set(branch_id),
        customer_id: Set(customer_id),
        subscription_id: Set(None),
        mac_address: Set("AA:BB:CC:DD:EE:FF".to_string()),
        assigned_ip: Set("10.0.200.10".to_string()),
        vlan_id: Set(Some(200)),
        bound_at: Set(now),
        is_active: Set(true),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    };

    let binding = active.insert(db).await.expect("Failed to create MAC binding");
    assert!(binding.id > 0);
    assert_eq!(binding.mac_address, "AA:BB:CC:DD:EE:FF");
    assert_eq!(binding.assigned_ip, "10.0.200.10");
    assert!(binding.is_active);
}

#[tokio::test]
async fn test_mac_binding_deactivation() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    let branch_id = TestFixture::create_branch(db).await;
    let customer_id = TestFixture::create_customer(db, branch_id).await;

    use crate::modules::network::domain::entities::mac_binding;
    let now = chrono::Utc::now();

    let created = mac_binding::ActiveModel {
        branch_id: Set(branch_id),
        customer_id: Set(customer_id),
        subscription_id: Set(None),
        mac_address: Set("11:22:33:44:55:66".to_string()),
        assigned_ip: Set("10.0.200.20".to_string()),
        bound_at: Set(now),
        is_active: Set(true),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    }
    .insert(db)
    .await
    .unwrap();

    let mut active: mac_binding::ActiveModel = created.into();
    active.is_active = Set(false);
    active.updated_at = Set(chrono::Utc::now());

    let updated = active.update(db).await.unwrap();
    assert!(!updated.is_active);
}

#[tokio::test]
async fn test_mac_binding_filter_by_customer() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    let branch_id = TestFixture::create_branch(db).await;
    let customer_a = TestFixture::create_customer(db, branch_id).await;
    let customer_b = TestFixture::create_customer(db, branch_id).await;

    use crate::modules::network::domain::entities::mac_binding;
    let now = chrono::Utc::now();

    for (cust, mac) in [(customer_a, "AA:AA:AA:AA:AA:01"), (customer_a, "AA:AA:AA:AA:AA:02"), (customer_b, "BB:BB:BB:BB:BB:01")] {
        mac_binding::ActiveModel {
            branch_id: Set(branch_id),
            customer_id: Set(cust),
            subscription_id: Set(None),
            mac_address: Set(mac.to_string()),
            assigned_ip: Set("10.0.200.50".to_string()),
            bound_at: Set(now),
            is_active: Set(true),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        }
        .insert(db)
        .await
        .unwrap();
    }

    let a_bindings = mac_binding::Entity::find()
        .filter(mac_binding::Column::CustomerId.eq(customer_a))
        .all(db)
        .await
        .unwrap();
    assert_eq!(a_bindings.len(), 2);

    let b_bindings = mac_binding::Entity::find()
        .filter(mac_binding::Column::CustomerId.eq(customer_b))
        .all(db)
        .await
        .unwrap();
    assert_eq!(b_bindings.len(), 1);
}

// ===========================================================================
// PPPoE session tests
// ===========================================================================

#[tokio::test]
async fn test_pppoe_session_crud() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    let branch_id = TestFixture::create_branch(db).await;
    let customer_id = TestFixture::create_customer(db, branch_id).await;

    use crate::modules::network::domain::entities::pppoe_session;
    let now = chrono::Utc::now();

    let active = pppoe_session::ActiveModel {
        branch_id: Set(branch_id),
        customer_id: Set(customer_id),
        subscription_id: Set(None),
        username: Set(format!("cust_{}", customer_id)),
        password_encrypted: Set("encrypted_password".to_string()),
        assigned_ip: Set(Some("10.0.200.10".to_string())),
        nas_port_id: Set(Some("Gi0/0/1".to_string())),
        session_start: Set(Some(now)),
        status: Set("active".to_string()),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    };

    let session = active.insert(db).await.expect("Failed to create PPPoE session");
    assert!(session.id > 0);
    assert_eq!(session.status, "active");
    assert!(session.session_start.is_some());
}

#[tokio::test]
async fn test_pppoe_session_status_transitions() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    let branch_id = TestFixture::create_branch(db).await;
    let customer_id = TestFixture::create_customer(db, branch_id).await;

    use crate::modules::network::domain::entities::pppoe_session;
    let now = chrono::Utc::now();

    let session = pppoe_session::ActiveModel {
        branch_id: Set(branch_id),
        customer_id: Set(customer_id),
        subscription_id: Set(None),
        username: Set("test_user".to_string()),
        password_encrypted: Set("enc".to_string()),
        session_start: Set(Some(now)),
        status: Set("active".to_string()),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    }
    .insert(db)
    .await
    .unwrap();

    // active -> disconnecting
    let mut active: pppoe_session::ActiveModel = session.into();
    active.status = Set("disconnecting".to_string());
    active.updated_at = Set(chrono::Utc::now());
    let disc = active.update(db).await.unwrap();
    assert_eq!(disc.status, "disconnecting");

    // disconnecting -> terminated
    let mut active: pppoe_session::ActiveModel = disc.into();
    active.status = Set("terminated".to_string());
    active.session_duration_seconds = Set(3600);
    active.updated_at = Set(chrono::Utc::now());
    let terminated = active.update(db).await.unwrap();
    assert_eq!(terminated.status, "terminated");
    assert_eq!(terminated.session_duration_seconds, 3600);
}

#[tokio::test]
async fn test_pppoe_session_traffic_counters() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    let branch_id = TestFixture::create_branch(db).await;
    let customer_id = TestFixture::create_customer(db, branch_id).await;

    use crate::modules::network::domain::entities::pppoe_session;
    let now = chrono::Utc::now();

    let session = pppoe_session::ActiveModel {
        branch_id: Set(branch_id),
        customer_id: Set(customer_id),
        subscription_id: Set(None),
        username: Set("traffic_user".to_string()),
        password_encrypted: Set("enc".to_string()),
        bytes_in: Set(1_000_000),
        bytes_out: Set(500_000),
        status: Set("active".to_string()),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    }
    .insert(db)
    .await
    .unwrap();

    // Update traffic counters
    let mut active: pppoe_session::ActiveModel = session.into();
    active.bytes_in = Set(5_000_000);
    active.bytes_out = Set(2_500_000);
    active.last_activity_at = Set(Some(chrono::Utc::now()));
    active.updated_at = Set(chrono::Utc::now());

    let updated = active.update(db).await.unwrap();
    assert_eq!(updated.bytes_in, 5_000_000);
    assert_eq!(updated.bytes_out, 2_500_000);
    assert!(updated.last_activity_at.is_some());
}

#[tokio::test]
async fn test_pppoe_session_filter_by_customer() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    let branch_id = TestFixture::create_branch(db).await;
    let customer_a = TestFixture::create_customer(db, branch_id).await;
    let customer_b = TestFixture::create_customer(db, branch_id).await;

    use crate::modules::network::domain::entities::pppoe_session;
    let now = chrono::Utc::now();

    for (cust, user) in [(customer_a, "user_a"), (customer_a, "user_a2"), (customer_b, "user_b")] {
        pppoe_session::ActiveModel {
            branch_id: Set(branch_id),
            customer_id: Set(cust),
            subscription_id: Set(None),
            username: Set(user.to_string()),
            password_encrypted: Set("enc".to_string()),
            status: Set("active".to_string()),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        }
        .insert(db)
        .await
        .unwrap();
    }

    let a_sessions = pppoe_session::Entity::find()
        .filter(pppoe_session::Column::CustomerId.eq(customer_a))
        .all(db)
        .await
        .unwrap();
    assert_eq!(a_sessions.len(), 2);
}

#[tokio::test]
async fn test_pppoe_session_with_nas_info() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    let branch_id = TestFixture::create_branch(db).await;
    let customer_id = TestFixture::create_customer(db, branch_id).await;

    use crate::modules::network::domain::entities::pppoe_session;
    let now = chrono::Utc::now();

    let session = pppoe_session::ActiveModel {
        branch_id: Set(branch_id),
        customer_id: Set(customer_id),
        subscription_id: Set(None),
        username: Set("nas_user".to_string()),
        password_encrypted: Set("enc".to_string()),
        pppoe_server_ip: Set(Some("10.0.0.1".to_string())),
        assigned_ip: Set(Some("10.0.200.50".to_string())),
        nas_port_id: Set(Some("Gi0/1/0".to_string())),
        nas_ip_address: Set(Some("192.168.1.1".to_string())),
        nas_session_id: Set(Some("sess-abc-123".to_string())),
        session_start: Set(Some(now)),
        status: Set("active".to_string()),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    }
    .insert(db)
    .await
    .unwrap();

    assert_eq!(session.pppoe_server_ip.as_deref(), Some("10.0.0.1"));
    assert_eq!(session.nas_ip_address.as_deref(), Some("192.168.1.1"));
    assert_eq!(session.nas_session_id.as_deref(), Some("sess-abc-123"));
}
