use sqlx::PgPool;
use crate::common::errors::app_error::AppError;
use crate::modules::referral::repository::referral_repository::ReferralRepository;
use crate::modules::referral::request::referral_request::*;
use crate::modules::referral::response::referral_response::*;

pub struct ReferralService<'a> { repo: ReferralRepository<'a> }
impl<'a> ReferralService<'a> {
    pub fn new(pool: &'a PgPool) -> Self { Self { repo: ReferralRepository::new(pool) } }
    pub async fn list_programs(&self) -> Result<Vec<ReferralProgramResponse>, AppError> {
        let programs = self.repo.list_programs().await?;
        Ok(programs.iter().map(|p| ReferralProgramResponse { id: p.id, name: p.name.clone(), status: p.status.clone(), referrer_reward_value: p.referrer_reward_value, referee_reward_value: p.referee_reward_value, created_at: p.created_at }).collect())
    }
    pub async fn create_program(&self, req: CreateReferralProgramRequest) -> Result<ReferralProgramResponse, AppError> {
        let p = self.repo.create_program(&req.name, &req.referrer_reward_type, req.referrer_reward_value, &req.referee_reward_type, req.referee_reward_value, req.start_date, req.end_date).await?;
        Ok(ReferralProgramResponse { id: p.id, name: p.name, status: p.status, referrer_reward_value: p.referrer_reward_value, referee_reward_value: p.referee_reward_value, created_at: p.created_at })
    }
}
