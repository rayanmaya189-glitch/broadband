use serde::Deserialize;
use validator::Validate;

use crate::common::utils::helpers::PaginationParams;

/// Login request payload.
#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,

    #[validate(length(min = 1, message = "Password is required"))]
    pub password: String,
}

/// Registration request payload.
#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,

    #[validate(length(min = 2, max = 255, message = "Name must be 2-255 characters"))]
    pub name: String,

    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,

    #[validate(length(min = 10, max = 20, message = "Invalid phone number"))]
    pub phone: String,

    pub branch_id: Option<i64>,
}

/// Create user request payload.
#[derive(Debug, Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,

    #[validate(length(min = 2, max = 255, message = "Name must be 2-255 characters"))]
    pub name: String,

    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,

    #[validate(length(min = 10, max = 20, message = "Invalid phone number"))]
    pub phone: String,

    pub branch_id: Option<i64>,
    pub role_id: i64,
    pub is_company_wide: Option<bool>,
}

/// Update user request payload.
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateUserRequest {
    #[validate(length(min = 2, max = 255, message = "Name must be 2-255 characters"))]
    pub name: Option<String>,
    pub phone: Option<String>,
    pub branch_id: Option<i64>,
    pub avatar_url: Option<String>,
}

/// Update user status request.
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateUserStatusRequest {
    #[validate(length(min = 1, message = "Status is required"))]
    pub status: String,
}

/// Update own profile request.
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateProfileRequest {
    #[validate(length(min = 2, max = 255, message = "Name must be 2-255 characters"))]
    pub name: Option<String>,
    pub phone: Option<String>,
    pub avatar_url: Option<String>,
}

/// Refresh token request.
#[derive(Debug, Deserialize, Validate)]
pub struct RefreshTokenRequest {
    #[validate(length(min = 1, message = "Refresh token is required"))]
    pub refresh_token: String,
}

/// Change password request.
#[derive(Debug, Deserialize, Validate)]
pub struct ChangePasswordRequest {
    #[validate(length(min = 1, message = "Current password is required"))]
    pub current_password: String,

    #[validate(length(min = 8, message = "New password must be at least 8 characters"))]
    pub new_password: String,
}

/// Logout request.
#[derive(Debug, Deserialize)]
pub struct LogoutRequest {
    pub refresh_token: Option<String>,
}

/// Query parameters for listing users.
#[derive(Debug, Deserialize)]
pub struct ListUsersQuery {
    #[serde(flatten)]
    pub pagination: PaginationParams,
    pub role_id: Option<i64>,
    pub branch_id: Option<i64>,
    pub is_active: Option<bool>,
}
