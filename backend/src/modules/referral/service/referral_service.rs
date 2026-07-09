use sqlx::PgPool;
use crate::common::cache::cached_repository::CacheHelper;
use crate::common::cache::redis::RedisService;
use crate::common::errors::app_error::AppError;
use crate::modules::referral::repository::referral_repository::ReferralRepository;
use crate::modules::referral::request::referral_request::*;
use crate::modules::referral::response::referral_response::*;

/// Cache TTL: wallet balance — 60 seconds for consistency.
const WALLET_CACHE_TTL: u64 = 60;

pub struct ReferralService<'a> {
    repo: ReferralRepository<'a>,
    wallet_cache: CacheHelper<'a>,
}
impl<'a> ReferralService<'a> {
    pub fn new(pool: &'a PgPool, redis: &'a RedisService) -> Self {
        Self {
            repo: ReferralRepository::new(pool),
            wallet_cache: CacheHelper::new(redis, "wallet", WALLET_CACHE_TTL),
        }
    }

    // ── Programs ───────────────────────────────────────────

    pub async fn list_programs(&self) -> Result<Vec<ReferralProgramResponse>, AppError> {
        let programs = self.repo.list_programs().await?;
        Ok(programs.iter().map(|p| ReferralProgramResponse { id: p.id, name: p.name.clone(), status: p.status.clone(), referrer_reward_value: p.referrer_reward_value, referee_reward_value: p.referee_reward_value, max_referrals_per_customer: p.max_referrals_per_customer, start_date: p.start_date, end_date: p.end_date, created_at: p.created_at }).collect())
    }

    pub async fn create_program(&self, req: CreateReferralProgramRequest) -> Result<ReferralProgramResponse, AppError> {
        let p = self.repo.create_program(&req.name, &req.referrer_reward_type, req.referrer_reward_value, &req.referee_reward_type, req.referee_reward_value, req.max_referrals_per_customer, req.start_date, req.end_date).await?;
        Ok(ReferralProgramResponse { id: p.id, name: p.name, status: p.status, referrer_reward_value: p.referrer_reward_value, referee_reward_value: p.referee_reward_value, max_referrals_per_customer: p.max_referrals_per_customer, start_date: p.start_date, end_date: p.end_date, created_at: p.created_at })
    }

    pub async fn update_program(&self, id: i64, req: UpdateReferralProgramRequest) -> Result<ReferralProgramResponse, AppError> {
        let p = self.repo.update_program(id, req.name.as_deref(), req.status.as_deref()).await.map_err(|_| AppError::NotFound("Program not found".into()))?;
        Ok(ReferralProgramResponse { id: p.id, name: p.name, status: p.status, referrer_reward_value: p.referrer_reward_value, referee_reward_value: p.referee_reward_value, max_referrals_per_customer: p.max_referrals_per_customer, start_date: p.start_date, end_date: p.end_date, created_at: p.created_at })
    }

    // ── Tracking ───────────────────────────────────────────

    pub async fn share_referral(&self, referrer_id: i64, req: ShareReferralRequest) -> Result<ReferralTrackingResponse, AppError> {
        let _ = self.repo.get_program(req.program_id).await?.ok_or_else(|| AppError::NotFound("Program not found".into()))?;
        // Generate a simple referral code
        let referral_code = format!("REF-{}", referrer_id);
        let tracking = self.repo.create_tracking(req.program_id, referrer_id, &referral_code, &req.referee_phone).await?;
        Ok(ReferralTrackingResponse { id: tracking.id, program_id: tracking.program_id, referrer_id: tracking.referrer_id, referee_id: tracking.referee_id, referral_code: tracking.referral_code, referee_phone: tracking.referee_phone, status: tracking.status, created_at: tracking.created_at })
    }

    pub async fn list_tracking(&self, query: TrackingQuery) -> Result<ReferralTrackingListResponse, AppError> {
        let page = query.page.unwrap_or(1);
        let per_page = query.per_page.unwrap_or(20);
        let (tracking, total) = self.repo.list_tracking(query.referrer_id, query.status.as_deref(), page, per_page).await?;
        let responses: Vec<ReferralTrackingResponse> = tracking.iter().map(|t| ReferralTrackingResponse { id: t.id, program_id: t.program_id, referrer_id: t.referrer_id, referee_id: t.referee_id, referral_code: t.referral_code.clone(), referee_phone: t.referee_phone.clone(), status: t.status.clone(), created_at: t.created_at }).collect();
        let total_pages = (total as f64 / per_page as f64).ceil() as i64;
        Ok(ReferralTrackingListResponse { referrals: responses, total, page, per_page, total_pages })
    }

    // ── Stats ──────────────────────────────────────────────

    pub async fn get_stats(&self, referrer_id: i64) -> Result<ReferralStatsResponse, AppError> {
        let (total, activated, rewarded) = self.repo.get_stats(referrer_id).await?;
        let by_status: Vec<StatusCount> = self.repo.get_program_stats().await?.into_iter().map(|(s, c)| StatusCount { status: s, count: c }).collect();
        Ok(ReferralStatsResponse { total_referrals: total, activated, rewarded, by_status })
    }

    // ── Wallet ─────────────────────────────────────────────

    pub async fn get_wallet(&self, customer_id: i64) -> Result<WalletResponse, AppError> {
        // Cache-aside: check Redis first
        if let Some(cached) = self.wallet_cache.get_by_id::<WalletResponse>(customer_id).await? {
            return Ok(cached);
        }
        let w = self.repo.get_wallet(customer_id).await?
            .ok_or_else(|| AppError::NotFound("No wallet found for this customer".into()))?;
        let resp = WalletResponse { id: w.id, customer_id: w.customer_id, balance: w.balance, total_earned: w.total_earned, total_spent: w.total_spent, status: w.status, created_at: w.created_at, updated_at: w.updated_at };
        self.wallet_cache.set_by_id(customer_id, &resp).await.ok();
        Ok(resp)
    }

    pub async fn get_or_create_wallet(&self, customer_id: i64) -> Result<WalletResponse, AppError> {
        let w = self.repo.get_or_create_wallet(customer_id).await?;
        let resp = WalletResponse { id: w.id, customer_id: w.customer_id, balance: w.balance, total_earned: w.total_earned, total_spent: w.total_spent, status: w.status, created_at: w.created_at, updated_at: w.updated_at };
        self.wallet_cache.set_by_id(customer_id, &resp).await.ok();
        Ok(resp)
    }

    pub async fn credit_wallet(&self, customer_id: i64, req: WalletCreditRequest, performed_by: i64) -> Result<WalletTransactionResponse, AppError> {
        let w = self.repo.get_or_create_wallet(customer_id).await?;
        let txn = self.repo.credit_wallet(w.id, req.amount, &req.transaction_type, req.reference_type.as_deref(), req.reference_id, req.description.as_deref(), Some(performed_by)).await?;
        // Invalidate wallet cache after mutation
        self.wallet_cache.invalidate_by_id(customer_id).await.ok();
        Ok(WalletTransactionResponse { id: txn.id, wallet_id: txn.wallet_id, transaction_type: txn.transaction_type, amount: txn.amount, balance_after: txn.balance_after, reference_type: txn.reference_type, reference_id: txn.reference_id, description: txn.description, performed_by: txn.performed_by, created_at: txn.created_at })
    }

    pub async fn debit_wallet(&self, customer_id: i64, req: WalletDebitRequest, performed_by: i64) -> Result<WalletTransactionResponse, AppError> {
        let w = self.repo.get_wallet(customer_id).await?
            .ok_or_else(|| AppError::NotFound("No wallet found for this customer".into()))?;
        // Atomic check-and-deduct: the repository uses WHERE balance >= amount,
        // so the balance check and deduction happen in a single DB operation.
        let txn = self.repo.debit_wallet(w.id, req.amount, &req.transaction_type, req.reference_type.as_deref(), req.reference_id, req.description.as_deref(), Some(performed_by)).await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => AppError::Validation("Insufficient wallet balance".into()),
                other => AppError::Database(other),
            })?;
        self.wallet_cache.invalidate_by_id(customer_id).await.ok();
        Ok(WalletTransactionResponse { id: txn.id, wallet_id: txn.wallet_id, transaction_type: txn.transaction_type, amount: txn.amount, balance_after: txn.balance_after, reference_type: txn.reference_type, reference_id: txn.reference_id, description: txn.description, performed_by: txn.performed_by, created_at: txn.created_at })
    }

    pub async fn list_wallet_transactions(&self, customer_id: i64, page: i64, per_page: i64) -> Result<WalletTransactionListResponse, AppError> {
        let w = self.repo.get_wallet(customer_id).await?
            .ok_or_else(|| AppError::NotFound("No wallet found for this customer".into()))?;
        let (txns, total) = self.repo.list_wallet_transactions(w.id, page, per_page).await?;
        let total_pages = (total as f64 / per_page as f64).ceil() as i64;
        let responses: Vec<WalletTransactionResponse> = txns.into_iter().map(|t| WalletTransactionResponse { id: t.id, wallet_id: t.wallet_id, transaction_type: t.transaction_type, amount: t.amount, balance_after: t.balance_after, reference_type: t.reference_type, reference_id: t.reference_id, description: t.description, performed_by: t.performed_by, created_at: t.created_at }).collect();
        Ok(WalletTransactionListResponse { transactions: responses, total, page, per_page, total_pages })
    }
}
