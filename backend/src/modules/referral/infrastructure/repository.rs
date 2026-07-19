use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder,
    QuerySelect, Set,
};

use crate::modules::referral::domain::entities::{customer_wallet, referral_tracking};
use crate::shared::errors::AppError;

pub struct ReferralRepository<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> ReferralRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }

    // ── Referral Tracking ─────────────────────────────────────────────

    pub async fn find_referral_by_id(
        &self,
        id: i64,
    ) -> Result<Option<referral_tracking::Model>, AppError> {
        Ok(referral_tracking::Entity::find_by_id(id)
            .one(self.db)
            .await?)
    }

    pub async fn find_referral_by_code(
        &self,
        code: &str,
    ) -> Result<Option<referral_tracking::Model>, AppError> {
        Ok(referral_tracking::Entity::find()
            .filter(referral_tracking::Column::ReferralCode.eq(code))
            .one(self.db)
            .await?)
    }

    pub async fn list_referrals_by_referrer(
        &self,
        referrer_id: i64,
    ) -> Result<Vec<referral_tracking::Model>, AppError> {
        Ok(referral_tracking::Entity::find()
            .filter(referral_tracking::Column::ReferrerId.eq(referrer_id))
            .order_by_desc(referral_tracking::Column::CreatedAt)
            .all(self.db)
            .await?)
    }

    pub async fn list_all_referrals(
        &self,
        status: Option<&str>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<referral_tracking::Model>, AppError> {
        let mut query = referral_tracking::Entity::find();
        if let Some(s) = status {
            query = query.filter(referral_tracking::Column::Status.eq(s));
        }
        Ok(query
            .order_by_desc(referral_tracking::Column::CreatedAt)
            .limit(limit as u64)
            .offset(offset as u64)
            .all(self.db)
            .await?)
    }

    pub async fn create_referral(
        &self,
        program_id: i64,
        referrer_id: i64,
        referral_code: String,
        referee_phone: String,
    ) -> Result<referral_tracking::Model, AppError> {
        let now = chrono::Utc::now();
        let model = referral_tracking::ActiveModel {
            program_id: Set(program_id),
            referrer_id: Set(referrer_id),
            referral_code: Set(referral_code),
            referee_phone: Set(referee_phone),
            status: Set("pending".to_string()),
            shared_at: Set(now),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };
        Ok(model.insert(self.db).await?)
    }

    pub async fn update_referral_status(
        &self,
        model: referral_tracking::Model,
        status: String,
        referee_id: Option<i64>,
    ) -> Result<referral_tracking::Model, AppError> {
        let mut active: referral_tracking::ActiveModel = model.into();
        active.status = Set(status.clone());
        if let Some(rid) = referee_id {
            active.referee_id = Set(Some(rid));
        }
        match status.as_str() {
            "registered" => {
                active.registered_at = Set(Some(chrono::Utc::now()));
            }
            "activated" => {
                active.activated_at = Set(Some(chrono::Utc::now()));
            }
            "rewarded" => {
                active.rewarded_at = Set(Some(chrono::Utc::now()));
            }
            _ => {}
        }
        active.updated_at = Set(chrono::Utc::now());
        Ok(active.update(self.db).await?)
    }

    // ── Wallet ────────────────────────────────────────────────────────

    pub async fn find_wallet_by_customer(
        &self,
        customer_id: i64,
    ) -> Result<Option<customer_wallet::Model>, AppError> {
        Ok(customer_wallet::Entity::find()
            .filter(customer_wallet::Column::CustomerId.eq(customer_id))
            .one(self.db)
            .await?)
    }

    pub async fn create_wallet(
        &self,
        customer_id: i64,
    ) -> Result<customer_wallet::Model, AppError> {
        let now = chrono::Utc::now();
        let model = customer_wallet::ActiveModel {
            customer_id: Set(customer_id),
            balance: Set(rust_decimal::Decimal::ZERO),
            total_earned: Set(rust_decimal::Decimal::ZERO),
            total_used: Set(rust_decimal::Decimal::ZERO),
            currency: Set("INR".to_string()),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };
        Ok(model.insert(self.db).await?)
    }

    pub async fn credit_wallet(
        &self,
        model: customer_wallet::Model,
        amount: rust_decimal::Decimal,
    ) -> Result<customer_wallet::Model, AppError> {
        let old_balance = model.balance;
        let old_earned = model.total_earned;
        let mut active: customer_wallet::ActiveModel = model.into();
        active.balance = Set(old_balance + amount);
        active.total_earned = Set(old_earned + amount);
        active.updated_at = Set(chrono::Utc::now());
        Ok(active.update(self.db).await?)
    }

    pub async fn debit_wallet(
        &self,
        model: customer_wallet::Model,
        amount: rust_decimal::Decimal,
    ) -> Result<customer_wallet::Model, AppError> {
        let old_balance = model.balance;
        let old_used = model.total_used;
        let mut active: customer_wallet::ActiveModel = model.into();
        active.balance = Set(old_balance - amount);
        active.total_used = Set(old_used + amount);
        active.updated_at = Set(chrono::Utc::now());
        Ok(active.update(self.db).await?)
    }
}
