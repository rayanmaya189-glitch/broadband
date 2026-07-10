//! SeaORM-based service for the Discovery domain.

use sea_orm::DatabaseConnection;

use crate::common::errors::app_error::AppError;
use crate::modules::discovery::repository::discovery_repository_seaorm::DiscoveryRepositorySeaorm;
use crate::modules::discovery::request::discovery_request::*;
use crate::modules::discovery::response::discovery_response::*;

pub struct DiscoveryServiceSeaorm<'a> {
    repo: DiscoveryRepositorySeaorm<'a>,
}

impl<'a> DiscoveryServiceSeaorm<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { repo: DiscoveryRepositorySeaorm::new(db) }
    }

    pub async fn list_scans(&self, branch_id: Option<i64>) -> Result<Vec<DiscoveryScanResponse>, AppError> {
        let scans = self.repo.list_scans(branch_id).await?;
        Ok(scans.into_iter().map(|s| DiscoveryScanResponse { id: s.id, branch_id: s.branch_id, name: s.name, scan_type: s.scan_type, is_active: s.is_active, last_scan_at: s.last_scan_at.map(|v| v.into()), created_at: s.created_at.into() }).collect())
    }

    pub async fn create_scan(&self, req: CreateDiscoveryScanRequest) -> Result<DiscoveryScanResponse, AppError> {
        let s = self.repo.create_scan(req.branch_id, &req.name, &req.scan_type).await?;
        Ok(DiscoveryScanResponse { id: s.id, branch_id: s.branch_id, name: s.name, scan_type: s.scan_type, is_active: s.is_active, last_scan_at: s.last_scan_at.map(|v| v.into()), created_at: s.created_at.into() })
    }

    pub async fn list_results(&self, status: Option<&str>, branch_id: Option<i64>) -> Result<Vec<DiscoveryResultResponse>, AppError> {
        let results = self.repo.list_results(status, branch_id).await?;
        Ok(results.into_iter().map(|r| DiscoveryResultResponse { id: r.id, scan_id: r.scan_id, discovered_ip: r.discovered_ip, vendor: r.vendor, model: r.model, firmware_version: r.firmware_version, status: r.status, discovered_at: r.discovered_at.into() }).collect())
    }

    pub async fn approve_result(&self, id: i64, reviewed_by: i64) -> Result<DiscoveryResultResponse, AppError> {
        let r = self.repo.approve_result(id, reviewed_by).await?;
        Ok(DiscoveryResultResponse { id: r.id, scan_id: r.scan_id, discovered_ip: r.discovered_ip, vendor: r.vendor, model: r.model, firmware_version: r.firmware_version, status: r.status, discovered_at: r.discovered_at.into() })
    }

    pub async fn reject_result(&self, id: i64, reviewed_by: i64, reason: &str) -> Result<DiscoveryResultResponse, AppError> {
        let r = self.repo.reject_result(id, reviewed_by, reason).await?;
        Ok(DiscoveryResultResponse { id: r.id, scan_id: r.scan_id, discovered_ip: r.discovered_ip, vendor: r.vendor, model: r.model, firmware_version: r.firmware_version, status: r.status, discovered_at: r.discovered_at.into() })
    }
}
