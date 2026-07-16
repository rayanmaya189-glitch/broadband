use async_trait::async_trait;
use sea_orm::{DatabaseConnection};
use crate::shared::errors::AppError;

pub type PlanModel = crate::modules::plans::domain::entities::plan::Model;
pub type PlanPricingModel = crate::modules::plans::domain::entities::plan_pricing::Model;
pub type SpeedProfileModel = crate::modules::plans::domain::entities::speed_profile::Model;

#[async_trait]
pub trait PlanServiceTrait: Send + Sync {
    async fn list_active_plans(
        &self,
        db: &DatabaseConnection,
    ) -> Result<Vec<PlanModel>, AppError>;

    async fn get_plan_with_pricing(
        &self,
        db: &DatabaseConnection,
        id: i64,
    ) -> Result<(PlanModel, Vec<PlanPricingModel>), AppError>;

    async fn create_plan(
        &self,
        db: &DatabaseConnection,
        slug: String,
        name: String,
        description: Option<String>,
        speed_label: String,
        download_mbps: i32,
        upload_mbps: i32,
        burst_mbps: Option<i32>,
        is_business: bool,
    ) -> Result<PlanModel, AppError>;

    async fn update_pricing(
        &self,
        db: &DatabaseConnection,
        plan_id: i64,
        billing_period_months: i32,
        price: sea_orm::prelude::Decimal,
    ) -> Result<PlanPricingModel, AppError>;

    async fn upsert_speed_profile(
        &self,
        db: &DatabaseConnection,
        plan_id: i64,
        name: String,
        download_limit_kbps: i32,
        upload_limit_kbps: i32,
        device_type: String,
    ) -> Result<SpeedProfileModel, AppError>;

    async fn list_all_plans(
        &self,
        db: &DatabaseConnection,
    ) -> Result<Vec<PlanModel>, AppError>;

    async fn approve_plan(
        &self,
        db: &DatabaseConnection,
        id: i64,
        approved_by: i64,
    ) -> Result<PlanModel, AppError>;

    async fn deactivate_plan(
        &self,
        db: &DatabaseConnection,
        id: i64,
    ) -> Result<(), AppError>;
}
