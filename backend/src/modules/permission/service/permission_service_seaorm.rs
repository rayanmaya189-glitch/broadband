//! SeaORM-based service for the Permission domain.
//! Zero plain SQL — uses PermissionRepositorySeaorm.

use sea_orm::DatabaseConnection;

use crate::common::errors::app_error::AppError;
use crate::common::utils::helpers::PaginatedResponse;
use crate::modules::permission::repository::permission_repository_seaorm::PermissionRepositorySeaorm;
use crate::modules::permission::request::permission_request::*;
use crate::modules::permission::response::permission_response::*;

pub struct PermissionServiceSeaorm<'a> {
    repo: PermissionRepositorySeaorm<'a>,
}

impl<'a> PermissionServiceSeaorm<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { repo: PermissionRepositorySeaorm::new(db) }
    }

    pub async fn list_permissions(&self, query: &ListPermissionsQuery) -> Result<PaginatedResponse<PermissionResponse>, AppError> {
        let offset = query.pagination.offset();
        let limit = query.pagination.limit_i64() as u32;
        self.repo.list(offset, limit, query.module.as_deref()).await
    }

    pub async fn create_permission(&self, req: &CreatePermissionRequest) -> Result<PermissionResponse, AppError> {
        let perm = self.repo.create(&req.name, &req.method, &req.api_url, &req.guard, &req.module).await?;
        Ok(PermissionResponse::from_model(perm))
    }

    pub async fn delete_permission(&self, id: i64) -> Result<MessageResponse, AppError> {
        if self.repo.find_by_id(id).await?.is_none() {
            return Err(AppError::NotFound("Permission not found".into()));
        }
        self.repo.delete(id).await?;
        Ok(MessageResponse { message: "Permission deleted successfully".into() })
    }
}
