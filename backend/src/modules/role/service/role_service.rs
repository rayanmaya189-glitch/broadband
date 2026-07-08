use sqlx::PgPool;

use crate::common::errors::app_error::AppError;
use crate::common::utils::helpers::PaginatedResponse;
use crate::modules::role::mapper::role_mapper::role_to_response;
use crate::modules::role::repository::role_repository::RoleRepository;
use crate::modules::role::request::role_request::*;
use crate::modules::role::response::role_response::*;

pub struct RoleService<'a> {
    repo: RoleRepository<'a>,
}

impl<'a> RoleService<'a> {
    pub fn new(pool: &'a PgPool) -> Self { Self { repo: RoleRepository::new(pool) } }

    pub async fn list_roles(&self, query: &ListRolesQuery) -> Result<PaginatedResponse<RoleResponse>, AppError> {
        let offset = query.pagination.offset();
        let limit = query.pagination.limit_i64() as u32;
        self.repo.list(offset, limit, query.is_active).await
    }

    pub async fn get_role(&self, role_id: i64) -> Result<RoleDetailResponse, AppError> {
        let role = self.repo.find_by_id(role_id).await?.ok_or_else(|| AppError::NotFound("Role not found".into()))?;
        Ok(role_to_response(&role))
    }

    pub async fn create_role(&self, req: &CreateRoleRequest) -> Result<RoleDetailResponse, AppError> {
        if self.repo.name_exists(&req.name, None).await? { return Err(AppError::Conflict("Role name already exists".into())); }
        let role = self.repo.create(&req.name, &req.display_name, req.description.as_deref()).await?;
        Ok(role_to_response(&role))
    }

    pub async fn update_role(&self, role_id: i64, req: &UpdateRoleRequest) -> Result<RoleDetailResponse, AppError> {
        let existing = self.repo.find_by_id(role_id).await?.ok_or_else(|| AppError::NotFound("Role not found".into()))?;
        if existing.is_system { return Err(AppError::Forbidden("Cannot modify system role".into())); }
        if let Some(ref name) = req.name {
            if self.repo.name_exists(name, Some(role_id)).await? { return Err(AppError::Conflict("Role name already exists".into())); }
        }
        let role = self.repo.update(role_id, req.name.as_deref(), req.display_name.as_deref(), req.description.as_deref()).await?;
        Ok(role_to_response(&role))
    }

    pub async fn deactivate_role(&self, role_id: i64) -> Result<MessageResponse, AppError> {
        let existing = self.repo.find_by_id(role_id).await?.ok_or_else(|| AppError::NotFound("Role not found".into()))?;
        if existing.is_system { return Err(AppError::Forbidden("Cannot deactivate system role".into())); }
        self.repo.deactivate(role_id).await?;
        Ok(MessageResponse { message: "Role deactivated successfully".into() })
    }
}
