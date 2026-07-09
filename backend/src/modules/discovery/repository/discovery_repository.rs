use sqlx::PgPool;
use crate::modules::discovery::model::discovery::{DiscoveryScan, DiscoveryResult};

pub struct DiscoveryRepository<'a> { pool: &'a PgPool }
impl<'a> DiscoveryRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self { Self { pool } }

    // ── Scans ──────────────────────────────────────────────

    pub async fn list_scans(&self, branch_id: Option<i64>) -> Result<Vec<DiscoveryScan>, sqlx::Error> {
        sqlx::query_as::<_, DiscoveryScan>("SELECT * FROM discovery_scans WHERE ($1::bigint IS NULL OR branch_id = $1) ORDER BY created_at DESC")
            .bind(branch_id).fetch_all(self.pool).await
    }

    pub async fn create_scan(&self, branch_id: i64, name: &str, scan_type: &str) -> Result<DiscoveryScan, sqlx::Error> {
        sqlx::query_as::<_, DiscoveryScan>("INSERT INTO discovery_scans (branch_id, name, scan_type) VALUES ($1,$2,$3) RETURNING *")
            .bind(branch_id).bind(name).bind(scan_type).fetch_one(self.pool).await
    }

    pub async fn update_scan_active(&self, id: i64, is_active: bool) -> Result<bool, sqlx::Error> {
        let r = sqlx::query("UPDATE discovery_scans SET is_active = $2, updated_at = NOW() WHERE id = $1")
            .bind(id).bind(is_active).execute(self.pool).await?;
        Ok(r.rows_affected() > 0)
    }

    // ── Results ────────────────────────────────────────────

    pub async fn list_results(&self, status: Option<&str>, branch_id: Option<i64>) -> Result<Vec<DiscoveryResult>, sqlx::Error> {
        sqlx::query_as::<_, DiscoveryResult>(
            "SELECT dr.* FROM discovery_results dr
             JOIN discovery_scans ds ON ds.id = dr.scan_id
             WHERE ($1::text IS NULL OR dr.status = $1) AND ($2::bigint IS NULL OR ds.branch_id = $2)
             ORDER BY dr.discovered_at DESC"
        ).bind(status).bind(branch_id).fetch_all(self.pool).await
    }

    pub async fn approve_result(&self, id: i64, reviewed_by: i64) -> Result<DiscoveryResult, sqlx::Error> {
        sqlx::query_as::<_, DiscoveryResult>("UPDATE discovery_results SET status = 'approved', reviewed_by = $2, reviewed_at = NOW() WHERE id = $1 AND status = 'pending' RETURNING *")
            .bind(id).bind(reviewed_by).fetch_one(self.pool).await
    }

    pub async fn reject_result(&self, id: i64, reviewed_by: i64, reason: &str) -> Result<DiscoveryResult, sqlx::Error> {
        sqlx::query_as::<_, DiscoveryResult>("UPDATE discovery_results SET status = 'rejected', reviewed_by = $2, reviewed_at = NOW(), rejection_reason = $3 WHERE id = $1 AND status = 'pending' RETURNING *")
            .bind(id).bind(reviewed_by).bind(reason).fetch_one(self.pool).await
    }

    // ── Dashboard ──────────────────────────────────────────

    pub async fn get_dashboard_stats(&self) -> Result<(i64, i64, i64, i64), sqlx::Error> {
        sqlx::query_as(
            "SELECT
                COUNT(*) FILTER (WHERE status = 'pending'),
                COUNT(*) FILTER (WHERE status = 'approved'),
                COUNT(*) FILTER (WHERE status = 'rejected'),
                COUNT(*) FILTER (WHERE discovered_at >= NOW() - INTERVAL '24 hours')
             FROM discovery_results"
        ).fetch_one(self.pool).await
    }

    pub async fn get_vendor_counts(&self) -> Result<Vec<(String, i64)>, sqlx::Error> {
        sqlx::query_as("SELECT COALESCE(vendor, 'Unknown'), COUNT(*) FROM discovery_results GROUP BY vendor ORDER BY count DESC")
            .fetch_all(self.pool).await
    }
}
