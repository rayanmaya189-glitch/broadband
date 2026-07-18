use crate::shared::errors::AppError;
use async_trait::async_trait;
use sea_orm::DatabaseConnection;

pub type BranchModel = crate::modules::branches::domain::entities::branch::Model;

#[async_trait]
pub trait BranchServiceTrait: Send + Sync {
    async fn list_branches(&self, db: &DatabaseConnection) -> Result<Vec<BranchModel>, AppError>;

    async fn get_branch(&self, db: &DatabaseConnection, id: i64) -> Result<BranchModel, AppError>;

    async fn create_branch(
        &self,
        db: &DatabaseConnection,
        name: String,
        code: String,
        branch_type: String,
        address: Option<String>,
        city: Option<String>,
        state: Option<String>,
        pincode: Option<String>,
        phone: Option<String>,
        email: Option<String>,
    ) -> Result<BranchModel, AppError>;

    async fn update_branch(
        &self,
        db: &DatabaseConnection,
        id: i64,
        name: Option<String>,
        address: Option<String>,
        city: Option<String>,
        state: Option<String>,
        pincode: Option<String>,
        phone: Option<String>,
        email: Option<String>,
    ) -> Result<BranchModel, AppError>;

    async fn set_branch_active(
        &self,
        db: &DatabaseConnection,
        id: i64,
        is_active: bool,
    ) -> Result<(), AppError>;
}
