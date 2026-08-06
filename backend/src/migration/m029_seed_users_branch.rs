use sea_orm_migration::prelude::*;

/// Seed development users and the "Aeroxe Broadband Jalgaon Branch" branch.
///
/// Creates one super admin, one tenant owner (ISP Owner scoped to the Jalgaon
/// branch), and one staff user per remaining system role — all with
/// `@aeroxe.com` emails and the `SEED_ADMIN_PASSWORD` password (fallback
/// `Aeroxe@123`). Users have 2FA disabled so they can log straight into the
/// admin portal with the default password.
#[derive(DeriveMigrationName)]
pub struct Migration;

const DEFAULT_PASSWORD: &str = "Aeroxe@123";

const BRANCH_SLUG: &str = "jalgaon-aeroxe";

/// (email, role slug, display name, branch-scoped)
const SEED_USERS: [(&str, &str, &str, bool); 10] = [
    (
        "super_admin@aeroxe.com",
        "super_admin",
        "Super Admin",
        false,
    ),
    (
        "isp_owner@aeroxe.com",
        "isp_owner",
        "Aeroxe Broadband Jalgaon Branch",
        true,
    ),
    (
        "network_admin@aeroxe.com",
        "network_admin",
        "Network Admin",
        true,
    ),
    (
        "noc_engineer@aeroxe.com",
        "noc_engineer",
        "NOC Engineer",
        true,
    ),
    (
        "field_technician@aeroxe.com",
        "field_technician",
        "Field Technician",
        true,
    ),
    (
        "customer_support@aeroxe.com",
        "customer_support",
        "Customer Support",
        true,
    ),
    ("sales_agent@aeroxe.com", "sales_agent", "Sales Agent", true),
    (
        "finance_manager@aeroxe.com",
        "finance_manager",
        "Finance Manager",
        true,
    ),
    (
        "billing_operator@aeroxe.com",
        "billing_operator",
        "Billing Operator",
        true,
    ),
    ("customer@aeroxe.com", "customer", "Customer", true),
];

/// Distinct 10-digit mobile numbers (unique column), stable across runs.
const PHONES: [&str; 10] = [
    "+919000000001",
    "+919000000002",
    "+919000000003",
    "+919000000004",
    "+919000000005",
    "+919000000006",
    "+919000000007",
    "+919000000008",
    "+919000000009",
    "+919000000010",
];

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let password = std::env::var("SEED_ADMIN_PASSWORD").unwrap_or_else(|_| {
            tracing::warn!(
                "SEED_ADMIN_PASSWORD not set; seeding users with default '{}'",
                DEFAULT_PASSWORD
            );
            DEFAULT_PASSWORD.to_string()
        });
        let hash = crate::modules::identity::application::services::IdentityService::hash_password(
            &password,
        )
        .map_err(|e| DbErr::Custom(format!("failed to hash seed password: {e}")))?;

        // 1. Create the Jalgaon branch.
        crate::migration::exec_stmt_raw(
            manager,
            "INSERT INTO branches.branches
                (name, slug, code, city, state, address, phone, email, timezone, is_active)
             VALUES
                ('Aeroxe Broadband Jalgaon Branch', 'jalgaon-aeroxe', 'JAL-AER',
                 'Jalgaon', 'Maharashtra', 'Plot 7, MIDC Area, Jalgaon, Maharashtra 425001',
                 '+912572230101', 'jalgaon@aeroxe.com', 'Asia/Kolkata', TRUE)
             ON CONFLICT (slug) DO NOTHING",
        )
        .await?;

        let branch_id_sql =
            format!("(SELECT id FROM branches.branches WHERE slug = '{BRANCH_SLUG}')");

        // 2. Insert users (super admin is company-wide, others scoped to the branch).
        for (i, (email, _role, name, branch_scoped)) in SEED_USERS.iter().enumerate() {
            let branch_expr = if *branch_scoped {
                branch_id_sql.as_str()
            } else {
                "NULL"
            };
            let sql = format!(
                "INSERT INTO identity.users
                    (email, phone, password_hash, name, branch_id, status,
                     failed_login_attempts, two_factor_enabled, phone_verified,
                     email_verified, created_at, updated_at)
                 VALUES
                    ('{}', '{}', '{}', '{}', {}, 'active', 0, FALSE, TRUE, TRUE, NOW(), NOW())
                 ON CONFLICT (email) DO NOTHING",
                email,
                PHONES[i],
                hash,
                name.replace('\'', "''"),
                branch_expr
            );
            crate::migration::exec_stmt_raw(manager, &sql).await?;
        }

        // 3. Link each user to its role, assigned by the super admin.
        let super_admin_id =
            "(SELECT id FROM identity.users WHERE email = 'super_admin@aeroxe.com')";
        for (email, role, _name, _branch_scoped) in SEED_USERS.iter() {
            let sql = format!(
                "INSERT INTO security.user_roles (user_id, role_id, assigned_by, is_active, created_at)
                 SELECT u.id, r.id, {super_admin_id}, TRUE, NOW()
                   FROM identity.users u, security.roles r
                  WHERE u.email = '{email}' AND r.slug = '{role}'
                 ON CONFLICT (user_id, role_id) DO NOTHING"
            );
            crate::migration::exec_stmt_raw(manager, &sql).await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let emails = SEED_USERS
            .iter()
            .map(|(email, _, _, _)| format!("'{email}'"))
            .collect::<Vec<_>>()
            .join(", ");

        crate::migration::exec_stmt_raw(
            manager,
            &format!(
                "DELETE FROM security.user_roles
                  WHERE user_id IN (SELECT id FROM identity.users WHERE email IN ({emails}))"
            ),
        )
        .await?;
        crate::migration::exec_stmt_raw(
            manager,
            &format!("DELETE FROM identity.users WHERE email IN ({emails})"),
        )
        .await?;
        crate::migration::exec_stmt_raw(
            manager,
            &format!("DELETE FROM branches.branches WHERE slug = '{BRANCH_SLUG}'"),
        )
        .await?;
        Ok(())
    }
}
