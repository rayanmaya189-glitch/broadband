use sqlx::PgPool;

use crate::common::errors::app_error::AppError;
use crate::common::utils::helpers::{total_pages, PaginatedResponse};
use crate::modules::permission::model::permission::Permission;
use crate::modules::permission::response::permission_response::PermissionResponse;

pub struct PermissionRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> PermissionRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self { Self { pool } }

    pub async fn find_by_id(&self, id: i64) -> Result<Option<Permission>, AppError> {
        let r = sqlx::query_as::<_, Permission>("SELECT id, name, method, api_url, guard, module, created_at FROM permissions WHERE id = $1").bind(id).fetch_optional(self.pool).await?;
        Ok(r)
    }

    pub async fn list(&self, offset: u32, limit: u32, module: Option<&str>) -> Result<PaginatedResponse<PermissionResponse>, AppError> {
        let limit_i64 = limit.min(100) as i64;
        let offset_i64 = offset as i64;
        let wc = if module.is_some() { "WHERE module = $1" } else { "" };

        let count_sql = format!("SELECT COUNT(*) FROM permissions {wc}");
        let total = if let Some(m) = module {
            sqlx::query_scalar::<_, i64>(&count_sql).bind(m).fetch_one(self.pool).await?
        } else {
            sqlx::query_scalar::<_, i64>(&count_sql).fetch_one(self.pool).await?
        };

        let data_sql = format!(
            "SELECT id, name, method, api_url, guard, module, created_at FROM permissions {wc} ORDER BY name LIMIT ${} OFFSET ${}",
            if module.is_some() { 2 } else { 1 },
            if module.is_some() { 3 } else { 2 },
        );
        let mut q = sqlx::query_as::<_, PermissionResponse>(&data_sql);
        if let Some(m) = module { q = q.bind(m); }
        q = q.bind(limit_i64).bind(offset_i64);
        let perms = q.fetch_all(self.pool).await?;
        let tp = total_pages(total, limit);
        Ok(PaginatedResponse { data: perms, total, page: (offset / limit) + 1, limit, total_pages: tp })
    }

    pub async fn create(&self, name: &str, method: &str, api_url: &str, guard: &str, module: &str) -> Result<Permission, AppError> {
        sqlx::query(
            "INSERT INTO permissions (name, method, api_url, guard, module) VALUES ($1, $2, $3, $4, $5) ON CONFLICT (name, method, api_url) DO NOTHING"
        ).bind(name).bind(method).bind(api_url).bind(guard).bind(module).execute(self.pool).await?;
        // Fetch the row (either newly inserted or existing)
        let perm = sqlx::query_as::<_, Permission>(
            "SELECT id, name, method, api_url, guard, module, created_at FROM permissions WHERE name = $1 AND method = $2 AND api_url = $3"
        ).bind(name).bind(method).bind(api_url).fetch_one(self.pool).await?;
        Ok(perm)
    }

    pub async fn delete(&self, id: i64) -> Result<(), AppError> {
        sqlx::query("DELETE FROM permissions WHERE id = $1").bind(id).execute(self.pool).await?;
        Ok(())
    }
}
