//! Integration tests for security/RBAC module

mod common;

use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use crate::common::TestDatabase;

/// Test role creation and hierarchy
#[tokio::test]
async fn test_role_crud() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    use crate::modules::security::domain::entities::role;

    let now = chrono::Utc::now();

    // Create parent role
    let parent = role::ActiveModel {
        name: Set("admin".to_string()),
        slug: Set("admin".to_string()),
        description: Set("Administrator role".to_string()),
        is_system: Set(true),
        is_active: Set(true),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    };
    let parent = parent.insert(db).await.expect("Failed to create parent role");
    assert!(parent.id > 0);

    // Create child role
    let child = role::ActiveModel {
        name: Set("operator".to_string()),
        slug: Set("operator".to_string()),
        description: Set("Operator role".to_string()),
        parent_role_id: Set(Some(parent.id)),
        is_system: Set(false),
        is_active: Set(true),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    };
    let child = child.insert(db).await.expect("Failed to create child role");
    assert_eq!(child.parent_role_id, Some(parent.id));

    // Verify hierarchy
    let found = role::Entity::find_by_id(child.id)
        .one(db)
        .await
        .expect("Failed to query role")
        .expect("Role not found");
    assert_eq!(found.parent_role_id, Some(parent.id));
}

/// Test permission creation
#[tokio::test]
async fn test_permission_crud() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    use crate::modules::security::domain::entities::permission;

    let now = chrono::Utc::now();
    let perm = permission::ActiveModel {
        name: Set("customer.view".to_string()),
        module: Set("customer".to_string()),
        resource: Set("customer".to_string()),
        action: Set("view".to_string()),
        description: Set("View customer details".to_string()),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    };

    let created = perm.insert(db).await.expect("Failed to create permission");
    assert!(created.id > 0);
    assert_eq!(created.module, "customer");
    assert_eq!(created.action, "view");
}

/// Test role-permission assignment
#[tokio::test]
async fn test_role_permission_assignment() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    use crate::modules::security::domain::entities::{permission, role, role_permission};

    let now = chrono::Utc::now();

    // Create role
    let role = role::ActiveModel {
        name: Set("support_agent".to_string()),
        slug: Set("support_agent".to_string()),
        is_active: Set(true),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    };
    let role = role.insert(db).await.expect("Failed to create role");

    // Create permission
    let perm = permission::ActiveModel {
        name: Set("ticket.view".to_string()),
        module: Set("ticket".to_string()),
        resource: Set("ticket".to_string()),
        action: Set("view".to_string()),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    };
    let perm = perm.insert(db).await.expect("Failed to create permission");

    // Assign permission to role
    let assignment = role_permission::ActiveModel {
        role_id: Set(role.id),
        permission_id: Set(perm.id),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    };
    let assignment = assignment.insert(db).await.expect("Failed to assign permission");
    assert_eq!(assignment.role_id, role.id);
    assert_eq!(assignment.permission_id, perm.id);

    // Verify assignment
    let assigned = role_permission::Entity::find()
        .filter(role_permission::Column::RoleId.eq(role.id))
        .all(db)
        .await
        .expect("Failed to query assignments");
    assert!(!assigned.is_empty());
}

/// Test user-role assignment
#[tokio::test]
async fn test_user_role_assignment() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    use crate::modules::security::domain::entities::{role, user_role};

    let now = chrono::Utc::now();

    // Create role
    let role = role::ActiveModel {
        name: Set("noc_engineer".to_string()),
        slug: Set("noc_engineer".to_string()),
        is_active: Set(true),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    };
    let role = role.insert(db).await.expect("Failed to create role");

    // Assign role to user
    let user_role = user_role::ActiveModel {
        user_id: Set(1),
        role_id: Set(role.id),
        assigned_by: Set(Some(1)),
        is_active: Set(true),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    };
    let assignment = user_role.insert(db).await.expect("Failed to assign user role");
    assert_eq!(assignment.user_id, 1);
    assert_eq!(assignment.role_id, role.id);
    assert!(assignment.is_active);
}
