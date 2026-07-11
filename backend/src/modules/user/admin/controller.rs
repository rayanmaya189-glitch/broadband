use axum::extract::{Json, Path, Query, State};
use validator::Validate;

use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::common::middleware::auth_middleware::UserContext;
use crate::modules::user::request::user_request::*;
use crate::modules::user::response::user_response::*;
use crate::modules::user::service::user_service::UserService;

/// List all users (admin: can filter, search, paginate).
pub async fn list_users(
    State(state): State<SharedState>,
    Query(query): Query<ListUsersQuery>,
) -> Result<Json<crate::common::utils::helpers::PaginatedResponse<UserResponse>>, AppError> {
    let svc = UserService::new(&state.db_seaorm);
    Ok(Json(svc.list_users(&query).await?))
}

/// Create a new user (admin).
pub async fn create_user(
    State(state): State<SharedState>,
    Json(req): Json<CreateUserRequest>,
) -> Result<Json<UserResponse>, AppError> {
    req.validate()?;
    let svc = UserService::new(&state.db_seaorm);
    Ok(Json(svc.create_user(&req).await?))
}

/// Get user by ID (admin).
pub async fn get_user(
    State(state): State<SharedState>,
    Path(user_id): Path<i64>,
) -> Result<Json<UserResponse>, AppError> {
    let svc = UserService::new(&state.db_seaorm);
    Ok(Json(svc.get_user(user_id).await?))
}

/// Update user (admin).
pub async fn update_user(
    State(state): State<SharedState>,
    Path(user_id): Path<i64>,
    Json(req): Json<UpdateUserRequest>,
) -> Result<Json<UserResponse>, AppError> {
    req.validate()?;
    let svc = UserService::new(&state.db_seaorm);
    Ok(Json(svc.update_user(user_id, &req).await?))
}

/// Soft-delete user (admin).
pub async fn delete_user(
    State(state): State<SharedState>,
    user: UserContext,
    Path(user_id): Path<i64>,
) -> Result<Json<MessageResponse>, AppError> {
    let svc = UserService::new(&state.db_seaorm);
    Ok(Json(svc.delete_user(user_id, user.user_id).await?))
}

/// Activate/deactivate user account (admin).
pub async fn update_user_status(
    State(state): State<SharedState>,
    user: UserContext,
    Path(user_id): Path<i64>,
    Json(req): Json<UpdateUserStatusRequest>,
) -> Result<Json<UserResponse>, AppError> {
    req.validate()?;
    let svc = UserService::new(&state.db_seaorm);
    Ok(Json(svc.update_status(user_id, &req, user.user_id).await?))
}
