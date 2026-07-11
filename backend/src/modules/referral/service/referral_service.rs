//! SeaORM-based service for the Referral domain.

use sea_orm::DatabaseConnection;

use crate::common::errors::app_error::AppError;
use crate::modules::referral::repository::referral_repository::ReferralRepository;
use crate::modules::referral::request::referral_request::*;
use crate::modules::referral::response::referral_response::*;

pub struct ReferralService<'a> {
    repo: ReferralRepository<'a>,
}

impl<'a> ReferralService<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { repo: ReferralRepository::new(db) }
    }

    pub async fn list_programs(&self) -> Result<Vec<ReferralProgramResponse>, AppError> {
        let programs = self.repo.list_programs().await?;
        Ok(programs.into_iter().map(|p| ReferralProgramResponse {
            id: p.id, name: p.name, status: p.status,
            referrer_reward_type: p.referrer_reward_type, referrer_reward_value: p.referrer_reward_value,
            referee_reward_type: p.referee_reward_type, referee_reward_value: p.referee_reward_value,
            max_referrals_per_customer: p.max_referrals_per_customer,
            start_date: p.start_date, end_date: p.end_date, created_at: p.created_at.into(),
        }).collect())
    }

    pub async fn create_program(&self, req: CreateReferralProgramRequest) -> Result<ReferralProgramResponse, AppError> {
        let p = self.repo.create_program(&req.name, &req.referrer_reward_type, req.referrer_reward_value, &req.referee_reward_type, req.referee_reward_value, req.max_referrals_per_customer, req.start_date, req.end_date).await?;
        Ok(ReferralProgramResponse {
            id: p.id, name: p.name, status: p.status,
            referrer_reward_type: p.referrer_reward_type, referrer_reward_value: p.referrer_reward_value,
            referee_reward_type: p.referee_reward_type, referee_reward_value: p.referee_reward_value,
            max_referrals_per_customer: p.max_referrals_per_customer,
            start_date: p.start_date, end_date: p.end_date, created_at: p.created_at.into(),
        })
    }

    pub async fn update_program(&self, id: i64, req: UpdateReferralProgramRequest) -> Result<ReferralProgramResponse, AppError> {
        let p = self.repo.update_program(id, req.name.as_deref(), req.status.as_deref()).await?;
        Ok(ReferralProgramResponse {
            id: p.id, name: p.name, status: p.status,
            referrer_reward_type: p.referrer_reward_type, referrer_reward_value: p.referrer_reward_value,
            referee_reward_type: p.referee_reward_type, referee_reward_value: p.referee_reward_value,
            max_referrals_per_customer: p.max_referrals_per_customer,
            start_date: p.start_date, end_date: p.end_date, created_at: p.created_at.into(),
        })
    }

    pub async fn list_tracking(&self, referrer_id: Option<i64>, status: Option<&str>, page: i64, per_page: i64) -> Result<(Vec<ReferralTrackingResponse>, i64), AppError> {
        let (tracking, total) = self.repo.list_tracking(referrer_id, status, page, per_page).await?;
        Ok((tracking.into_iter().map(|t| ReferralTrackingResponse {
            id: t.id, program_id: t.program_id, referrer_id: t.referrer_id,
            referee_id: t.referee_id, referral_code: t.referral_code,
            referee_phone: t.referee_phone, status: t.status,
            created_at: t.created_at.into(),
        }).collect(), total))
    }

    pub async fn share_referral(&self, req: ShareReferralRequest) -> Result<ReferralTrackingResponse, AppError> {
        let t = self.repo.create_tracking(req.program_id, 0, "REF000", &req.referee_phone).await?;
        Ok(ReferralTrackingResponse {
            id: t.id, program_id: t.program_id, referrer_id: t.referrer_id,
            referee_id: t.referee_id, referral_code: t.referral_code,
            referee_phone: t.referee_phone, status: t.status,
            created_at: t.created_at.into(),
        })
    }

    pub async fn get_stats(&self, _referrer_id: i64) -> Result<serde_json::Value, AppError> {
        let (tracking, total) = self.repo.list_tracking(None, None, 1, 100000).await?;
        let activated = tracking.iter().filter(|t| t.status == "activated").count() as i64;
        let rewarded = tracking.iter().filter(|t| t.status == "rewarded").count() as i64;
        Ok(serde_json::json!({
            "total_referrals": total,
            "activated": activated,
            "rewarded": rewarded,
        }))
    }

    pub async fn get_wallet(&self, customer_id: i64) -> Result<CustomerWalletResponse, AppError> {
        let w = self.repo.get_or_create_wallet(customer_id).await?;
        Ok(CustomerWalletResponse {
            id: w.id, customer_id: w.customer_id, balance: w.balance,
            total_earned: w.total_earned, total_spent: w.total_spent, status: w.status,
            created_at: w.created_at.into(), updated_at: w.updated_at.into(),
        })
    }

    pub async fn get_or_create_wallet(&self, customer_id: i64) -> Result<CustomerWalletResponse, AppError> {
        let w = self.repo.get_or_create_wallet(customer_id).await?;
        Ok(CustomerWalletResponse {
            id: w.id, customer_id: w.customer_id, balance: w.balance,
            total_earned: w.total_earned, total_spent: w.total_spent, status: w.status,
            created_at: w.created_at.into(), updated_at: w.updated_at.into(),
        })
    }

    pub async fn credit_wallet(&self, customer_id: i64, req: WalletCreditRequest) -> Result<MessageResponse, AppError> {
        let w = self.repo.get_or_create_wallet(customer_id).await?;
        self.repo.credit_wallet(w.id, req.amount, &req.transaction_type, req.reference_type.as_deref(), req.reference_id, req.description.as_deref(), None).await?;
        Ok(MessageResponse { message: "Wallet credited".into() })
    }

    pub async fn debit_wallet(&self, customer_id: i64, req: WalletDebitRequest) -> Result<MessageResponse, AppError> {
        let w = self.repo.get_or_create_wallet(customer_id).await?;
        self.repo.debit_wallet(w.id, req.amount, &req.transaction_type, req.reference_type.as_deref(), req.reference_id, req.description.as_deref(), None).await?;
        Ok(MessageResponse { message: "Wallet debited".into() })
    }

    pub async fn list_wallet_transactions(&self, customer_id: i64) -> Result<(Vec<WalletTransactionResponse>, i64), AppError> {
        let w = self.repo.get_or_create_wallet(customer_id).await?;
        let (txns, total) = self.repo.list_wallet_transactions(w.id, 1, 100000).await?;
        Ok((txns.into_iter().map(|t| WalletTransactionResponse {
            id: t.id, wallet_id: t.wallet_id, transaction_type: t.transaction_type,
            amount: t.amount, balance_after: t.balance_after,
            reference_type: t.reference_type, reference_id: t.reference_id,
            description: t.description, performed_by: t.performed_by,
            created_at: t.created_at.into(),
        }).collect(), total))
    }
}
