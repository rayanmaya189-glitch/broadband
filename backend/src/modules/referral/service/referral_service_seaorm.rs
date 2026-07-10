//! SeaORM-based service for the Referral domain.

use sea_orm::DatabaseConnection;

use crate::common::errors::app_error::AppError;
use crate::modules::referral::repository::referral_repository_seaorm::ReferralRepositorySeaorm;
use crate::modules::referral::request::referral_request::*;
use crate::modules::referral::response::referral_response::*;

pub struct ReferralServiceSeaorm<'a> {
    repo: ReferralRepositorySeaorm<'a>,
}

impl<'a> ReferralServiceSeaorm<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { repo: ReferralRepositorySeaorm::new(db) }
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

    pub async fn get_wallet(&self, customer_id: i64) -> Result<CustomerWalletResponse, AppError> {
        let w = self.repo.get_or_create_wallet(customer_id).await?;
        Ok(CustomerWalletResponse {
            id: w.id, customer_id: w.customer_id, balance: w.balance,
            total_earned: w.total_earned, total_spent: w.total_spent, status: w.status,
            created_at: w.created_at.into(), updated_at: w.updated_at.into(),
        })
    }
}
