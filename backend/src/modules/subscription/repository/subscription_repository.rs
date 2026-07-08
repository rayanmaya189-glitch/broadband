use sqlx::PgPool;

use crate::common::errors::app_error::AppError;
use crate::common::utils::helpers::{total_pages, PaginatedResponse};
use crate::modules::subscription::model::subscription::Subscription;
use crate::modules::subscription::response::subscription_response::SubscriptionResponse;

pub struct SubscriptionRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> SubscriptionRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self { Self { pool } }

    pub async fn find_by_id(&self, id: i64) -> Result<Option<Subscription>, AppError> {
        let r = sqlx::query_as::<_, Subscription>(
            "SELECT id, customer_id, branch_id, plan_id, status, billing_period_months, start_date, end_date, next_billing_date, auto_renew, created_at, updated_at FROM subscriptions WHERE id = $1",
        ).bind(id).fetch_optional(self.pool).await?;
        Ok(r)
    }

    pub async fn create(
        &self, customer_id: i64, branch_id: i64, plan_id: i64,
        billing_period_months: i32, start_date: chrono::NaiveDate,
        auto_renew: bool,
    ) -> Result<Subscription, AppError> {
        let r = sqlx::query_as::<_, Subscription>(
            "INSERT INTO subscriptions (customer_id, branch_id, plan_id, billing_period_months, start_date, auto_renew) VALUES ($1, $2, $3, $4, $5, $6) RETURNING id, customer_id, branch_id, plan_id, status, billing_period_months, start_date, end_date, next_billing_date, auto_renew, created_at, updated_at",
        ).bind(customer_id).bind(branch_id).bind(plan_id).bind(billing_period_months).bind(start_date).bind(auto_renew).fetch_one(self.pool).await?;
        Ok(r)
    }

    pub async fn update_status(&self, id: i64, status: &str) -> Result<Subscription, AppError> {
        let r = sqlx::query_as::<_, Subscription>(
            "UPDATE subscriptions SET status = $2, updated_at = NOW() WHERE id = $1 RETURNING id, customer_id, branch_id, plan_id, status, billing_period_months, start_date, end_date, next_billing_date, auto_renew, created_at, updated_at",
        ).bind(id).bind(status).fetch_one(self.pool).await?;
        Ok(r)
    }

    pub async fn cancel(&self, id: i64) -> Result<Subscription, AppError> {
        let r = sqlx::query_as::<_, Subscription>(
            "UPDATE subscriptions SET status = 'cancelled', auto_renew = false, end_date = CURRENT_DATE, updated_at = NOW() WHERE id = $1 RETURNING id, customer_id, branch_id, plan_id, status, billing_period_months, start_date, end_date, next_billing_date, auto_renew, created_at, updated_at",
        ).bind(id).fetch_one(self.pool).await?;
        Ok(r)
    }

    pub async fn list(
        &self, offset: u32, limit: u32, status: Option<&str>,
        customer_id: Option<i64>, branch_id: Option<i64>,
    ) -> Result<PaginatedResponse<SubscriptionResponse>, AppError> {
        let limit_i64 = limit.min(100) as i64;
        let offset_i64 = offset as i64;
        let mut conditions = Vec::new();
        let mut idx = 1;
        if status.is_some() { conditions.push(format!("status = ${idx}")); idx += 1; }
        if customer_id.is_some() { conditions.push(format!("customer_id = ${idx}")); idx += 1; }
        if branch_id.is_some() { conditions.push(format!("branch_id = ${idx}")); idx += 1; }
        let wc = if conditions.is_empty() { String::new() } else { format!("WHERE {}", conditions.join(" AND ")) };

        let count_sql = format!("SELECT COUNT(*) FROM subscriptions {wc}");
        let mut cq = sqlx::query_scalar::<_, i64>(&count_sql);
        if let Some(v) = status { cq = cq.bind(v); }
        if let Some(v) = customer_id { cq = cq.bind(v); }
        if let Some(v) = branch_id { cq = cq.bind(v); }
        let total = cq.fetch_one(self.pool).await?;

        let lp = idx;
        let op = idx + 1;
        let data_sql = format!("SELECT id, customer_id, branch_id, plan_id, status, billing_period_months, start_date, end_date, next_billing_date, auto_renew, created_at, updated_at FROM subscriptions {wc} ORDER BY created_at DESC LIMIT ${lp} OFFSET ${op}");
        let mut dq = sqlx::query_as::<_, SubscriptionResponse>(&data_sql);
        if let Some(v) = status { dq = dq.bind(v); }
        if let Some(v) = customer_id { dq = dq.bind(v); }
        if let Some(v) = branch_id { dq = dq.bind(v); }
        dq = dq.bind(limit_i64).bind(offset_i64);
        let subs = dq.fetch_all(self.pool).await?;
        let tp = total_pages(total, limit);
        Ok(PaginatedResponse { data: subs, total, page: (offset / limit) + 1, limit, total_pages: tp })
    }
}
