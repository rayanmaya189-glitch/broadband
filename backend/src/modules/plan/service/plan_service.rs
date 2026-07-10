//! SeaORM-based service for the Plan domain.
//!
//! Uses `PlanRepository` which operates on `DatabaseConnection`
//! instead of the legacy `PgPool`.

use sea_orm::DatabaseConnection;

use crate::common::cache::cached_repository::CacheHelper;
use crate::common::cache::redis::RedisService;
use crate::common::errors::app_error::AppError;
use crate::common::utils::helpers::PaginatedResponse;
use crate::modules::plan::repository::plan_repository::PlanRepository;
use crate::modules::plan::request::plan_request::*;
use crate::modules::plan::response::plan_response::*;

/// Cache TTL: plans change infrequently — 5 minutes.
const PLAN_CACHE_TTL: u64 = 300;

pub struct PlanService<'a> {
    repo: PlanRepository<'a>,
    cache: CacheHelper<'a>,
}

impl<'a> PlanService<'a> {
    pub fn new(db: &'a DatabaseConnection, redis: &'a RedisService) -> Self {
        Self {
            repo: PlanRepository::new(db),
            cache: CacheHelper::new(redis, "plan", PLAN_CACHE_TTL),
        }
    }

    pub async fn list_plans(
        &self,
        query: &ListPlansQuery,
    ) -> Result<PaginatedResponse<PlanResponse>, AppError> {
        let offset = query.pagination.offset();
        let limit = query.pagination.limit_i64() as u32;
        self.repo
            .list(offset, limit, query.is_active, query.category.as_deref())
            .await
    }

    pub async fn get_plan(&self, id: i64) -> Result<PlanDetailResponse, AppError> {
        // Cache-aside: check Redis first
        if let Some(cached) = self.cache.get_by_id::<PlanDetailResponse>(id).await? {
            return Ok(cached);
        }
        // Cache miss — fetch from DB via SeaORM
        let p = self
            .repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound("Plan not found".into()))?;
        let resp = PlanResponse::from_model(p);
        // Populate cache (best-effort)
        self.cache.set_by_id(id, &resp).await.ok();
        Ok(resp)
    }

    pub async fn create_plan(&self, req: &CreatePlanRequest) -> Result<PlanDetailResponse, AppError> {
        if self.repo.code_exists(&req.code, None).await? {
            return Err(AppError::Conflict("Plan code already exists".into()));
        }
        let gst = req
            .gst_percent
            .unwrap_or_else(|| rust_decimal_macros::dec!(18.00));
        let p = self
            .repo
            .create(
                &req.name,
                &req.code,
                req.description.as_deref(),
                req.speed_down_mbps,
                req.speed_up_mbps,
                req.data_cap_gb,
                req.price_monthly,
                req.price_quarterly,
                req.price_half_yearly,
                req.price_yearly,
                gst,
                req.is_promotional.unwrap_or(false),
                req.category.as_deref().unwrap_or("standard"),
            )
            .await?;
        self.cache.invalidate_prefix().await.ok();
        Ok(PlanResponse::from_model(p))
    }

    pub async fn update_plan(
        &self,
        id: i64,
        req: &UpdatePlanRequest,
    ) -> Result<PlanDetailResponse, AppError> {
        if self.repo.find_by_id(id).await?.is_none() {
            return Err(AppError::NotFound("Plan not found".into()));
        }
        let p = self
            .repo
            .update(
                id,
                req.name.as_deref(),
                req.description.as_deref(),
                req.speed_down_mbps,
                req.speed_up_mbps,
                req.data_cap_gb,
                req.price_monthly,
                req.price_quarterly,
                req.price_half_yearly,
                req.price_yearly,
                req.gst_percent,
                req.is_active,
                req.is_promotional,
                req.category.as_deref(),
            )
            .await?;
        self.cache.invalidate_by_id(id).await.ok();
        Ok(PlanResponse::from_model(p))
    }

    pub async fn delete_plan(&self, id: i64) -> Result<MessageResponse, AppError> {
        if self.repo.find_by_id(id).await?.is_none() {
            return Err(AppError::NotFound("Plan not found".into()));
        }
        self.repo.soft_delete(id).await?;
        self.cache.invalidate_by_id(id).await.ok();
        Ok(MessageResponse {
            message: "Plan deactivated successfully".into(),
        })
    }

    // ── Publish / Unpublish ─────────────────────────────────

    pub async fn publish_plan(&self, id: i64) -> Result<PlanDetailResponse, AppError> {
        let p = self
            .repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound("Plan not found".into()))?;
        if p.is_active {
            return Err(AppError::Validation("Plan is already published".into()));
        }
        let p = self.repo.publish(id).await?;
        self.cache.invalidate_by_id(id).await.ok();
        Ok(PlanResponse::from_model(p))
    }

    pub async fn unpublish_plan(&self, id: i64) -> Result<PlanDetailResponse, AppError> {
        let p = self
            .repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound("Plan not found".into()))?;
        if !p.is_active {
            return Err(AppError::Validation(
                "Plan is already unpublished".into(),
            ));
        }
        let p = self.repo.unpublish(id).await?;
        self.cache.invalidate_by_id(id).await.ok();
        Ok(PlanResponse::from_model(p))
    }

    // ── Clone ───────────────────────────────────────────────

    pub async fn clone_plan(&self, id: i64) -> Result<PlanCloneResponse, AppError> {
        let original = self
            .repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound("Plan not found".into()))?;
        let original_name = original.name.clone();
        let new_plan = self.repo.clone_plan(id).await?;
        self.cache.invalidate_prefix().await.ok();
        Ok(PlanCloneResponse {
            original_plan_id: original.id,
            new_plan_id: new_plan.id,
            new_plan_code: new_plan.code,
            message: format!("Plan '{}' cloned successfully", original_name),
        })
    }

    // ── Speed Profiles ──────────────────────────────────────

    pub async fn get_speed_profile(&self, plan_id: i64) -> Result<SpeedProfileResponse, AppError> {
        self.repo
            .find_by_id(plan_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Plan not found".into()))?;
        let sp = self
            .repo
            .get_speed_profile(plan_id)
            .await?
            .ok_or_else(|| AppError::NotFound("No speed profile found for this plan".into()))?;
        Ok(SpeedProfileResponse::from_model(sp))
    }

    pub async fn create_speed_profile(
        &self,
        plan_id: i64,
        req: &CreateSpeedProfileRequest,
    ) -> Result<SpeedProfileResponse, AppError> {
        self.repo
            .find_by_id(plan_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Plan not found".into()))?;
        let sp = self
            .repo
            .upsert_speed_profile(
                plan_id,
                &req.name,
                req.download_limit_kbps,
                req.upload_limit_kbps,
                req.burst_download_kbps,
                req.burst_upload_kbps,
                req.burst_duration_seconds,
                req.priority_queue,
                req.qos_marking.as_deref(),
                req.htb_parent_queue.as_deref(),
                req.fq_codel_enabled,
                req.device_type.as_deref(),
            )
            .await?;
        self.cache.invalidate_by_id(plan_id).await.ok();
        Ok(SpeedProfileResponse::from_model(sp))
    }

    pub async fn delete_speed_profile(&self, plan_id: i64) -> Result<MessageResponse, AppError> {
        self.repo
            .find_by_id(plan_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Plan not found".into()))?;
        self.repo.delete_speed_profile(plan_id).await?;
        self.cache.invalidate_by_id(plan_id).await.ok();
        Ok(MessageResponse {
            message: "Speed profile deleted".into(),
        })
    }

    // ── Plan Pricing ────────────────────────────────────────

    pub async fn list_pricing(
        &self,
        plan_id: i64,
    ) -> Result<Vec<PlanPricingResponse>, AppError> {
        self.repo
            .find_by_id(plan_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Plan not found".into()))?;
        let pricing = self.repo.list_pricing(plan_id).await?;
        Ok(pricing
            .into_iter()
            .map(PlanPricingResponse::from_model)
            .collect())
    }

    pub async fn update_pricing(
        &self,
        plan_id: i64,
        req: &UpdatePlanPricingRequest,
    ) -> Result<PlanPricingResponse, AppError> {
        self.repo
            .find_by_id(plan_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Plan not found".into()))?;
        if !(1..=12).contains(&req.billing_period_months) {
            return Err(AppError::Validation(
                "Billing period must be 1-12 months".into(),
            ));
        }
        let p = self
            .repo
            .upsert_pricing(plan_id, req.billing_period_months, req.price)
            .await?;
        self.cache.invalidate_by_id(plan_id).await.ok();
        Ok(PlanPricingResponse::from_model(p))
    }
}
