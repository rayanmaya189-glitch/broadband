use sqlx::PgPool;
use crate::modules::coverage::model::coverage::*;

pub struct CoverageRepository<'a> { pool: &'a PgPool }
impl<'a> CoverageRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self { Self { pool } }

    // ── Coverage Areas ──────────────────────────────────────

    pub async fn list(&self, branch_id: Option<i64>) -> Result<Vec<CoverageArea>, sqlx::Error> {
        sqlx::query_as::<_, CoverageArea>(
            "SELECT id, branch_id, name, description, area_type, pincodes, is_active, max_customers, current_customers, fiber_available, estimated_installation_days, created_at FROM coverage_areas WHERE ($1::bigint IS NULL OR branch_id = $1) AND is_active = true ORDER BY name"
        ).bind(branch_id).fetch_all(self.pool).await
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<CoverageArea>, sqlx::Error> {
        sqlx::query_as::<_, CoverageArea>(
            "SELECT id, branch_id, name, description, area_type, pincodes, is_active, max_customers, current_customers, fiber_available, estimated_installation_days, created_at FROM coverage_areas WHERE id = $1"
        ).bind(id).fetch_optional(self.pool).await
    }

    pub async fn create(&self, branch_id: i64, name: &str, description: Option<&str>, area_type: &str, fiber_available: bool, est_days: Option<i32>, max_customers: Option<i32>) -> Result<CoverageArea, sqlx::Error> {
        sqlx::query_as::<_, CoverageArea>(
            "INSERT INTO coverage_areas (branch_id, name, description, area_type, fiber_available, estimated_installation_days, max_customers) VALUES ($1,$2,$3,$4,$5,$6,$7) RETURNING id, branch_id, name, description, area_type, pincodes, is_active, max_customers, current_customers, fiber_available, estimated_installation_days, created_at"
        ).bind(branch_id).bind(name).bind(description).bind(area_type).bind(fiber_available).bind(est_days).bind(max_customers).fetch_one(self.pool).await
    }

    pub async fn update(&self, id: i64, name: Option<&str>, description: Option<&str>, area_type: Option<&str>, fiber_available: Option<bool>, est_days: Option<i32>, max_customers: Option<i32>) -> Result<CoverageArea, sqlx::Error> {
        sqlx::query_as::<_, CoverageArea>(
            "UPDATE coverage_areas SET name = COALESCE($2, name), description = COALESCE($3, description), area_type = COALESCE($4, area_type), fiber_available = COALESCE($5, fiber_available), estimated_installation_days = COALESCE($6, estimated_installation_days), max_customers = COALESCE($7, max_customers), updated_at = NOW() WHERE id = $1 RETURNING id, branch_id, name, description, area_type, pincodes, is_active, max_customers, current_customers, fiber_available, estimated_installation_days, created_at"
        ).bind(id).bind(name).bind(description).bind(area_type).bind(fiber_available).bind(est_days).bind(max_customers).fetch_one(self.pool).await
    }

    pub async fn check_pincode(&self, pincode: &str) -> Result<Option<CoverageArea>, sqlx::Error> {
        sqlx::query_as::<_, CoverageArea>(
            "SELECT ca.id, ca.branch_id, ca.name, ca.description, ca.area_type, ca.pincodes, ca.is_active, ca.max_customers, ca.current_customers, ca.fiber_available, ca.estimated_installation_days, ca.created_at FROM coverage_areas ca JOIN coverage_pincode_map cpm ON ca.id = cpm.coverage_area_id WHERE cpm.pincode = $1 AND ca.is_active = true LIMIT 1"
        ).bind(pincode).fetch_optional(self.pool).await
    }

    pub async fn deactivate(&self, id: i64) -> Result<bool, sqlx::Error> {
        let r = sqlx::query("UPDATE coverage_areas SET is_active = false WHERE id = $1").bind(id).execute(self.pool).await?;
        Ok(r.rows_affected() > 0)
    }

    // ── Pincode Management ──────────────────────────────────

    pub async fn list_pincodes(&self, area_id: i64) -> Result<Vec<CoveragePincode>, sqlx::Error> {
        sqlx::query_as::<_, CoveragePincode>(
            "SELECT id, coverage_area_id, pincode, city, district, state, is_active, created_at FROM coverage_pincode_map WHERE coverage_area_id = $1 ORDER BY pincode"
        ).bind(area_id).fetch_all(self.pool).await
    }

    pub async fn add_pincode(&self, area_id: i64, pincode: &str, city: &str, district: Option<&str>, state: Option<&str>) -> Result<CoveragePincode, sqlx::Error> {
        sqlx::query_as::<_, CoveragePincode>(
            "INSERT INTO coverage_pincode_map (coverage_area_id, pincode, city, district, state) VALUES ($1,$2,$3,$4,$5) ON CONFLICT (coverage_area_id, pincode) DO UPDATE SET is_active = true RETURNING id, coverage_area_id, pincode, city, district, state, is_active, created_at"
        ).bind(area_id).bind(pincode).bind(city).bind(district).bind(state).fetch_one(self.pool).await
    }

    pub async fn remove_pincode(&self, area_id: i64, pincode: &str) -> Result<bool, sqlx::Error> {
        let r = sqlx::query("DELETE FROM coverage_pincode_map WHERE coverage_area_id = $1 AND pincode = $2")
            .bind(area_id).bind(pincode).execute(self.pool).await?;
        Ok(r.rows_affected() > 0)
    }

    // ── Stats ───────────────────────────────────────────────

    pub async fn get_stats(&self) -> Result<CoverageStats, sqlx::Error> {
        let row = sqlx::query_as::<_, (i64, i64, i64, i64, i64)>(
            "SELECT
                (SELECT COUNT(*) FROM coverage_areas) as total_areas,
                (SELECT COUNT(*) FROM coverage_areas WHERE is_active = true) as active_areas,
                (SELECT COUNT(*) FROM coverage_pincode_map WHERE is_active = true) as total_pincodes,
                (SELECT COALESCE(SUM(current_customers), 0) FROM coverage_areas) as total_customers,
                (SELECT COUNT(*) FROM coverage_areas WHERE fiber_available = true AND is_active = true) as fiber_available_areas"
        ).fetch_one(self.pool).await?;
        Ok(CoverageStats { total_areas: row.0, active_areas: row.1, total_pincodes: row.2, total_customers: row.3, fiber_available_areas: row.4 })
    }
}
