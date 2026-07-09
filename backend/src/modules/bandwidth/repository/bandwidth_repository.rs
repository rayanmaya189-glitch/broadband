use sqlx::PgPool;
use crate::modules::bandwidth::model::bandwidth::{BandwidthProfile, BandwidthApplication, BandwidthUsage};

pub struct BandwidthRepository<'a> { pool: &'a PgPool }
impl<'a> BandwidthRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self { Self { pool } }
    pub fn pool(&self) -> &'a PgPool { self.pool }

    pub async fn list(&self, page: i64, per_page: i64) -> Result<(Vec<BandwidthProfile>, i64), sqlx::Error> {
        let offset = (page - 1) * per_page;
        let count_row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM bandwidth_profiles").fetch_one(self.pool).await?;
        let profiles: Vec<BandwidthProfile> = sqlx::query_as("SELECT * FROM bandwidth_profiles ORDER BY created_at DESC LIMIT $1 OFFSET $2")
            .bind(per_page).bind(offset).fetch_all(self.pool).await?;
        Ok((profiles, count_row.0))
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<BandwidthProfile>, sqlx::Error> {
        sqlx::query_as::<_, BandwidthProfile>("SELECT * FROM bandwidth_profiles WHERE id = $1").bind(id).fetch_optional(self.pool).await
    }

    pub async fn create(&self, name: &str, description: Option<&str>, plan_id: Option<i64>, download: i32, upload: i32, burst_down: Option<i32>, burst_up: Option<i32>, burst_dur: Option<i32>, priority: Option<i32>) -> Result<BandwidthProfile, sqlx::Error> {
        sqlx::query_as::<_, BandwidthProfile>("INSERT INTO bandwidth_profiles (name, description, plan_id, download_kbps, upload_kbps, burst_download_kbps, burst_upload_kbps, burst_duration_seconds, priority) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9) RETURNING *")
            .bind(name).bind(description).bind(plan_id).bind(download).bind(upload).bind(burst_down).bind(burst_up).bind(burst_dur).bind(priority).fetch_one(self.pool).await
    }

    pub async fn update(&self, id: i64, name: Option<&str>, description: Option<&str>, download: Option<i32>, upload: Option<i32>, burst_down: Option<i32>, burst_up: Option<i32>, is_active: Option<bool>) -> Result<BandwidthProfile, sqlx::Error> {
        sqlx::query_as::<_, BandwidthProfile>("UPDATE bandwidth_profiles SET name = COALESCE($2, name), description = COALESCE($3, description), download_kbps = COALESCE($4, download_kbps), upload_kbps = COALESCE($5, upload_kbps), burst_download_kbps = COALESCE($6, burst_download_kbps), burst_upload_kbps = COALESCE($7, burst_upload_kbps), is_active = COALESCE($8, is_active), updated_at = NOW() WHERE id = $1 RETURNING *")
            .bind(id).bind(name).bind(description).bind(download).bind(upload).bind(burst_down).bind(burst_up).bind(is_active).fetch_one(self.pool).await
    }

    pub async fn delete(&self, id: i64) -> Result<bool, sqlx::Error> {
        let r = sqlx::query("DELETE FROM bandwidth_profiles WHERE id = $1").bind(id).execute(self.pool).await?;
        Ok(r.rows_affected() > 0)
    }

    // ── Apply to Subscription ──────────────────────────────

    pub async fn apply_to_subscription(&self, profile_id: i64, subscription_id: i64, device_id: i64) -> Result<BandwidthApplication, sqlx::Error> {
        sqlx::query_as::<_, BandwidthApplication>(
            "INSERT INTO bandwidth_applications (profile_id, subscription_id, device_id, status) VALUES ($1,$2,$3,'pending') RETURNING *"
        ).bind(profile_id).bind(subscription_id).bind(device_id).fetch_one(self.pool).await
    }

    pub async fn list_applications(&self, profile_id: Option<i64>, page: i64, per_page: i64) -> Result<(Vec<BandwidthApplication>, i64), sqlx::Error> {
        let offset = (page - 1) * per_page;
        let count_row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM bandwidth_applications WHERE ($1::bigint IS NULL OR profile_id = $1)")
            .bind(profile_id).fetch_one(self.pool).await?;
        let apps: Vec<BandwidthApplication> = sqlx::query_as("SELECT * FROM bandwidth_applications WHERE ($1::bigint IS NULL OR profile_id = $1) ORDER BY created_at DESC LIMIT $2 OFFSET $3")
            .bind(profile_id).bind(per_page).bind(offset).fetch_all(self.pool).await?;
        Ok((apps, count_row.0))
    }

    pub async fn update_application_status(&self, id: i64, status: &str, failed_reason: Option<&str>) -> Result<BandwidthApplication, sqlx::Error> {
        sqlx::query_as::<_, BandwidthApplication>(
            "UPDATE bandwidth_applications SET status = $2, failed_reason = $3, applied_at = CASE WHEN $2 = 'applied' THEN NOW() ELSE applied_at END, retry_count = retry_count + CASE WHEN $2 = 'failed' THEN 1 ELSE 0 END, updated_at = NOW() WHERE id = $1 RETURNING *"
        ).bind(id).bind(status).bind(failed_reason).fetch_one(self.pool).await
    }

    // ── Usage Tracking ─────────────────────────────────────

    pub async fn get_usage(&self, subscription_id: i64, page: i64, per_page: i64) -> Result<(Vec<BandwidthUsage>, i64), sqlx::Error> {
        let offset = (page - 1) * per_page;
        let count_row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM bandwidth_usage WHERE subscription_id = $1")
            .bind(subscription_id).fetch_one(self.pool).await?;
        let usage: Vec<BandwidthUsage> = sqlx::query_as("SELECT * FROM bandwidth_usage WHERE subscription_id = $1 ORDER BY recorded_at DESC LIMIT $2 OFFSET $3")
            .bind(subscription_id).bind(per_page).bind(offset).fetch_all(self.pool).await?;
        Ok((usage, count_row.0))
    }

    pub async fn get_usage_summary(&self, subscription_id: i64) -> Result<(i64, i64), sqlx::Error> {
        sqlx::query_as(
            "SELECT COALESCE(SUM(download_bytes), 0), COALESCE(SUM(upload_bytes), 0) FROM bandwidth_usage WHERE subscription_id = $1 AND recorded_at >= date_trunc('month', NOW())"
        ).bind(subscription_id).fetch_one(self.pool).await
    }
}
