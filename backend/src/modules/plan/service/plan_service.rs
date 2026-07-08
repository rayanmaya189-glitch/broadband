use sqlx::PgPool;

use crate::common::errors::app_error::AppError;
use crate::common::utils::helpers::PaginatedResponse;
use crate::modules::plan::mapper::plan_mapper::plan_to_response;
use crate::modules::plan::repository::plan_repository::PlanRepository;
use crate::modules::plan::request::plan_request::*;
use crate::modules::plan::response::plan_response::*;

pub struct PlanService<'a> {
    repo: PlanRepository<'a>,
}

impl<'a> PlanService<'a> {
    pub fn new(pool: &'a PgPool) -> Self { Self { repo: PlanRepository::new(pool) } }

    pub async fn list_plans(&self, query: &ListPlansQuery) -> Result<PaginatedResponse<PlanResponse>, AppError> {
        let offset = query.pagination.offset();
        let limit = query.pagination.limit_i64() as u32;
        self.repo.list(offset, limit, query.is_active, query.category.as_deref()).await
    }

    pub async fn get_plan(&self, id: i64) -> Result<PlanDetailResponse, AppError> {
        let p = self.repo.find_by_id(id).await?.ok_or_else(|| AppError::NotFound("Plan not found".into()))?;
        Ok(plan_to_response(&p))
    }

    pub async fn create_plan(&self, req: &CreatePlanRequest) -> Result<PlanDetailResponse, AppError> {
        if self.repo.code_exists(&req.code, None).await? {
            return Err(AppError::Conflict("Plan code already exists".into()));
        }
        let gst = req.gst_percent.unwrap_or_else(|| rust_decimal_macros::dec!(18.00));
        let p = self.repo.create(&req.name, &req.code, req.description.as_deref(), req.speed_down_mbps, req.speed_up_mbps, req.data_cap_gb, req.price_monthly, req.price_quarterly, req.price_half_yearly, req.price_yearly, gst, req.is_promotional.unwrap_or(false), req.category.as_deref().unwrap_or("standard")).await?;
        Ok(plan_to_response(&p))
    }

    pub async fn update_plan(&self, id: i64, req: &UpdatePlanRequest) -> Result<PlanDetailResponse, AppError> {
        if self.repo.find_by_id(id).await?.is_none() {
            return Err(AppError::NotFound("Plan not found".into()));
        }
        let p = self.repo.update(id, req.name.as_deref(), req.description.as_deref(), req.speed_down_mbps, req.speed_up_mbps, req.data_cap_gb, req.price_monthly, req.price_quarterly, req.price_half_yearly, req.price_yearly, req.gst_percent, req.is_active, req.is_promotional, req.category.as_deref()).await?;
        Ok(plan_to_response(&p))
    }

    pub async fn delete_plan(&self, id: i64) -> Result<MessageResponse, AppError> {
        if self.repo.find_by_id(id).await?.is_none() {
            return Err(AppError::NotFound("Plan not found".into()));
        }
        self.repo.soft_delete(id).await?;
        Ok(MessageResponse { message: "Plan deactivated successfully".into() })
    }
}
