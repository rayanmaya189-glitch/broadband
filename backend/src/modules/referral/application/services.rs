use crate::modules::referral::domain::entities::{
    CustomerWallet, CustomerWalletActiveModel, CustomerWalletColumn, ReferralProgram,
    ReferralProgramActiveModel, ReferralTracking, ReferralTrackingActiveModel,
    WalletTransactionActiveModel,
};
use crate::shared::errors::AppError;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DatabaseConnection, EntityTrait,
    PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, Set, TransactionTrait,
};

pub struct ReferralService;

impl ReferralService {
    pub async fn list_referrals(
        db: &DatabaseConnection,
        page: u64,
        limit: u64,
    ) -> Result<
        (
            Vec<crate::modules::referral::domain::entities::referral_tracking::Model>,
            u64,
        ),
        AppError,
    > {
        let paginator = ReferralTracking::find().paginate(db, limit);
        let total = paginator.num_items().await?;
        let items = paginator.fetch_page(page.saturating_sub(1)).await?;
        Ok((items, total))
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
        db: &impl ConnectionTrait,
        customer_id: i64,
    ) -> Result<crate::modules::referral::domain::entities::customer_wallet::Model, AppError> {
        // FOR UPDATE: within a transaction this serializes concurrent reward
        // credits for the same customer so the balance cannot be lost.
        let wallet = CustomerWallet::find()
            .filter(
                crate::modules::referral::domain::entities::customer_wallet::Column::CustomerId
                    .eq(customer_id),
            )
            .lock_exclusive()
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

    /// Link a newly-registered customer to their pending referral, matched by
    /// referee phone number. Returns `None` when no qualifying referral exists.
    pub async fn register_referee(
        db: &impl ConnectionTrait,
        referee_phone: &str,
        referee_id: i64,
    ) -> Result<Option<ReferralTrackingModel>, AppError> {
        use crate::modules::referral::domain::entities::referral_tracking::Column;
        let referral = ReferralTracking::find()
            .filter(Column::RefereePhone.eq(referee_phone))
            .filter(Column::Status.eq("pending"))
            .order_by_asc(Column::CreatedAt)
            .one(db)
            .await?;

        let Some(referral) = referral else {
            return Ok(None);
        };

        let now = chrono::Utc::now();
        let mut active: ReferralTrackingActiveModel = referral.into();
        active.referee_id = Set(Some(referee_id));
        active.status = Set("registered".to_string());
        active.registered_at = Set(Some(now));
        active.updated_at = Set(now);
        Ok(Some(active.update(db).await?))
    }

    /// Reward the referrer(s) once a referee becomes an active paying customer
    /// (i.e. after their first subscription is created). Idempotent: referrals
    /// that are already rewarded or not yet registered are left untouched.
    pub async fn reward_activated_referee(
        db: &DatabaseConnection,
        referee_id: i64,
    ) -> Result<(), AppError> {
        use crate::modules::referral::domain::entities::referral_tracking::Column;
        let referrals = ReferralTracking::find()
            .filter(Column::RefereeId.eq(Some(referee_id)))
            .filter(Column::Status.eq("registered"))
            .all(db)
            .await?;

        for referral in referrals {
            Self::reward_referral(db, referral.id).await?;
        }
        Ok(())
    }

    async fn reward_referral(db: &DatabaseConnection, referral_id: i64) -> Result<(), AppError> {
        use crate::modules::referral::domain::entities::referral_tracking::Column;
        use sea_orm::sea_query::{LockBehavior, LockType};

        let txn = db.begin().await?;

        let referral = ReferralTracking::find()
            .filter(Column::Id.eq(referral_id))
            .lock_with_behavior(LockType::Update, LockBehavior::SkipLocked)
            .one(&txn)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Referral {} not found", referral_id)))?;

        // Idempotency guard: only reward a registered referral once.
        if referral.status != "registered" {
            return Ok(());
        }

        let referral_id = referral.id;
        let referrer_id = referral.referrer_id;

        let program = ReferralProgram::find_by_id(referral.program_id)
            .one(&txn)
            .await?
            .ok_or_else(|| AppError::NotFound("Referral program not found".to_string()))?;

        if !program.is_active {
            return Ok(());
        }

        let now = chrono::Utc::now();
        let reward = program.reward_value;

        // Credit the referrer's wallet and record the transaction.
        let wallet = Self::get_or_create_wallet(&txn, referrer_id).await?;
        let wallet_id = wallet.id;
        let wallet_balance = wallet.balance;
        let wallet_earned = wallet.total_earned;
        let mut wallet_active: CustomerWalletActiveModel = wallet.into();
        wallet_active.balance = Set(wallet_balance + reward);
        wallet_active.total_earned = Set(wallet_earned + reward);
        wallet_active.updated_at = Set(now);
        wallet_active.update(&txn).await?;

        WalletTransactionActiveModel {
            wallet_id: Set(wallet_id),
            transaction_type: Set("referral_reward".to_string()),
            amount: Set(reward),
            reference_id: Set(Some(referral_id)),
            reference_type: Set(Some("referral".to_string())),
            description: Set(Some(format!(
                "Referral reward for program '{}'",
                program.name
            ))),
            created_at: Set(now),
            ..Default::default()
        }
        .insert(&txn)
        .await?;

        // Mark the referral rewarded.
        let mut referral_active: ReferralTrackingActiveModel = referral.into();
        referral_active.status = Set("rewarded".to_string());
        referral_active.referrer_reward_status = Set(Some("credited".to_string()));
        referral_active.referrer_reward_amount = Set(Some(reward));
        referral_active.referee_reward_status = Set(Some("applied".to_string()));
        referral_active.rewarded_at = Set(Some(now));
        referral_active.updated_at = Set(now);
        referral_active.update(&txn).await?;

        // Publish the reward event within the same transaction.
        let event = crate::shared::event_contracts::referral::ReferralRewardedV1 {
            referral_id,
            referrer_reward: reward,
            referee_reward: sea_orm::prelude::Decimal::ZERO,
        };
        crate::infrastructure::messaging::outbox::insert_outbox_event(
            &txn,
            "referral.rewarded",
            "referral",
            referral_id,
            serde_json::to_value(event)
                .map_err(|e| AppError::Internal(anyhow::anyhow!("Event serialization: {}", e)))?,
            None,
            Some(referrer_id),
            None,
        )
        .await?;

        txn.commit().await?;
        Ok(())
    }

    pub async fn get_referral_code(
        db: &DatabaseConnection,
        user_id: i64,
    ) -> Result<String, AppError> {
        let refs = ReferralTracking::find()
            .filter(
                crate::modules::referral::domain::entities::referral_tracking::Column::ReferrerId
                    .eq(user_id),
            )
            .all(db)
            .await?;
        if let Some(first) = refs.first() {
            Ok(first.referral_code.clone())
        } else {
            use rand::Rng;
            let code: String = rand::thread_rng()
                .sample_iter(&rand::distributions::Alphanumeric)
                .take(8)
                .map(char::from)
                .map(|c| c.to_ascii_uppercase())
                .collect();
            Ok(format!("REF-{}-{}", user_id, code))
        }
    }

    pub async fn get_referral_stats(
        db: &DatabaseConnection,
        user_id: i64,
    ) -> Result<ReferralStats, AppError> {
        let refs = ReferralTracking::find()
            .filter(
                crate::modules::referral::domain::entities::referral_tracking::Column::ReferrerId
                    .eq(user_id),
            )
            .all(db)
            .await?;

        let total_shared = refs.len() as i64;
        let total_registered = refs
            .iter()
            .filter(|r| r.status == "registered" || r.status == "active" || r.status == "rewarded")
            .count() as i64;
        let total_active = refs
            .iter()
            .filter(|r| r.status == "active" || r.status == "rewarded")
            .count() as i64;
        let total_rewarded = refs.iter().filter(|r| r.status == "rewarded").count() as i64;

        let total_reward_amount: sea_orm::prelude::Decimal =
            refs.iter().filter_map(|r| r.referrer_reward_amount).sum();

        let wallet = Self::get_or_create_wallet(db, user_id).await?;

        Ok(ReferralStats {
            referral_code: Self::get_referral_code(db, user_id).await?,
            total_shared,
            total_registered,
            total_active,
            total_rewarded,
            total_reward_amount: total_reward_amount.to_string(),
            wallet_balance: wallet.balance.to_string(),
            wallet_total_earned: wallet.total_earned.to_string(),
        })
    }

    pub async fn list_my_referrals(
        db: &DatabaseConnection,
        user_id: i64,
    ) -> Result<Vec<ReferralTrackingModel>, AppError> {
        let refs = ReferralTracking::find()
            .filter(
                crate::modules::referral::domain::entities::referral_tracking::Column::ReferrerId
                    .eq(user_id),
            )
            .order_by_desc(
                crate::modules::referral::domain::entities::referral_tracking::Column::SharedAt,
            )
            .all(db)
            .await?;
        Ok(refs)
    }

    // --- Program management ---

    pub async fn list_programs(
        db: &DatabaseConnection,
        page: u64,
        limit: u64,
    ) -> Result<(Vec<ReferralProgramModel>, u64), AppError> {
        let q = ReferralProgram::find().order_by_desc(ReferralProgramColumn::CreatedAt);
        let t = q.clone().count(db).await?;
        let items = q.paginate(db, limit).fetch_page(page).await?;
        Ok((items, t))
    }

    pub async fn create_program(
        db: &DatabaseConnection,
        name: String,
        reward_type: String,
        reward_value: sea_orm::prelude::Decimal,
        max_referrals_per_user: Option<i32>,
        valid_from: chrono::NaiveDate,
        valid_until: chrono::NaiveDate,
    ) -> Result<ReferralProgramModel, AppError> {
        let now = chrono::Utc::now();
        let m = ReferralProgramActiveModel {
            name: Set(name),
            reward_type: Set(reward_type),
            reward_value: Set(reward_value),
            max_referrals_per_user: Set(max_referrals_per_user),
            valid_from: Set(valid_from),
            valid_until: Set(valid_until),
            is_active: Set(true),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };
        Ok(m.insert(db).await?)
    }

    pub async fn update_program(
        db: &DatabaseConnection,
        id: i64,
        name: Option<String>,
        reward_type: Option<String>,
        reward_value: Option<sea_orm::prelude::Decimal>,
        max_referrals_per_user: Option<Option<i32>>,
        valid_from: Option<chrono::NaiveDate>,
        valid_until: Option<chrono::NaiveDate>,
    ) -> Result<ReferralProgramModel, AppError> {
        let m = ReferralProgram::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| AppError::NotFound("Referral program not found".into()))?;
        let mut active: ReferralProgramActiveModel = m.into();
        if let Some(v) = name {
            active.name = Set(v);
        }
        if let Some(v) = reward_type {
            active.reward_type = Set(v);
        }
        if let Some(v) = reward_value {
            active.reward_value = Set(v);
        }
        if let Some(v) = max_referrals_per_user {
            active.max_referrals_per_user = Set(v);
        }
        if let Some(v) = valid_from {
            active.valid_from = Set(v);
        }
        if let Some(v) = valid_until {
            active.valid_until = Set(v);
        }
        active.updated_at = Set(chrono::Utc::now());
        Ok(active.update(db).await?)
    }

    pub async fn delete_program(db: &DatabaseConnection, id: i64) -> Result<(), AppError> {
        let m = ReferralProgram::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| AppError::NotFound("Referral program not found".into()))?;
        let mut active: ReferralProgramActiveModel = m.into();
        active.is_active = Set(false);
        active.updated_at = Set(chrono::Utc::now());
        active.update(db).await?;
        Ok(())
    }

    // --- Analytics ---

    pub async fn get_analytics(db: &DatabaseConnection) -> Result<ReferralAnalytics, AppError> {
        let total_referrals = ReferralTracking::find().count(db).await?;
        let all = ReferralTracking::find().all(db).await?;

        let total_active = all
            .iter()
            .filter(|r| r.status == "active" || r.status == "rewarded")
            .count() as i64;
        let total_rewarded = all.iter().filter(|r| r.status == "rewarded").count() as i64;
        let total_shared = all
            .iter()
            .filter(|r| r.status == "pending" || r.status == "shared")
            .count() as i64;
        let conversion_rate = if total_referrals > 0 {
            (total_rewarded as f64 / total_referrals as f64) * 100.0
        } else {
            0.0
        };
        let total_rewards_paid: sea_orm::prelude::Decimal =
            all.iter().filter_map(|r| r.referrer_reward_amount).sum();

        Ok(ReferralAnalytics {
            total_referrals: total_referrals as i64,
            total_shared,
            total_active,
            total_rewarded,
            conversion_rate: (conversion_rate * 100.0).round() / 100.0,
            total_rewards_paid: total_rewards_paid.to_string(),
        })
    }

    // --- Wallet operations ---

    pub async fn list_wallets(
        db: &DatabaseConnection,
        page: u64,
        limit: u64,
    ) -> Result<(Vec<CustomerWalletModel>, u64), AppError> {
        let q = CustomerWallet::find().order_by_desc(CustomerWalletColumn::CreatedAt);
        let t = q.clone().count(db).await?;
        let items = q.paginate(db, limit).fetch_page(page).await?;
        Ok((items, t))
    }

    pub async fn adjust_wallet(
        db: &DatabaseConnection,
        wallet_id: i64,
        amount: sea_orm::prelude::Decimal,
        reason: String,
        admin_user_id: i64,
    ) -> Result<WalletTransactionModel, AppError> {
        let wallet = CustomerWallet::find_by_id(wallet_id)
            .one(db)
            .await?
            .ok_or_else(|| AppError::NotFound("Wallet not found".into()))?;

        let new_balance = wallet.balance + amount;
        let mut wallet_active: CustomerWalletActiveModel = wallet.into();
        wallet_active.balance = Set(new_balance);
        if amount > sea_orm::prelude::Decimal::ZERO {
            let prev_earned = match &wallet_active.total_earned {
                sea_orm::ActiveValue::Set(v) | sea_orm::ActiveValue::Unchanged(v) => *v,
                sea_orm::ActiveValue::NotSet => sea_orm::prelude::Decimal::ZERO,
            };
            wallet_active.total_earned = Set(prev_earned + amount);
        }
        wallet_active.updated_at = Set(chrono::Utc::now());
        wallet_active.update(db).await?;

        let tx = WalletTransactionActiveModel {
            wallet_id: Set(wallet_id),
            transaction_type: Set("manual_adjustment".to_string()),
            amount: Set(amount),
            reference_id: Set(Some(admin_user_id)),
            reference_type: Set(Some("admin_adjustment".to_string())),
            description: Set(Some(reason)),
            created_at: Set(chrono::Utc::now()),
            ..Default::default()
        };
        Ok(tx.insert(db).await?)
    }
}

// --- Response types ---

use crate::modules::referral::domain::entities::customer_wallet::Model as CustomerWalletModel;
use crate::modules::referral::domain::entities::referral_program::Column as ReferralProgramColumn;
use crate::modules::referral::domain::entities::referral_program::Model as ReferralProgramModel;
use crate::modules::referral::domain::entities::referral_tracking::Model as ReferralTrackingModel;
use crate::modules::referral::domain::entities::wallet_transaction::Model as WalletTransactionModel;

#[derive(Debug, serde::Serialize)]
pub struct ReferralStats {
    pub referral_code: String,
    pub total_shared: i64,
    pub total_registered: i64,
    pub total_active: i64,
    pub total_rewarded: i64,
    pub total_reward_amount: String,
    pub wallet_balance: String,
    pub wallet_total_earned: String,
}

#[derive(Debug, serde::Serialize)]
pub struct ReferralAnalytics {
    pub total_referrals: i64,
    pub total_shared: i64,
    pub total_active: i64,
    pub total_rewarded: i64,
    pub conversion_rate: f64,
    pub total_rewards_paid: String,
}
