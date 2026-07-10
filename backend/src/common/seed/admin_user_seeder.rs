use sqlx::PgPool;

use crate::common::config::config::Config;
use crate::common::security::crypto::hash_password;

/// Seed the superadmin user on server startup.
/// Reads credentials from environment variables SUPERADMIN_EMAIL and SUPERADMIN_PASSWORD,
/// falling back to defaults (admin/Admin@123) if not set.
pub async fn seed_admin_user(pool: &PgPool) -> Result<(), sqlx::Error> {
    let config = Config::get();
    
    let email = config.superadmin_email.as_deref().unwrap_or("admin");
    let password = config.superadmin_password.as_deref().unwrap_or("Admin@123");

    // Check if the superadmin user already exists
    let existing_user = sqlx::query_scalar::<_, i64>(
        "SELECT id FROM users WHERE email = $1"
    )
    .bind(email)
    .fetch_optional(pool)
    .await?;

    if existing_user.is_some() {
        tracing::debug!(email = email, "Superadmin user already exists, skipping creation");
        return Ok(());
    }

    // Get the super_admin role ID
    let role_id = sqlx::query_scalar::<_, i64>(
        "SELECT id FROM roles WHERE name = 'super_admin'"
    )
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| {
        tracing::error!("super_admin role not found in database");
        sqlx::Error::RowNotFound
    })?;

    // Hash the password
    let password_hash = match hash_password(password) {
        Ok(h) => h,
        Err(e) => {
            tracing::error!(error = %e, "Failed to hash superadmin password");
            return Ok(());
        }
    };

    // Create the superadmin user
    let user = sqlx::query_as::<_, crate::modules::user::model::user::User>(
        "INSERT INTO users (email, password_hash, name, phone, role_id, is_company_wide, is_active) 
         VALUES ($1, $2, $3, $4, $5, true, true) 
         RETURNING id, email, password_hash, name, phone, avatar_url, 
         role_id, branch_id, is_company_wide, is_active, is_locked, 
         locked_until, failed_attempts, last_login_at, 
         two_factor_enabled, created_at, updated_at"
    )
    .bind(email)
    .bind(&password_hash)
    .bind("Super Admin")
    .bind("0000000000")
    .bind(role_id)
    .fetch_one(pool)
    .await?;

    tracing::info!(
        user_id = user.id,
        email = %user.email,
        "Superadmin user created successfully"
    );

    Ok(())
}
