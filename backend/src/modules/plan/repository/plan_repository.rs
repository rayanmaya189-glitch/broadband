use sqlx::PgPool;

use crate::common::errors::app_error::AppError;
use crate::common::utils::helpers::{total_pages, PaginatedResponse};
use crate::modules::plan::model::plan::Plan;
use crate::modules::plan::response::plan_response::PlanResponse;

pub struct PlanRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> PlanRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self { Self { pool } }

    pub async fn find_by_id(&self, id: i64) -> Result<Option<Plan>, AppError> {
        let r = sqlx::query_as::<_, Plan>(
            "SELECT id, name, code, description, speed_down_mbps, speed_up_mbps, data_cap_gb, price_monthly, price_quarterly, price_half_yearly, price_yearly, gst_percent, is_active, is_promotional, category, created_at, updated_at FROM plans WHERE id = $1",
        ).bind(id).fetch_optional(self.pool).await?;
        Ok(r)
    }

    pub async fn find_by_code(&self, code: &str) -> Result<Option<Plan>, AppError> {
        let r = sqlx::query_as::<_, Plan>(
            "SELECT id, name, code, description, speed_down_mbps, speed_up_mbps, data_cap_gb, price_monthly, price_quarterly, price_half_yearly, price_yearly, gst_percent, is_active, is_promotional, category, created_at, updated_at FROM plans WHERE code = $1",
        ).bind(code).fetch_optional(self.pool).await?;
        Ok(r)
    }

    pub async fn create(
        &self, name: &str, code: &str, description: Option<&str>,
        speed_down: i32, speed_up: i32, data_cap: Option<i32>,
        price_monthly: rust_decimal::Decimal, price_quarterly: Option<rust_decimal::Decimal>,
        price_half_yearly: Option<rust_decimal::Decimal>, price_yearly: Option<rust_decimal::Decimal>,
        gst_percent: rust_decimal::Decimal, is_promotional: bool, category: &str,
    ) -> Result<Plan, AppError> {
        let r = sqlx::query_as::<_, Plan>(
            "INSERT INTO plans (name, code, description, speed_down_mbps, speed_up_mbps, data_cap_gb, price_monthly, price_quarterly, price_half_yearly, price_yearly, gst_percent, is_promotional, category) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13) RETURNING id, name, code, description, speed_down_mbps, speed_up_mbps, data_cap_gb, price_monthly, price_quarterly, price_half_yearly, price_yearly, gst_percent, is_active, is_promotional, category, created_at, updated_at",
        ).bind(name).bind(code).bind(description).bind(speed_down).bind(speed_up).bind(data_cap).bind(price_monthly).bind(price_quarterly).bind(price_half_yearly).bind(price_yearly).bind(gst_percent).bind(is_promotional).bind(category).fetch_one(self.pool).await?;
        Ok(r)
    }

    pub async fn update(
        &self, id: i64, name: Option<&str>, description: Option<&str>,
        speed_down: Option<i32>, speed_up: Option<i32>, data_cap: Option<i32>,
        price_monthly: Option<rust_decimal::Decimal>, price_quarterly: Option<rust_decimal::Decimal>,
        price_half_yearly: Option<rust_decimal::Decimal>, price_yearly: Option<rust_decimal::Decimal>,
        gst_percent: Option<rust_decimal::Decimal>, is_active: Option<bool>,
        is_promotional: Option<bool>, category: Option<&str>,
    ) -> Result<Plan, AppError> {
        let r = sqlx::query_as::<_, Plan>(
            "UPDATE plans SET name = COALESCE($2, name), description = COALESCE($3, description), speed_down_mbps = COALESCE($4, speed_down_mbps), speed_up_mbps = COALESCE($5, speed_up_mbps), data_cap_gb = COALESCE($6, data_cap_gb), price_monthly = COALESCE($7, price_monthly), price_quarterly = COALESCE($8, price_quarterly), price_half_yearly = COALESCE($9, price_half_yearly), price_yearly = COALESCE($10, price_yearly), gst_percent = COALESCE($11, gst_percent), is_active = COALESCE($12, is_active), is_promotional = COALESCE($13, is_promotional), category = COALESCE($14, category), updated_at = NOW() WHERE id = $1 RETURNING id, name, code, description, speed_down_mbps, speed_up_mbps, data_cap_gb, price_monthly, price_quarterly, price_half_yearly, price_yearly, gst_percent, is_active, is_promotional, category, created_at, updated_at",
        ).bind(id).bind(name).bind(description).bind(speed_down).bind(speed_up).bind(data_cap).bind(price_monthly).bind(price_quarterly).bind(price_half_yearly).bind(price_yearly).bind(gst_percent).bind(is_active).bind(is_promotional).bind(category).fetch_one(self.pool).await?;
        Ok(r)
    }

    pub async fn soft_delete(&self, id: i64) -> Result<(), AppError> {
        sqlx::query("UPDATE plans SET is_active = false, updated_at = NOW() WHERE id = $1").bind(id).execute(self.pool).await?;
        Ok(())
    }

    pub async fn list(
        &self, offset: u32, limit: u32, is_active: Option<bool>, category: Option<&str>,
    ) -> Result<PaginatedResponse<PlanResponse>, AppError> {
        let limit_i64 = limit.min(100) as i64;
        let offset_i64 = offset as i64;
        let mut conditions = Vec::new();
        let mut idx = 1;
        if is_active.is_some() { conditions.push(format!("is_active = ${idx}")); idx += 1; }
        if category.is_some() { conditions.push(format!("category = ${idx}")); idx += 1; }
        let wc = if conditions.is_empty() { String::new() } else { format!("WHERE {}", conditions.join(" AND ")) };

        let count_sql = format!("SELECT COUNT(*) FROM plans {wc}");
        let mut cq = sqlx::query_scalar::<_, i64>(&count_sql);
        if let Some(v) = is_active { cq = cq.bind(v); }
        if let Some(v) = category { cq = cq.bind(v); }
        let total = cq.fetch_one(self.pool).await?;

        let lp = idx;
        let op = idx + 1;
        let data_sql = format!("SELECT id, name, code, description, speed_down_mbps, speed_up_mbps, data_cap_gb, price_monthly, price_quarterly, price_half_yearly, price_yearly, gst_percent, is_active, is_promotional, category, created_at, updated_at FROM plans {wc} ORDER BY created_at DESC LIMIT ${lp} OFFSET ${op}");
        let mut dq = sqlx::query_as::<_, PlanResponse>(&data_sql);
        if let Some(v) = is_active { dq = dq.bind(v); }
        if let Some(v) = category { dq = dq.bind(v); }
        dq = dq.bind(limit_i64).bind(offset_i64);
        let plans = dq.fetch_all(self.pool).await?;
        let tp = total_pages(total, limit);
        Ok(PaginatedResponse { data: plans, total, page: (offset / limit) + 1, limit, total_pages: tp })
    }

    pub async fn code_exists(&self, code: &str, exclude: Option<i64>) -> Result<bool, AppError> {
        let r = if let Some(id) = exclude {
            sqlx::query_scalar::<_, bool>("SELECT EXISTS(SELECT 1 FROM plans WHERE code = $1 AND id != $2)").bind(code).bind(id).fetch_one(self.pool).await?
        } else {
            sqlx::query_scalar::<_, bool>("SELECT EXISTS(SELECT 1 FROM plans WHERE code = $1)").bind(code).fetch_one(self.pool).await?
        };
        Ok(r)
    }
}
