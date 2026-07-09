use sqlx::PgPool;
use rust_decimal::Decimal;
use crate::modules::referral::model::referral::{ReferralProgram, ReferralTracking, CustomerWallet, WalletTransaction};

pub struct ReferralRepository<'a> { pool: &'a PgPool }
impl<'a> ReferralRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self { Self { pool } }
    pub fn pool(&self) -> &'a PgPool { self.pool }

    // ── Programs ───────────────────────────────────────────

    pub async fn list_programs(&self) -> Result<Vec<ReferralProgram>, sqlx::Error> {
        sqlx::query_as::<_, ReferralProgram>("SELECT * FROM referral_programs ORDER BY created_at DESC")
            .fetch_all(self.pool).await
    }

    pub async fn get_program(&self, id: i64) -> Result<Option<ReferralProgram>, sqlx::Error> {
        sqlx::query_as::<_, ReferralProgram>("SELECT * FROM referral_programs WHERE id = $1")
            .bind(id).fetch_optional(self.pool).await
    }

    pub async fn create_program(&self, name: &str, rr_type: &str, rr_val: rust_decimal::Decimal, rf_type: &str, rf_val: rust_decimal::Decimal, max_referrals: Option<i32>, start: chrono::NaiveDate, end: chrono::NaiveDate) -> Result<ReferralProgram, sqlx::Error> {
        sqlx::query_as::<_, ReferralProgram>("INSERT INTO referral_programs (name, referrer_reward_type, referrer_reward_value, referee_reward_type, referee_reward_value, max_referrals_per_customer, start_date, end_date) VALUES ($1,$2,$3,$4,$5,$6,$7,$8) RETURNING *")
            .bind(name).bind(rr_type).bind(rr_val).bind(rf_type).bind(rf_val).bind(max_referrals).bind(start).bind(end).fetch_one(self.pool).await
    }

    pub async fn update_program(&self, id: i64, name: Option<&str>, status: Option<&str>) -> Result<ReferralProgram, sqlx::Error> {
        sqlx::query_as::<_, ReferralProgram>("UPDATE referral_programs SET name = COALESCE($2, name), status = COALESCE($3, status), updated_at = NOW() WHERE id = $1 RETURNING *")
            .bind(id).bind(name).bind(status).fetch_one(self.pool).await
    }

    // ── Tracking ───────────────────────────────────────────

    pub async fn list_tracking(&self, referrer_id: Option<i64>, status: Option<&str>, page: i64, per_page: i64) -> Result<(Vec<ReferralTracking>, i64), sqlx::Error> {
        let offset = (page - 1) * per_page;
        let count_row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM referral_tracking WHERE ($1::bigint IS NULL OR referrer_id = $1) AND ($2::text IS NULL OR status = $2)")
            .bind(referrer_id).bind(status).fetch_one(self.pool).await?;
        let tracking: Vec<ReferralTracking> = sqlx::query_as("SELECT * FROM referral_tracking WHERE ($1::bigint IS NULL OR referrer_id = $1) AND ($2::text IS NULL OR status = $2) ORDER BY created_at DESC LIMIT $3 OFFSET $4")
            .bind(referrer_id).bind(status).bind(per_page).bind(offset).fetch_all(self.pool).await?;
        Ok((tracking, count_row.0))
    }

    pub async fn create_tracking(&self, program_id: i64, referrer_id: i64, referral_code: &str, referee_phone: &str) -> Result<ReferralTracking, sqlx::Error> {
        sqlx::query_as::<_, ReferralTracking>("INSERT INTO referral_tracking (program_id, referrer_id, referral_code, referee_phone, status) VALUES ($1,$2,$3,$4,'pending') RETURNING *")
            .bind(program_id).bind(referrer_id).bind(referral_code).bind(referee_phone).fetch_one(self.pool).await
    }

    pub async fn update_tracking_status(&self, id: i64, status: &str, referee_id: Option<i64>) -> Result<ReferralTracking, sqlx::Error> {
        sqlx::query_as::<_, ReferralTracking>("UPDATE referral_tracking SET status = $2, referee_id = COALESCE($3, referee_id), updated_at = NOW() WHERE id = $1 RETURNING *")
            .bind(id).bind(status).bind(referee_id).fetch_one(self.pool).await
    }

    // ── Stats ──────────────────────────────────────────────

    pub async fn get_stats(&self, referrer_id: i64) -> Result<(i64, i64, i64), sqlx::Error> {
        sqlx::query_as(
            "SELECT COUNT(*), COUNT(*) FILTER (WHERE status = 'activated'), COUNT(*) FILTER (WHERE status = 'rewarded') FROM referral_tracking WHERE referrer_id = $1"
        ).bind(referrer_id).fetch_one(self.pool).await
    }

    pub async fn get_program_stats(&self) -> Result<Vec<(String, i64)>, sqlx::Error> {
        sqlx::query_as("SELECT status, COUNT(*) FROM referral_tracking GROUP BY status ORDER BY count DESC")
            .fetch_all(self.pool).await
    }

    // ── Wallet ──────────────────────────────────────────────

    pub async fn get_wallet(&self, customer_id: i64) -> Result<Option<CustomerWallet>, sqlx::Error> {
        sqlx::query_as::<_, CustomerWallet>("SELECT * FROM customer_wallets WHERE customer_id = $1")
            .bind(customer_id).fetch_optional(self.pool).await
    }

    pub async fn get_or_create_wallet(&self, customer_id: i64) -> Result<CustomerWallet, sqlx::Error> {
        sqlx::query_as::<_, CustomerWallet>(
            "INSERT INTO customer_wallets (customer_id) VALUES ($1) ON CONFLICT (customer_id) DO UPDATE SET updated_at = NOW() RETURNING *"
        ).bind(customer_id).fetch_one(self.pool).await
    }

    pub async fn credit_wallet(&self, wallet_id: i64, amount: Decimal, txn_type: &str, ref_type: Option<&str>, ref_id: Option<i64>, desc: Option<&str>, performed_by: Option<i64>) -> Result<WalletTransaction, sqlx::Error> {
        sqlx::query("UPDATE customer_wallets SET balance = balance + $2, total_earned = total_earned + $2, updated_at = NOW() WHERE id = $1")
            .bind(wallet_id).bind(amount).execute(self.pool).await?;
        let wallet = sqlx::query_as::<_, CustomerWallet>("SELECT * FROM customer_wallets WHERE id = $1")
            .bind(wallet_id).fetch_one(self.pool).await?;
        sqlx::query_as::<_, WalletTransaction>(
            "INSERT INTO wallet_transactions (wallet_id, transaction_type, amount, balance_after, reference_type, reference_id, description, performed_by) VALUES ($1,$2,$3,$4,$5,$6,$7,$8) RETURNING *"
        ).bind(wallet_id).bind(txn_type).bind(amount).bind(wallet.balance)
            .bind(ref_type).bind(ref_id).bind(desc).bind(performed_by)
            .fetch_one(self.pool).await
    }

    pub async fn debit_wallet(&self, wallet_id: i64, amount: Decimal, txn_type: &str, ref_type: Option<&str>, ref_id: Option<i64>, desc: Option<&str>, performed_by: Option<i64>) -> Result<WalletTransaction, sqlx::Error> {
        // Atomic check-and-deduct: WHERE balance >= $2 prevents overdrafts.
        // If no rows are affected, the balance was insufficient.
        let result = sqlx::query("UPDATE customer_wallets SET balance = balance - $2, total_spent = total_spent + $2, updated_at = NOW() WHERE id = $1 AND balance >= $2")
            .bind(wallet_id).bind(amount).execute(self.pool).await?;
        if result.rows_affected() == 0 {
            return Err(sqlx::Error::RowNotFound);
        }
        let wallet = sqlx::query_as::<_, CustomerWallet>("SELECT * FROM customer_wallets WHERE id = $1")
            .bind(wallet_id).fetch_one(self.pool).await?;
        sqlx::query_as::<_, WalletTransaction>(
            "INSERT INTO wallet_transactions (wallet_id, transaction_type, amount, balance_after, reference_type, reference_id, description, performed_by) VALUES ($1,$2,$3,$4,$5,$6,$7,$8) RETURNING *"
        ).bind(wallet_id).bind(txn_type).bind(amount).bind(wallet.balance)
            .bind(ref_type).bind(ref_id).bind(desc).bind(performed_by)
            .fetch_one(self.pool).await
    }

    pub async fn list_wallet_transactions(&self, wallet_id: i64, page: i64, per_page: i64) -> Result<(Vec<WalletTransaction>, i64), sqlx::Error> {
        let offset = (page - 1) * per_page;
        let count_row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM wallet_transactions WHERE wallet_id = $1")
            .bind(wallet_id).fetch_one(self.pool).await?;
        let txns: Vec<WalletTransaction> = sqlx::query_as::<_, WalletTransaction>(
            "SELECT * FROM wallet_transactions WHERE wallet_id = $1 ORDER BY created_at DESC LIMIT $2 OFFSET $3"
        ).bind(wallet_id).bind(per_page).bind(offset).fetch_all(self.pool).await?;
        Ok((txns, count_row.0))
    }
}
