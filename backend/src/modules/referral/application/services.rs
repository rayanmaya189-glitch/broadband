use crate::modules::referral::domain::entities::{
    CustomerWallet, CustomerWalletActiveModel, ReferralTracking, ReferralTrackingActiveModel,
};
use crate::shared::errors::AppError;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set, PaginatorTrait};

pub struct ReferralService;

impl ReferralService {
    pub async fn list_referrals(
        db: &DatabaseConnection,
        _page: u64,
        _limit: u64,
    ) -> Result<(Vec<crate::modules::referral::domain::entities::referral_tracking::Model>, u64), AppError>
    {
        { let q = ReferralTracking::find(); let t = q.clone().count(db).await?; Ok((q.all(db).await?, t)) }
    }

    pub async fn create_referral(
        db: &DatabaseConnection,
        program_id: i64,
        referrer_id: i64,
        referee_phone: String,
        referral_code: String,
    ) -> Result<crate::modules::referral::domain::entities::referral_tracking::Model, AppError>
    {
        let now = chrono::Utc::now();
        let ref_ = ReferralTrackingActiveModel {
            program_id: Set(program_id),
            referrer_id: Set(referrer_id),
            referee_phone: Set(referee_phone),
            referral_code: Set(referral_code),
            status: Set("pending".to_string()),
            shared_at: Set(now),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };
        Ok(ref_.insert(db).await?)
    }

    pub async fn get_or_create_wallet(
        db: &DatabaseConnection,
        customer_id: i64,
    ) -> Result<crate::modules::referral::domain::entities::customer_wallet::Model, AppError> {
        let wallet = CustomerWallet::find()
            .filter(
                crate::modules::referral::domain::entities::customer_wallet::Column::CustomerId
                    .eq(customer_id),
            )
            .one(db)
            .await?;
        if let Some(w) = wallet {
            Ok(w)
        } else {
            let now = chrono::Utc::now();
            let w = CustomerWalletActiveModel {
                customer_id: Set(customer_id),
                balance: Set(sea_orm::prelude::Decimal::ZERO),
                total_earned: Set(sea_orm::prelude::Decimal::ZERO),
                total_used: Set(sea_orm::prelude::Decimal::ZERO),
                currency: Set("INR".to_string()),
                created_at: Set(now),
                updated_at: Set(now),
                ..Default::default()
            };
            Ok(w.insert(db).await?)
        }
    }
}

