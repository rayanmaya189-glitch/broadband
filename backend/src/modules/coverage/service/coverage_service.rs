use sqlx::PgPool;
use crate::common::errors::app_error::AppError;
use crate::modules::coverage::repository::coverage_repository::CoverageRepository;
use crate::modules::coverage::request::coverage_request::*;
use crate::modules::coverage::response::coverage_response::*;

pub struct CoverageService<'a> { repo: CoverageRepository<'a> }
impl<'a> CoverageService<'a> {
    pub fn new(pool: &'a PgPool) -> Self { Self { repo: CoverageRepository::new(pool) } }

    pub async fn list_areas(&self, branch_id: Option<i64>) -> Result<Vec<CoverageAreaResponse>, AppError> {
        let areas = self.repo.list(branch_id).await?;
        Ok(areas.into_iter().map(|a| CoverageAreaResponse { id: a.id, branch_id: a.branch_id, name: a.name, description: a.description, area_type: a.area_type, is_active: a.is_active, fiber_available: a.fiber_available, estimated_installation_days: a.estimated_installation_days, current_customers: a.current_customers, created_at: a.created_at }).collect())
    }

    pub async fn get_area(&self, id: i64) -> Result<CoverageAreaResponse, AppError> {
        let a = self.repo.get_by_id(id).await?.ok_or_else(|| AppError::NotFound("Coverage area not found".into()))?;
        Ok(CoverageAreaResponse { id: a.id, branch_id: a.branch_id, name: a.name, description: a.description, area_type: a.area_type, is_active: a.is_active, fiber_available: a.fiber_available, estimated_installation_days: a.estimated_installation_days, current_customers: a.current_customers, created_at: a.created_at })
    }

    pub async fn create_area(&self, req: CreateCoverageAreaRequest) -> Result<CoverageAreaResponse, AppError> {
        let a = self.repo.create(req.branch_id, &req.name, req.description.as_deref(), &req.area_type, req.fiber_available.unwrap_or(true), req.estimated_installation_days, req.max_customers).await?;
        Ok(CoverageAreaResponse { id: a.id, branch_id: a.branch_id, name: a.name, description: a.description, area_type: a.area_type, is_active: a.is_active, fiber_available: a.fiber_available, estimated_installation_days: a.estimated_installation_days, current_customers: a.current_customers, created_at: a.created_at })
    }

    pub async fn update_area(&self, id: i64, req: UpdateCoverageAreaRequest) -> Result<CoverageAreaResponse, AppError> {
        let a = self.repo.update(id, req.name.as_deref(), req.description.as_deref(), req.area_type.as_deref(), req.fiber_available, req.estimated_installation_days, req.max_customers).await.map_err(|_| AppError::NotFound("Coverage area not found".into()))?;
        Ok(CoverageAreaResponse { id: a.id, branch_id: a.branch_id, name: a.name, description: a.description, area_type: a.area_type, is_active: a.is_active, fiber_available: a.fiber_available, estimated_installation_days: a.estimated_installation_days, current_customers: a.current_customers, created_at: a.created_at })
    }

    pub async fn check_availability(&self, req: CheckAvailabilityRequest) -> Result<AvailabilityCheckResponse, AppError> {
        match self.repo.check_pincode(&req.pincode).await? {
            Some(area) => Ok(AvailabilityCheckResponse { available: true, area_name: Some(area.name), estimated_days: area.estimated_installation_days, message: "Service available".into() }),
            None => Ok(AvailabilityCheckResponse { available: false, area_name: None, estimated_days: None, message: "Service not yet available in this area".into() }),
        }
    }

    pub async fn delete_area(&self, id: i64) -> Result<MessageResponse, AppError> {
        if !self.repo.deactivate(id).await? { return Err(AppError::NotFound("Coverage area not found".into())); }
        Ok(MessageResponse { message: "Area deactivated".into() })
    }

    // ── Pincode Management ──────────────────────────────────

    pub async fn list_pincodes(&self, area_id: i64) -> Result<Vec<CoveragePincodeResponse>, AppError> {
        self.repo.get_by_id(area_id).await?.ok_or_else(|| AppError::NotFound("Coverage area not found".into()))?;
        let pincodes = self.repo.list_pincodes(area_id).await?;
        Ok(pincodes.into_iter().map(|p| CoveragePincodeResponse { id: p.id, pincode: p.pincode, city: p.city, district: p.district, state: p.state, is_active: p.is_active, created_at: p.created_at }).collect())
    }

    pub async fn add_pincode(&self, area_id: i64, req: AddPincodeRequest) -> Result<CoveragePincodeResponse, AppError> {
        self.repo.get_by_id(area_id).await?.ok_or_else(|| AppError::NotFound("Coverage area not found".into()))?;
        let p = self.repo.add_pincode(area_id, &req.pincode, &req.city, req.district.as_deref(), req.state.as_deref()).await?;
        Ok(CoveragePincodeResponse { id: p.id, pincode: p.pincode, city: p.city, district: p.district, state: p.state, is_active: p.is_active, created_at: p.created_at })
    }

    pub async fn remove_pincode(&self, area_id: i64, pincode: &str) -> Result<MessageResponse, AppError> {
        if !self.repo.remove_pincode(area_id, pincode).await? { return Err(AppError::NotFound("Pincode not found".into())); }
        Ok(MessageResponse { message: "Pincode removed".into() })
    }

    // ── Stats ───────────────────────────────────────────────

    pub async fn get_stats(&self) -> Result<CoverageStatsResponse, AppError> {
        let s = self.repo.get_stats().await?;
        Ok(CoverageStatsResponse { total_areas: s.total_areas, active_areas: s.active_areas, total_pincodes: s.total_pincodes, total_customers: s.total_customers, fiber_available_areas: s.fiber_available_areas })
    }
}
