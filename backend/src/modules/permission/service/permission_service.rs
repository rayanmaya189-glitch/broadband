use sqlx::PgPool;

use crate::common::errors::app_error::AppError;
use crate::common::utils::helpers::PaginatedResponse;
use crate::modules::permission::mapper::permission_mapper::permission_to_response;
use crate::modules::permission::repository::permission_repository::PermissionRepository;
use crate::modules::permission::request::permission_request::*;
use crate::modules::permission::response::permission_response::*;

pub struct PermissionService<'a> {
    repo: PermissionRepository<'a>,
}

impl<'a> PermissionService<'a> {
    pub fn new(pool: &'a PgPool) -> Self { Self { repo: PermissionRepository::new(pool) } }

    pub async fn list_permissions(&self, query: &ListPermissionsQuery) -> Result<PaginatedResponse<PermissionResponse>, AppError> {
        let offset = query.pagination.offset();
        let limit = query.pagination.limit_i64() as u32;
        self.repo.list(offset, limit, query.module.as_deref()).await
    }

    pub async fn create_permission(&self, req: &CreatePermissionRequest) -> Result<PermissionResponse, AppError> {
        let perm = self.repo.create(&req.name, &req.method, &req.api_url, &req.guard, &req.module).await?;
        Ok(permission_to_response(&perm))
    }

    pub async fn delete_permission(&self, id: i64) -> Result<MessageResponse, AppError> {
        if self.repo.find_by_id(id).await?.is_none() { return Err(AppError::NotFound("Permission not found".into())); }
        self.repo.delete(id).await?;
        Ok(MessageResponse { message: "Permission deleted successfully".into() })
    }
}
