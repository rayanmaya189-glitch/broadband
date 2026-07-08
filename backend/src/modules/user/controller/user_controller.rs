use axum::extract::{Json, Path, Query, State};
use validator::Validate;

use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::common::middleware::auth_middleware::{Claims, UserContext};
use crate::modules::user::request::user_request::*;
use crate::modules::user::response::user_response::*;
use crate::modules::user::service::user_service::UserService;

pub async fn login(State(state): State<SharedState>, Json(req): Json<LoginRequest>) -> Result<Json<LoginResponse>, AppError> {
    req.validate()?;
    let svc = UserService::new(&state.db);
    Ok(Json(svc.login(&req, None, None).await?))
}

pub async fn register(State(state): State<SharedState>, Json(req): Json<RegisterRequest>) -> Result<Json<RegisterResponse>, AppError> {
    req.validate()?;
    let svc = UserService::new(&state.db);
    Ok(Json(svc.register(&req).await?))
}

pub async fn refresh_token(State(state): State<SharedState>, Json(req): Json<RefreshTokenRequest>) -> Result<Json<TokenRefreshResponse>, AppError> {
    req.validate()?;
    let svc = UserService::new(&state.db);
    Ok(Json(svc.refresh_token(&req).await?))
}

pub async fn logout(State(state): State<SharedState>, _user: UserContext, Json(req): Json<LogoutRequest>) -> Result<Json<MessageResponse>, AppError> {
    let svc = UserService::new(&state.db);
    Ok(Json(svc.logout(&req).await?))
}

pub async fn logout_all(State(state): State<SharedState>, user: UserContext) -> Result<Json<MessageResponse>, AppError> {
    let svc = UserService::new(&state.db);
    Ok(Json(svc.logout_all(user.user_id).await?))
}

pub async fn change_password(State(state): State<SharedState>, user: UserContext, Json(req): Json<ChangePasswordRequest>) -> Result<Json<MessageResponse>, AppError> {
    req.validate()?;
    let svc = UserService::new(&state.db);
    Ok(Json(svc.change_password(user.user_id, &req).await?))
}

pub async fn me(State(state): State<SharedState>, claims: Claims) -> Result<Json<AuthUserResponse>, AppError> {
    let svc = UserService::new(&state.db);
    Ok(Json(svc.get_current_user(&claims).await?))
}

pub async fn list_sessions(State(state): State<SharedState>, user: UserContext, claims: Claims) -> Result<Json<Vec<SessionResponse>>, AppError> {
    let svc = UserService::new(&state.db);
    Ok(Json(svc.list_sessions(user.user_id, &claims.jti).await?))
}

pub async fn list_users(State(state): State<SharedState>, Query(query): Query<ListUsersQuery>) -> Result<Json<crate::common::utils::helpers::PaginatedResponse<UserResponse>>, AppError> {
    let svc = UserService::new(&state.db);
    Ok(Json(svc.list_users(&query).await?))
}

pub async fn create_user(State(state): State<SharedState>, Json(req): Json<CreateUserRequest>) -> Result<Json<UserResponse>, AppError> {
    req.validate()?;
    let svc = UserService::new(&state.db);
    Ok(Json(svc.create_user(&req).await?))
}

pub async fn get_user(State(state): State<SharedState>, Path(user_id): Path<i64>) -> Result<Json<UserResponse>, AppError> {
    let svc = UserService::new(&state.db);
    Ok(Json(svc.get_user(user_id).await?))
}

pub async fn update_user(State(state): State<SharedState>, Path(user_id): Path<i64>, Json(req): Json<UpdateUserRequest>) -> Result<Json<UserResponse>, AppError> {
    req.validate()?;
    let svc = UserService::new(&state.db);
    Ok(Json(svc.update_user(user_id, &req).await?))
}

pub async fn delete_user(State(state): State<SharedState>, user: UserContext, Path(user_id): Path<i64>) -> Result<Json<MessageResponse>, AppError> {
    let svc = UserService::new(&state.db);
    Ok(Json(svc.delete_user(user_id, user.user_id).await?))
}

pub async fn update_user_status(State(state): State<SharedState>, user: UserContext, Path(user_id): Path<i64>, Json(req): Json<UpdateUserStatusRequest>) -> Result<Json<UserResponse>, AppError> {
    req.validate()?;
    let svc = UserService::new(&state.db);
    Ok(Json(svc.update_status(user_id, &req, user.user_id).await?))
}

pub async fn get_me(State(state): State<SharedState>, user: UserContext) -> Result<Json<UserResponse>, AppError> {
    let svc = UserService::new(&state.db);
    Ok(Json(svc.get_user(user.user_id).await?))
}

pub async fn update_me(State(state): State<SharedState>, user: UserContext, Json(req): Json<UpdateProfileRequest>) -> Result<Json<UserResponse>, AppError> {
    req.validate()?;
    let svc = UserService::new(&state.db);
    let update = UpdateUserRequest { name: req.name, phone: req.phone, branch_id: None, avatar_url: req.avatar_url };
    Ok(Json(svc.update_user(user.user_id, &update).await?))
}
