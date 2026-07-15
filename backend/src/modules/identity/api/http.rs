use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::modules::identity::application::services::IdentityService;
use crate::shared::app_state::AppState;
use crate::shared::errors::AppError;
use crate::shared::middleware::auth::UserContext;

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub phone: String,
    pub name: String,
    pub password: String,
    #[serde(default)]
    pub branch_id: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct RefreshTokenRequest {
    pub refresh_token: String,
}

#[derive(Debug, Serialize)]
pub struct RefreshTokenResponse {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub user: UserResponse,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: i64,
    pub email: String,
    pub phone: String,
    pub name: String,
    pub avatar_url: Option<String>,
    #[serde(default)]
    pub branch_id: Option<i64>,
    pub status: String,
    pub last_login_at: Option<String>,
}

/// POST /api/v1/auth/register
pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(req): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<AuthResponse>), AppError> {
    let email = req.email.clone();
    let password = req.password.clone();
    let user = IdentityService::register(
        &state.db,
        req.email,
        req.phone,
        req.name,
        req.password,
        req.branch_id,
    )
    .await?;

    let mut redis = state.redis.clone();
    let (access_token, refresh_token, _) =
        IdentityService::login(&state.db, &mut redis, &state.settings, &email, &password).await?;

    Ok((
        StatusCode::CREATED,
        Json(AuthResponse {
            access_token,
            refresh_token,
            user: UserResponse {
                id: user.id,
                email: user.email,
                phone: user.phone,
                name: user.name,
                avatar_url: user.avatar_url,
                branch_id: user.branch_id,
                status: user.status,
                last_login_at: user.last_login_at.map(|dt| dt.to_rfc3339()),
            },
        }),
    ))
}

/// POST /api/v1/auth/login
pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    let mut redis = state.redis.clone();
    let (access_token, refresh_token, user) = IdentityService::login(
        &state.db,
        &mut redis,
        &state.settings,
        &req.email,
        &req.password,
    )
    .await?;

    Ok(Json(AuthResponse {
        access_token,
        refresh_token,
        user: UserResponse {
            id: user.id,
            email: user.email,
            phone: user.phone,
            name: user.name,
            avatar_url: user.avatar_url,
            branch_id: user.branch_id,
            status: user.status,
            last_login_at: user.last_login_at.map(|dt| dt.to_rfc3339()),
        },
    }))
}

/// POST /api/v1/auth/refresh
pub async fn refresh_token(
    State(state): State<Arc<AppState>>,
    Json(req): Json<RefreshTokenRequest>,
) -> Result<Json<RefreshTokenResponse>, AppError> {
    let mut redis = state.redis.clone();
    let (access_token, refresh_token, _) =
        IdentityService::refresh_token(&state.db, &mut redis, &state.settings, &req.refresh_token)
            .await?;

    Ok(Json(RefreshTokenResponse {
        access_token,
        refresh_token,
    }))
}

/// GET /api/v1/users/me
pub async fn get_current_user(
    State(state): State<Arc<AppState>>,
    user: UserContext,
) -> Result<Json<UserResponse>, AppError> {
    let user_model = IdentityService::get_user(&state.db, user.user_id).await?;
    Ok(Json(UserResponse {
        id: user_model.id,
        email: user_model.email,
        phone: user_model.phone,
        name: user_model.name,
        avatar_url: user_model.avatar_url,
        branch_id: user_model.branch_id,
        status: user_model.status,
        last_login_at: user_model.last_login_at.map(|dt| dt.to_rfc3339()),
    }))
}

/// GET /api/v1/users
pub async fn list_users(
    State(state): State<Arc<AppState>>,
    user: UserContext,
) -> Result<Json<Vec<UserResponse>>, AppError> {
    if !user.is_company_wide {
        return Err(AppError::Forbidden("Insufficient permissions".to_string()));
    }
    let users = IdentityService::list_users(&state.db).await?;
    Ok(Json(
        users
            .into_iter()
            .map(|u| UserResponse {
                id: u.id,
                email: u.email,
                phone: u.phone,
                name: u.name,
                avatar_url: u.avatar_url,
                branch_id: u.branch_id,
                status: u.status,
                last_login_at: u.last_login_at.map(|dt| dt.to_rfc3339()),
            })
            .collect(),
    ))
}
