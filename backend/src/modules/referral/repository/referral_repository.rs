use sqlx::PgPool;
use crate::modules::referral::model::referral::ReferralProgram;

pub struct ReferralRepository<'a> { pool: &'a PgPool }
impl<'a> ReferralRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self { Self { pool } }
    pub async fn list_programs(&self) -> Result<Vec<ReferralProgram>, sqlx::Error> { sqlx::query_as::<_, ReferralProgram>("SELECT * FROM referral_programs ORDER BY created_at DESC").fetch_all(self.pool).await }
    pub async fn create_program(&self, name: &str, rr_type: &str, rr_val: rust_decimal::Decimal, rf_type: &str, rf_val: rust_decimal::Decimal, start: chrono::NaiveDate, end: chrono::NaiveDate) -> Result<ReferralProgram, sqlx::Error> {
        sqlx::query_as::<_, ReferralProgram>("INSERT INTO referral_programs (name, referrer_reward_type, referrer_reward_value, referee_reward_type, referee_reward_value, start_date, end_date) VALUES ($1,$2,$3,$4,$5,$6,$7) RETURNING *").bind(name).bind(rr_type).bind(rr_val).bind(rf_type).bind(rf_val).bind(start).bind(end).fetch_one(self.pool).await
    }
}
