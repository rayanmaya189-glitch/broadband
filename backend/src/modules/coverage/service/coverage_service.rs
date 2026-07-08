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
        Ok(areas.iter().map(|a| CoverageAreaResponse { id: a.id, branch_id: a.branch_id, name: a.name.clone(), description: a.description.clone(), area_type: a.area_type.clone(), is_active: a.is_active, fiber_available: a.fiber_available, estimated_installation_days: a.estimated_installation_days, current_customers: a.current_customers, created_at: a.created_at }).collect())
    }

    pub async fn create_area(&self, req: CreateCoverageAreaRequest) -> Result<CoverageAreaResponse, AppError> {
        let a = self.repo.create(req.branch_id, &req.name, req.description.as_deref(), &req.area_type, req.fiber_available.unwrap_or(true), req.estimated_installation_days, req.max_customers).await?;
        Ok(CoverageAreaResponse { id: a.id, branch_id: a.branch_id, name: a.name, description: a.description, area_type: a.area_type, is_active: a.is_active, fiber_available: a.fiber_available, estimated_installation_days: a.estimated_installation_days, current_customers: a.current_customers, created_at: a.created_at })
    }

    pub async fn check_availability(&self, req: CheckAvailabilityRequest) -> Result<AvailabilityCheckResponse, AppError> {
        match self.repo.check_pincode(&req.pincode).await? {
            Some(area) => Ok(AvailabilityCheckResponse { available: true, area_name: Some(area.name), estimated_days: area.estimated_installation_days, message: "Service available".into() }),
            None => Ok(AvailabilityCheckResponse { available: false, area_name: None, estimated_days: None, message: "Service not yet available in this area".into() }),
        }
    }

    pub async fn delete_area(&self, id: i64) -> Result<MessageResponse, AppError> {
        if !self.repo.delete(id).await? { return Err(AppError::NotFound("Coverage area not found".into())); }
        Ok(MessageResponse { message: "Area deactivated".into() })
    }
}
