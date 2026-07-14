use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait, Set, ActiveModelTrait};

use crate::modules::role::model::role_entity;
use crate::modules::role::model::role_permission_entity;
use crate::modules::permission::model::permission_entity;

/// Seed roles with their respective permissions.
/// This assigns permissions to all system roles on server startup.
/// Uses ON CONFLICT DO NOTHING to ensure idempotency.
pub async fn seed_roles(db: &DatabaseConnection) -> Result<(), sea_orm::DbErr> {
    // First, ensure all system roles exist (they may already be seeded by migration)
    let roles = vec![
        ("super_admin", "Super Admin", "Full system access", true),
        ("isp_owner", "ISP Owner", "Business owner access", true),
        ("admin", "Admin", "Administrative access", true),
        ("finance_manager", "Finance Manager", "Financial operations", true),
        ("branch_manager", "Branch Manager", "Branch-level management", false),
        ("branch_operator", "Branch Operator", "Branch-level operations", false),
        ("network_engineer", "Network Engineer", "Network management", false),
        ("field_technician", "Field Technician", "Installation & field work", false),
        ("customer_support", "Customer Support", "Ticket handling", false),
        ("reseller", "Reseller", "Reseller portal access", false),
    ];

    for (name, display_name, description, is_system) in &roles {
        let existing = role_entity::Entity::find()
            .filter(role_entity::Column::Name.eq(*name))
            .one(db)
            .await?;

        if existing.is_none() {
            let new_role = role_entity::ActiveModel {
                name: Set(name.to_string()),
                display_name: Set(display_name.to_string()),
                description: Set(Some(description.to_string())),
                is_system: Set(*is_system),
                ..Default::default()
            };
            new_role.insert(db).await?;
        }
    }

    tracing::info!("System roles ensured in database");

    // Assign permissions to roles
    // Super Admin gets ALL permissions
    assign_all_permissions_to_role(db, "super_admin").await?;
    
    // ISP Owner gets most permissions except system-level audit
    assign_role_permissions(db, "isp_owner", &[
        "auth.*", "user.*", "role.*", "permission.*",
        "branch.*", "customer.*", "plan.*", "subscription.*",
        "billing.*", "ticket.*", "device.*", "bandwidth.*",
        "network.*", "coverage.*", "installation.*", "inventory.*",
        "lead.*", "referral.*", "notification.*", "event.*",
        "document.*", "accounting.*", "payment_gateway.*", "discovery.*",
    ]).await?;

    // Admin gets operational permissions
    assign_role_permissions(db, "admin", &[
        "auth.*", "user.*", "branch.*", "customer.*", 
        "plan.*", "subscription.*", "ticket.*", "device.*",
        "bandwidth.*", "network.*", "coverage.*", "installation.*",
        "inventory.*", "lead.*", "notification.*", "document.*",
    ]).await?;

    // Finance Manager gets billing and accounting permissions
    assign_role_permissions(db, "finance_manager", &[
        "auth.*", "user.me", "user.sessions",
        "billing.*", "accounting.*", "payment_gateway.*",
        "customer.view", "customer.get", "subscription.view", "subscription.get",
        "plan.view", "plan.get", "report.*", "notification.*",
    ]).await?;

    // Branch Manager gets branch-level permissions
    assign_role_permissions(db, "branch_manager", &[
        "auth.*", "user.me", "user.sessions", "user.view",
        "branch.view", "branch.users.view",
        "customer.*", "plan.view", "subscription.*",
        "ticket.*", "installation.*", "inventory.view",
        "notification.*", "document.*",
    ]).await?;

    // Branch Operator gets basic operational permissions
    assign_role_permissions(db, "branch_operator", &[
        "auth.*", "user.me", "user.sessions",
        "customer.view", "customer.create", "customer.get",
        "plan.view", "subscription.view", "ticket.*",
        "installation.view", "notification.*",
    ]).await?;

    // Network Engineer gets network and device permissions
    assign_role_permissions(db, "network_engineer", &[
        "auth.*", "user.me", "user.sessions",
        "device.*", "bandwidth.*", "network.*", "discovery.*",
        "customer.view", "subscription.view",
        "notification.*", "document.*",
    ]).await?;

    // Field Technician gets installation and field permissions
    assign_role_permissions(db, "field_technician", &[
        "auth.*", "user.me", "user.sessions",
        "installation.*", "inventory.view", "inventory.get",
        "customer.view", "customer.get",
        "ticket.view", "ticket.get", "ticket.update", "ticket.comments.*",
        "notification.*",
    ]).await?;

    // Customer Support gets ticket and customer permissions
    assign_role_permissions(db, "customer_support", &[
        "auth.*", "user.me", "user.sessions",
        "customer.view", "customer.get", "customer.profile.view",
        "ticket.*", "notification.*",
        "plan.view", "subscription.view",
    ]).await?;

    // Reseller gets limited permissions
    assign_role_permissions(db, "reseller", &[
        "auth.*", "user.me", "user.sessions",
        "customer.view", "customer.create", "customer.get",
        "plan.view", "subscription.view",
        "notification.*",
    ]).await?;

    tracing::info!("Role permissions seeded successfully");
    Ok(())
}

/// Assign ALL permissions to a role (super_admin)
async fn assign_all_permissions_to_role(db: &DatabaseConnection, role_name: &str) -> Result<(), sea_orm::DbErr> {
    let role = role_entity::Entity::find()
        .filter(role_entity::Column::Name.eq(role_name))
        .one(db)
        .await?;
    
    let role_id = match role {
        Some(r) => r.id,
        None => {
            tracing::warn!(role = role_name, "Role not found, skipping permission assignment");
            return Ok(());
        }
    };

    // Delete existing permissions for this role
    role_permission_entity::Entity::delete_many()
        .filter(role_permission_entity::Column::RoleId.eq(role_id))
        .exec(db)
        .await?;

    // Get all permissions
    let all_permissions = permission_entity::Entity::find().all(db).await?;

    // Assign ALL permissions
    let mut count = 0;
    for perm in all_permissions {
        let new_rp = role_permission_entity::ActiveModel {
            role_id: Set(role_id),
            permission_id: Set(perm.id),
            ..Default::default()
        };
        new_rp.insert(db).await?;
        count += 1;
    }

    tracing::info!(
        role = role_name, 
        permissions_assigned = count,
        "Assigned all permissions to role"
    );
    
    Ok(())
}

/// Assign permissions matching patterns to a role
async fn assign_role_permissions(db: &DatabaseConnection, role_name: &str, permission_patterns: &[&str]) -> Result<(), sea_orm::DbErr> {
    let role = role_entity::Entity::find()
        .filter(role_entity::Column::Name.eq(role_name))
        .one(db)
        .await?;
    
    let role_id = match role {
        Some(r) => r.id,
        None => {
            tracing::warn!(role = role_name, "Role not found, skipping permission assignment");
            return Ok(());
        }
    };

    // Delete existing permissions for this role
    role_permission_entity::Entity::delete_many()
        .filter(role_permission_entity::Column::RoleId.eq(role_id))
        .exec(db)
        .await?;

    // For each pattern, assign matching permissions
    let mut total_assigned: i64 = 0;
    
    for pattern in permission_patterns {
        if pattern.ends_with(".*") {
            // Wildcard pattern: match module prefix
            let module_prefix = pattern.trim_end_matches(".*");
            let matching_perms = permission_entity::Entity::find()
                .filter(permission_entity::Column::Name.starts_with(module_prefix))
                .all(db)
                .await?;
            
            for perm in matching_perms {
                let new_rp = role_permission_entity::ActiveModel {
                    role_id: Set(role_id),
                    permission_id: Set(perm.id),
                    ..Default::default()
                };
                new_rp.insert(db).await?;
                total_assigned += 1;
            }
        } else {
            // Exact permission name
            let perm = permission_entity::Entity::find()
                .filter(permission_entity::Column::Name.eq(*pattern))
                .one(db)
                .await?;
            
            if let Some(perm) = perm {
                let new_rp = role_permission_entity::ActiveModel {
                    role_id: Set(role_id),
                    permission_id: Set(perm.id),
                    ..Default::default()
                };
                new_rp.insert(db).await?;
                total_assigned += 1;
            }
        }
    }

    tracing::debug!(
        role = role_name, 
        permissions_assigned = total_assigned,
        "Assigned permissions to role"
    );
    
    Ok(())
}
