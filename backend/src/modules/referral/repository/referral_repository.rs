//! SeaORM-based repository for the Referral domain.

use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel,
    PaginatorTrait, QueryFilter, QueryOrder, Set,
};

use crate::common::errors::app_error::AppError;
use crate::modules::referral::model::referral_program_entity::{self, Model as ReferralProgramModel};
use crate::modules::referral::model::referral_tracking_entity::{self, Model as ReferralTrackingModel};
use crate::modules::referral::model::customer_wallet_entity::{self, Model as CustomerWalletModel};
use crate::modules::referral::model::wallet_transaction_entity::{self, Model as WalletTransactionModel};

pub struct ReferralRepository<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> ReferralRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self { Self { db } }

    pub async fn list_programs(&self) -> Result<Vec<ReferralProgramModel>, AppError> {
        Ok(referral_program_entity::Entity::find().order_by_desc(referral_program_entity::Column::CreatedAt).all(self.db).await?)
    }

    pub async fn get_program(&self, id: i64) -> Result<Option<ReferralProgramModel>, AppError> {
        Ok(referral_program_entity::Entity::find_by_id(id).one(self.db).await?)
    }

    pub async fn create_program(&self, name: &str, rr_type: &str, rr_val: rust_decimal::Decimal, rf_type: &str, rf_val: rust_decimal::Decimal, max_referrals: Option<i32>, start: chrono::NaiveDate, end: chrono::NaiveDate) -> Result<ReferralProgramModel, AppError> {
        let now = chrono::Utc::now();
        let active = referral_program_entity::ActiveModel {
            name: Set(name.to_owned()),
            status: Set("active".to_owned()),
            referrer_reward_type: Set(rr_type.to_owned()),
            referrer_reward_value: Set(rr_val),
            referee_reward_type: Set(rf_type.to_owned()),
            referee_reward_value: Set(rf_val),
            max_referrals_per_customer: Set(max_referrals),
            start_date: Set(start),
            end_date: Set(end),
            created_at: Set(now.into()),
            updated_at: Set(now.into()),
            ..Default::default()
        };
        Ok(active.insert(self.db).await?)
    }

    pub async fn update_program(&self, id: i64, name: Option<&str>, status: Option<&str>) -> Result<ReferralProgramModel, AppError> {
        let existing = referral_program_entity::Entity::find_by_id(id).one(self.db).await?
            .ok_or_else(|| AppError::NotFound("Program not found".into()))?;
        let mut active = existing.into_active_model();
        if let Some(v) = name { active.name = Set(v.to_owned()); }
        if let Some(v) = status { active.status = Set(v.to_owned()); }
        active.updated_at = Set(chrono::Utc::now().into());
        Ok(active.update(self.db).await?)
    }

    pub async fn list_tracking(&self, referrer_id: Option<i64>, status: Option<&str>, page: i64, per_page: i64) -> Result<(Vec<ReferralTrackingModel>, i64), AppError> {
        let page_size = per_page as u64;
        let page_num = if per_page > 0 { ((page - 1).max(0) as u64 * page_size) / page_size } else { 0 };
        let mut select = referral_tracking_entity::Entity::find();
        if let Some(rid) = referrer_id { select = select.filter(referral_tracking_entity::Column::ReferrerId.eq(rid)); }
        if let Some(s) = status { select = select.filter(referral_tracking_entity::Column::Status.eq(s)); }
        let total = select.clone().count(self.db).await?;
        let tracking = select.order_by_desc(referral_tracking_entity::Column::CreatedAt).paginate(self.db, page_size).fetch_page(page_num).await?;
        Ok((tracking, total as i64))
    }

    pub async fn create_tracking(&self, program_id: i64, referrer_id: i64, referral_code: &str, referee_phone: &str) -> Result<ReferralTrackingModel, AppError> {
        let now = chrono::Utc::now();
        let active = referral_tracking_entity::ActiveModel {
            program_id: Set(program_id),
            referrer_id: Set(referrer_id),
            referral_code: Set(referral_code.to_owned()),
            referee_phone: Set(referee_phone.to_owned()),
            status: Set("pending".to_owned()),
            created_at: Set(now.into()),
            updated_at: Set(now.into()),
            ..Default::default()
        };
        Ok(active.insert(self.db).await?)
    }

    pub async fn update_tracking_status(&self, id: i64, status: &str, referee_id: Option<i64>) -> Result<ReferralTrackingModel, AppError> {
        let existing = referral_tracking_entity::Entity::find_by_id(id).one(self.db).await?
            .ok_or_else(|| AppError::NotFound("Tracking not found".into()))?;
        let mut active = existing.into_active_model();
        active.status = Set(status.to_owned());
        if let Some(rid) = referee_id { active.referee_id = Set(Some(rid)); }
        active.updated_at = Set(chrono::Utc::now().into());
        Ok(active.update(self.db).await?)
    }

    pub async fn get_wallet(&self, customer_id: i64) -> Result<Option<CustomerWalletModel>, AppError> {
        Ok(customer_wallet_entity::Entity::find()
            .filter(customer_wallet_entity::Column::CustomerId.eq(customer_id))
            .one(self.db).await?)
    }

    pub async fn get_or_create_wallet(&self, customer_id: i64) -> Result<CustomerWalletModel, AppError> {
        let existing = customer_wallet_entity::Entity::find()
            .filter(customer_wallet_entity::Column::CustomerId.eq(customer_id))
            .one(self.db).await?;
        match existing {
            Some(w) => Ok(w),
            None => {
                let now = chrono::Utc::now();
                let active = customer_wallet_entity::ActiveModel {
                    customer_id: Set(customer_id),
                    balance: Set(rust_decimal::Decimal::ZERO),
                    total_earned: Set(rust_decimal::Decimal::ZERO),
                    total_spent: Set(rust_decimal::Decimal::ZERO),
                    status: Set("active".to_owned()),
                    created_at: Set(now.into()),
                    updated_at: Set(now.into()),
                    ..Default::default()
                };
                Ok(active.insert(self.db).await?)
            }
        }
    }

    pub async fn credit_wallet(&self, wallet_id: i64, amount: rust_decimal::Decimal, txn_type: &str, ref_type: Option<&str>, ref_id: Option<i64>, desc: Option<&str>, performed_by: Option<i64>) -> Result<WalletTransactionModel, AppError> {
        let wallet = customer_wallet_entity::Entity::find_by_id(wallet_id).one(self.db).await?
            .ok_or_else(|| AppError::NotFound("Wallet not found".into()))?;
        // Save values before moving wallet into active model
        let current_balance = wallet.balance;
        let current_earned = wallet.total_earned;
        let new_balance = current_balance + amount;

        let mut wallet_active = wallet.into_active_model();
        wallet_active.balance = Set(new_balance);
        wallet_active.total_earned = Set(current_earned + amount);
        wallet_active.updated_at = Set(chrono::Utc::now().into());
        wallet_active.update(self.db).await?;

        let now = chrono::Utc::now();
        let txn_active = wallet_transaction_entity::ActiveModel {
            wallet_id: Set(wallet_id),
            transaction_type: Set(txn_type.to_owned()),
            amount: Set(amount),
            balance_after: Set(new_balance),
            reference_type: Set(ref_type.map(|s| s.to_owned())),
            reference_id: Set(ref_id),
            description: Set(desc.map(|s| s.to_owned())),
            performed_by: Set(performed_by),
            created_at: Set(now.into()),
            ..Default::default()
        };
        Ok(txn_active.insert(self.db).await?)
    }

    pub async fn debit_wallet(&self, wallet_id: i64, amount: rust_decimal::Decimal, txn_type: &str, ref_type: Option<&str>, ref_id: Option<i64>, desc: Option<&str>, performed_by: Option<i64>) -> Result<WalletTransactionModel, AppError> {
        let wallet = customer_wallet_entity::Entity::find_by_id(wallet_id).one(self.db).await?
            .ok_or_else(|| AppError::NotFound("Wallet not found".into()))?;
        if wallet.balance < amount {
            return Err(AppError::Validation("Insufficient wallet balance".into()));
        }
        // Save values before moving wallet into active model
        let current_balance = wallet.balance;
        let current_spent = wallet.total_spent;
        let new_balance = current_balance - amount;

        let mut wallet_active = wallet.into_active_model();
        wallet_active.balance = Set(new_balance);
        wallet_active.total_spent = Set(current_spent + amount);
        wallet_active.updated_at = Set(chrono::Utc::now().into());
        wallet_active.update(self.db).await?;

        let now = chrono::Utc::now();
        let txn_active = wallet_transaction_entity::ActiveModel {
            wallet_id: Set(wallet_id),
            transaction_type: Set(txn_type.to_owned()),
            amount: Set(amount),
            balance_after: Set(new_balance),
            reference_type: Set(ref_type.map(|s| s.to_owned())),
            reference_id: Set(ref_id),
            description: Set(desc.map(|s| s.to_owned())),
            performed_by: Set(performed_by),
            created_at: Set(now.into()),
            ..Default::default()
        };
        Ok(txn_active.insert(self.db).await?)
    }

    pub async fn list_wallet_transactions(&self, wallet_id: i64, page: i64, per_page: i64) -> Result<(Vec<WalletTransactionModel>, i64), AppError> {
        let page_size = per_page as u64;
        let page_num = if per_page > 0 { ((page - 1).max(0) as u64 * page_size) / page_size } else { 0 };
        let select = wallet_transaction_entity::Entity::find()
            .filter(wallet_transaction_entity::Column::WalletId.eq(wallet_id));
        let total = select.clone().count(self.db).await?;
        let txns = select.order_by_desc(wallet_transaction_entity::Column::CreatedAt).paginate(self.db, page_size).fetch_page(page_num).await?;
        Ok((txns, total as i64))
    }
}
