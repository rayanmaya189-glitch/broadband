use sqlx::PgPool;

use crate::common::errors::app_error::AppError;
use crate::common::utils::helpers::{total_pages, PaginatedResponse};
use crate::modules::branch::model::branch::Branch;
use crate::modules::branch::response::branch_response::BranchResponse;

pub struct BranchRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> BranchRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self { Self { pool } }

    pub async fn find_by_id(&self, id: i64) -> Result<Option<Branch>, AppError> {
        let r = sqlx::query_as::<_, Branch>(
            "SELECT id, name, code, address, city, state, pincode, phone, email, is_active, timezone, created_at, updated_at FROM branches WHERE id = $1",
        ).bind(id).fetch_optional(self.pool).await?;
        Ok(r)
    }

    pub async fn list(&self, offset: u32, limit: u32, is_active: Option<bool>, city: Option<&str>, search: Option<&str>) -> Result<PaginatedResponse<BranchResponse>, AppError> {
        let limit_i64 = limit.min(100) as i64;
        let offset_i64 = offset as i64;
        let mut conditions = Vec::new();
        let mut idx = 1;
        if is_active.is_some() { conditions.push(format!("is_active = ${idx}")); idx += 1; }
        if city.is_some() { conditions.push(format!("city = ${idx}")); idx += 1; }
        if search.is_some() { conditions.push(format!("(name ILIKE ${idx} OR code ILIKE ${idx} OR city ILIKE ${idx})")); idx += 1; }
        let wc = if conditions.is_empty() { String::new() } else { format!("WHERE {}", conditions.join(" AND ")) };

        let count_sql = format!("SELECT COUNT(*) FROM branches {wc}");
        let mut cq = sqlx::query_scalar::<_, i64>(&count_sql);
        if let Some(v) = is_active { cq = cq.bind(v); }
        if let Some(v) = city { cq = cq.bind(v); }
        if let Some(v) = search { cq = cq.bind(format!("%{v}%")); }
        let total = cq.fetch_one(self.pool).await?;

        let lp = idx;
        let op = idx + 1;
        let data_sql = format!("SELECT id, name, code, address, city, state, pincode, phone, email, is_active, timezone, created_at, updated_at FROM branches {wc} ORDER BY name ASC LIMIT ${lp} OFFSET ${op}");
        let mut dq = sqlx::query_as::<_, BranchResponse>(&data_sql);
        if let Some(v) = is_active { dq = dq.bind(v); }
        if let Some(v) = city { dq = dq.bind(v); }
        if let Some(v) = search { dq = dq.bind(format!("%{v}%")); }
        dq = dq.bind(limit_i64).bind(offset_i64);
        let branches = dq.fetch_all(self.pool).await?;
        let tp = total_pages(total, limit);
        Ok(PaginatedResponse { data: branches, total, page: (offset / limit) + 1, limit, total_pages: tp })
    }

    pub async fn create(&self, name: &str, code: &str, address: Option<&str>, city: Option<&str>, state: Option<&str>, pincode: Option<&str>, phone: Option<&str>, email: Option<&str>, timezone: &str) -> Result<Branch, AppError> {
        let r = sqlx::query_as::<_, Branch>(
            "INSERT INTO branches (name, code, address, city, state, pincode, phone, email, timezone) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) RETURNING id, name, code, address, city, state, pincode, phone, email, is_active, timezone, created_at, updated_at",
        ).bind(name).bind(code).bind(address).bind(city).bind(state).bind(pincode).bind(phone).bind(email).bind(timezone).fetch_one(self.pool).await?;
        Ok(r)
    }

    pub async fn update(&self, id: i64, name: Option<&str>, address: Option<&str>, city: Option<&str>, state: Option<&str>, pincode: Option<&str>, phone: Option<&str>, email: Option<&str>, timezone: Option<&str>) -> Result<Branch, AppError> {
        let r = sqlx::query_as::<_, Branch>(
            "UPDATE branches SET name = COALESCE($2, name), address = COALESCE($3, address), city = COALESCE($4, city), state = COALESCE($5, state), pincode = COALESCE($6, pincode), phone = COALESCE($7, phone), email = COALESCE($8, email), timezone = COALESCE($9, timezone), updated_at = NOW() WHERE id = $1 RETURNING id, name, code, address, city, state, pincode, phone, email, is_active, timezone, created_at, updated_at",
        ).bind(id).bind(name).bind(address).bind(city).bind(state).bind(pincode).bind(phone).bind(email).bind(timezone).fetch_one(self.pool).await?;
        Ok(r)
    }

    pub async fn deactivate(&self, id: i64) -> Result<(), AppError> {
        sqlx::query("UPDATE branches SET is_active = false, updated_at = NOW() WHERE id = $1").bind(id).execute(self.pool).await?;
        Ok(())
    }

    pub async fn code_exists(&self, code: &str, exclude: Option<i64>) -> Result<bool, AppError> {
        let r = if let Some(id) = exclude {
            sqlx::query_scalar::<_, bool>("SELECT EXISTS(SELECT 1 FROM branches WHERE code = $1 AND id != $2)").bind(code).bind(id).fetch_one(self.pool).await?
        } else {
            sqlx::query_scalar::<_, bool>("SELECT EXISTS(SELECT 1 FROM branches WHERE code = $1)").bind(code).fetch_one(self.pool).await?
        };
        Ok(r)
    }

    pub async fn count_active_customers(&self, id: i64) -> Result<i64, AppError> {
        let c = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM customers WHERE branch_id = $1 AND status = 'active'").bind(id).fetch_one(self.pool).await?;
        Ok(c)
    }
}
