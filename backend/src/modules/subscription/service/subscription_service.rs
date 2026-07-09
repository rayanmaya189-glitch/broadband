use sqlx::PgPool;
use rust_decimal::Decimal;

use crate::common::cache::cached_repository::CacheHelper;
use crate::common::cache::redis::RedisService;
use crate::common::errors::app_error::AppError;
use crate::common::utils::helpers::PaginatedResponse;
use crate::modules::subscription::mapper::subscription_mapper::subscription_to_response;
use crate::modules::subscription::repository::subscription_repository::SubscriptionRepository;
use crate::modules::subscription::request::subscription_request::*;
use crate::modules::subscription::response::subscription_response::*;

/// Cache TTL: subscriptions — 2 minutes.
const SUB_CACHE_TTL: u64 = 120;

pub struct SubscriptionService<'a> {
    repo: SubscriptionRepository<'a>,
    cache: CacheHelper<'a>,
}

impl<'a> SubscriptionService<'a> {
    pub fn new(pool: &'a PgPool, redis: &'a RedisService) -> Self {
        Self {
            repo: SubscriptionRepository::new(pool),
            cache: CacheHelper::new(redis, "sub", SUB_CACHE_TTL),
        }
    }

    pub async fn list_subscriptions(&self, query: &ListSubscriptionsQuery) -> Result<PaginatedResponse<SubscriptionResponse>, AppError> {
        let offset = query.pagination.offset();
        let limit = query.pagination.limit_i64() as u32;
        self.repo.list(offset, limit, query.status.as_deref(), query.customer_id, query.branch_id).await
    }

    pub async fn get_subscription(&self, id: i64) -> Result<SubscriptionDetailResponse, AppError> {
        // Cache-aside: check Redis first
        if let Some(cached) = self.cache.get_by_id::<SubscriptionDetailResponse>(id).await? {
            return Ok(cached);
        }
        let s = self.repo.find_by_id(id).await?.ok_or_else(|| AppError::NotFound("Subscription not found".into()))?;
        let resp = subscription_to_response(&s);
        self.cache.set_by_id(id, &resp).await.ok();
        Ok(resp)
    }

    pub async fn create_subscription(&self, req: &CreateSubscriptionRequest) -> Result<SubscriptionDetailResponse, AppError> {
        let months = req.billing_period_months.unwrap_or(1);
        if !(1..=12).contains(&months) {
            return Err(AppError::Validation("Billing period must be 1-12 months".into()));
        }
        let s = self.repo.create(req.customer_id, req.branch_id, req.plan_id, months, req.start_date, req.auto_renew.unwrap_or(true)).await?;
        self.repo.record_history(s.id, "created", None, Some(&serde_json::json!({"plan_id": s.plan_id, "billing_period": months}).to_string()), None, None).await.ok();
        Ok(subscription_to_response(&s))
    }

    pub async fn suspend_subscription(&self, id: i64, reason: Option<&str>) -> Result<SubscriptionDetailResponse, AppError> {
        let s = self.repo.find_by_id(id).await?.ok_or_else(|| AppError::NotFound("Subscription not found".into()))?;
        if s.status != "active" { return Err(AppError::Validation("Only active subscriptions can be suspended".into())); }
        let old_data = serde_json::json!({"status": s.status}).to_string();
        let s = self.repo.update_status(id, "suspended").await?;
        let new_data = serde_json::json!({"status": "suspended"}).to_string();
        self.repo.record_history(id, "suspended", Some(&old_data), Some(&new_data), None, reason).await.ok();
        self.cache.invalidate_by_id(id).await.ok();
        Ok(subscription_to_response(&s))
    }

    pub async fn reactivate_subscription(&self, id: i64) -> Result<SubscriptionDetailResponse, AppError> {
        let s = self.repo.find_by_id(id).await?.ok_or_else(|| AppError::NotFound("Subscription not found".into()))?;
        if s.status != "suspended" { return Err(AppError::Validation("Only suspended subscriptions can be reactivated".into())); }
        let s = self.repo.update_status(id, "active").await?;
        self.repo.record_history(id, "reactivated", None, Some(&serde_json::json!({"status": "active"}).to_string()), None, None).await.ok();
        self.cache.invalidate_by_id(id).await.ok();
        Ok(subscription_to_response(&s))
    }

    pub async fn cancel_subscription(&self, id: i64, reason: Option<&str>) -> Result<MessageResponse, AppError> {
        let s = self.repo.find_by_id(id).await?.ok_or_else(|| AppError::NotFound("Subscription not found".into()))?;
        if s.status == "cancelled" || s.status == "expired" {
            return Err(AppError::Validation("Subscription is already cancelled or expired".into()));
        }
        self.repo.cancel(id).await?;
        self.repo.record_history(id, "cancelled", None, Some(&serde_json::json!({"status": "cancelled"}).to_string()), None, reason).await.ok();
        self.cache.invalidate_by_id(id).await.ok();
        Ok(MessageResponse { message: "Subscription cancelled successfully".into() })
    }

    // ── Upgrade / Downgrade with Pro-Rata Billing ───────────

    pub async fn upgrade_subscription(&self, id: i64, req: &UpgradeDowngradeRequest) -> Result<UpgradeDowngradeResponse, AppError> {
        let sub = self.repo.find_by_id(id).await?.ok_or_else(|| AppError::NotFound("Subscription not found".into()))?;
        if sub.status != "active" {
            return Err(AppError::Validation("Only active subscriptions can be upgraded".into()));
        }

        let pool: &PgPool = self.repo.get_pool();
        let old_plan: crate::modules::plan::model::plan::Plan = sqlx::query_as(
            "SELECT id, name, code, description, speed_down_mbps, speed_up_mbps, data_cap_gb, price_monthly, price_quarterly, price_half_yearly, price_yearly, gst_percent, is_active, is_promotional, category, created_at, updated_at FROM plans WHERE id = $1"
        ).bind(sub.plan_id).fetch_optional(pool).await?.ok_or_else(|| AppError::NotFound("Old plan not found".into()))?;

        let new_plan: crate::modules::plan::model::plan::Plan = sqlx::query_as(
            "SELECT id, name, code, description, speed_down_mbps, speed_up_mbps, data_cap_gb, price_monthly, price_quarterly, price_half_yearly, price_yearly, gst_percent, is_active, is_promotional, category, created_at, updated_at FROM plans WHERE id = $1"
        ).bind(req.new_plan_id).fetch_optional(pool).await?.ok_or_else(|| AppError::NotFound("New plan not found".into()))?;

        if new_plan.price_monthly < old_plan.price_monthly {
            return Err(AppError::Validation("Use downgrade endpoint for cheaper plans".into()));
        }

        let today = chrono::Utc::now().date_naive();
        let billing_period_days = sub.billing_period_months * 30;
        let days_elapsed = (today - sub.start_date).num_days() as i32;
        let remaining_days = billing_period_days - days_elapsed;
        if remaining_days <= 0 {
            return Err(AppError::Validation("Cannot upgrade after billing period ended".into()));
        }

        let mut pro_rata = calculate_pro_rata(old_plan.price_monthly, new_plan.price_monthly, billing_period_days, days_elapsed);
        pro_rata.old_plan_id = sub.plan_id;
        pro_rata.new_plan_id = req.new_plan_id;

        let old_data = serde_json::json!({"plan_id": sub.plan_id, "status": sub.status}).to_string();
        let new_sub = self.repo.change_plan(id, req.new_plan_id, sub.plan_id).await?;
        let new_data = serde_json::json!({"plan_id": new_sub.plan_id, "pro_rata_adjustment": pro_rata.adjustment}).to_string();
        self.repo.record_history(id, "upgraded", Some(&old_data), Some(&new_data), None, req.reason.as_deref()).await.ok();
        self.cache.invalidate_by_id(id).await.ok();

        let msg = if pro_rata.adjustment > Decimal::ZERO {
            format!("Upgrade complete. Additional charge of ₹{}", pro_rata.adjustment)
        } else {
            format!("Upgrade complete. Credit of ₹{}", -pro_rata.adjustment)
        };

        Ok(UpgradeDowngradeResponse {
            subscription: subscription_to_response(&new_sub),
            pro_rata,
            message: msg,
        })
    }

    pub async fn downgrade_subscription(&self, id: i64, req: &UpgradeDowngradeRequest) -> Result<UpgradeDowngradeResponse, AppError> {
        let sub = self.repo.find_by_id(id).await?.ok_or_else(|| AppError::NotFound("Subscription not found".into()))?;
        if sub.status != "active" {
            return Err(AppError::Validation("Only active subscriptions can be downgraded".into()));
        }

        let pool: &PgPool = self.repo.get_pool();
        let old_plan: crate::modules::plan::model::plan::Plan = sqlx::query_as(
            "SELECT id, name, code, description, speed_down_mbps, speed_up_mbps, data_cap_gb, price_monthly, price_quarterly, price_half_yearly, price_yearly, gst_percent, is_active, is_promotional, category, created_at, updated_at FROM plans WHERE id = $1"
        ).bind(sub.plan_id).fetch_optional(pool).await?.ok_or_else(|| AppError::NotFound("Old plan not found".into()))?;

        let new_plan: crate::modules::plan::model::plan::Plan = sqlx::query_as(
            "SELECT id, name, code, description, speed_down_mbps, speed_up_mbps, data_cap_gb, price_monthly, price_quarterly, price_half_yearly, price_yearly, gst_percent, is_active, is_promotional, category, created_at, updated_at FROM plans WHERE id = $1"
        ).bind(req.new_plan_id).fetch_optional(pool).await?.ok_or_else(|| AppError::NotFound("New plan not found".into()))?;

        if new_plan.price_monthly >= old_plan.price_monthly {
            return Err(AppError::Validation("Use upgrade endpoint for same or higher plans".into()));
        }

        let today = chrono::Utc::now().date_naive();
        let billing_period_days = sub.billing_period_months * 30;
        let days_elapsed = (today - sub.start_date).num_days() as i32;
        let remaining_days = billing_period_days - days_elapsed;

        let pro_rata = if remaining_days > 0 {
            calculate_pro_rata(old_plan.price_monthly, new_plan.price_monthly, billing_period_days, days_elapsed)
        } else {
            ProRataAdjustment {
                old_plan_id: sub.plan_id, new_plan_id: req.new_plan_id,
                old_plan_price: old_plan.price_monthly, new_plan_price: new_plan.price_monthly,
                old_plan_credit: Decimal::ZERO, new_plan_charge: Decimal::ZERO, adjustment: Decimal::ZERO,
                remaining_days: 0, billing_period_days,
            }
        };

        let old_data = serde_json::json!({"plan_id": sub.plan_id}).to_string();

        if remaining_days > 0 {
            let scheduled_date = sub.start_date + chrono::Duration::days(billing_period_days as i64);
            let new_sub = self.repo.schedule_downgrade(id, req.new_plan_id, sub.plan_id, scheduled_date).await?;
            let new_data = serde_json::json!({"plan_id": new_sub.plan_id, "scheduled_date": scheduled_date}).to_string();
            self.repo.record_history(id, "downgrade_scheduled", Some(&old_data), Some(&new_data), None, req.reason.as_deref()).await.ok();
            self.cache.invalidate_by_id(id).await.ok();
            Ok(UpgradeDowngradeResponse {
                subscription: subscription_to_response(&new_sub),
                pro_rata: pro_rata.clone(),
                message: format!("Downgrade scheduled for {}. Credit of ₹{}", scheduled_date, -pro_rata.adjustment),
            })
        } else {
            let new_sub = self.repo.change_plan(id, req.new_plan_id, sub.plan_id).await?;
            let new_data = serde_json::json!({"plan_id": new_sub.plan_id}).to_string();
            self.repo.record_history(id, "downgraded", Some(&old_data), Some(&new_data), None, req.reason.as_deref()).await.ok();
            self.cache.invalidate_by_id(id).await.ok();
            Ok(UpgradeDowngradeResponse {
                subscription: subscription_to_response(&new_sub),
                pro_rata: pro_rata.clone(),
                message: format!("Downgrade applied immediately. Credit of ₹{}", -pro_rata.adjustment),
            })
        }
    }

    // ── History ─────────────────────────────────────────────

    pub async fn get_history(&self, id: i64, query: &SubscriptionHistoryQuery) -> Result<Vec<SubscriptionHistoryEntry>, AppError> {
        self.repo.find_by_id(id).await?.ok_or_else(|| AppError::NotFound("Subscription not found".into()))?;
        let offset = query.pagination.offset();
        let limit = query.pagination.limit_i64() as u32;
        self.repo.get_history(id, offset, limit).await
    }
}

/// Calculate pro-rata adjustment for mid-cycle plan changes.
pub fn calculate_pro_rata(
    old_plan_price: Decimal,
    new_plan_price: Decimal,
    billing_period_days: i32,
    days_used: i32,
) -> ProRataAdjustment {
    let remaining_days = billing_period_days - days_used;
    let bd = Decimal::from(billing_period_days);
    let rd = Decimal::from(remaining_days);
    let old_daily = old_plan_price / bd;
    let new_daily = new_plan_price / bd;
    let credit = (old_daily * rd).round_dp(2);
    let charge = (new_daily * rd).round_dp(2);
    let adjustment = (charge - credit).round_dp(2);

    ProRataAdjustment {
        old_plan_id: 0,
        new_plan_id: 0,
        old_plan_price,
        new_plan_price,
        old_plan_credit: credit,
        new_plan_charge: charge,
        adjustment,
        remaining_days,
        billing_period_days,
    }
}
