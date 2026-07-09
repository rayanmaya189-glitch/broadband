use sqlx::PgPool;
use crate::common::errors::app_error::AppError;
use crate::modules::discovery::repository::discovery_repository::DiscoveryRepository;
use crate::modules::discovery::request::discovery_request::*;
use crate::modules::discovery::response::discovery_response::*;

pub struct DiscoveryService<'a> { repo: DiscoveryRepository<'a> }
impl<'a> DiscoveryService<'a> {
    pub fn new(pool: &'a PgPool) -> Self { Self { repo: DiscoveryRepository::new(pool) } }

    pub async fn list_scans(&self, branch_id: Option<i64>) -> Result<Vec<ScanResponse>, AppError> {
        let s = self.repo.list_scans(branch_id).await?;
        Ok(s.iter().map(|x| ScanResponse { id: x.id, branch_id: x.branch_id, name: x.name.clone(), scan_type: x.scan_type.clone(), is_active: x.is_active, last_scan_at: x.last_scan_at, created_at: x.created_at }).collect())
    }

    pub async fn create_scan(&self, req: CreateScanRequest) -> Result<ScanResponse, AppError> {
        let s = self.repo.create_scan(req.branch_id, &req.name, &req.scan_type).await?;
        Ok(ScanResponse { id: s.id, branch_id: s.branch_id, name: s.name, scan_type: s.scan_type, is_active: s.is_active, last_scan_at: s.last_scan_at, created_at: s.created_at })
    }

    pub async fn start_scan(&self, id: i64) -> Result<MessageResponse, AppError> {
        if !self.repo.update_scan_active(id, true).await? { return Err(AppError::NotFound("Scan not found".into())); }
        Ok(MessageResponse { message: "Scan started".into() })
    }

    pub async fn stop_scan(&self, id: i64) -> Result<MessageResponse, AppError> {
        if !self.repo.update_scan_active(id, false).await? { return Err(AppError::NotFound("Scan not found".into())); }
        Ok(MessageResponse { message: "Scan stopped".into() })
    }

    pub async fn list_results(&self, query: DiscoveryQuery) -> Result<Vec<ResultResponse>, AppError> {
        let r = self.repo.list_results(query.status.as_deref(), query.branch_id).await?;
        Ok(r.iter().map(|x| ResultResponse { id: x.id, scan_id: x.scan_id, discovered_ip: x.discovered_ip.clone(), vendor: x.vendor.clone(), model: x.model.clone(), firmware_version: x.firmware_version.clone(), status: x.status.clone(), discovered_at: x.discovered_at }).collect())
    }

    pub async fn approve_result(&self, id: i64, reviewed_by: i64) -> Result<ResultResponse, AppError> {
        let r = self.repo.approve_result(id, reviewed_by).await.map_err(|_| AppError::NotFound("Result not found or already processed".into()))?;
        Ok(ResultResponse { id: r.id, scan_id: r.scan_id, discovered_ip: r.discovered_ip, vendor: r.vendor, model: r.model, firmware_version: r.firmware_version, status: r.status, discovered_at: r.discovered_at })
    }

    pub async fn reject_result(&self, id: i64, reviewed_by: i64, reason: &str) -> Result<ResultResponse, AppError> {
        let r = self.repo.reject_result(id, reviewed_by, reason).await.map_err(|_| AppError::NotFound("Result not found or already processed".into()))?;
        Ok(ResultResponse { id: r.id, scan_id: r.scan_id, discovered_ip: r.discovered_ip, vendor: r.vendor, model: r.model, firmware_version: r.firmware_version, status: r.status, discovered_at: r.discovered_at })
    }

    pub async fn get_dashboard(&self) -> Result<DashboardResponse, AppError> {
        let (pending, approved, rejected, recent_24h) = self.repo.get_dashboard_stats().await?;
        let by_vendor: Vec<VendorCount> = self.repo.get_vendor_counts().await?.into_iter().map(|(v, c)| VendorCount { vendor: v, count: c }).collect();
        Ok(DashboardResponse { pending, approved, rejected, recent_24h, by_vendor })
    }
}
