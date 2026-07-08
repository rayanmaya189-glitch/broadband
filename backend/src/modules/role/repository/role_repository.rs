use sqlx::PgPool;

use crate::common::errors::app_error::AppError;
use crate::common::utils::helpers::{total_pages, PaginatedResponse};
use crate::modules::role::model::role::Role;
use crate::modules::role::response::role_response::RoleResponse;

pub struct RoleRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> RoleRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self { Self { pool } }

    pub async fn find_by_id(&self, role_id: i64) -> Result<Option<Role>, AppError> {
        let r = sqlx::query_as::<_, Role>(
            "SELECT id, name, display_name, description, is_system, is_active, created_at, updated_at FROM roles WHERE id = $1",
        ).bind(role_id).fetch_optional(self.pool).await?;
        Ok(r)
    }

    pub async fn find_by_name(&self, name: &str) -> Result<Option<Role>, AppError> {
        let r = sqlx::query_as::<_, Role>(
            "SELECT id, name, display_name, description, is_system, is_active, created_at, updated_at FROM roles WHERE name = $1",
        ).bind(name).fetch_optional(self.pool).await?;
        Ok(r)
    }

    pub async fn list(&self, offset: u32, limit: u32, is_active: Option<bool>) -> Result<PaginatedResponse<RoleResponse>, AppError> {
        let limit_i64 = limit.min(100) as i64;
        let offset_i64 = offset as i64;
        let wc = if is_active.is_some() { "WHERE is_active = $1" } else { "" };

        let count_sql = format!("SELECT COUNT(*) FROM roles {wc}");
        let total = if let Some(v) = is_active {
            sqlx::query_scalar::<_, i64>(&count_sql).bind(v).fetch_one(self.pool).await?
        } else {
            sqlx::query_scalar::<_, i64>(&count_sql).fetch_one(self.pool).await?
        };

        let data_sql = format!(
            "SELECT id, name, display_name, description, is_system, is_active, created_at, updated_at FROM roles {wc} ORDER BY name LIMIT ${} OFFSET ${}",
            if is_active.is_some() { 2 } else { 1 },
            if is_active.is_some() { 3 } else { 2 },
        );
        let mut q = sqlx::query_as::<_, RoleResponse>(&data_sql);
        if let Some(v) = is_active { q = q.bind(v); }
        q = q.bind(limit_i64).bind(offset_i64);
        let roles = q.fetch_all(self.pool).await?;
        let tp = total_pages(total, limit);
        Ok(PaginatedResponse { data: roles, total, page: (offset / limit) + 1, limit, total_pages: tp })
    }

    pub async fn create(&self, name: &str, display_name: &str, description: Option<&str>) -> Result<Role, AppError> {
        let r = sqlx::query_as::<_, Role>(
            "INSERT INTO roles (name, display_name, description) VALUES ($1, $2, $3) RETURNING id, name, display_name, description, is_system, is_active, created_at, updated_at",
        ).bind(name).bind(display_name).bind(description).fetch_one(self.pool).await?;
        Ok(r)
    }

    pub async fn update(&self, role_id: i64, name: Option<&str>, display_name: Option<&str>, description: Option<&str>) -> Result<Role, AppError> {
        let r = sqlx::query_as::<_, Role>(
            "UPDATE roles SET name = COALESCE($2, name), display_name = COALESCE($3, display_name), description = COALESCE($4, description), updated_at = NOW() WHERE id = $1 RETURNING id, name, display_name, description, is_system, is_active, created_at, updated_at",
        ).bind(role_id).bind(name).bind(display_name).bind(description).fetch_one(self.pool).await?;
        Ok(r)
    }

    pub async fn deactivate(&self, role_id: i64) -> Result<(), AppError> {
        sqlx::query("UPDATE roles SET is_active = false, updated_at = NOW() WHERE id = $1 AND is_system = false").bind(role_id).execute(self.pool).await?;
        Ok(())
    }

    pub async fn name_exists(&self, name: &str, exclude: Option<i64>) -> Result<bool, AppError> {
        let r = if let Some(id) = exclude {
            sqlx::query_scalar::<_, bool>("SELECT EXISTS(SELECT 1 FROM roles WHERE name = $1 AND id != $2)").bind(name).bind(id).fetch_one(self.pool).await?
        } else {
            sqlx::query_scalar::<_, bool>("SELECT EXISTS(SELECT 1 FROM roles WHERE name = $1)").bind(name).fetch_one(self.pool).await?
        };
        Ok(r)
    }
}
