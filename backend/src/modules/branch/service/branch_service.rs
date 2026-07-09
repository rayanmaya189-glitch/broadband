use sqlx::PgPool;

use crate::common::errors::app_error::AppError;
use crate::common::utils::helpers::PaginatedResponse;
use crate::modules::branch::mapper::branch_mapper::branch_to_response;
use crate::modules::branch::repository::branch_repository::BranchRepository;
use crate::modules::branch::request::branch_request::*;
use crate::modules::branch::response::branch_response::*;

pub struct BranchService<'a> {
    repo: BranchRepository<'a>,
}

impl<'a> BranchService<'a> {
    pub fn new(pool: &'a PgPool) -> Self { Self { repo: BranchRepository::new(pool) } }

    pub async fn list_branches(&self, query: &ListBranchesQuery) -> Result<PaginatedResponse<BranchResponse>, AppError> {
        let offset = query.pagination.offset();
        let limit = query.pagination.limit_i64() as u32;
        self.repo.list(offset, limit, query.is_active, query.city.as_deref(), query.pagination.search.as_deref()).await
    }

    pub async fn get_branch(&self, id: i64) -> Result<BranchDetailResponse, AppError> {
        let b = self.repo.find_by_id(id).await?.ok_or_else(|| AppError::NotFound("Branch not found".into()))?;
        Ok(branch_to_response(&b))
    }

    pub async fn create_branch(&self, req: &CreateBranchRequest) -> Result<BranchDetailResponse, AppError> {
        if self.repo.code_exists(&req.code, None).await? { return Err(AppError::Conflict("Branch code already exists".into())); }
        let tz = req.timezone.as_deref().unwrap_or("Asia/Kolkata");
        let b = self.repo.create(&req.name, &req.code, req.address.as_deref(), req.city.as_deref(), req.state.as_deref(), req.pincode.as_deref(), req.phone.as_deref(), req.email.as_deref(), tz).await?;
        Ok(branch_to_response(&b))
    }

    pub async fn update_branch(&self, id: i64, req: &UpdateBranchRequest) -> Result<BranchDetailResponse, AppError> {
        if self.repo.find_by_id(id).await?.is_none() { return Err(AppError::NotFound("Branch not found".into())); }
        let b = self.repo.update(id, req.name.as_deref(), req.address.as_deref(), req.city.as_deref(), req.state.as_deref(), req.pincode.as_deref(), req.phone.as_deref(), req.email.as_deref(), req.timezone.as_deref()).await?;
        Ok(branch_to_response(&b))
    }

    pub async fn deactivate_branch(&self, id: i64) -> Result<MessageResponse, AppError> {
        let _ = self.repo.find_by_id(id).await?.ok_or_else(|| AppError::NotFound("Branch not found".into()))?;
        let count = self.repo.count_active_customers(id).await?;
        if count > 0 { return Err(AppError::Validation("Cannot deactivate branch with active customers".into())); }
        self.repo.deactivate(id).await?;
        Ok(MessageResponse { message: "Branch deactivated successfully".into() })
    }

    // ── Working Hours ──────────────────────────────────────

    pub async fn get_working_hours(&self, branch_id: i64) -> Result<Vec<WorkingHourResponse>, AppError> {
        if self.repo.find_by_id(branch_id).await?.is_none() { return Err(AppError::NotFound("Branch not found".into())); }
        self.repo.get_working_hours(branch_id).await
    }

    pub async fn update_working_hours(&self, branch_id: i64, req: &UpdateWorkingHoursRequest) -> Result<Vec<WorkingHourResponse>, AppError> {
        if self.repo.find_by_id(branch_id).await?.is_none() { return Err(AppError::NotFound("Branch not found".into())); }
        self.repo.upsert_working_hours(branch_id, &req.hours).await
    }

    // ── User-Branch Assignment ─────────────────────────────

    pub async fn assign_user(&self, branch_id: i64, req: &AssignUserToBranchRequest) -> Result<MessageResponse, AppError> {
        if self.repo.find_by_id(branch_id).await?.is_none() { return Err(AppError::NotFound("Branch not found".into())); }
        self.repo.assign_user(branch_id, req.user_id, req.is_primary.unwrap_or(false)).await?;
        Ok(MessageResponse { message: "User assigned to branch successfully".into() })
    }

    pub async fn remove_user(&self, branch_id: i64, user_id: i64) -> Result<MessageResponse, AppError> {
        self.repo.remove_user(branch_id, user_id).await?;
        Ok(MessageResponse { message: "User removed from branch".into() })
    }

    pub async fn list_branch_users(&self, branch_id: i64) -> Result<Vec<BranchUserResponse>, AppError> {
        self.repo.list_branch_users(branch_id).await
    }

    pub async fn get_branch_stats(&self, branch_id: i64) -> Result<BranchStatsResponse, AppError> {
        let (total_customers, active_customers, total_subscriptions, active_subscriptions) = self.repo.get_branch_stats(branch_id).await?;
        Ok(BranchStatsResponse { branch_id, total_customers, active_customers, total_subscriptions, active_subscriptions })
    }
}
