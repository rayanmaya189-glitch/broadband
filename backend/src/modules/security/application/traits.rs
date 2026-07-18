use crate::shared::errors::AppError;
use async_trait::async_trait;
use sea_orm::DatabaseConnection;

pub type RoleModel = crate::modules::security::domain::entities::role::Model;
pub type PermissionModel = crate::modules::security::domain::entities::permission::Model;
pub type UserRoleModel = crate::modules::security::domain::entities::user_role::Model;
pub type RolePermissionModel = crate::modules::security::domain::entities::role_permission::Model;

#[async_trait]
pub trait SecurityServiceTrait: Send + Sync {
    async fn list_roles(&self, db: &DatabaseConnection) -> Result<Vec<RoleModel>, AppError>;

    async fn create_role(
        &self,
        db: &DatabaseConnection,
        name: String,
        description: Option<String>,
    ) -> Result<RoleModel, AppError>;

    async fn list_permissions(
        &self,
        db: &DatabaseConnection,
    ) -> Result<Vec<PermissionModel>, AppError>;

    async fn assign_role_to_user(
        &self,
        db: &DatabaseConnection,
        user_id: i64,
        role_id: i64,
    ) -> Result<UserRoleModel, AppError>;

    async fn assign_permission_to_role(
        &self,
        db: &DatabaseConnection,
        role_id: i64,
        permission_id: i64,
    ) -> Result<RolePermissionModel, AppError>;

    async fn get_user_permissions(
        &self,
        db: &DatabaseConnection,
        user_id: i64,
    ) -> Result<Vec<String>, AppError>;
}
