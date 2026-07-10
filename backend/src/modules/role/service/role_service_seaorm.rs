//! SeaORM-based service for the Role domain.
//! Zero plain SQL — uses RoleRepositorySeaorm.

use sea_orm::DatabaseConnection;

use crate::common::errors::app_error::AppError;
use crate::common::utils::helpers::PaginatedResponse;
use crate::modules::role::repository::role_repository_seaorm::RoleRepositorySeaorm;
use crate::modules::role::request::role_request::*;
use crate::modules::role::response::role_response::*;

pub struct RoleServiceSeaorm<'a> {
    repo: RoleRepositorySeaorm<'a>,
}

impl<'a> RoleServiceSeaorm<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { repo: RoleRepositorySeaorm::new(db) }
    }

    pub async fn list_roles(&self, query: &ListRolesQuery) -> Result<PaginatedResponse<RoleResponse>, AppError> {
        let offset = query.pagination.offset();
        let limit = query.pagination.limit_i64() as u32;
        self.repo.list(offset, limit, query.is_active).await
    }

    pub async fn get_role(&self, role_id: i64) -> Result<RoleDetailResponse, AppError> {
        let role = self.repo.find_by_id(role_id).await?
            .ok_or_else(|| AppError::NotFound("Role not found".into()))?;
        Ok(RoleResponse::from_model(role))
    }

    pub async fn create_role(&self, req: &CreateRoleRequest) -> Result<RoleDetailResponse, AppError> {
        if self.repo.name_exists(&req.name, None).await? {
            return Err(AppError::Conflict("Role name already exists".into()));
        }
        let role = self.repo.create(&req.name, &req.display_name, req.description.as_deref()).await?;
        Ok(RoleResponse::from_model(role))
    }

    pub async fn update_role(&self, role_id: i64, req: &UpdateRoleRequest) -> Result<RoleDetailResponse, AppError> {
        let existing = self.repo.find_by_id(role_id).await?
            .ok_or_else(|| AppError::NotFound("Role not found".into()))?;
        if existing.is_system {
            return Err(AppError::Forbidden("Cannot modify system role".into()));
        }
        if let Some(ref name) = req.name {
            if self.repo.name_exists(name, Some(role_id)).await? {
                return Err(AppError::Conflict("Role name already exists".into()));
            }
        }
        let role = self.repo.update(role_id, req.name.as_deref(), req.display_name.as_deref(), req.description.as_deref()).await?;
        Ok(RoleResponse::from_model(role))
    }

    pub async fn deactivate_role(&self, role_id: i64) -> Result<MessageResponse, AppError> {
        self.repo.find_by_id(role_id).await?
            .ok_or_else(|| AppError::NotFound("Role not found".into()))?;
        self.repo.deactivate(role_id).await?;
        Ok(MessageResponse { message: "Role deactivated successfully".into() })
    }

    // ── Permission Assignment ──────────────────────────────

    pub async fn assign_permissions(&self, role_id: i64, req: &AssignPermissionsRequest) -> Result<MessageResponse, AppError> {
        self.repo.find_by_id(role_id).await?
            .ok_or_else(|| AppError::NotFound("Role not found".into()))?;
        self.repo.assign_permissions(role_id, &req.permission_ids).await?;
        Ok(MessageResponse { message: format!("Assigned {} permissions", req.permission_ids.len()) })
    }

    pub async fn remove_permission(&self, role_id: i64, permission_id: i64) -> Result<MessageResponse, AppError> {
        self.repo.remove_permission(role_id, permission_id).await?;
        Ok(MessageResponse { message: "Permission removed from role".into() })
    }

    // ── User-Role Management ───────────────────────────────

    pub async fn list_user_roles(&self, user_id: i64) -> Result<Vec<RoleResponse>, AppError> {
        self.repo.list_user_roles(user_id).await
    }

    pub async fn assign_role_to_user(&self, user_id: i64, req: &AssignUserRoleRequest) -> Result<MessageResponse, AppError> {
        self.repo.find_by_id(req.role_id).await?
            .ok_or_else(|| AppError::NotFound("Role not found".into()))?;
        self.repo.assign_role_to_user(user_id, req.role_id).await?;
        Ok(MessageResponse { message: "Role assigned to user".into() })
    }

    pub async fn revoke_role_from_user(&self, user_id: i64, role_id: i64) -> Result<MessageResponse, AppError> {
        self.repo.revoke_role_from_user(user_id, role_id).await?;
        Ok(MessageResponse { message: "Role revoked from user".into() })
    }
}
