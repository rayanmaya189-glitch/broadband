use crate::modules::plans::domain::entities::{
    Plan, PlanActiveModel, PlanColumn, PlanPricing, PlanPricingActiveModel, PlanPricingColumn,
    SpeedProfile, SpeedProfileActiveModel, SpeedProfileColumn,
};
use crate::shared::errors::AppError;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};

pub struct PlanService;

impl PlanService {
    pub async fn list_active_plans(
        db: &DatabaseConnection,
    ) -> Result<Vec<crate::modules::plans::domain::entities::plan::Model>, AppError> {
        let plans = Plan::find()
            .filter(PlanColumn::IsActive.eq(true))
            .filter(PlanColumn::ReviewStatus.eq("approved"))
            .all(db)
            .await?;
        Ok(plans)
    }

    pub async fn get_plan_with_pricing(
        db: &DatabaseConnection,
        id: i64,
    ) -> Result<
        (
            crate::modules::plans::domain::entities::plan::Model,
            Vec<crate::modules::plans::domain::entities::plan_pricing::Model>,
        ),
        AppError,
    > {
        let plan = Plan::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Plan {} not found", id)))?;
        let pricing = PlanPricing::find()
            .filter(PlanPricingColumn::PlanId.eq(id))
            .filter(PlanPricingColumn::IsActive.eq(true))
            .all(db)
            .await?;
        Ok((plan, pricing))
    }

    pub async fn create_plan(
        db: &DatabaseConnection,
        slug: String,
        name: String,
        description: Option<String>,
        speed_label: String,
        download_mbps: i32,
        upload_mbps: i32,
        burst_mbps: Option<i32>,
        is_business: bool,
    ) -> Result<crate::modules::plans::domain::entities::plan::Model, AppError> {
        let now = chrono::Utc::now();
        let new_plan = PlanActiveModel {
            slug: Set(slug),
            name: Set(name),
            description: Set(description),
            speed_label: Set(speed_label),
            download_mbps: Set(download_mbps),
            upload_mbps: Set(upload_mbps),
            burst_mbps: Set(burst_mbps),
            is_business: Set(is_business),
            is_active: Set(true),
            review_status: Set(Some("pending".to_string())),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };
        Ok(new_plan.insert(db).await?)
    }

    pub async fn update_pricing(
        db: &DatabaseConnection,
        plan_id: i64,
        billing_period_months: i32,
        price: sea_orm::prelude::Decimal,
    ) -> Result<crate::modules::plans::domain::entities::plan_pricing::Model, AppError> {
        let existing = PlanPricing::find()
            .filter(PlanPricingColumn::PlanId.eq(plan_id))
            .filter(PlanPricingColumn::BillingPeriodMonths.eq(billing_period_months))
            .one(db)
            .await?;
        if let Some(p) = existing {
            let mut active: PlanPricingActiveModel = p.into();
            active.price = Set(price);
            Ok(active.update(db).await?)
        } else {
            let new_pricing = PlanPricingActiveModel {
                plan_id: Set(plan_id),
                billing_period_months: Set(billing_period_months),
                price: Set(price),
                is_active: Set(true),
                created_at: Set(chrono::Utc::now()),
                ..Default::default()
            };
            Ok(new_pricing.insert(db).await?)
        }
    }

    pub async fn upsert_speed_profile(
        db: &DatabaseConnection,
        plan_id: i64,
        name: String,
        download_limit_kbps: i32,
        upload_limit_kbps: i32,
        device_type: String,
    ) -> Result<crate::modules::plans::domain::entities::speed_profile::Model, AppError> {
        let existing = SpeedProfile::find()
            .filter(SpeedProfileColumn::PlanId.eq(plan_id))
            .one(db)
            .await?;
        if let Some(sp) = existing {
            let mut active: SpeedProfileActiveModel = sp.into();
            active.name = Set(name);
            active.download_limit_kbps = Set(download_limit_kbps);
            active.upload_limit_kbps = Set(upload_limit_kbps);
            active.device_type = Set(device_type);
            active.updated_at = Set(chrono::Utc::now());
            Ok(active.update(db).await?)
        } else {
            let new_sp = SpeedProfileActiveModel {
                plan_id: Set(plan_id),
                name: Set(name),
                download_limit_kbps: Set(download_limit_kbps),
                upload_limit_kbps: Set(upload_limit_kbps),
                device_type: Set(device_type),
                fq_codel_enabled: Set(Some(true)),
                created_at: Set(chrono::Utc::now()),
                updated_at: Set(chrono::Utc::now()),
                ..Default::default()
            };
            Ok(new_sp.insert(db).await?)
        }
    }

    pub async fn list_all_plans(
        db: &DatabaseConnection,
    ) -> Result<Vec<crate::modules::plans::domain::entities::plan::Model>, AppError> {
        let plans = Plan::find().all(db).await?;
        Ok(plans)
    }

    pub async fn approve_plan(
        db: &DatabaseConnection,
        id: i64,
        approved_by: i64,
    ) -> Result<crate::modules::plans::domain::entities::plan::Model, AppError> {
        let plan = Plan::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Plan {} not found", id)))?;
        let mut active: PlanActiveModel = plan.into();
        active.review_status = Set(Some("approved".to_string()));
        active.approved_by = Set(Some(approved_by));
        active.approved_at = Set(Some(chrono::Utc::now()));
        active.updated_at = Set(chrono::Utc::now());
        Ok(active.update(db).await?)
    }

    pub async fn deactivate_plan(db: &DatabaseConnection, id: i64) -> Result<(), AppError> {
        let plan = Plan::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Plan {} not found", id)))?;
        let mut active: PlanActiveModel = plan.into();
        active.is_active = Set(false);
        active.updated_at = Set(chrono::Utc::now());
        active.update(db).await?;
        Ok(())
    }
}
