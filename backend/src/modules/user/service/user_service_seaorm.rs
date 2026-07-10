use sea_orm::*;

use crate::common::errors::app_error::AppError;
use crate::common::utils::helpers::PaginatedResponse;
use crate::modules::user::repository::user_repository_seaorm::UserRepositorySeaorm;
use crate::modules::user::request::user_request::*;
use crate::modules::user::response::user_response::*;
use crate::modules::user::model::user_entity::Model as UserModel;
use crate::modules::user::model::refresh_token_entity::Model as RefreshTokenModel;

pub struct UserServiceSeaorm {
    repo: UserRepositorySeaorm,
}

impl UserServiceSeaorm {
    pub fn new(db: &sea_orm::DatabaseConnection) -> Self {
        Self {
            repo: UserRepositorySeaorm::new(db),
        }
    }

    pub async fn list_users(
        &self,
        query: &ListUsersQuery,
    ) -> Result<PaginatedResponse<UserResponse>, AppError> {
        let offset = query.pagination.offset();
        let limit = query.pagination.limit;
        self.repo
            .list(offset, limit, query.role_id, query.branch_id, query.is_active, query.pagination.search.as_deref())
            .await
    }

    pub async fn get_user(&self, user_id: i64) -> Result<UserDetailResponse, AppError> {
        let model = self.repo.find_by_id(user_id).await?
            .ok_or_else(|| AppError::NotFound("User not found".into()))?;
        Ok(UserResponse::from_model(model, None))
    }

    pub async fn create_user(&self, req: &CreateUserRequest) -> Result<UserDetailResponse, AppError> {
        if self.repo.email_exists(&req.email, None).await? {
            return Err(AppError::Conflict("Email already registered".into()));
        }
        if self.repo.phone_exists(&req.phone, None).await? {
            return Err(AppError::Conflict("Phone already registered".into()));
        }

        let password_hash = crate::common::security::crypto::hash_password(&req.password)?;
        let model = self.repo
            .create(&req.email, &password_hash, &req.name, &req.phone, req.role_id, req.branch_id, req.is_company_wide.unwrap_or(false))
            .await?;
        Ok(UserResponse::from_model(model, None))
    }

    pub async fn update_user(&self, user_id: i64, req: &UpdateUserRequest) -> Result<UserDetailResponse, AppError> {
        if self.repo.find_by_id(user_id).await?.is_none() {
            return Err(AppError::NotFound("User not found".into()));
        }
        if let Some(ref phone) = req.phone {
            if self.repo.phone_exists(phone, Some(user_id)).await? {
                return Err(AppError::Conflict("Phone already registered".into()));
            }
        }
        let model = self.repo
            .update(user_id, req.name.as_deref(), req.phone.as_deref(), req.branch_id, req.avatar_url.as_deref())
            .await?;
        Ok(UserResponse::from_model(model, None))
    }

    pub async fn delete_user(&self, user_id: i64, current_id: i64) -> Result<MessageResponse, AppError> {
        if user_id == current_id {
            return Err(AppError::Validation("Cannot delete your own account".into()));
        }
        let model = self.repo.find_by_id(user_id).await?
            .ok_or_else(|| AppError::NotFound("User not found".into()))?;
        if model.is_locked {
            return Err(AppError::Forbidden("Cannot deactivate a locked user".into()));
        }
        self.repo.soft_delete(user_id).await?;
        Ok(MessageResponse { message: "User deleted successfully".into() })
    }

    pub async fn update_status(&self, user_id: i64, req: &UpdateUserStatusRequest, current_id: i64) -> Result<UserDetailResponse, AppError> {
        if user_id == current_id {
            return Err(AppError::Validation("Cannot change your own status".into()));
        }
        let is_active = match req.status.as_str() {
            "active" => true,
            "inactive" => false,
            _ => return Err(AppError::Validation("Invalid status".into())),
        };
        let model = self.repo.update_status(user_id, is_active).await?;
        Ok(UserResponse::from_model(model, None))
    }

    pub async fn change_password(&self, user_id: i64, req: &ChangePasswordRequest) -> Result<MessageResponse, AppError> {
        let model = self.repo.find_by_id(user_id).await?
            .ok_or_else(|| AppError::NotFound("User not found".into()))?;
        if !crate::common::security::crypto::verify_password(&req.current_password, &model.password_hash)? {
            return Err(AppError::Validation("Invalid current password".into()));
        }
        let new_hash = crate::common::security::crypto::hash_password(&req.new_password)?;
        self.repo.update_password(user_id, &new_hash).await?;
        self.repo.revoke_all_user_tokens(user_id).await?;
        Ok(MessageResponse { message: "Password changed successfully".into() })
    }

    pub async fn update_last_login(&self, user_id: i64) -> Result<(), AppError> {
        self.repo.update_last_login(user_id).await
    }

    pub async fn increment_failed_attempts(&self, user_id: i64) -> Result<i32, AppError> {
        self.repo.increment_failed_attempts(user_id).await
    }

    pub async fn email_exists(&self, email: &str, exclude: Option<i64>) -> Result<bool, AppError> {
        self.repo.email_exists(email, exclude).await
    }

    pub async fn phone_exists(&self, phone: &str, exclude: Option<i64>) -> Result<bool, AppError> {
        self.repo.phone_exists(phone, exclude).await
    }

    // ── Refresh Token management ────────────────────────────

    pub async fn create_refresh_token(
        &self,
        user_id: i64,
        token_hash: &str,
        device_info: Option<&str>,
        ip_address: Option<&str>,
        expires_at: chrono::DateTime<chrono::Utc>,
    ) -> Result<RefreshTokenModel, AppError> {
        self.repo.create_refresh_token(user_id, token_hash, device_info, ip_address, expires_at).await
    }

    pub async fn find_valid_refresh_token(&self, token_hash: &str) -> Result<Option<RefreshTokenModel>, AppError> {
        self.repo.find_valid_refresh_token(token_hash).await
    }

    pub async fn revoke_refresh_token(&self, token_hash: &str) -> Result<(), AppError> {
        self.repo.revoke_refresh_token(token_hash).await
    }

    pub async fn revoke_all_user_tokens(&self, user_id: i64) -> Result<u64, AppError> {
        self.repo.revoke_all_user_tokens(user_id).await
    }

    pub async fn count_active_tokens(&self, user_id: i64) -> Result<i64, AppError> {
        self.repo.count_active_tokens(user_id).await
    }

    pub async fn revoke_oldest_token(&self, user_id: i64) -> Result<(), AppError> {
        self.repo.revoke_oldest_token(user_id).await
    }

    pub async fn list_active_sessions(&self, user_id: i64) -> Result<Vec<RefreshTokenModel>, AppError> {
        self.repo.list_active_sessions(user_id).await
    }
}
