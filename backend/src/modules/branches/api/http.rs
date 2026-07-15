use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::shared::app_state::AppState;
use crate::shared::errors::AppError;
use crate::shared::middleware::auth::{UserContext, require_permission};
use crate::modules::branches::application::services::BranchService;

#[derive(Debug, Serialize)]
pub struct BranchResponse {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub code: String,
    pub city: String,
    pub state: String,
    pub address: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub timezone: String,
    pub is_active: bool,
}

impl From<crate::modules::branches::domain::entities::branch::Model> for BranchResponse {
    fn from(b: crate::modules::branches::domain::entities::branch::Model) -> Self {
        Self {
            id: b.id,
            name: b.name,
            slug: b.slug,
            code: b.code,
            city: b.city,
            state: b.state,
            address: b.address,
            phone: b.phone,
            email: b.email,
            timezone: b.timezone,
            is_active: b.is_active,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateBranchRequest {
    pub name: String,
    pub slug: String,
    pub code: String,
    pub city: String,
    #[serde(default)]
    pub state: Option<String>,
    #[serde(default)]
    pub address: Option<String>,
    #[serde(default)]
    pub phone: Option<String>,
    #[serde(default)]
    pub email: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateBranchRequest {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub city: Option<String>,
    #[serde(default)]
    pub state: Option<String>,
    #[serde(default)]
    pub address: Option<String>,
    #[serde(default)]
    pub phone: Option<String>,
    #[serde(default)]
    pub email: Option<String>,
}

/// GET /api/v1/branches
pub async fn list_branches(
    State(state): State<Arc<AppState>>,
    user: UserContext,
) -> Result<Json<Vec<BranchResponse>>, AppError> {
    require_permission(&user, "branch.view").map_err(|e| AppError::Forbidden(e.1))?;
    let branches = BranchService::list_branches(&state.db).await?;
    Ok(Json(branches.into_iter().map(BranchResponse::from).collect()))
}

/// POST /api/v1/branches
pub async fn create_branch(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Json(req): Json<CreateBranchRequest>,
) -> Result<(StatusCode, Json<BranchResponse>), AppError> {
    require_permission(&user, "branch.create").map_err(|e| AppError::Forbidden(e.1))?;
    let branch = BranchService::create_branch(
        &state.db,
        req.name,
        req.slug,
        req.code,
        req.city,
        req.state.unwrap_or_else(|| "Maharashtra".to_string()),
        req.address,
        req.phone,
        req.email,
    ).await?;
    Ok((StatusCode::CREATED, Json(BranchResponse::from(branch))))
}

/// GET /api/v1/branches/:id
pub async fn get_branch(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Path(id): Path<i64>,
) -> Result<Json<BranchResponse>, AppError> {
    require_permission(&user, "branch.view").map_err(|e| AppError::Forbidden(e.1))?;
    let branch = BranchService::get_branch(&state.db, id).await?;
    Ok(Json(BranchResponse::from(branch)))
}

/// PUT /api/v1/branches/:id
pub async fn update_branch(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Path(id): Path<i64>,
    Json(req): Json<UpdateBranchRequest>,
) -> Result<Json<BranchResponse>, AppError> {
    require_permission(&user, "branch.update").map_err(|e| AppError::Forbidden(e.1))?;
    let branch = BranchService::update_branch(
        &state.db, id, req.name, req.city, req.state, req.address, req.phone, req.email,
    ).await?;
    Ok(Json(BranchResponse::from(branch)))
}

/// DELETE /api/v1/branches/:id
pub async fn delete_branch(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Path(id): Path<i64>,
) -> Result<StatusCode, AppError> {
    require_permission(&user, "branch.delete").map_err(|e| AppError::Forbidden(e.1))?;
    BranchService::deactivate_branch(&state.db, id).await?;
    Ok(StatusCode::NO_CONTENT)
}
