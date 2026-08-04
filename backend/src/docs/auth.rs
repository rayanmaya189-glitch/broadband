/// OpenAPI schemas and stub handlers for Auth endpoints.
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

// ── Request / Response types ─────────────────────────────────────────

#[derive(Debug, Deserialize, ToSchema)]
pub struct RegisterRequest {
    /// User email address
    pub email: String,
    /// Phone number
    pub phone: String,
    /// Display name
    pub name: String,
    /// Password
    pub password: String,
    /// Branch ID (optional for company-wide users)
    #[serde(default)]
    pub branch_id: Option<i64>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct LoginRequest {
    /// User email address
    pub email: String,
    /// User password
    pub password: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct Login2FARequest {
    /// Pending 2FA token from initial login
    pub pending_token: String,
    /// TOTP code from authenticator app
    pub code: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct UserResponse {
    /// User ID
    pub id: i64,
    /// User email address
    pub email: String,
    /// Phone number
    pub phone: String,
    /// Display name
    pub name: String,
    /// Avatar URL
    pub avatar_url: Option<String>,
    /// Branch ID (null for company-wide users)
    #[serde(default)]
    pub branch_id: Option<i64>,
    /// Account status
    pub status: String,
    /// Last login timestamp (RFC 3339)
    pub last_login_at: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AuthResponse {
    /// Whether 2FA verification is required
    pub requires_2fa: bool,
    /// JWT access token (null if 2FA required)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    /// Refresh token (null if 2FA required)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
    /// Authenticated user details
    pub user: Option<UserResponse>,
    /// Pending token for 2FA step (null if not required)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_token: Option<String>,
    /// Status message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct OtpRequestRequest {
    /// Phone number in E.164 format (e.g. +919876543210)
    pub phone: String,
    /// Delivery channel: sms, whatsapp, telegram or firebase (default: sms)
    #[serde(default)]
    pub channel: Option<String>,
    /// Firebase cloud messaging token (required when channel = firebase)
    #[serde(default)]
    pub fcm_token: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct OtpRequestResponse {
    /// Whether the OTP was generated and sent
    pub success: bool,
    /// Channel the OTP was delivered via
    pub channel: String,
    /// OTP validity window in seconds
    pub expires_in_secs: u64,
    /// Human-readable confirmation message
    pub message: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct OtpVerifyRequest {
    /// Phone number the OTP was sent to
    pub phone: String,
    /// Delivery channel used at request time (default: sms)
    #[serde(default)]
    pub channel: Option<String>,
    /// OTP code received by the user
    pub code: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct RefreshTokenRequest {
    /// Refresh token
    pub refresh_token: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct Setup2FARequest {
    /// User ID to set up 2FA for
    pub user_id: i64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct Setup2FAResponse {
    /// TOTP secret key
    pub secret: String,
    /// QR code URL for authenticator apps
    pub qr_code_url: String,
    /// One-time backup codes
    pub backup_codes: Vec<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct Confirm2FARequest {
    /// TOTP code to verify setup
    pub code: String,
    /// Secret from setup step
    pub secret: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct Verify2FARequest {
    /// TOTP code from authenticator app
    pub code: String,
    /// Pending 2FA token from initial login
    pub pending_token: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct VerifyBackupCodeRequest {
    /// Backup code
    pub code: String,
    /// Pending 2FA token
    pub pending_token: String,
}

// ── Stub handler functions (for OpenAPI spec only) ───────────────────

/// Register a new user account
#[utoipa::path(
    post,
    path = "/api/v1/auth/register",
    tag = "Auth",
    request_body = RegisterRequest,
    responses(
        (status = 201, description = "Registration successful"),
        (status = 409, description = "Email or phone already registered"),
        (status = 422, description = "Validation error")
    )
)]
pub async fn register() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Login with email and password
#[utoipa::path(
    post,
    path = "/api/v1/auth/login",
    tag = "Auth",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = AuthResponse),
        (status = 401, description = "Invalid credentials"),
        (status = 423, description = "Account locked")
    )
)]
pub async fn login() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Two-step 2FA login: verify TOTP code after password
#[utoipa::path(
    post,
    path = "/api/v1/auth/login/2fa",
    tag = "Auth",
    request_body = Login2FARequest,
    responses(
        (status = 200, description = "2FA verified, login complete", body = AuthResponse),
        (status = 401, description = "Invalid 2FA code or expired token")
    )
)]
pub async fn login_2fa() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Refresh an access token using a refresh token
#[utoipa::path(
    post,
    path = "/api/v1/auth/refresh",
    tag = "Auth",
    request_body = RefreshTokenRequest,
    responses(
        (status = 200, description = "Token refreshed", body = AuthResponse),
        (status = 401, description = "Invalid or expired refresh token")
    )
)]
pub async fn refresh_token() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Initiate TOTP 2FA setup — returns secret and QR code
#[utoipa::path(
    post,
    path = "/api/v1/auth/2fa/setup",
    tag = "Auth",
    request_body = Setup2FARequest,
    responses(
        (status = 200, description = "2FA setup initiated", body = Setup2FAResponse)
    )
)]
pub async fn setup_2fa() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Confirm 2FA setup by verifying a TOTP code
#[utoipa::path(
    post,
    path = "/api/v1/auth/2fa/confirm",
    tag = "Auth",
    request_body = Confirm2FARequest,
    responses(
        (status = 200, description = "2FA enabled successfully"),
        (status = 400, description = "Invalid TOTP code")
    )
)]
pub async fn confirm_2fa() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Verify a TOTP 2FA code during login
#[utoipa::path(
    post,
    path = "/api/v1/auth/2fa/verify",
    tag = "Auth",
    request_body = Verify2FARequest,
    responses(
        (status = 200, description = "2FA verified, login complete", body = AuthResponse),
        (status = 401, description = "Invalid 2FA code")
    )
)]
pub async fn verify_2fa() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Verify a backup code as an alternative to TOTP
#[utoipa::path(
    post,
    path = "/api/v1/auth/2fa/backup-verify",
    tag = "Auth",
    request_body = VerifyBackupCodeRequest,
    responses(
        (status = 200, description = "Backup code verified, login complete", body = AuthResponse),
        (status = 401, description = "Invalid backup code")
    )
)]
pub async fn verify_backup_code() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Disable 2FA for the authenticated user
#[utoipa::path(
    delete,
    path = "/api/v1/auth/2fa/disable",
    tag = "Auth",
    responses(
        (status = 200, description = "2FA disabled successfully"),
        (status = 401, description = "Unauthorized")
    ),
    security(("bearer_auth" = []))
)]
pub async fn disable_2fa() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Request an OTP for phone-based login
#[utoipa::path(
    post,
    path = "/api/v1/auth/otp/request",
    tag = "Auth",
    request_body = OtpRequestRequest,
    responses(
        (status = 200, description = "OTP generated and delivered", body = OtpRequestResponse),
        (status = 400, description = "Invalid phone number or channel"),
        (status = 429, description = "Too many requests, OTP rate limit exceeded")
    )
)]
pub async fn request_otp() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Verify an OTP and complete phone-based login
#[utoipa::path(
    post,
    path = "/api/v1/auth/otp/verify",
    tag = "Auth",
    request_body = OtpVerifyRequest,
    responses(
        (status = 200, description = "OTP verified, login complete", body = AuthResponse),
        (status = 400, description = "Invalid or expired OTP"),
        (status = 404, description = "No account is linked to this phone number")
    )
)]
pub async fn verify_otp() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Receive Telegram bot updates for phone-to-chat binding
#[utoipa::path(
    post,
    path = "/api/v1/auth/otp/telegram-webhook",
    tag = "Auth",
    request_body = serde_json::Value,
    responses(
        (status = 200, description = "Update processed (always returns ok)")
    )
)]
pub async fn telegram_webhook() -> axum::Json<serde_json::Value> {
    unimplemented!()
}
