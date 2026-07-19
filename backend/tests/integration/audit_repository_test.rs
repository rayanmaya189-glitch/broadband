//! Integration tests for audit module using testcontainers
//!
//! Covers audit log insertion, searching by various dimensions (action,
//! resource type, user, result), JSON data storage, and ordering.

mod common;

use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use crate::common::{TestDatabase, TestFixture};

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

async fn insert_audit_log(
    db: &sea_orm::DatabaseConnection,
    user_id: Option<i64>,
    action: &str,
    resource_type: Option<&str>,
    result: &str,
) -> crate::modules::audit::domain::entities::audit_log::Model {
    use crate::modules::audit::domain::entities::audit_log;

    let active = audit_log::ActiveModel {
        user_id: Set(user_id),
        user_email: Set(user_id.map(|_| "admin@aeroxe.com".to_string())),
        user_role: Set(user_id.map(|_| "super_admin".to_string())),
        action: Set(action.to_string()),
        resource_type: Set(resource_type.map(|s| s.to_string())),
        resource_id: Set(None),
        ip_address: Set(Some("192.168.1.1".to_string())),
        user_agent: Set(Some("Mozilla/5.0".to_string())),
        result: Set(result.to_string()),
        old_data: Set(None),
        new_data: Set(None),
        metadata: Set(None),
        created_at: Set(chrono::Utc::now()),
        ..Default::default()
    };
    active.insert(db).await.expect("Failed to insert audit log")
}

// ===========================================================================
// Basic insertion
// ===========================================================================

#[tokio::test]
async fn test_audit_log_insert() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    let log = insert_audit_log(db, Some(1), "USER_LOGIN", Some("auth"), "granted").await;

    assert!(log.id > 0);
    assert_eq!(log.user_id, Some(1));
    assert_eq!(log.action, "USER_LOGIN");
    assert_eq!(log.resource_type, Some("auth".to_string()));
    assert_eq!(log.result, "granted");
}

#[tokio::test]
async fn test_audit_log_insert_with_json_data() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    use crate::modules::audit::domain::entities::audit_log;

    let old = serde_json::json!({"status": "active", "name": "Old Name"});
    let new = serde_json::json!({"status": "suspended", "name": "New Name"});
    let meta = serde_json::json!({"reason": "policy_violation", "case_id": "CS-001"});

    let active = audit_log::ActiveModel {
        user_id: Set(Some(1)),
        user_email: Set(Some("admin@aeroxe.com".to_string())),
        user_role: Set(Some("super_admin".to_string())),
        action: Set("CUSTOMER_UPDATE".to_string()),
        resource_type: Set(Some("customer".to_string())),
        resource_id: Set(Some("42".to_string())),
        ip_address: Set(Some("10.0.0.1".to_string())),
        user_agent: Set(None),
        result: Set("granted".to_string()),
        old_data: Set(Some(old.clone())),
        new_data: Set(Some(new.clone())),
        metadata: Set(Some(meta.clone())),
        created_at: Set(chrono::Utc::now()),
        ..Default::default()
    };

    let log = active.insert(db).await.expect("Failed to insert audit log");
    assert!(log.old_data.is_some());
    assert!(log.new_data.is_some());
    assert!(log.metadata.is_some());
}

#[tokio::test]
async fn test_audit_log_insert_without_user() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    use crate::modules::audit::domain::entities::audit_log;

    let active = audit_log::ActiveModel {
        user_id: Set(None),
        user_email: Set(None),
        user_role: Set(None),
        action: Set("SYSTEM_EVENT".to_string()),
        result: Set("granted".to_string()),
        created_at: Set(chrono::Utc::now()),
        ..Default::default()
    };

    let log = active.insert(db).await.expect("Failed to insert audit log");
    assert!(log.user_id.is_none());
    assert!(log.user_email.is_none());
}

// ===========================================================================
// Search by action
// ===========================================================================

#[tokio::test]
async fn test_search_by_action() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    insert_audit_log(db, Some(1), "USER_LOGIN", Some("auth"), "granted").await;
    insert_audit_log(db, Some(1), "USER_LOGOUT", Some("auth"), "granted").await;
    insert_audit_log(db, Some(1), "USER_LOGIN", Some("auth"), "denied").await;

    use crate::modules::audit::domain::entities::audit_log;

    let login_logs = audit_log::Entity::find()
        .filter(audit_log::Column::Action.eq("USER_LOGIN"))
        .all(db)
        .await
        .unwrap();

    assert_eq!(login_logs.len(), 2);
}

#[tokio::test]
async fn test_search_by_action_pattern() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    insert_audit_log(db, Some(1), "CUSTOMER_CREATE", Some("customer"), "granted").await;
    insert_audit_log(db, Some(1), "CUSTOMER_UPDATE", Some("customer"), "granted").await;
    insert_audit_log(db, Some(1), "CUSTOMER_DELETE", Some("customer"), "denied").await;
    insert_audit_log(db, Some(1), "SUBSCRIPTION_CREATE", Some("subscription"), "granted").await;

    use crate::modules::audit::domain::entities::audit_log;

    let customer_logs = audit_log::Entity::find()
        .filter(audit_log::Column::Action.contains("CUSTOMER"))
        .all(db)
        .await
        .unwrap();

    assert_eq!(customer_logs.len(), 3);
}

// ===========================================================================
// Search by resource type
// ===========================================================================

#[tokio::test]
async fn test_search_by_resource_type() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    insert_audit_log(db, Some(1), "CREATE", Some("customer"), "granted").await;
    insert_audit_log(db, Some(1), "UPDATE", Some("customer"), "granted").await;
    insert_audit_log(db, Some(1), "CREATE", Some("invoice"), "granted").await;

    use crate::modules::audit::domain::entities::audit_log;

    let customer_logs = audit_log::Entity::find()
        .filter(audit_log::Column::ResourceType.eq("customer"))
        .all(db)
        .await
        .unwrap();
    assert_eq!(customer_logs.len(), 2);

    let invoice_logs = audit_log::Entity::find()
        .filter(audit_log::Column::ResourceType.eq("invoice"))
        .all(db)
        .await
        .unwrap();
    assert_eq!(invoice_logs.len(), 1);
}

#[tokio::test]
async fn test_search_by_resource_id() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    insert_audit_log(db, Some(1), "UPDATE", Some("customer"), "granted").await;
    insert_audit_log(db, Some(1), "DELETE", Some("customer"), "denied").await;

    use crate::modules::audit::domain::entities::audit_log;

    let logs_for_42 = audit_log::Entity::find()
        .filter(audit_log::Column::ResourceId.eq("42"))
        .all(db)
        .await
        .unwrap();

    // Both logs have resource_id = None in insert_audit_log helper, so this tests None
    // Let's insert with explicit resource_id
    let active = audit_log::ActiveModel {
        user_id: Set(Some(1)),
        action: Set("VIEW".to_string()),
        resource_type: Set(Some("customer".to_string())),
        resource_id: Set(Some("42".to_string())),
        result: Set("granted".to_string()),
        created_at: Set(chrono::Utc::now()),
        ..Default::default()
    };
    active.insert(db).await.unwrap();

    let found = audit_log::Entity::find()
        .filter(audit_log::Column::ResourceId.eq("42"))
        .all(db)
        .await
        .unwrap();
    assert_eq!(found.len(), 1);
    assert_eq!(found[0].resource_id.as_deref(), Some("42"));
}

// ===========================================================================
// Search by result
// ===========================================================================

#[tokio::test]
async fn test_search_by_result_granted() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    insert_audit_log(db, Some(1), "LOGIN", Some("auth"), "granted").await;
    insert_audit_log(db, Some(1), "LOGIN", Some("auth"), "denied").await;
    insert_audit_log(db, Some(1), "LOGIN", Some("auth"), "granted").await;
    insert_audit_log(db, Some(1), "LOGIN", Some("auth"), "expired").await;

    use crate::modules::audit::domain::entities::audit_log;

    let granted = audit_log::Entity::find()
        .filter(audit_log::Column::Result.eq("granted"))
        .all(db)
        .await
        .unwrap();
    assert_eq!(granted.len(), 2);

    let denied = audit_log::Entity::find()
        .filter(audit_log::Column::Result.eq("denied"))
        .all(db)
        .await
        .unwrap();
    assert_eq!(denied.len(), 1);

    let expired = audit_log::Entity::find()
        .filter(audit_log::Column::Result.eq("expired"))
        .all(db)
        .await
        .unwrap();
    assert_eq!(expired.len(), 1);
}

// ===========================================================================
// Search by user
// ===========================================================================

#[tokio::test]
async fn test_search_by_user_id() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    insert_audit_log(db, Some(1), "ACTION_A", None, "granted").await;
    insert_audit_log(db, Some(1), "ACTION_B", None, "granted").await;
    insert_audit_log(db, Some(2), "ACTION_A", None, "granted").await;

    use crate::modules::audit::domain::entities::audit_log;

    let user1_logs = audit_log::Entity::find()
        .filter(audit_log::Column::UserId.eq(1))
        .all(db)
        .await
        .unwrap();
    assert_eq!(user1_logs.len(), 2);

    let user2_logs = audit_log::Entity::find()
        .filter(audit_log::Column::UserId.eq(2))
        .all(db)
        .await
        .unwrap();
    assert_eq!(user2_logs.len(), 1);
}

#[tokio::test]
async fn test_search_by_user_email() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    use crate::modules::audit::domain::entities::audit_log;

    for email in ["alice@aeroxe.com", "alice@aeroxe.com", "bob@aeroxe.com"] {
        let active = audit_log::ActiveModel {
            user_id: Set(Some(1)),
            user_email: Set(Some(email.to_string())),
            action: Set("TEST".to_string()),
            result: Set("granted".to_string()),
            created_at: Set(chrono::Utc::now()),
            ..Default::default()
        };
        active.insert(db).await.unwrap();
    }

    let alice_logs = audit_log::Entity::find()
        .filter(audit_log::Column::UserEmail.eq("alice@aeroxe.com"))
        .all(db)
        .await
        .unwrap();
    assert_eq!(alice_logs.len(), 2);
}

// ===========================================================================
// Bulk insertion and counting
// ===========================================================================

#[tokio::test]
async fn test_bulk_audit_log_insertion() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    for i in 0..100 {
        insert_audit_log(db, Some(i % 5), "BULK_ACTION", Some("test"), "granted").await;
    }

    use crate::modules::audit::domain::entities::audit_log;

    let all = audit_log::Entity::find()
        .filter(audit_log::Column::Action.eq("BULK_ACTION"))
        .all(db)
        .await
        .unwrap();
    assert_eq!(all.len(), 100);
}

#[tokio::test]
async fn test_audit_log_filter_by_ip_address() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    use crate::modules::audit::domain::entities::audit_log;

    for (ip, idx) in [("10.0.0.1", 1), ("10.0.0.2", 2), ("10.0.0.1", 3)] {
        let active = audit_log::ActiveModel {
            user_id: Set(Some(1)),
            action: Set(format!("ACTION_{}", idx)),
            ip_address: Set(Some(ip.to_string())),
            result: Set("granted".to_string()),
            created_at: Set(chrono::Utc::now()),
            ..Default::default()
        };
        active.insert(db).await.unwrap();
    }

    let from_ip1 = audit_log::Entity::find()
        .filter(audit_log::Column::IpAddress.eq("10.0.0.1"))
        .all(db)
        .await
        .unwrap();
    assert_eq!(from_ip1.len(), 2);
}

// ===========================================================================
// Metadata / JSON queries
// ===========================================================================

#[tokio::test]
async fn test_audit_log_metadata_storage() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    use crate::modules::audit::domain::entities::audit_log;

    let meta = serde_json::json!({
        "session_id": "abc-123",
        "geo_location": {"lat": 12.97, "lng": 77.59},
        "flags": ["mfa_verified", "new_device"]
    });

    let active = audit_log::ActiveModel {
        user_id: Set(Some(1)),
        action: Set("METADATA_TEST".to_string()),
        result: Set("granted".to_string()),
        metadata: Set(Some(meta.clone())),
        created_at: Set(chrono::Utc::now()),
        ..Default::default()
    };

    let log = active.insert(db).await.unwrap();
    let stored = log.metadata.unwrap();
    assert_eq!(stored["session_id"], "abc-123");
    assert_eq!(stored["geo_location"]["lat"], 12.97);
}

#[tokio::test]
async fn test_audit_log_old_new_data_tracking() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    use crate::modules::audit::domain::entities::audit_log;

    let old = serde_json::json!({"status": "active", "speed_mbps": 100});
    let new = serde_json::json!({"status": "active", "speed_mbps": 200});

    let active = audit_log::ActiveModel {
        user_id: Set(Some(1)),
        action: Set("PLAN_CHANGE".to_string()),
        resource_type: Set(Some("subscription".to_string())),
        resource_id: Set(Some("99".to_string())),
        result: Set("granted".to_string()),
        old_data: Set(Some(old)),
        new_data: Set(Some(new)),
        created_at: Set(chrono::Utc::now()),
        ..Default::default()
    };

    let log = active.insert(db).await.unwrap();
    assert_eq!(log.old_data.as_ref().unwrap()["speed_mbps"], 100);
    assert_eq!(log.new_data.as_ref().unwrap()["speed_mbps"], 200);
}

// ===========================================================================
// Combined filters
// ===========================================================================

#[tokio::test]
async fn test_combined_filter_action_and_result() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    insert_audit_log(db, Some(1), "LOGIN", Some("auth"), "granted").await;
    insert_audit_log(db, Some(1), "LOGIN", Some("auth"), "denied").await;
    insert_audit_log(db, Some(1), "LOGOUT", Some("auth"), "granted").await;

    use crate::modules::audit::domain::entities::audit_log;

    let login_denied = audit_log::Entity::find()
        .filter(
            audit_log::Column::Action
                .eq("LOGIN")
                .and(audit_log::Column::Result.eq("denied")),
        )
        .all(db)
        .await
        .unwrap();
    assert_eq!(login_denied.len(), 1);
}

#[tokio::test]
async fn test_combined_filter_user_and_resource() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    insert_audit_log(db, Some(1), "CREATE", Some("customer"), "granted").await;
    insert_audit_log(db, Some(1), "UPDATE", Some("invoice"), "granted").await;
    insert_audit_log(db, Some(2), "CREATE", Some("customer"), "granted").await;

    use crate::modules::audit::domain::entities::audit_log;

    let user1_customer = audit_log::Entity::find()
        .filter(
            audit_log::Column::UserId
                .eq(1)
                .and(audit_log::Column::ResourceType.eq("customer")),
        )
        .all(db)
        .await
        .unwrap();
    assert_eq!(user1_customer.len(), 1);
}

// ===========================================================================
// Ordering
// ===========================================================================

#[tokio::test]
async fn test_audit_log_ordering_by_created_at() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    use crate::modules::audit::domain::entities::audit_log;
    use sea_orm::QueryOrder;

    // Insert with slight time differences
    for i in 0..3 {
        let active = audit_log::ActiveModel {
            user_id: Set(Some(1)),
            action: Set(format!("ORDER_TEST_{}", i)),
            result: Set("granted".to_string()),
            created_at: Set(chrono::Utc::now()),
            ..Default::default()
        };
        active.insert(db).await.unwrap();
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
    }

    let logs = audit_log::Entity::find()
        .filter(audit_log::Column::Action.contains("ORDER_TEST"))
        .order_by_asc(audit_log::Column::CreatedAt)
        .all(db)
        .await
        .unwrap();

    assert_eq!(logs.len(), 3);
    assert!(logs[0].created_at <= logs[1].created_at);
    assert!(logs[1].created_at <= logs[2].created_at);
}
