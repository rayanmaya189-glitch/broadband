use sqlx::PgPool;
use crate::common::errors::app_error::AppError;
use crate::modules::discovery::repository::discovery_repository::DiscoveryRepository;
use crate::modules::discovery::request::discovery_request::*;
use crate::modules::discovery::response::discovery_response::*;

pub struct DiscoveryService<'a> { repo: DiscoveryRepository<'a> }
impl<'a> DiscoveryService<'a> {
    pub fn new(pool: &'a PgPool) -> Self { Self { repo: DiscoveryRepository::new(pool) } }

    pub async fn list_scans(&self) -> Result<Vec<ScanResponse>, AppError> {
        let s = self.repo.list_scans().await?;
        Ok(s.iter().map(|x| ScanResponse { id: x.id, branch_id: x.branch_id, name: x.name.clone(), scan_type: x.scan_type.clone(), is_active: x.is_active, created_at: x.created_at }).collect())
    }

    pub async fn create_scan(&self, req: CreateScanRequest) -> Result<ScanResponse, AppError> {
        let s = self.repo.create_scan(req.branch_id, &req.name, &req.scan_type).await?;
        Ok(ScanResponse { id: s.id, branch_id: s.branch_id, name: s.name, scan_type: s.scan_type, is_active: s.is_active, created_at: s.created_at })
    }

    pub async fn list_results(&self) -> Result<Vec<ResultResponse>, AppError> {
        let r = self.repo.list_results().await?;
        Ok(r.iter().map(|x| ResultResponse { id: x.id, discovered_ip: x.discovered_ip.clone(), vendor: x.vendor.clone(), model: x.model.clone(), status: x.status.clone(), discovered_at: x.discovered_at }).collect())
    }
}
