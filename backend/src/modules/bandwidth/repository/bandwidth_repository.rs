use sqlx::PgPool;
use crate::modules::bandwidth::model::bandwidth::BandwidthProfile;

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
}
