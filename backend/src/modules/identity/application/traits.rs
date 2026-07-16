use async_trait::async_trait;
use sea_orm::{DatabaseConnection};
use crate::shared::errors::AppError;

pub type UserModel = crate::modules::identity::domain::entities::user::Model;
pub type UserSessionModel = crate::modules::identity::domain::entities::user_session::Model;

#[async_trait]
pub trait IdentityServiceTrait: Send + Sync {
    async fn list_users(
        &self,
        db: &DatabaseConnection,
    ) -> Result<Vec<UserModel>, AppError>;

    async fn get_user(
        &self,
        db: &DatabaseConnection,
        id: i64,
    ) -> Result<UserModel, AppError>;

    async fn create_user(
        &self,
        db: &DatabaseConnection,
        email: String,
        name: String,
        password_hash: String,
        phone: Option<String>,
    ) -> Result<UserModel, AppError>;

    async fn update_user_status(
        &self,
        db: &DatabaseConnection,
        id: i64,
        status: &str,
    ) -> Result<UserModel, AppError>;

    async fn login(
        &self,
        db: &DatabaseConnection,
        email: &str,
        password: &str,
    ) -> Result<(String, String, UserModel), AppError>;

    async fn refresh_token(
        &self,
        db: &DatabaseConnection,
        refresh_token: &str,
    ) -> Result<(String, String), AppError>;
}
