use sqlx::PgPool;
use crate::modules::discovery::model::discovery::{DiscoveryScan, DiscoveryResult};

pub struct DiscoveryRepository<'a> { pool: &'a PgPool }
impl<'a> DiscoveryRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self { Self { pool } }

    pub async fn list_scans(&self) -> Result<Vec<DiscoveryScan>, sqlx::Error> {
        sqlx::query_as::<_, DiscoveryScan>("SELECT * FROM discovery_scans ORDER BY created_at DESC")
            .fetch_all(self.pool).await
    }

    pub async fn create_scan(&self, branch_id: i64, name: &str, scan_type: &str) -> Result<DiscoveryScan, sqlx::Error> {
        sqlx::query_as::<_, DiscoveryScan>("INSERT INTO discovery_scans (branch_id, name, scan_type) VALUES ($1,$2,$3) RETURNING *")
            .bind(branch_id).bind(name).bind(scan_type).fetch_one(self.pool).await
    }

    pub async fn list_results(&self) -> Result<Vec<DiscoveryResult>, sqlx::Error> {
        sqlx::query_as::<_, DiscoveryResult>("SELECT * FROM discovery_results ORDER BY discovered_at DESC")
            .fetch_all(self.pool).await
    }
}
