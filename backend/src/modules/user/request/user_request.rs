use utoipa::ToSchema;
use serde::Deserialize;
use validator::Validate;

use crate::common::utils::helpers::PaginationParams;

/// Login request payload.
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct LoginRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,

    #[validate(length(min = 1, message = "Password is required"))]
    pub password: String,
}

/// Registration request payload.
#[derive(Debug, Deserialize, Validate, ToSchema)]
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
#[derive(Debug, Deserialize, Validate, ToSchema)]
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
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateUserRequest {
    #[validate(length(min = 2, max = 255, message = "Name must be 2-255 characters"))]
    pub name: Option<String>,
    pub phone: Option<String>,
    pub branch_id: Option<i64>,
    pub avatar_url: Option<String>,
}

/// Update user status request.
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateUserStatusRequest {
    #[validate(length(min = 1, message = "Status is required"))]
    pub status: String,
}

/// Update own profile request.
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateProfileRequest {
    #[validate(length(min = 2, max = 255, message = "Name must be 2-255 characters"))]
    pub name: Option<String>,
    pub phone: Option<String>,
    pub avatar_url: Option<String>,
}

/// Refresh token request.
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct RefreshTokenRequest {
    #[validate(length(min = 1, message = "Refresh token is required"))]
    pub refresh_token: String,
}

/// Change password request.
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct ChangePasswordRequest {
    #[validate(length(min = 1, message = "Current password is required"))]
    pub current_password: String,

    #[validate(length(min = 8, message = "New password must be at least 8 characters"))]
    pub new_password: String,
}

/// Logout request.
#[derive(Debug, Deserialize, ToSchema)]
pub struct LogoutRequest {
    pub refresh_token: Option<String>,
}

/// Query parameters for listing users.
#[derive(Debug, Deserialize, ToSchema)]
pub struct ListUsersQuery {
    #[serde(flatten)]
    pub pagination: PaginationParams,
    pub role_id: Option<i64>,
    pub branch_id: Option<i64>,
    pub is_active: Option<bool>,
}

// ── OTP Login ──────────────────────────────────────────────

/// Send OTP to phone for login.
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct SendOtpRequest {
    #[validate(length(min = 10, max = 20, message = "Invalid phone number"))]
    pub phone: String,
}

/// Verify OTP and login.
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct VerifyOtpRequest {
    #[validate(length(min = 10, max = 20, message = "Invalid phone number"))]
    pub phone: String,
    #[validate(length(min = 6, max = 6, message = "OTP must be 6 digits"))]
    pub otp: String,
}

// ── Password Reset ─────────────────────────────────────────

/// Request password reset (sends email).
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct PasswordResetRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
}

/// Confirm password reset with token.
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct PasswordResetConfirmRequest {
    pub token: String,
    #[validate(length(min = 8, message = "New password must be at least 8 characters"))]
    pub new_password: String,
}

// ── 2FA (TOTP) ────────────────────────────────────────────

/// Verify 2FA code during login.
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct Verify2FaRequest {
    pub temp_token: String,
    #[validate(length(min = 6, max = 6, message = "TOTP code must be 6 digits"))]
    pub code: String,
}

/// Enable 2FA — returns secret and QR code URL.
#[derive(Debug, Deserialize, ToSchema)]
pub struct Enable2FaRequest {}

/// Confirm 2FA setup after scanning QR.
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct Confirm2FaRequest {
    #[validate(length(min = 6, max = 6, message = "TOTP code must be 6 digits"))]
    pub code: String,
}
