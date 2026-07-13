use axum::extract::{Json, Path, Query, State};
use validator::Validate;

use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::common::middleware::auth_middleware::{Claims, UserContext};
use crate::modules::user::request::user_request::*;
use crate::modules::user::response::user_response::*;
use crate::modules::user::service::user_service::UserService;

#[utoipa::path(
    post,
    path = "/api/v1/auth/login",
    tag = "Auth",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = LoginResponse),
        (status = 401, description = "Invalid credentials"),
        (status = 422, description = "Validation error")
    )
)]
pub async fn login(State(state): State<SharedState>, Json(req): Json<LoginRequest>) -> Result<Json<LoginResponse>, AppError> {
    req.validate()?;
    let svc = UserService::new(&state.db, &state.redis);
    Ok(Json(svc.login(&req, None, None).await?))
}

#[utoipa::path(
    post,
    path = "/api/v1/auth/register",
    tag = "Auth",
    request_body = RegisterRequest,
    responses(
        (status = 200, description = "Registration successful", body = RegisterResponse),
        (status = 409, description = "User already exists"),
        (status = 422, description = "Validation error")
    )
)]
pub async fn register(State(state): State<SharedState>, Json(req): Json<RegisterRequest>) -> Result<Json<RegisterResponse>, AppError> {
    req.validate()?;
    let svc = UserService::new(&state.db, &state.redis);
    Ok(Json(svc.register(&req).await?))
}

#[utoipa::path(
    post,
    path = "/api/v1/auth/refresh",
    tag = "Auth",
    request_body = RefreshTokenRequest,
    responses(
        (status = 200, description = "Token refreshed", body = TokenRefreshResponse),
        (status = 401, description = "Invalid refresh token")
    )
)]
pub async fn refresh_token(State(state): State<SharedState>, Json(req): Json<RefreshTokenRequest>) -> Result<Json<TokenRefreshResponse>, AppError> {
    req.validate()?;
    let svc = UserService::new(&state.db, &state.redis);
    Ok(Json(svc.refresh_token(&req).await?))
}

#[utoipa::path(
    post,
    path = "/api/v1/auth/logout",
    tag = "Auth",
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "Logout successful"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn logout(State(state): State<SharedState>, _user: UserContext, Json(req): Json<LogoutRequest>) -> Result<Json<MessageResponse>, AppError> {
    let svc = UserService::new(&state.db, &state.redis);
    Ok(Json(svc.logout(&req).await?))
}

#[utoipa::path(
    post,
    path = "/api/v1/auth/logout/all",
    tag = "Auth",
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "All sessions invalidated"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn logout_all(State(state): State<SharedState>, user: UserContext) -> Result<Json<MessageResponse>, AppError> {
    let svc = UserService::new(&state.db, &state.redis);
    Ok(Json(svc.logout_all(user.user_id).await?))
}

#[utoipa::path(
    post,
    path = "/api/v1/auth/password/change",
    tag = "Auth",
    security(
        ("bearer_auth" = [])
    ),
    request_body = ChangePasswordRequest,
    responses(
        (status = 200, description = "Password changed"),
        (status = 401, description = "Unauthorized"),
        (status = 422, description = "Validation error")
    )
)]
pub async fn change_password(State(state): State<SharedState>, user: UserContext, Json(req): Json<ChangePasswordRequest>) -> Result<Json<MessageResponse>, AppError> {
    req.validate()?;
    let svc = UserService::new(&state.db, &state.redis);
    Ok(Json(svc.change_password(user.user_id, &req).await?))
}

#[utoipa::path(
    get,
    path = "/api/v1/auth/me",
    tag = "Auth",
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "Current user info", body = AuthUserResponse),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn me(State(state): State<SharedState>, claims: Claims) -> Result<Json<AuthUserResponse>, AppError> {
    let svc = UserService::new(&state.db, &state.redis);
    Ok(Json(svc.get_current_user(&claims).await?))
}

#[utoipa::path(
    get,
    path = "/api/v1/auth/sessions",
    tag = "Auth",
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "List of active sessions", body = Vec<SessionResponse>),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn list_sessions(State(state): State<SharedState>, user: UserContext, claims: Claims) -> Result<Json<Vec<SessionResponse>>, AppError> {
    let svc = UserService::new(&state.db, &state.redis);
    Ok(Json(svc.list_sessions(user.user_id, &claims.jti).await?))
}

#[utoipa::path(
    get,
    path = "/api/v1/users",
    tag = "Users",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("page" = Option<i64>, Query, description = "Page number"),
        ("limit" = Option<i64>, Query, description = "Items per page"),
        ("search" = Option<String>, Query, description = "Search term")
    ),
    responses(
        (status = 200, description = "List of users"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn list_users(State(state): State<SharedState>, Query(query): Query<ListUsersQuery>) -> Result<Json<crate::common::utils::helpers::PaginatedResponse<UserResponse>>, AppError> {
    let svc = UserService::new(&state.db, &state.redis);
    Ok(Json(svc.list_users(&query).await?))
}

#[utoipa::path(
    post,
    path = "/api/v1/users",
    tag = "Users",
    security(
        ("bearer_auth" = [])
    ),
    request_body = CreateUserRequest,
    responses(
        (status = 200, description = "User created", body = UserResponse),
        (status = 409, description = "Email or phone already exists"),
        (status = 422, description = "Validation error")
    )
)]
pub async fn create_user(State(state): State<SharedState>, Json(req): Json<CreateUserRequest>) -> Result<Json<UserResponse>, AppError> {
    req.validate()?;
    let svc = UserService::new(&state.db, &state.redis);
    Ok(Json(svc.create_user(&req).await?))
}

#[utoipa::path(
    get,
    path = "/api/v1/users/{id}",
    tag = "Users",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("id" = i64, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User details", body = UserResponse),
        (status = 404, description = "User not found"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn get_user(State(state): State<SharedState>, Path(user_id): Path<i64>) -> Result<Json<UserResponse>, AppError> {
    let svc = UserService::new(&state.db, &state.redis);
    Ok(Json(svc.get_user(user_id).await?))
}

#[utoipa::path(
    put,
    path = "/api/v1/users/{id}",
    tag = "Users",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("id" = i64, Path, description = "User ID")
    ),
    request_body = UpdateUserRequest,
    responses(
        (status = 200, description = "User updated", body = UserResponse),
        (status = 404, description = "User not found"),
        (status = 422, description = "Validation error")
    )
)]
pub async fn update_user(State(state): State<SharedState>, Path(user_id): Path<i64>, Json(req): Json<UpdateUserRequest>) -> Result<Json<UserResponse>, AppError> {
    req.validate()?;
    let svc = UserService::new(&state.db, &state.redis);
    Ok(Json(svc.update_user(user_id, &req).await?))
}

#[utoipa::path(
    delete,
    path = "/api/v1/users/{id}",
    tag = "Users",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("id" = i64, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User deleted"),
        (status = 404, description = "User not found"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn delete_user(State(state): State<SharedState>, user: UserContext, Path(user_id): Path<i64>) -> Result<Json<MessageResponse>, AppError> {
    let svc = UserService::new(&state.db, &state.redis);
    Ok(Json(svc.delete_user(user_id, user.user_id).await?))
}

#[utoipa::path(
    put,
    path = "/api/v1/users/{id}/status",
    tag = "Users",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("id" = i64, Path, description = "User ID")
    ),
    request_body = UpdateUserStatusRequest,
    responses(
        (status = 200, description = "Status updated", body = UserResponse),
        (status = 404, description = "User not found"),
        (status = 422, description = "Validation error")
    )
)]
pub async fn update_user_status(State(state): State<SharedState>, user: UserContext, Path(user_id): Path<i64>, Json(req): Json<UpdateUserStatusRequest>) -> Result<Json<UserResponse>, AppError> {
    req.validate()?;
    let svc = UserService::new(&state.db, &state.redis);
    Ok(Json(svc.update_status(user_id, &req, user.user_id).await?))
}

#[utoipa::path(
    get,
    path = "/api/v1/users/me",
    tag = "Users",
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "Current user profile", body = UserResponse),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn get_me(State(state): State<SharedState>, user: UserContext) -> Result<Json<UserResponse>, AppError> {
    let svc = UserService::new(&state.db, &state.redis);
    Ok(Json(svc.get_user(user.user_id).await?))
}

#[utoipa::path(
    put,
    path = "/api/v1/users/me",
    tag = "Users",
    security(
        ("bearer_auth" = [])
    ),
    request_body = UpdateProfileRequest,
    responses(
        (status = 200, description = "Profile updated", body = UserResponse),
        (status = 422, description = "Validation error")
    )
)]
pub async fn update_me(State(state): State<SharedState>, user: UserContext, Json(req): Json<UpdateProfileRequest>) -> Result<Json<UserResponse>, AppError> {
    req.validate()?;
    let svc = UserService::new(&state.db, &state.redis);
    let update = UpdateUserRequest { name: req.name, phone: req.phone, branch_id: None, avatar_url: req.avatar_url };
    Ok(Json(svc.update_user(user.user_id, &update).await?))
}

// ── OTP Login ──────────────────────────────────────────────

#[utoipa::path(
    post,
    path = "/api/v1/auth/login/otp/send",
    tag = "Auth",
    request_body = SendOtpRequest,
    responses(
        (status = 200, description = "OTP sent", body = OtpSentResponse),
        (status = 429, description = "Rate limit exceeded")
    )
)]
pub async fn send_otp(State(state): State<SharedState>, Json(req): Json<SendOtpRequest>) -> Result<Json<OtpSentResponse>, AppError> {
    req.validate()?;
    let svc = UserService::new(&state.db, &state.redis);
    Ok(Json(svc.send_otp(&req).await?))
}

#[utoipa::path(
    post,
    path = "/api/v1/auth/login/otp/verify",
    tag = "Auth",
    request_body = VerifyOtpRequest,
    responses(
        (status = 200, description = "OTP verified, login successful", body = LoginResponse),
        (status = 401, description = "Invalid OTP"),
        (status = 422, description = "Validation error")
    )
)]
pub async fn verify_otp(State(state): State<SharedState>, Json(req): Json<VerifyOtpRequest>) -> Result<Json<LoginResponse>, AppError> {
    req.validate()?;
    let svc = UserService::new(&state.db, &state.redis);
    Ok(Json(svc.verify_otp(&req, None, None).await?))
}

// ── Password Reset ─────────────────────────────────────────

#[utoipa::path(
    post,
    path = "/api/v1/auth/password/reset/request",
    tag = "Auth",
    request_body = PasswordResetRequest,
    responses(
        (status = 200, description = "Reset email sent", body = PasswordResetResponse),
        (status = 404, description = "User not found")
    )
)]
pub async fn request_password_reset(State(state): State<SharedState>, Json(req): Json<PasswordResetRequest>) -> Result<Json<PasswordResetResponse>, AppError> {
    req.validate()?;
    let svc = UserService::new(&state.db, &state.redis);
    Ok(Json(svc.request_password_reset(&req).await?))
}

#[utoipa::path(
    post,
    path = "/api/v1/auth/password/reset/confirm",
    tag = "Auth",
    request_body = PasswordResetConfirmRequest,
    responses(
        (status = 200, description = "Password reset successful"),
        (status = 400, description = "Invalid or expired token"),
        (status = 422, description = "Validation error")
    )
)]
pub async fn confirm_password_reset(State(state): State<SharedState>, Json(req): Json<PasswordResetConfirmRequest>) -> Result<Json<MessageResponse>, AppError> {
    req.validate()?;
    let svc = UserService::new(&state.db, &state.redis);
    Ok(Json(svc.confirm_password_reset(&req).await?))
}

// ── 2FA (TOTP) ────────────────────────────────────────────

#[utoipa::path(
    post,
    path = "/api/v1/auth/2fa/enable",
    tag = "Auth",
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "2FA setup initiated", body = TwoFaSetupResponse),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn enable_2fa(State(state): State<SharedState>, user: UserContext) -> Result<Json<TwoFaSetupResponse>, AppError> {
    let svc = UserService::new(&state.db, &state.redis);
    Ok(Json(svc.enable_2fa(user.user_id).await?))
}

#[utoipa::path(
    post,
    path = "/api/v1/auth/2fa/confirm",
    tag = "Auth",
    security(
        ("bearer_auth" = [])
    ),
    request_body = Confirm2FaRequest,
    responses(
        (status = 200, description = "2FA enabled", body = TwoFaEnabledResponse),
        (status = 401, description = "Invalid code"),
        (status = 422, description = "Validation error")
    )
)]
pub async fn confirm_2fa(State(state): State<SharedState>, user: UserContext, Json(req): Json<Confirm2FaRequest>) -> Result<Json<TwoFaEnabledResponse>, AppError> {
    req.validate()?;
    let svc = UserService::new(&state.db, &state.redis);
    Ok(Json(svc.confirm_2fa(user.user_id, &req).await?))
}

#[utoipa::path(
    post,
    path = "/api/v1/auth/2fa/verify",
    tag = "Auth",
    request_body = Verify2FaRequest,
    responses(
        (status = 200, description = "2FA verified, login successful", body = LoginResponse),
        (status = 401, description = "Invalid code or token"),
        (status = 422, description = "Validation error")
    )
)]
pub async fn verify_2fa_login(State(state): State<SharedState>, Json(req): Json<Verify2FaRequest>) -> Result<Json<LoginResponse>, AppError> {
    req.validate()?;
    let svc = UserService::new(&state.db, &state.redis);
    Ok(Json(svc.verify_2fa_login(&req).await?))
}

#[utoipa::path(
    post,
    path = "/api/v1/auth/2fa/disable",
    tag = "Auth",
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "2FA disabled"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn disable_2fa(State(state): State<SharedState>, user: UserContext) -> Result<Json<MessageResponse>, AppError> {
    let svc = UserService::new(&state.db, &state.redis);
    Ok(Json(svc.disable_2fa(user.user_id).await?))
}
