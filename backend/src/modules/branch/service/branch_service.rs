use sea_orm::DatabaseConnection;

use crate::common::errors::app_error::AppError;
use crate::common::utils::helpers::PaginatedResponse;
use crate::modules::branch::repository::branch_repository::BranchRepository;
use crate::modules::branch::request::branch_request::*;
use crate::modules::branch::response::branch_response::*;

pub struct BranchService {
    repo: BranchRepository,
}

impl BranchService {
    pub fn new(db: &DatabaseConnection) -> Self {
        Self {
            repo: BranchRepository::new(db),
        }
    }

    pub async fn list_branches(
        &self,
        query: &ListBranchesQuery,
    ) -> Result<PaginatedResponse<BranchResponse>, AppError> {
        let offset = query.pagination.offset();
        let limit = query.pagination.limit;
        self.repo
            .list(
                offset,
                limit,
                query.is_active,
                query.city.as_deref(),
                query.pagination.search.as_deref(),
            )
            .await
    }

    pub async fn get_branch(&self, id: i64) -> Result<BranchDetailResponse, AppError> {
        let model = self
            .repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound("Branch not found".into()))?;
        Ok(BranchResponse::from_model(model))
    }

    pub async fn create_branch(
        &self,
        req: &CreateBranchRequest,
    ) -> Result<BranchDetailResponse, AppError> {
        if self.repo.code_exists(&req.code, None).await? {
            return Err(AppError::Conflict("Branch code already exists".into()));
        }
        let tz = req.timezone.as_deref().unwrap_or("Asia/Kolkata");
        let model = self
            .repo
            .create(
                &req.name,
                &req.code,
                req.address.as_deref(),
                req.city.as_deref(),
                req.state.as_deref(),
                req.pincode.as_deref(),
                req.phone.as_deref(),
                req.email.as_deref(),
                tz,
            )
            .await?;
        Ok(BranchResponse::from_model(model))
    }

    pub async fn update_branch(
        &self,
        id: i64,
        req: &UpdateBranchRequest,
    ) -> Result<BranchDetailResponse, AppError> {
        if self.repo.find_by_id(id).await?.is_none() {
            return Err(AppError::NotFound("Branch not found".into()));
        }
        let model = self
            .repo
            .update(
                id,
                req.name.as_deref(),
                req.address.as_deref(),
                req.city.as_deref(),
                req.state.as_deref(),
                req.pincode.as_deref(),
                req.phone.as_deref(),
                req.email.as_deref(),
                req.timezone.as_deref(),
            )
            .await?;
        Ok(BranchResponse::from_model(model))
    }

    pub async fn deactivate_branch(&self, id: i64) -> Result<MessageResponse, AppError> {
        if self.repo.find_by_id(id).await?.is_none() {
            return Err(AppError::NotFound("Branch not found".into()));
        }
        self.repo.deactivate(id).await?;
        Ok(MessageResponse {
            message: "Branch deactivated successfully".into(),
        })
    }

    // ── Working Hours ──────────────────────────────────────

    pub async fn get_working_hours(&self, branch_id: i64) -> Result<Vec<WorkingHourResponse>, AppError> {
        if self.repo.find_by_id(branch_id).await?.is_none() {
            return Err(AppError::NotFound("Branch not found".into()));
        }
        let models = self.repo.get_working_hours(branch_id).await?;
        Ok(models.into_iter().map(WorkingHourResponse::from_model).collect())
    }

    pub async fn update_working_hours(
        &self,
        branch_id: i64,
        req: &UpdateWorkingHoursRequest,
    ) -> Result<Vec<WorkingHourResponse>, AppError> {
        if self.repo.find_by_id(branch_id).await?.is_none() {
            return Err(AppError::NotFound("Branch not found".into()));
        }
        for h in &req.hours {
            let open_time = chrono::NaiveTime::parse_from_str(&h.open_time, "%H:%M").ok();
            let close_time = chrono::NaiveTime::parse_from_str(&h.close_time, "%H:%M").ok();
            self.repo
                .upsert_working_hours(branch_id, h.day_of_week, open_time, close_time, h.is_closed)
                .await?;
        }
        let models = self.repo.get_working_hours(branch_id).await?;
        Ok(models.into_iter().map(WorkingHourResponse::from_model).collect())
    }

    // ── User-Branch Assignment ─────────────────────────────

    pub async fn assign_user(
        &self,
        branch_id: i64,
        req: &AssignUserToBranchRequest,
    ) -> Result<MessageResponse, AppError> {
        if self.repo.find_by_id(branch_id).await?.is_none() {
            return Err(AppError::NotFound("Branch not found".into()));
        }
        self.repo
            .assign_user(branch_id, req.user_id, req.is_primary.unwrap_or(false))
            .await?;
        Ok(MessageResponse {
            message: "User assigned to branch successfully".into(),
        })
    }

    pub async fn remove_user(&self, branch_id: i64, user_id: i64) -> Result<MessageResponse, AppError> {
        self.repo.remove_user(branch_id, user_id).await?;
        Ok(MessageResponse {
            message: "User removed from branch".into(),
        })
    }

    pub async fn list_branch_users(&self, branch_id: i64) -> Result<Vec<BranchUserResponse>, AppError> {
        let models = self.repo.list_branch_users(branch_id).await?;
        Ok(models.into_iter().map(BranchUserResponse::from_model).collect())
    }

    pub async fn get_branch_stats(&self, branch_id: i64) -> Result<BranchStatsResponse, AppError> {
        self.repo.get_branch_stats(branch_id).await
    }
}
