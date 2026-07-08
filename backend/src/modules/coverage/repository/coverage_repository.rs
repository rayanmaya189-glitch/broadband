use sqlx::PgPool;
use crate::modules::coverage::model::coverage::CoverageArea;

pub struct CoverageRepository<'a> { pool: &'a PgPool }
impl<'a> CoverageRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self { Self { pool } }
    pub fn pool(&self) -> &'a PgPool { self.pool }

    pub async fn list(&self, branch_id: Option<i64>) -> Result<Vec<CoverageArea>, sqlx::Error> {
        sqlx::query_as::<_, CoverageArea>("SELECT * FROM coverage_areas WHERE ($1::bigint IS NULL OR branch_id = $1) AND is_active = true ORDER BY name")
            .bind(branch_id).fetch_all(self.pool).await
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<CoverageArea>, sqlx::Error> {
        sqlx::query_as::<_, CoverageArea>("SELECT * FROM coverage_areas WHERE id = $1").bind(id).fetch_optional(self.pool).await
    }

    pub async fn create(&self, branch_id: i64, name: &str, description: Option<&str>, area_type: &str, fiber_available: bool, est_days: Option<i32>, max_customers: Option<i32>) -> Result<CoverageArea, sqlx::Error> {
        sqlx::query_as::<_, CoverageArea>("INSERT INTO coverage_areas (branch_id, name, description, area_type, fiber_available, estimated_installation_days, max_customers) VALUES ($1,$2,$3,$4,$5,$6,$7) RETURNING *")
            .bind(branch_id).bind(name).bind(description).bind(area_type).bind(fiber_available).bind(est_days).bind(max_customers).fetch_one(self.pool).await
    }

    pub async fn check_pincode(&self, pincode: &str) -> Result<Option<CoverageArea>, sqlx::Error> {
        sqlx::query_as::<_, CoverageArea>("SELECT ca.* FROM coverage_areas ca JOIN coverage_pincode_map cpm ON ca.id = cpm.coverage_area_id WHERE cpm.pincode = $1 AND ca.is_active = true LIMIT 1")
            .bind(pincode).fetch_optional(self.pool).await
    }

    pub async fn delete(&self, id: i64) -> Result<bool, sqlx::Error> {
        let r = sqlx::query("UPDATE coverage_areas SET is_active = false WHERE id = $1").bind(id).execute(self.pool).await?;
        Ok(r.rows_affected() > 0)
    }
}
