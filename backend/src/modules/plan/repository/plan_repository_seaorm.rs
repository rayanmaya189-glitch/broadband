//! SeaORM-based repository for the Plan domain.
//!
//! Replaces the legacy sqlx-based `PlanRepository` with SeaORM entity queries.
//! All raw SQL is eliminated — queries use `EntityTrait`, `Select`, and `ActiveModel`.

use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel,
    PaginatorTrait, QueryFilter, QueryOrder, Set,
};

use crate::common::errors::app_error::AppError;
use crate::common::utils::helpers::{total_pages, PaginatedResponse};
use crate::modules::plan::model::plan_entity::{self, Model as PlanModel};
use crate::modules::plan::model::plan_pricing_entity::{self, Model as PlanPricingModel};
use crate::modules::plan::model::speed_profile_entity::{self, Model as SpeedProfileModel};
use crate::modules::plan::response::plan_response::PlanResponse;

/// SeaORM Plan repository — operates on `DatabaseConnection` instead of `PgPool`.
pub struct PlanRepositorySeaorm<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> PlanRepositorySeaorm<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }

    // ── Plan CRUD ───────────────────────────────────────────

    pub async fn find_by_id(&self, id: i64) -> Result<Option<PlanModel>, AppError> {
        let model = plan_entity::Entity::find_by_id(id)
            .one(self.db)
            .await?;
        Ok(model)
    }

    pub async fn find_by_code(&self, code: &str) -> Result<Option<PlanModel>, AppError> {
        let model = plan_entity::Entity::find()
            .filter(plan_entity::Column::Code.eq(code))
            .one(self.db)
            .await?;
        Ok(model)
    }

    pub async fn create(
        &self,
        name: &str,
        code: &str,
        description: Option<&str>,
        speed_down: i32,
        speed_up: i32,
        data_cap: Option<i32>,
        price_monthly: rust_decimal::Decimal,
        price_quarterly: Option<rust_decimal::Decimal>,
        price_half_yearly: Option<rust_decimal::Decimal>,
        price_yearly: Option<rust_decimal::Decimal>,
        gst_percent: rust_decimal::Decimal,
        is_promotional: bool,
        category: &str,
    ) -> Result<PlanModel, AppError> {
        let now = chrono::Utc::now();
        let active_model = plan_entity::ActiveModel {
            name: Set(name.to_owned()),
            code: Set(code.to_owned()),
            description: Set(description.map(|s| s.to_owned())),
            speed_down_mbps: Set(speed_down),
            speed_up_mbps: Set(speed_up),
            data_cap_gb: Set(data_cap),
            price_monthly: Set(price_monthly),
            price_quarterly: Set(price_quarterly),
            price_half_yearly: Set(price_half_yearly),
            price_yearly: Set(price_yearly),
            gst_percent: Set(gst_percent),
            is_active: Set(true),
            is_promotional: Set(is_promotional),
            category: Set(category.to_owned()),
            created_at: Set(now.into()),
            updated_at: Set(now.into()),
            ..Default::default()
        };
        let model = active_model.insert(self.db).await?;
        Ok(model)
    }

    pub async fn update(
        &self,
        id: i64,
        name: Option<&str>,
        description: Option<&str>,
        speed_down: Option<i32>,
        speed_up: Option<i32>,
        data_cap: Option<i32>,
        price_monthly: Option<rust_decimal::Decimal>,
        price_quarterly: Option<rust_decimal::Decimal>,
        price_half_yearly: Option<rust_decimal::Decimal>,
        price_yearly: Option<rust_decimal::Decimal>,
        gst_percent: Option<rust_decimal::Decimal>,
        is_active: Option<bool>,
        is_promotional: Option<bool>,
        category: Option<&str>,
    ) -> Result<PlanModel, AppError> {
        let existing = plan_entity::Entity::find_by_id(id)
            .one(self.db)
            .await?
            .ok_or_else(|| AppError::NotFound("Plan not found".into()))?;

        let mut active = existing.into_active_model();
        if let Some(v) = name {
            active.name = Set(v.to_owned());
        }
        if let Some(v) = description {
            active.description = Set(Some(v.to_owned()));
        }
        if let Some(v) = speed_down {
            active.speed_down_mbps = Set(v);
        }
        if let Some(v) = speed_up {
            active.speed_up_mbps = Set(v);
        }
        if let Some(v) = data_cap {
            active.data_cap_gb = Set(Some(v));
        }
        if let Some(v) = price_monthly {
            active.price_monthly = Set(v);
        }
        if let Some(v) = price_quarterly {
            active.price_quarterly = Set(Some(v));
        }
        if let Some(v) = price_half_yearly {
            active.price_half_yearly = Set(Some(v));
        }
        if let Some(v) = price_yearly {
            active.price_yearly = Set(Some(v));
        }
        if let Some(v) = gst_percent {
            active.gst_percent = Set(v);
        }
        if let Some(v) = is_active {
            active.is_active = Set(v);
        }
        if let Some(v) = is_promotional {
            active.is_promotional = Set(v);
        }
        if let Some(v) = category {
            active.category = Set(v.to_owned());
        }
        active.updated_at = Set(chrono::Utc::now().into());

        let model = active.update(self.db).await?;
        Ok(model)
    }

    pub async fn soft_delete(&self, id: i64) -> Result<(), AppError> {
        let existing = plan_entity::Entity::find_by_id(id)
            .one(self.db)
            .await?
            .ok_or_else(|| AppError::NotFound("Plan not found".into()))?;

        let mut active = existing.into_active_model();
        active.is_active = Set(false);
        active.updated_at = Set(chrono::Utc::now().into());
        active.update(self.db).await?;
        Ok(())
    }

    pub async fn list(
        &self,
        offset: u32,
        limit: u32,
        is_active: Option<bool>,
        category: Option<&str>,
    ) -> Result<PaginatedResponse<PlanResponse>, AppError> {
        let page_size = (limit.min(100)) as u64;

        // Build base query
        let mut select = plan_entity::Entity::find();
        if let Some(v) = is_active {
            select = select.filter(plan_entity::Column::IsActive.eq(v));
        }
        if let Some(v) = category {
            select = select.filter(plan_entity::Column::Category.eq(v));
        }

        // Count total
        let total = select.clone().count(self.db).await?;

        // Fetch page
        let page_num = if limit > 0 { (offset / limit) as u64 } else { 0 };
        let models = select
            .order_by_desc(plan_entity::Column::CreatedAt)
            .paginate(self.db, page_size)
            .fetch_page(page_num)
            .await?;

        let plans: Vec<PlanResponse> = models.into_iter().map(PlanResponse::from_model).collect();
        let total_i64 = total as i64;
        let tp = total_pages(total_i64, limit);
        Ok(PaginatedResponse {
            data: plans,
            total: total_i64,
            page: (page_num as u32 + 1),
            limit,
            total_pages: tp,
        })
    }

    pub async fn code_exists(&self, code: &str, exclude: Option<i64>) -> Result<bool, AppError> {
        let mut select = plan_entity::Entity::find()
            .filter(plan_entity::Column::Code.eq(code));
        if let Some(id) = exclude {
            select = select.filter(plan_entity::Column::Id.ne(id));
        }
        let count = select.count(self.db).await?;
        Ok(count > 0)
    }

    // ── Publish / Unpublish ─────────────────────────────────

    pub async fn publish(&self, id: i64) -> Result<PlanModel, AppError> {
        let existing = plan_entity::Entity::find_by_id(id)
            .one(self.db)
            .await?
            .ok_or_else(|| AppError::NotFound("Plan not found".into()))?;

        let mut active = existing.into_active_model();
        active.is_active = Set(true);
        active.updated_at = Set(chrono::Utc::now().into());
        let model = active.update(self.db).await?;
        Ok(model)
    }

    pub async fn unpublish(&self, id: i64) -> Result<PlanModel, AppError> {
        let existing = plan_entity::Entity::find_by_id(id)
            .one(self.db)
            .await?
            .ok_or_else(|| AppError::NotFound("Plan not found".into()))?;

        let mut active = existing.into_active_model();
        active.is_active = Set(false);
        active.updated_at = Set(chrono::Utc::now().into());
        let model = active.update(self.db).await?;
        Ok(model)
    }

    // ── Clone ───────────────────────────────────────────────

    pub async fn clone_plan(&self, id: i64) -> Result<PlanModel, AppError> {
        let original = plan_entity::Entity::find_by_id(id)
            .one(self.db)
            .await?
            .ok_or_else(|| AppError::NotFound("Plan not found".into()))?;

        let now = chrono::Utc::now();
        let ts = now.timestamp();
        let new_active = plan_entity::ActiveModel {
            name: Set(format!("{} (Copy)", original.name)),
            code: Set(format!("{}-copy-{}", original.code, ts)),
            description: Set(original.description.clone()),
            speed_down_mbps: Set(original.speed_down_mbps),
            speed_up_mbps: Set(original.speed_up_mbps),
            data_cap_gb: Set(original.data_cap_gb),
            price_monthly: Set(original.price_monthly),
            price_quarterly: Set(original.price_quarterly),
            price_half_yearly: Set(original.price_half_yearly),
            price_yearly: Set(original.price_yearly),
            gst_percent: Set(original.gst_percent),
            is_active: Set(false),
            is_promotional: Set(original.is_promotional),
            category: Set(original.category.clone()),
            created_at: Set(now.into()),
            updated_at: Set(now.into()),
            ..Default::default()
        };
        let model = new_active.insert(self.db).await?;
        Ok(model)
    }

    // ── Speed Profiles ──────────────────────────────────────

    pub async fn get_speed_profile(
        &self,
        plan_id: i64,
    ) -> Result<Option<SpeedProfileModel>, AppError> {
        let model = speed_profile_entity::Entity::find()
            .filter(speed_profile_entity::Column::PlanId.eq(plan_id))
            .filter(speed_profile_entity::Column::IsActive.eq(true))
            .one(self.db)
            .await?;
        Ok(model)
    }

    pub async fn upsert_speed_profile(
        &self,
        plan_id: i64,
        name: &str,
        download_kbps: i32,
        upload_kbps: i32,
        burst_down: Option<i32>,
        burst_up: Option<i32>,
        burst_duration: Option<i32>,
        priority: Option<i32>,
        qos: Option<&str>,
        htb_parent: Option<&str>,
        fq_codel: Option<bool>,
        device_type: Option<&str>,
    ) -> Result<SpeedProfileModel, AppError> {
        let now = chrono::Utc::now();
        let existing = self.get_speed_profile(plan_id).await?;

        if let Some(sp) = existing {
            let mut active = sp.into_active_model();
            active.name = Set(name.to_owned());
            active.download_limit_kbps = Set(download_kbps);
            active.upload_limit_kbps = Set(upload_kbps);
            active.burst_download_kbps = Set(burst_down);
            active.burst_upload_kbps = Set(burst_up);
            if let Some(v) = burst_duration {
                active.burst_duration_seconds = Set(v);
            }
            if let Some(v) = priority {
                active.priority_queue = Set(v);
            }
            active.qos_marking = Set(qos.map(|s| s.to_owned()));
            active.htb_parent_queue = Set(htb_parent.map(|s| s.to_owned()));
            if let Some(v) = fq_codel {
                active.fq_codel_enabled = Set(v);
            }
            if let Some(v) = device_type {
                active.device_type = Set(v.to_owned());
            }
            active.updated_at = Set(now.into());
            let model = active.update(self.db).await?;
            Ok(model)
        } else {
            let active_model = speed_profile_entity::ActiveModel {
                plan_id: Set(plan_id),
                name: Set(name.to_owned()),
                download_limit_kbps: Set(download_kbps),
                upload_limit_kbps: Set(upload_kbps),
                burst_download_kbps: Set(burst_down),
                burst_upload_kbps: Set(burst_up),
                burst_duration_seconds: Set(burst_duration.unwrap_or(30)),
                priority_queue: Set(priority.unwrap_or(1)),
                qos_marking: Set(qos.map(|s| s.to_owned())),
                htb_parent_queue: Set(htb_parent.map(|s| s.to_owned())),
                fq_codel_enabled: Set(fq_codel.unwrap_or(true)),
                device_type: Set(device_type.unwrap_or("mikrotik").to_owned()),
                is_active: Set(true),
                created_at: Set(now.into()),
                updated_at: Set(now.into()),
                ..Default::default()
            };
            let model = active_model.insert(self.db).await?;
            Ok(model)
        }
    }

    pub async fn delete_speed_profile(&self, plan_id: i64) -> Result<(), AppError> {
        let existing = speed_profile_entity::Entity::find()
            .filter(speed_profile_entity::Column::PlanId.eq(plan_id))
            .filter(speed_profile_entity::Column::IsActive.eq(true))
            .one(self.db)
            .await?;

        if let Some(sp) = existing {
            let mut active = sp.into_active_model();
            active.is_active = Set(false);
            active.updated_at = Set(chrono::Utc::now().into());
            active.update(self.db).await?;
        }
        Ok(())
    }

    // ── Plan Pricing ────────────────────────────────────────

    pub async fn list_pricing(
        &self,
        plan_id: i64,
    ) -> Result<Vec<PlanPricingModel>, AppError> {
        let models = plan_pricing_entity::Entity::find()
            .filter(plan_pricing_entity::Column::PlanId.eq(plan_id))
            .order_by_asc(plan_pricing_entity::Column::BillingPeriodMonths)
            .all(self.db)
            .await?;
        Ok(models)
    }

    pub async fn upsert_pricing(
        &self,
        plan_id: i64,
        months: i32,
        price: rust_decimal::Decimal,
    ) -> Result<PlanPricingModel, AppError> {
        // Calculate savings vs monthly price
        let plan = plan_entity::Entity::find_by_id(plan_id)
            .one(self.db)
            .await?
            .ok_or_else(|| AppError::NotFound("Plan not found".into()))?;

        let monthly_price = plan.price_monthly;
        let expected = monthly_price * rust_decimal::Decimal::from(months);
        let savings = expected - price;
        let savings_pct = if expected > rust_decimal::Decimal::ZERO {
            (savings / expected * rust_decimal::Decimal::from(100)).round_dp(2)
        } else {
            rust_decimal::Decimal::ZERO
        };

        let now = chrono::Utc::now();

        // Check for existing pricing entry
        let existing = plan_pricing_entity::Entity::find()
            .filter(plan_pricing_entity::Column::PlanId.eq(plan_id))
            .filter(plan_pricing_entity::Column::BillingPeriodMonths.eq(months))
            .one(self.db)
            .await?;

        if let Some(ep) = existing {
            let mut active = ep.into_active_model();
            active.price = Set(price);
            active.savings_amount = Set(Some(savings));
            active.savings_percent = Set(Some(savings_pct));
            active.updated_at = Set(now.into());
            let model = active.update(self.db).await?;
            Ok(model)
        } else {
            let active_model = plan_pricing_entity::ActiveModel {
                plan_id: Set(plan_id),
                billing_period_months: Set(months),
                price: Set(price),
                savings_amount: Set(Some(savings)),
                savings_percent: Set(Some(savings_pct)),
                is_active: Set(true),
                created_at: Set(now.into()),
                updated_at: Set(now.into()),
                ..Default::default()
            };
            let model = active_model.insert(self.db).await?;
            Ok(model)
        }
    }
}
