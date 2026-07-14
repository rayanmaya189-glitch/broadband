use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait, Set, ActiveModelTrait};

use crate::common::config::config::Config;
use crate::common::security::crypto::hash_password;
use crate::modules::user::model::user_entity;
use crate::modules::role::model::role_entity;

/// Seed the superadmin user on server startup.
/// Reads credentials from environment variables SUPERADMIN_EMAIL and SUPERADMIN_PASSWORD,
/// falling back to defaults (admin/Admin@123) if not set.
pub async fn seed_admin_user(db: &DatabaseConnection) -> Result<(), sea_orm::DbErr> {
    let config = Config::get();
    
    let email = config.superadmin_email.as_deref().unwrap_or("admin");
    let password = config.superadmin_password.as_deref().unwrap_or("Admin@123");

    // Check if the superadmin user already exists
    let existing_user = user_entity::Entity::find()
        .filter(user_entity::Column::Email.eq(email))
        .one(db)
        .await?;

    if existing_user.is_some() {
        tracing::debug!(email = email, "Superadmin user already exists, skipping creation");
        return Ok(());
    }

    // Get the super_admin role ID
    let role = role_entity::Entity::find()
        .filter(role_entity::Column::Name.eq("super_admin"))
        .one(db)
        .await?;

    let role_id = match role {
        Some(r) => r.id,
        None => {
            tracing::error!("super_admin role not found in database");
            return Err(sea_orm::DbErr::Custom("super_admin role not found".to_string()));
        }
    };

    // Hash the password
    let password_hash = match hash_password(password) {
        Ok(h) => h,
        Err(e) => {
            tracing::error!(error = %e, "Failed to hash superadmin password");
            return Ok(());
        }
    };

    // Create the superadmin user
    let new_user = user_entity::ActiveModel {
        email: Set(email.to_string()),
        password_hash: Set(password_hash),
        name: Set("Super Admin".to_string()),                phone: Set(Some("0000000000".to_string())),
        role_id: Set(role_id),
        is_company_wide: Set(true),
        is_active: Set(true),
        ..Default::default()
    };

    let user = new_user.insert(db).await?;

    tracing::info!(
        user_id = user.id,
        email = %email,
        "Superadmin user created successfully"
    );

    Ok(())
}
