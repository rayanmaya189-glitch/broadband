use sqlx::PgPool;

use crate::common::errors::app_error::AppError;
use crate::common::utils::helpers::{total_pages, PaginatedResponse};
use crate::modules::user::model::user::{RefreshToken, User};
use crate::modules::user::response::user_response::UserResponse;

/// User repository — SQLx queries for user management.
pub struct UserRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> UserRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    pub async fn find_by_id(&self, user_id: i64) -> Result<Option<User>, AppError> {
        let user = sqlx::query_as::<_, User>(
            "SELECT id, email, password_hash, name, phone, avatar_url, \
             role_id, branch_id, is_company_wide, is_active, is_locked, \
             locked_until, failed_attempts, last_login_at, \
             two_factor_enabled, created_at, updated_at \
             FROM users WHERE id = $1",
        )
        .bind(user_id)
        .fetch_optional(self.pool)
        .await?;
        Ok(user)
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError> {
        let user = sqlx::query_as::<_, User>(
            "SELECT id, email, password_hash, name, phone, avatar_url, \
             role_id, branch_id, is_company_wide, is_active, is_locked, \
             locked_until, failed_attempts, last_login_at, \
             two_factor_enabled, created_at, updated_at \
             FROM users WHERE email = $1",
        )
        .bind(email)
        .fetch_optional(self.pool)
        .await?;
        Ok(user)
    }

    pub async fn find_by_phone(&self, phone: &str) -> Result<Option<User>, AppError> {
        let user = sqlx::query_as::<_, User>(
            "SELECT id, email, password_hash, name, phone, avatar_url, \
             role_id, branch_id, is_company_wide, is_active, is_locked, \
             locked_until, failed_attempts, last_login_at, \
             two_factor_enabled, created_at, updated_at \
             FROM users WHERE phone = $1",
        )
        .bind(phone)
        .fetch_optional(self.pool)
        .await?;
        Ok(user)
    }

    pub async fn create(
        &self, email: &str, password_hash: &str, name: &str, phone: &str,
        role_id: i64, branch_id: Option<i64>, is_company_wide: bool,
    ) -> Result<User, AppError> {
        let user = sqlx::query_as::<_, User>(
            "INSERT INTO users (email, password_hash, name, phone, role_id, branch_id, is_company_wide) \
             VALUES ($1, $2, $3, $4, $5, $6, $7) \
             RETURNING id, email, password_hash, name, phone, avatar_url, \
             role_id, branch_id, is_company_wide, is_active, is_locked, \
             locked_until, failed_attempts, last_login_at, \
             two_factor_enabled, created_at, updated_at",
        )
        .bind(email).bind(password_hash).bind(name).bind(phone)
        .bind(role_id).bind(branch_id).bind(is_company_wide)
        .fetch_one(self.pool).await?;
        Ok(user)
    }

    pub async fn update(
        &self, user_id: i64, name: Option<&str>, phone: Option<&str>,
        branch_id: Option<i64>, avatar_url: Option<&str>,
    ) -> Result<User, AppError> {
        let user = sqlx::query_as::<_, User>(
            "UPDATE users SET name = COALESCE($2, name), phone = COALESCE($3, phone), \
             branch_id = COALESCE($4, branch_id), avatar_url = COALESCE($5, avatar_url), \
             updated_at = NOW() WHERE id = $1 \
             RETURNING id, email, password_hash, name, phone, avatar_url, \
             role_id, branch_id, is_company_wide, is_active, is_locked, \
             locked_until, failed_attempts, last_login_at, \
             two_factor_enabled, created_at, updated_at",
        )
        .bind(user_id).bind(name).bind(phone).bind(branch_id).bind(avatar_url)
        .fetch_one(self.pool).await?;
        Ok(user)
    }

    pub async fn soft_delete(&self, user_id: i64) -> Result<(), AppError> {
        sqlx::query("UPDATE users SET is_active = false, updated_at = NOW() WHERE id = $1")
            .bind(user_id).execute(self.pool).await?;
        Ok(())
    }

    pub async fn update_status(&self, user_id: i64, is_active: bool) -> Result<User, AppError> {
        let user = sqlx::query_as::<_, User>(
            "UPDATE users SET is_active = $2, updated_at = NOW() WHERE id = $1 \
             RETURNING id, email, password_hash, name, phone, avatar_url, \
             role_id, branch_id, is_company_wide, is_active, is_locked, \
             locked_until, failed_attempts, last_login_at, \
             two_factor_enabled, created_at, updated_at",
        )
        .bind(user_id).bind(is_active).fetch_one(self.pool).await?;
        Ok(user)
    }

    pub async fn update_last_login(&self, user_id: i64) -> Result<(), AppError> {
        sqlx::query("UPDATE users SET last_login_at = NOW(), failed_attempts = 0, updated_at = NOW() WHERE id = $1")
            .bind(user_id).execute(self.pool).await?;
        Ok(())
    }

    pub async fn increment_failed_attempts(&self, user_id: i64) -> Result<i32, AppError> {
        let result = sqlx::query_scalar::<_, i32>(
            "UPDATE users SET failed_attempts = failed_attempts + 1, updated_at = NOW() \
             WHERE id = $1 RETURNING failed_attempts",
        ).bind(user_id).fetch_one(self.pool).await?;
        if result >= 5 {
            sqlx::query("UPDATE users SET is_locked = true, locked_until = NOW() + INTERVAL '30 minutes', updated_at = NOW() WHERE id = $1")
                .bind(user_id).execute(self.pool).await?;
        }
        Ok(result)
    }

    pub async fn update_password(&self, user_id: i64, new_hash: &str) -> Result<(), AppError> {
        sqlx::query("UPDATE users SET password_hash = $2, updated_at = NOW() WHERE id = $1")
            .bind(user_id).bind(new_hash).execute(self.pool).await?;
        Ok(())
    }

    pub async fn list(
        &self, offset: u32, limit: u32, role_id: Option<i64>,
        branch_id: Option<i64>, is_active: Option<bool>, search: Option<&str>,
    ) -> Result<PaginatedResponse<UserResponse>, AppError> {
        let limit_i64 = limit.min(100) as i64;
        let offset_i64 = offset as i64;
        let mut conditions = Vec::new();
        let mut idx = 1;

        if role_id.is_some() { conditions.push(format!("u.role_id = ${idx}")); idx += 1; }
        if branch_id.is_some() { conditions.push(format!("u.branch_id = ${idx}")); idx += 1; }
        if is_active.is_some() { conditions.push(format!("u.is_active = ${idx}")); idx += 1; }
        if search.is_some() {
            conditions.push(format!("(u.name ILIKE ${idx} OR u.email ILIKE ${idx} OR u.phone ILIKE ${idx})"));
            idx += 1;
        }

        let wc = if conditions.is_empty() { String::new() } else { format!("WHERE {}", conditions.join(" AND ")) };

        let count_sql = format!("SELECT COUNT(*) FROM users u {wc}");
        let mut cq = sqlx::query_scalar::<_, i64>(&count_sql);
        if let Some(v) = role_id { cq = cq.bind(v); }
        if let Some(v) = branch_id { cq = cq.bind(v); }
        if let Some(v) = is_active { cq = cq.bind(v); }
        if let Some(v) = search { cq = cq.bind(format!("%{v}%")); }
        let total = cq.fetch_one(self.pool).await?;

        let lp = idx;
        let op = idx + 1;
        let data_sql = format!(
            "SELECT u.id, u.email, u.name, u.phone, u.avatar_url, u.role_id, \
             r.name as role_name, u.branch_id, u.is_company_wide, u.is_active, \
             u.is_locked, u.two_factor_enabled, u.last_login_at, u.created_at, u.updated_at \
             FROM users u LEFT JOIN roles r ON r.id = u.role_id {wc} \
             ORDER BY u.created_at DESC LIMIT ${lp} OFFSET ${op}"
        );
        let mut dq = sqlx::query_as::<_, UserResponse>(&data_sql);
        if let Some(v) = role_id { dq = dq.bind(v); }
        if let Some(v) = branch_id { dq = dq.bind(v); }
        if let Some(v) = is_active { dq = dq.bind(v); }
        if let Some(v) = search { dq = dq.bind(format!("%{v}%")); }
        dq = dq.bind(limit_i64).bind(offset_i64);

        let users = dq.fetch_all(self.pool).await?;
        let total_pages = total_pages(total, limit);
        Ok(PaginatedResponse { data: users, total, page: (offset / limit) + 1, limit, total_pages })
    }

    pub async fn email_exists(&self, email: &str, exclude: Option<i64>) -> Result<bool, AppError> {
        let r = if let Some(id) = exclude {
            sqlx::query_scalar::<_, bool>("SELECT EXISTS(SELECT 1 FROM users WHERE email = $1 AND id != $2)")
                .bind(email).bind(id).fetch_one(self.pool).await?
        } else {
            sqlx::query_scalar::<_, bool>("SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)")
                .bind(email).fetch_one(self.pool).await?
        };
        Ok(r)
    }

    pub async fn phone_exists(&self, phone: &str, exclude: Option<i64>) -> Result<bool, AppError> {
        let r = if let Some(id) = exclude {
            sqlx::query_scalar::<_, bool>("SELECT EXISTS(SELECT 1 FROM users WHERE phone = $1 AND id != $2)")
                .bind(phone).bind(id).fetch_one(self.pool).await?
        } else {
            sqlx::query_scalar::<_, bool>("SELECT EXISTS(SELECT 1 FROM users WHERE phone = $1)")
                .bind(phone).fetch_one(self.pool).await?
        };
        Ok(r)
    }

    pub async fn role_exists(&self, role_id: i64) -> Result<bool, AppError> {
        let r = sqlx::query_scalar::<_, bool>("SELECT EXISTS(SELECT 1 FROM roles WHERE id = $1)")
            .bind(role_id).fetch_one(self.pool).await?;
        Ok(r)
    }

    pub async fn get_role_name(&self, role_id: i64) -> Result<Option<String>, AppError> {
        let n = sqlx::query_scalar::<_, String>("SELECT name FROM roles WHERE id = $1")
            .bind(role_id).fetch_optional(self.pool).await?;
        Ok(n)
    }

    pub async fn resolve_permissions(&self, role_id: i64) -> Result<Vec<String>, AppError> {
        let rows = sqlx::query_scalar::<_, String>(
            "SELECT DISTINCT p.name FROM permissions p \
             INNER JOIN role_permissions rp ON rp.permission_id = p.id \
             WHERE rp.role_id = $1 ORDER BY p.name",
        ).bind(role_id).fetch_all(self.pool).await?;
        Ok(rows)
    }

    // ── Refresh Token queries ────────────────────────────────

    pub async fn create_refresh_token(
        &self, user_id: i64, token_hash: &str, device_info: Option<&str>,
        ip_address: Option<&str>, expires_at: chrono::DateTime<chrono::Utc>,
    ) -> Result<RefreshToken, AppError> {
        let row = sqlx::query_as::<_, RefreshToken>(
            "INSERT INTO refresh_tokens (user_id, token_hash, device_info, ip_address, expires_at) \
             VALUES ($1, $2, $3, $4::inet, $5) \
             RETURNING id, user_id, token_hash, device_info, ip_address::text, expires_at, created_at, revoked_at",
        )
        .bind(user_id).bind(token_hash).bind(device_info).bind(ip_address).bind(expires_at)
        .fetch_one(self.pool).await?;
        Ok(row)
    }

    pub async fn find_valid_refresh_token(&self, token_hash: &str) -> Result<Option<RefreshToken>, AppError> {
        let row = sqlx::query_as::<_, RefreshToken>(
            "SELECT id, user_id, token_hash, device_info, ip_address::text, expires_at, created_at, revoked_at \
             FROM refresh_tokens WHERE token_hash = $1 AND revoked_at IS NULL AND expires_at > NOW()",
        ).bind(token_hash).fetch_optional(self.pool).await?;
        Ok(row)
    }

    pub async fn revoke_refresh_token(&self, token_hash: &str) -> Result<(), AppError> {
        sqlx::query("UPDATE refresh_tokens SET revoked_at = NOW() WHERE token_hash = $1 AND revoked_at IS NULL")
            .bind(token_hash).execute(self.pool).await?;
        Ok(())
    }

    pub async fn revoke_all_user_tokens(&self, user_id: i64) -> Result<u64, AppError> {
        let r = sqlx::query("UPDATE refresh_tokens SET revoked_at = NOW() WHERE user_id = $1 AND revoked_at IS NULL")
            .bind(user_id).execute(self.pool).await?;
        Ok(r.rows_affected())
    }

    pub async fn count_active_tokens(&self, user_id: i64) -> Result<i64, AppError> {
        let c = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM refresh_tokens WHERE user_id = $1 AND revoked_at IS NULL AND expires_at > NOW()",
        ).bind(user_id).fetch_one(self.pool).await?;
        Ok(c)
    }

    pub async fn revoke_oldest_token(&self, user_id: i64) -> Result<(), AppError> {
        sqlx::query(
            "UPDATE refresh_tokens SET revoked_at = NOW() WHERE id = ( \
             SELECT id FROM refresh_tokens WHERE user_id = $1 AND revoked_at IS NULL \
             ORDER BY created_at ASC LIMIT 1)",
        ).bind(user_id).execute(self.pool).await?;
        Ok(())
    }

    pub async fn list_active_sessions(&self, user_id: i64) -> Result<Vec<RefreshToken>, AppError> {
        let rows = sqlx::query_as::<_, RefreshToken>(
            "SELECT id, user_id, token_hash, device_info, ip_address::text, expires_at, created_at, revoked_at \
             FROM refresh_tokens WHERE user_id = $1 AND revoked_at IS NULL AND expires_at > NOW() \
             ORDER BY created_at DESC",
        ).bind(user_id).fetch_all(self.pool).await?;
        Ok(rows)
    }
}
