use sea_orm::{DatabaseConnection, EntityTrait, ModelTrait, QueryFilter, ColumnTrait, ActiveModelTrait, Set};
use argon2::{Argon2, PasswordHasher, PasswordHash, PasswordVerifier};
use jsonwebtoken::{encode, Header, EncodingKey};
use rand::Rng;
use sha2::{Digest, Sha256};
use chrono::{Utc, Duration};
use redis::AsyncCommands;

use tracing::{info, debug};

use crate::shared::errors::AppError;
use crate::modules::identity::domain::entities::{user, user_session};
use crate::modules::security::domain::entities::{role, user_role, role_permission, permission as perm_entity};

/// Redis key prefix for user permissions
const REDIS_PERMS_PREFIX: &str = "aeroxe:user:";

pub struct IdentityService;

impl IdentityService {
    pub fn hash_password(password: &str) -> Result<String, AppError> {
        let salt = argon2::password_hash::SaltString::generate(&mut rand::thread_rng());
        let hash = Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Password hashing failed: {}", e)))?
            .to_string();
        Ok(hash)
    }

    pub fn verify_password(password: &str, hash: &str) -> bool {
        let parsed = PasswordHash::new(hash).ok();
        if let Some(parsed) = parsed {
            Argon2::default().verify_password(password.as_bytes(), &parsed).is_ok()
        } else {
            false
        }
    }

    pub async fn register(
        db: &DatabaseConnection, email: String, phone: String, name: String,
        password: String, branch_id: Option<i64>,
    ) -> Result<user::Model, AppError> {
        let existing = user::Entity::find()
            .filter(user::Column::Email.eq(&email))
            .one(db).await?;
        if existing.is_some() {
            return Err(AppError::Conflict("Email already registered".to_string()));
        }
        let existing = user::Entity::find()
            .filter(user::Column::Phone.eq(&phone))
            .one(db).await?;
        if existing.is_some() {
            return Err(AppError::Conflict("Phone already registered".to_string()));
        }

        let password_hash = Self::hash_password(&password)?;
        let now = Utc::now();

        let new_user = user::ActiveModel {
            email: Set(email), phone: Set(phone), password_hash: Set(Some(password_hash)),
            name: Set(name), branch_id: Set(branch_id), status: Set("active".to_string()),
            failed_login_attempts: Set(0), two_factor_enabled: Set(false),
            phone_verified: Set(false), email_verified: Set(false),
            created_at: Set(now), updated_at: Set(now), ..Default::default()
        };
        let result = new_user.insert(db).await?;
        Ok(result)
    }

    /// Look up the user's role name, branch_id, and permissions from the security module.
    async fn load_user_context(db: &DatabaseConnection, user_model: &user::Model) -> Result<(String, Option<i64>, bool, Vec<String>), AppError> {
        use std::collections::BTreeSet;

        // Find active user roles
        let user_roles = user_role::Entity::find()
            .filter(user_role::Column::UserId.eq(user_model.id))
            .filter(user_role::Column::IsActive.eq(true))
            .all(db).await?;

        let mut role_name = String::new();
        let mut is_company_wide = false;
        let mut role_ids: Vec<i64> = Vec::new();

        for ur in &user_roles {
            role_ids.push(ur.role_id);
            if let Ok(Some(role_model)) = role::Entity::find_by_id(ur.role_id).filter(role::Column::IsActive.eq(true)).one(db).await {
                if role_name.is_empty() {
                    role_name = role_model.name.clone();
                }
                if role_model.is_company_wide {
                    is_company_wide = true;
                }
            }
        }

        // Gather permissions through role_permissions filtered by user's role_ids
        let mut permission_set: BTreeSet<String> = BTreeSet::new();
        if !role_ids.is_empty() {
            let rps = role_permission::Entity::find()
                .filter(role_permission::Column::RoleId.is_in(role_ids.clone()))
                .all(db).await?;
            let perm_ids: Vec<i64> = rps.iter().map(|rp| rp.permission_id).collect();
            if !perm_ids.is_empty() {
                let perms = perm_entity::Entity::find()
                    .filter(perm_entity::Column::Id.is_in(perm_ids))
                    .all(db).await?;
                for p in perms {
                    permission_set.insert(p.name);
                }
            }
        }

        Ok((role_name, user_model.branch_id, is_company_wide, permission_set.into_iter().collect()))
    }

    /// Redis key for a user's permissions
    fn redis_perms_key(user_id: i64) -> String {
        format!("{}{}:perms", REDIS_PERMS_PREFIX, user_id)
    }

    /// Store user permissions in Redis with TTL
    async fn store_permissions_in_redis(
        redis: &mut redis::aio::ConnectionManager,
        user_id: i64,
        permissions: &[String],
        ttl_secs: i64,
    ) -> Result<(), AppError> {
        let key = Self::redis_perms_key(user_id);
        let payload = serde_json::to_string(permissions)
            .map_err(|e| AppError::Internal(anyhow::anyhow!("JSON serialization error: {}", e)))?;
        let _: () = redis.set_ex(&key, payload, ttl_secs as u64)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Redis set error: {}", e)))?;
        info!(user_id = user_id, perms_count = permissions.len(), ttl_secs = ttl_secs, "Stored permissions in Redis");
        Ok(())
    }

    /// Retrieve user permissions from Redis
    pub(crate) async fn get_permissions_from_redis(
        redis: &mut redis::aio::ConnectionManager,
        user_id: i64,
    ) -> Result<Vec<String>, AppError> {
        let key = Self::redis_perms_key(user_id);
        let result: Option<String> = redis.get(&key)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Redis get error: {}", e)))?;
        match result {
            Some(json) => {
                let perms: Vec<String> = serde_json::from_str(&json)
                    .map_err(|e| AppError::Internal(anyhow::anyhow!("JSON deserialization error: {}", e)))?;
                debug!(user_id = user_id, perms_count = perms.len(), "Fetched permissions from Redis");
                Ok(perms)
            }
            None => {
                debug!(user_id = user_id, "No permissions found in Redis (cache miss)");
                Ok(Vec::new())
            }
        }
    }

    /// Invalidate user permissions in Redis (e.g., on role change)
    pub(crate) async fn invalidate_permissions(
        redis: &mut redis::aio::ConnectionManager,
        user_id: i64,
    ) -> Result<(), AppError> {
        let key = Self::redis_perms_key(user_id);
        let _: () = redis.del(&key)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Redis del error: {}", e)))?;
        info!(user_id = user_id, "Invalidated permissions in Redis (role change)");
        Ok(())
    }

    /// Get remaining TTL (in seconds) for a Redis key. Returns 0 if key doesn't exist.
    async fn get_redis_ttl(
        redis: &mut redis::aio::ConnectionManager,
        key: &str,
    ) -> Result<i64, AppError> {
        let ttl: i64 = redis.ttl(key)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Redis ttl error: {}", e)))?;
        Ok(ttl)
    }

    /// Refresh access token using a valid refresh token.
    /// Checks if Redis permissions key is about to expire and re-stores if needed.
    /// If the remaining TTL is below 60% of the refresh token TTL, permissions are
    /// re-stored with the full TTL to prevent expiry between refresh cycles.
    pub async fn refresh_token(
        db: &DatabaseConnection,
        redis: &mut redis::aio::ConnectionManager,
        settings: &crate::config::settings::Settings,
        refresh_token: &str,
    ) -> Result<(String, String, user::Model), AppError> {
        let token_hash = Self::hash_token(refresh_token);

        // Find the session by refresh token hash
        let session = user_session::Entity::find()
            .filter(user_session::Column::RefreshTokenHash.eq(&token_hash))
            .one(db).await?
            .ok_or_else(|| AppError::Unauthorized)?;

        // Check if session is expired
        if Utc::now() > session.expires_at {
            // Delete expired session
            session.delete(db).await?;
            return Err(AppError::Unauthorized);
        }

        // Load the user
        let user_model = user::Entity::find_by_id(session.user_id)
            .one(db).await?
            .ok_or_else(|| AppError::NotFound(format!("User {} not found", session.user_id)))?;

        if user_model.status != "active" {
            session.delete(db).await?;
            return Err(AppError::Unauthorized);
        }

        let (role, branch_id, is_company_wide, permissions) = Self::load_user_context(db, &user_model).await?;

        // Check if Redis permissions key is about to expire.
        // If TTL is below 60% of max (refresh_token_ttl), re-store with full TTL.
        let perms_key = Self::redis_perms_key(user_model.id);
        let remaining_ttl = Self::get_redis_ttl(redis, &perms_key).await?;
        let threshold = (settings.jwt_refresh_token_ttl_secs * 60) / 100; // 60% of max TTL

        if remaining_ttl < threshold {
            info!(
                user_id = user_model.id,
                remaining_ttl = remaining_ttl,
                threshold = threshold,
                "Permissions TTL below threshold during refresh, re-storing"
            );
            Self::store_permissions_in_redis(
                redis, user_model.id, &permissions, settings.jwt_refresh_token_ttl_secs,
            ).await?;
        } else {
            debug!(
                user_id = user_model.id,
                remaining_ttl = remaining_ttl,
                threshold = threshold,
                "Permissions TTL sufficient during refresh, no re-store needed"
            );
        }

        // Delete old session (rotate refresh token)
        session.delete(db).await?;

        // Generate new tokens
        let access_token = Self::generate_access_token(&user_model, settings, &role, branch_id, is_company_wide)?;
        let new_refresh_token = Self::generate_refresh_token();
        let new_refresh_hash = Self::hash_token(&new_refresh_token);

        // Create new session
        let new_session = user_session::ActiveModel {
            user_id: Set(user_model.id),
            refresh_token_hash: Set(new_refresh_hash),
            expires_at: Set(Utc::now() + Duration::seconds(settings.jwt_refresh_token_ttl_secs)),
            created_at: Set(Utc::now()),
            ..Default::default()
        };
        new_session.insert(db).await?;

        Ok((access_token, new_refresh_token, user_model))
    }

    pub async fn login(
        db: &DatabaseConnection,
        redis: &mut redis::aio::ConnectionManager,
        settings: &crate::config::settings::Settings,
        email: &str, password: &str,
    ) -> Result<(String, String, user::Model), AppError> {
        let user_model = user::Entity::find()
            .filter(user::Column::Email.eq(email))
            .one(db).await?
            .ok_or_else(|| AppError::Unauthorized)?;

        if user_model.status == "locked" {
            if let Some(locked_until) = user_model.locked_until {
                if Utc::now() < locked_until {
                    return Err(AppError::AccountLocked);
                }
            }
        }

        if user_model.status != "active" {
            return Err(AppError::Unauthorized);
        }

        let password_hash = user_model.password_hash.as_deref().ok_or_else(|| AppError::Unauthorized)?;
        if !Self::verify_password(password, password_hash) {
            let mut active: user::ActiveModel = user_model.clone().into();
            let new_attempts = user_model.failed_login_attempts + 1;
            active.failed_login_attempts = Set(new_attempts);
            if new_attempts >= 5 {
                active.status = Set("locked".to_string());
                active.locked_until = Set(Some(Utc::now() + Duration::minutes(30)));
            }
            active.updated_at = Set(Utc::now());
            active.update(db).await?;
            return Err(AppError::Unauthorized);
        }

        let mut active: user::ActiveModel = user_model.clone().into();
        active.failed_login_attempts = Set(0);
        active.locked_until = Set(None);
        active.last_login_at = Set(Some(Utc::now()));
        active.updated_at = Set(Utc::now());
        let updated_user = active.update(db).await?;

        let (role, branch_id, is_company_wide, permissions) = Self::load_user_context(db, &updated_user).await?;

        // Store permissions in Redis (not in JWT) - reduces token size and prevents leak
        // Use refresh token TTL so permissions survive token refresh cycles
        Self::store_permissions_in_redis(redis, updated_user.id, &permissions, settings.jwt_refresh_token_ttl_secs).await?;

        let access_token = Self::generate_access_token(&updated_user, settings, &role, branch_id, is_company_wide)?;
        let refresh_token = Self::generate_refresh_token();
        let refresh_token_hash = Self::hash_token(&refresh_token);

        let session = user_session::ActiveModel {
            user_id: Set(updated_user.id),
            refresh_token_hash: Set(refresh_token_hash),
            expires_at: Set(Utc::now() + Duration::seconds(settings.jwt_refresh_token_ttl_secs)),
            created_at: Set(Utc::now()),
            ..Default::default()
        };
        session.insert(db).await?;

        Ok((access_token, refresh_token, updated_user))
    }

    /// Generate JWT with identity claims only (no permissions - stored in Redis)
    fn generate_access_token(
        user: &user::Model,
        settings: &crate::config::settings::Settings,
        role: &str,
        branch_id: Option<i64>,
        is_company_wide: bool,
    ) -> Result<String, AppError> {
        let claims = serde_json::json!({
            "sub": user.id.to_string(),
            "email": user.email,
            "name": user.name,
            "role": role,
            "branch_id": branch_id,
            "is_company_wide": is_company_wide,
            "iat": Utc::now().timestamp(),
            "exp": (Utc::now() + Duration::seconds(settings.jwt_access_token_ttl_secs)).timestamp(),
        });
        let token = encode(
            &Header::new(jsonwebtoken::Algorithm::HS256),
            &claims,
            &EncodingKey::from_secret(settings.jwt_secret.as_bytes()),
        ).map_err(|e| AppError::Internal(anyhow::anyhow!("JWT encoding error: {}", e)))?;
        Ok(token)
    }

    fn generate_refresh_token() -> String {
        let mut rng = rand::thread_rng();
        (0..64).map(|_| {
            let idx = rng.gen_range(0..62);
            b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789"[idx] as char
        }).collect()
    }

    fn hash_token(token: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(token.as_bytes());
        hex::encode(hasher.finalize())
    }

    pub async fn get_user(db: &DatabaseConnection, id: i64) -> Result<user::Model, AppError> {
        user::Entity::find_by_id(id).one(db).await?.ok_or_else(|| AppError::NotFound(format!("User {} not found", id)))
    }

    pub async fn list_users(db: &DatabaseConnection) -> Result<Vec<user::Model>, AppError> {
        let users = user::Entity::find()
            .filter(user::Column::DeletedAt.is_null())
            .all(db).await?;
        Ok(users)
    }
}
