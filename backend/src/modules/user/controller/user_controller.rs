use axum::extract::{Json, Path, Query, State};
use validator::Validate;

use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::common::middleware::auth_middleware::UserContext;
use crate::modules::user::request::user_request::*;
use crate::modules::user::response::user_response::*;
use crate::modules::user::service::user_service::UserService;

pub async fn list_users(
    State(state): State<SharedState>,
    Query(query): Query<ListUsersQuery>,
) -> Result<Json<crate::common::utils::helpers::PaginatedResponse<UserResponse>>, AppError> {
    let svc = UserService::new(&state.db_seaorm);
    Ok(Json(svc.list_users(&query).await?))
}

pub async fn create_user(
    State(state): State<SharedState>,
    Json(req): Json<CreateUserRequest>,
) -> Result<Json<UserResponse>, AppError> {
    req.validate()?;
    let svc = UserService::new(&state.db_seaorm);
    Ok(Json(svc.create_user(&req).await?))
}

pub async fn get_user(
    State(state): State<SharedState>,
    Path(user_id): Path<i64>,
) -> Result<Json<UserResponse>, AppError> {
    let svc = UserService::new(&state.db_seaorm);
    Ok(Json(svc.get_user(user_id).await?))
}

pub async fn update_user(
    State(state): State<SharedState>,
    Path(user_id): Path<i64>,
    Json(req): Json<UpdateUserRequest>,
) -> Result<Json<UserResponse>, AppError> {
    req.validate()?;
    let svc = UserService::new(&state.db_seaorm);
    Ok(Json(svc.update_user(user_id, &req).await?))
}

pub async fn delete_user(
    State(state): State<SharedState>,
    user: UserContext,
    Path(user_id): Path<i64>,
) -> Result<Json<MessageResponse>, AppError> {
    let svc = UserService::new(&state.db_seaorm);
    Ok(Json(svc.delete_user(user_id, user.user_id).await?))
}

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

pub async fn get_me(
    State(state): State<SharedState>,
    user: UserContext,
) -> Result<Json<UserResponse>, AppError> {
    let svc = UserService::new(&state.db_seaorm);
    Ok(Json(svc.get_user(user.user_id).await?))
}

pub async fn update_me(
    State(state): State<SharedState>,
    user: UserContext,
    Json(req): Json<UpdateProfileRequest>,
) -> Result<Json<UserResponse>, AppError> {
    req.validate()?;
    let svc = UserService::new(&state.db_seaorm);
    let update = UpdateUserRequest {
        name: req.name,
        phone: req.phone,
        branch_id: None,
        avatar_url: req.avatar_url,
    };
    Ok(Json(svc.update_user(user.user_id, &update).await?))
}

pub async fn change_password(
    State(state): State<SharedState>,
    user: UserContext,
    Json(req): Json<ChangePasswordRequest>,
) -> Result<Json<MessageResponse>, AppError> {
    req.validate()?;
    let svc = UserService::new(&state.db_seaorm);
    Ok(Json(svc.change_password(user.user_id, &req).await?))
}

// ── Auth endpoints (stub implementations) ────────────────────

pub async fn login(
    State(state): State<SharedState>,
    Json(req): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    let _ = state;
    let _ = req;
    Err(AppError::Internal(anyhow::anyhow!("Not yet implemented")))
}

pub async fn register(
    State(state): State<SharedState>,
    Json(req): Json<CreateUserRequest>,
) -> Result<Json<UserResponse>, AppError> {
    let svc = UserService::new(&state.db_seaorm);
    Ok(Json(svc.create_user(&req).await?))
}

pub async fn logout(
    State(state): State<SharedState>,
    user: UserContext,
) -> Result<Json<MessageResponse>, AppError> {
    let _ = state;
    let _ = user;
    Ok(Json(MessageResponse { message: "Logged out".into() }))
}

pub async fn logout_all(
    State(state): State<SharedState>,
    user: UserContext,
) -> Result<Json<MessageResponse>, AppError> {
    let _ = state;
    let _ = user;
    Ok(Json(MessageResponse { message: "All sessions logged out".into() }))
}

pub async fn refresh_token(
    State(state): State<SharedState>,
    Json(req): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    let _ = state;
    let _ = req;
    Err(AppError::Internal(anyhow::anyhow!("Not yet implemented")))
}

pub async fn me(
    State(state): State<SharedState>,
    user: UserContext,
) -> Result<Json<UserResponse>, AppError> {
    let svc = UserService::new(&state.db_seaorm);
    Ok(Json(svc.get_user(user.user_id).await?))
}

pub async fn send_otp(
    State(state): State<SharedState>,
    Json(req): Json<serde_json::Value>,
) -> Result<Json<MessageResponse>, AppError> {
    let _ = state;
    let _ = req;
    Err(AppError::Internal(anyhow::anyhow!("Not yet implemented")))
}

pub async fn verify_otp(
    State(state): State<SharedState>,
    Json(req): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    let _ = state;
    let _ = req;
    Err(AppError::Internal(anyhow::anyhow!("Not yet implemented")))
}

pub async fn request_password_reset(
    State(state): State<SharedState>,
    Json(req): Json<serde_json::Value>,
) -> Result<Json<MessageResponse>, AppError> {
    let _ = state;
    let _ = req;
    Err(AppError::Internal(anyhow::anyhow!("Not yet implemented")))
}

pub async fn confirm_password_reset(
    State(state): State<SharedState>,
    Json(req): Json<serde_json::Value>,
) -> Result<Json<MessageResponse>, AppError> {
    let _ = state;
    let _ = req;
    Err(AppError::Internal(anyhow::anyhow!("Not yet implemented")))
}

pub async fn list_sessions(
    State(state): State<SharedState>,
    user: UserContext,
) -> Result<Json<Vec<serde_json::Value>>, AppError> {
    let _ = state;
    let _ = user;
    Ok(Json(vec![]))
}

pub async fn enable_2fa(
    State(state): State<SharedState>,
    user: UserContext,
) -> Result<Json<serde_json::Value>, AppError> {
    let _ = state;
    let _ = user;
    Err(AppError::Internal(anyhow::anyhow!("Not yet implemented")))
}

pub async fn confirm_2fa(
    State(state): State<SharedState>,
    user: UserContext,
    Json(req): Json<serde_json::Value>,
) -> Result<Json<MessageResponse>, AppError> {
    let _ = state;
    let _ = user;
    let _ = req;
    Err(AppError::Internal(anyhow::anyhow!("Not yet implemented")))
}

pub async fn disable_2fa(
    State(state): State<SharedState>,
    user: UserContext,
    Json(req): Json<serde_json::Value>,
) -> Result<Json<MessageResponse>, AppError> {
    let _ = state;
    let _ = user;
    let _ = req;
    Err(AppError::Internal(anyhow::anyhow!("Not yet implemented")))
}

pub async fn verify_2fa_login(
    State(state): State<SharedState>,
    Json(req): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    let _ = state;
    let _ = req;
    Err(AppError::Internal(anyhow::anyhow!("Not yet implemented")))
}
