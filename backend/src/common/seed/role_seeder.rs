use sqlx::PgPool;

/// Seed roles with their respective permissions.
/// This assigns permissions to all system roles on server startup.
/// Uses ON CONFLICT DO NOTHING to ensure idempotency.
pub async fn seed_roles(pool: &PgPool) -> Result<(), sqlx::Error> {
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
        sqlx::query(
            "INSERT INTO roles (name, display_name, description, is_system) 
             VALUES ($1, $2, $3, $4) 
             ON CONFLICT (name) DO NOTHING"
        )
        .bind(name)
        .bind(display_name)
        .bind(description)
        .bind(is_system)
        .execute(pool)
        .await?;
    }

    tracing::info!("System roles ensured in database");

    // Assign permissions to roles
    // Super Admin gets ALL permissions
    assign_all_permissions_to_role(pool, "super_admin").await?;
    
    // ISP Owner gets most permissions except system-level audit
    assign_role_permissions(pool, "isp_owner", &[
        "auth.*", "user.*", "role.*", "permission.*",
        "branch.*", "customer.*", "plan.*", "subscription.*",
        "billing.*", "ticket.*", "device.*", "bandwidth.*",
        "network.*", "coverage.*", "installation.*", "inventory.*",
        "lead.*", "referral.*", "notification.*", "event.*",
        "document.*", "accounting.*", "payment_gateway.*", "discovery.*",
    ]).await?;

    // Admin gets operational permissions
    assign_role_permissions(pool, "admin", &[
        "auth.*", "user.*", "branch.*", "customer.*", 
        "plan.*", "subscription.*", "ticket.*", "device.*",
        "bandwidth.*", "network.*", "coverage.*", "installation.*",
        "inventory.*", "lead.*", "notification.*", "document.*",
    ]).await?;

    // Finance Manager gets billing and accounting permissions
    assign_role_permissions(pool, "finance_manager", &[
        "auth.*", "user.me", "user.sessions",
        "billing.*", "accounting.*", "payment_gateway.*",
        "customer.view", "customer.get", "subscription.view", "subscription.get",
        "plan.view", "plan.get", "report.*", "notification.*",
    ]).await?;

    // Branch Manager gets branch-level permissions
    assign_role_permissions(pool, "branch_manager", &[
        "auth.*", "user.me", "user.sessions", "user.view",
        "branch.view", "branch.users.view",
        "customer.*", "plan.view", "subscription.*",
        "ticket.*", "installation.*", "inventory.view",
        "notification.*", "document.*",
    ]).await?;

    // Branch Operator gets basic operational permissions
    assign_role_permissions(pool, "branch_operator", &[
        "auth.*", "user.me", "user.sessions",
        "customer.view", "customer.create", "customer.get",
        "plan.view", "subscription.view", "ticket.*",
        "installation.view", "notification.*",
    ]).await?;

    // Network Engineer gets network and device permissions
    assign_role_permissions(pool, "network_engineer", &[
        "auth.*", "user.me", "user.sessions",
        "device.*", "bandwidth.*", "network.*", "discovery.*",
        "customer.view", "subscription.view",
        "notification.*", "document.*",
    ]).await?;

    // Field Technician gets installation and field permissions
    assign_role_permissions(pool, "field_technician", &[
        "auth.*", "user.me", "user.sessions",
        "installation.*", "inventory.view", "inventory.get",
        "customer.view", "customer.get",
        "ticket.view", "ticket.get", "ticket.update", "ticket.comments.*",
        "notification.*",
    ]).await?;

    // Customer Support gets ticket and customer permissions
    assign_role_permissions(pool, "customer_support", &[
        "auth.*", "user.me", "user.sessions",
        "customer.view", "customer.get", "customer.profile.view",
        "ticket.*", "notification.*",
        "plan.view", "subscription.view",
    ]).await?;

    // Reseller gets limited permissions
    assign_role_permissions(pool, "reseller", &[
        "auth.*", "user.me", "user.sessions",
        "customer.view", "customer.create", "customer.get",
        "plan.view", "subscription.view",
        "notification.*",
    ]).await?;

    tracing::info!("Role permissions seeded successfully");
    Ok(())
}

/// Assign ALL permissions to a role (super_admin)
async fn assign_all_permissions_to_role(pool: &PgPool, role_name: &str) -> Result<(), sqlx::Error> {
    let role = sqlx::query_scalar::<_, i64>("SELECT id FROM roles WHERE name = $1")
        .bind(role_name)
        .fetch_optional(pool)
        .await?;
    
    let role_id = match role {
        Some(id) => id,
        None => {
            tracing::warn!(role = role_name, "Role not found, skipping permission assignment");
            return Ok(());
        }
    };

    // Delete existing permissions for this role
    sqlx::query("DELETE FROM role_permissions WHERE role_id = $1")
        .bind(role_id)
        .execute(pool)
        .await?;

    // Assign ALL permissions
    let result = sqlx::query(
        "INSERT INTO role_permissions (role_id, permission_id) 
         SELECT $1, id FROM permissions 
         ON CONFLICT DO NOTHING"
    )
    .bind(role_id)
    .execute(pool)
    .await?;

    tracing::info!(
        role = role_name, 
        permissions_assigned = result.rows_affected(),
        "Assigned all permissions to role"
    );
    
    Ok(())
}

/// Assign permissions matching patterns to a role
async fn assign_role_permissions(pool: &PgPool, role_name: &str, permission_patterns: &[&str]) -> Result<(), sqlx::Error> {
    let role = sqlx::query_scalar::<_, i64>("SELECT id FROM roles WHERE name = $1")
        .bind(role_name)
        .fetch_optional(pool)
        .await?;
    
    let role_id = match role {
        Some(id) => id,
        None => {
            tracing::warn!(role = role_name, "Role not found, skipping permission assignment");
            return Ok(());
        }
    };

    // Delete existing permissions for this role
    sqlx::query("DELETE FROM role_permissions WHERE role_id = $1")
        .bind(role_id)
        .execute(pool)
        .await?;

    // For each pattern, assign matching permissions
    let mut total_assigned: i64 = 0;
    
    for pattern in permission_patterns {
        if pattern.ends_with(".*") {
            // Wildcard pattern: match module prefix
            let module_prefix = pattern.trim_end_matches(".*");
            let result = sqlx::query(
                "INSERT INTO role_permissions (role_id, permission_id) 
                 SELECT $1, id FROM permissions WHERE name LIKE $2 
                 ON CONFLICT DO NOTHING"
            )
            .bind(role_id)
            .bind(format!("{}%", module_prefix))
            .execute(pool)
            .await?;
            total_assigned += result.rows_affected() as i64;
        } else {
            // Exact permission name
            let result = sqlx::query(
                "INSERT INTO role_permissions (role_id, permission_id) 
                 SELECT $1, id FROM permissions WHERE name = $2 
                 ON CONFLICT DO NOTHING"
            )
            .bind(role_id)
            .bind(pattern)
            .execute(pool)
            .await?;
            total_assigned += result.rows_affected() as i64;
        }
    }

    tracing::debug!(
        role = role_name, 
        permissions_assigned = total_assigned,
        "Assigned permissions to role"
    );
    
    Ok(())
}
