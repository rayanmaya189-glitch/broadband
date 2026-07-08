use sqlx::PgPool;

use crate::common::errors::app_error::AppError;
use crate::common::security::crypto::{hash_password, sha256, verify_password};
use crate::common::security::jwt::{create_access_token, create_refresh_token_pair, refresh_token_expiry, MAX_SESSIONS};
use crate::common::utils::helpers::PaginatedResponse;
use crate::common::middleware::auth_middleware::Claims;
use crate::modules::user::mapper::user_mapper::{user_to_auth_response, user_to_response};
use crate::modules::user::repository::user_repository::UserRepository;
use crate::modules::user::request::user_request::*;
use crate::modules::user::response::user_response::*;

pub struct UserService<'a> {
    repo: UserRepository<'a>,
}

impl<'a> UserService<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { repo: UserRepository::new(pool) }
    }

    pub async fn login(&self, req: &LoginRequest, ip: Option<&str>, device: Option<&str>) -> Result<LoginResponse, AppError> {
        let user = self.repo.find_by_email(&req.email).await?
            .ok_or_else(|| AppError::Validation("Invalid email or password".into()))?;

        if user.is_locked {
            if let Some(locked) = user.locked_until {
                if locked > chrono::Utc::now() {
                    return Err(AppError::Forbidden("Account is locked".into()));
                }
            }
        }
        if !user.is_active { return Err(AppError::Forbidden("Account is inactive".into())); }

        let valid = verify_password(&req.password, &user.password_hash)?;
        if !valid {
            self.repo.increment_failed_attempts(user.id).await?;
            return Err(AppError::Validation("Invalid credentials".into()));
        }

        let role_name = self.repo.get_role_name(user.role_id).await?.unwrap_or_else(|| "user".into());
        let permissions = self.repo.resolve_permissions(user.role_id).await?;
        let (access_token, _) = create_access_token(user.id, &user.email, &role_name, user.role_id, user.branch_id, user.is_company_wide, &permissions)?;
        let (raw_refresh, hash_refresh) = create_refresh_token_pair();
        let expiry = refresh_token_expiry();

        let count = self.repo.count_active_tokens(user.id).await?;
        if count >= MAX_SESSIONS { self.repo.revoke_oldest_token(user.id).await?; }
        self.repo.create_refresh_token(user.id, &hash_refresh, device, ip, expiry).await?;
        self.repo.update_last_login(user.id).await?;

        let config = crate::common::config::config::Config::get();
        let user_resp = user_to_auth_response(&user, &role_name);
        Ok(LoginResponse { access_token, refresh_token: raw_refresh, token_type: "Bearer", expires_in: config.jwt_access_expiry_hours * 3600, user: user_resp })
    }

    pub async fn register(&self, req: &RegisterRequest) -> Result<RegisterResponse, AppError> {
        if self.repo.find_by_email(&req.email).await?.is_some() {
            return Err(AppError::Conflict("Email already registered".into()));
        }
        if self.repo.find_by_phone(&req.phone).await?.is_some() {
            return Err(AppError::Conflict("Phone already registered".into()));
        }

        let password_hash = hash_password(&req.password)?;
        let user = self.repo.create(&req.email, &password_hash, &req.name, &req.phone, 10, req.branch_id, false).await?;

        let role_name = self.repo.get_role_name(user.role_id).await?.unwrap_or_else(|| "customer".into());
        let permissions = self.repo.resolve_permissions(user.role_id).await?;
        let (access_token, _) = create_access_token(user.id, &user.email, &role_name, user.role_id, user.branch_id, user.is_company_wide, &permissions)?;
        let (raw_refresh, hash_refresh) = create_refresh_token_pair();
        let expiry = refresh_token_expiry();
        self.repo.create_refresh_token(user.id, &hash_refresh, None, None, expiry).await?;
        self.repo.update_last_login(user.id).await?;

        let config = crate::common::config::config::Config::get();
        let user_resp = user_to_auth_response(&user, &role_name);
        Ok(RegisterResponse { user: user_resp, access_token, refresh_token: raw_refresh, token_type: "Bearer", expires_in: config.jwt_access_expiry_hours * 3600 })
    }

    pub async fn refresh_token(&self, req: &RefreshTokenRequest) -> Result<TokenRefreshResponse, AppError> {
        let hash = sha256(req.refresh_token.as_bytes());
        let token_row = self.repo.find_valid_refresh_token(&hash).await?
            .ok_or_else(|| AppError::Unauthorized)?;
        let user = self.repo.find_by_id(token_row.user_id).await?
            .ok_or_else(|| AppError::Unauthorized)?;
        if !user.is_active { return Err(AppError::Forbidden("Account is inactive".into())); }

        self.repo.revoke_refresh_token(&hash).await?;
        let role_name = self.repo.get_role_name(user.role_id).await?.unwrap_or_else(|| "user".into());
        let permissions = self.repo.resolve_permissions(user.role_id).await?;
        let (access_token, _) = create_access_token(user.id, &user.email, &role_name, user.role_id, user.branch_id, user.is_company_wide, &permissions)?;
        let (raw_refresh, hash_refresh) = create_refresh_token_pair();
        let expiry = refresh_token_expiry();
        self.repo.create_refresh_token(user.id, &hash_refresh, token_row.device_info.as_deref(), token_row.ip_address.as_deref(), expiry).await?;

        let config = crate::common::config::config::Config::get();
        Ok(TokenRefreshResponse { access_token, refresh_token: raw_refresh, token_type: "Bearer", expires_in: config.jwt_access_expiry_hours * 3600 })
    }

    pub async fn logout(&self, req: &LogoutRequest) -> Result<MessageResponse, AppError> {
        if let Some(ref token) = req.refresh_token {
            let hash = sha256(token.as_bytes());
            self.repo.revoke_refresh_token(&hash).await?;
        }
        Ok(MessageResponse { message: "Logged out successfully".into() })
    }

    pub async fn logout_all(&self, user_id: i64) -> Result<MessageResponse, AppError> {
        let count = self.repo.revoke_all_user_tokens(user_id).await?;
        Ok(MessageResponse { message: format!("Revoked {count} active sessions") })
    }

    pub async fn change_password(&self, user_id: i64, req: &ChangePasswordRequest) -> Result<MessageResponse, AppError> {
        let user = self.repo.find_by_id(user_id).await?.ok_or_else(|| AppError::NotFound("User not found".into()))?;
        if !verify_password(&req.current_password, &user.password_hash)? {
            return Err(AppError::Validation("Invalid current password".into()));
        }
        let new_hash = hash_password(&req.new_password)?;
        self.repo.update_password(user_id, &new_hash).await?;
        self.repo.revoke_all_user_tokens(user_id).await?;
        Ok(MessageResponse { message: "Password changed successfully".into() })
    }

    pub async fn get_current_user(&self, claims: &Claims) -> Result<AuthUserResponse, AppError> {
        let user = self.repo.find_by_id(claims.sub).await?.ok_or_else(|| AppError::NotFound("User not found".into()))?;
        Ok(user_to_auth_response(&user, &claims.role))
    }

    pub async fn list_sessions(&self, user_id: i64, current_jti: &str) -> Result<Vec<SessionResponse>, AppError> {
        let sessions = self.repo.list_active_sessions(user_id).await?;
        Ok(sessions.into_iter().map(|s| SessionResponse { id: s.id, device_info: s.device_info, ip_address: s.ip_address, created_at: s.created_at, expires_at: s.expires_at, is_current: s.token_hash == current_jti }).collect())
    }

    pub async fn list_users(&self, query: &ListUsersQuery) -> Result<PaginatedResponse<UserResponse>, AppError> {
        let offset = query.pagination.offset();
        let limit = query.pagination.limit_i64() as u32;
        self.repo.list(offset, limit, query.role_id, query.branch_id, query.is_active, query.pagination.search.as_deref()).await
    }

    pub async fn get_user(&self, user_id: i64) -> Result<UserDetailResponse, AppError> {
        let user = self.repo.find_by_id(user_id).await?.ok_or_else(|| AppError::NotFound("User not found".into()))?;
        let role_name = self.repo.get_role_name(user.role_id).await?;
        Ok(user_to_response(&user, role_name.as_deref()))
    }

    pub async fn create_user(&self, req: &CreateUserRequest) -> Result<UserDetailResponse, AppError> {
        if self.repo.email_exists(&req.email, None).await? { return Err(AppError::Conflict("Email already registered".into())); }
        if self.repo.phone_exists(&req.phone, None).await? { return Err(AppError::Conflict("Phone already registered".into())); }
        if !self.repo.role_exists(req.role_id).await? { return Err(AppError::NotFound("Role not found".into())); }

        let password_hash = hash_password(&req.password)?;
        let user = self.repo.create(&req.email, &password_hash, &req.name, &req.phone, req.role_id, req.branch_id, req.is_company_wide.unwrap_or(false)).await?;
        let role_name = self.repo.get_role_name(user.role_id).await?;
        Ok(user_to_response(&user, role_name.as_deref()))
    }

    pub async fn update_user(&self, user_id: i64, req: &UpdateUserRequest) -> Result<UserDetailResponse, AppError> {
        if self.repo.find_by_id(user_id).await?.is_none() { return Err(AppError::NotFound("User not found".into())); }
        if let Some(ref phone) = req.phone {
            if self.repo.phone_exists(phone, Some(user_id)).await? { return Err(AppError::Conflict("Phone already registered".into())); }
        }
        let user = self.repo.update(user_id, req.name.as_deref(), req.phone.as_deref(), req.branch_id, req.avatar_url.as_deref()).await?;
        let role_name = self.repo.get_role_name(user.role_id).await?;
        Ok(user_to_response(&user, role_name.as_deref()))
    }

    pub async fn delete_user(&self, user_id: i64, current_id: i64) -> Result<MessageResponse, AppError> {
        if user_id == current_id { return Err(AppError::Validation("Cannot delete your own account".into())); }
        let user = self.repo.find_by_id(user_id).await?.ok_or_else(|| AppError::NotFound("User not found".into()))?;
        let role_name = self.repo.get_role_name(user.role_id).await?;
        if role_name.as_deref() == Some("super_admin") { return Err(AppError::Forbidden("Cannot deactivate a super admin".into())); }
        self.repo.soft_delete(user_id).await?;
        Ok(MessageResponse { message: "User deleted successfully".into() })
    }

    pub async fn update_status(&self, user_id: i64, req: &UpdateUserStatusRequest, current_id: i64) -> Result<UserDetailResponse, AppError> {
        if user_id == current_id { return Err(AppError::Validation("Cannot change your own status".into())); }
        let is_active = match req.status.as_str() {
            "active" => true,
            "inactive" => false,
            _ => return Err(AppError::Validation("Invalid status".into())),
        };
        let existing = self.repo.find_by_id(user_id).await?.ok_or_else(|| AppError::NotFound("User not found".into()))?;
        let role_name = self.repo.get_role_name(existing.role_id).await?;
        if role_name.as_deref() == Some("super_admin") && !is_active { return Err(AppError::Forbidden("Cannot deactivate a super admin".into())); }
        let user = self.repo.update_status(user_id, is_active).await?;
        Ok(user_to_response(&user, role_name.as_deref()))
    }
}
