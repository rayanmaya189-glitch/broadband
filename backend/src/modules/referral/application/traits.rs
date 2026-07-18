use crate::shared::errors::AppError;
use async_trait::async_trait;
use sea_orm::DatabaseConnection;

pub type ReferralTrackingModel =
    crate::modules::referral::domain::entities::referral_tracking::Model;
pub type CustomerWalletModel = crate::modules::referral::domain::entities::customer_wallet::Model;

#[async_trait]
pub trait ReferralServiceTrait: Send + Sync {
    async fn list_referrals(
        &self,
        db: &DatabaseConnection,
        referrer_id: Option<i64>,
    ) -> Result<Vec<ReferralTrackingModel>, AppError>;

    async fn create_referral(
        &self,
        db: &DatabaseConnection,
        referrer_id: i64,
        referred_customer_id: i64,
        referral_code: String,
    ) -> Result<ReferralTrackingModel, AppError>;

    async fn get_or_create_wallet(
        &self,
        db: &DatabaseConnection,
        customer_id: i64,
    ) -> Result<CustomerWalletModel, AppError>;

    async fn credit_wallet(
        &self,
        db: &DatabaseConnection,
        customer_id: i64,
        amount: sea_orm::prelude::Decimal,
        description: &str,
    ) -> Result<CustomerWalletModel, AppError>;
}
