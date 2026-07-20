use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::modules::branches::application::services::BranchService;
use crate::shared::app_state::AppState;
use crate::shared::errors::AppError;
use crate::shared::middleware::auth::{require_permission, UserContext};

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
    Ok(Json(
        branches.into_iter().map(BranchResponse::from).collect(),
    ))
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
    )
    .await?;
    if let Err(e) = crate::infrastructure::messaging::outbox::insert_outbox_event(
        &state.db,
        "branch.created",
        "branch",
        branch.id,
        serde_json::json!({"branch_id": branch.id, "name": branch.name}),
        None,
        Some(user.user_id),
        user.branch_id,
    )
    .await
    {
        tracing::error!(error = %e, "Failed to publish branch.created event");
    }
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
        &state.db,
        id,
        req.name,
        req.city,
        req.state,
        req.address,
        req.phone,
        req.email,
    )
    .await?;
    if let Err(e) = crate::infrastructure::messaging::outbox::insert_outbox_event(
        &state.db,
        "branch.updated",
        "branch",
        branch.id,
        serde_json::json!({"branch_id": branch.id, "name": branch.name}),
        None,
        Some(user.user_id),
        user.branch_id,
    )
    .await
    {
        tracing::error!(error = %e, "Failed to publish branch.updated event");
    }
    Ok(Json(BranchResponse::from(branch)))
}

/// GET /api/v1/branches/hierarchy
pub async fn get_branch_hierarchy(
    State(state): State<Arc<AppState>>,
    user: UserContext,
) -> Result<Json<serde_json::Value>, AppError> {
    require_permission(&user, "branch.view").map_err(|e| AppError::Forbidden(e.1))?;
    let hierarchy = BranchService::get_hierarchy(&state.db).await?;
    Ok(Json(hierarchy))
}

/// DELETE /api/v1/branches/:id
pub async fn delete_branch(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Path(id): Path<i64>,
) -> Result<StatusCode, AppError> {
    require_permission(&user, "branch.delete").map_err(|e| AppError::Forbidden(e.1))?;
    BranchService::deactivate_branch(&state.db, id).await?;
    if let Err(e) = crate::infrastructure::messaging::outbox::insert_outbox_event(
        &state.db,
        "branch.deleted",
        "branch",
        id,
        serde_json::json!({"branch_id": id}),
        None,
        Some(user.user_id),
        user.branch_id,
    )
    .await
    {
        tracing::error!(error = %e, "Failed to publish branch.deleted event");
    }
    Ok(StatusCode::NO_CONTENT)
}

// ─── Working Hours ──────────────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct WorkingHoursResponse {
    pub id: i64,
    pub branch_id: i64,
    pub day_of_week: i32,
    pub open_time: String,
    pub close_time: String,
    pub is_closed: bool,
}

impl From<crate::modules::branches::domain::entities::branch_working_hours::Model>
    for WorkingHoursResponse
{
    fn from(h: crate::modules::branches::domain::entities::branch_working_hours::Model) -> Self {
        Self {
            id: h.id,
            branch_id: h.branch_id,
            day_of_week: h.day_of_week,
            open_time: h.open_time,
            close_time: h.close_time,
            is_closed: h.is_closed,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct WorkingHoursEntry {
    pub day_of_week: i32,
    pub open_time: String,
    pub close_time: String,
    pub is_closed: bool,
}

/// GET /api/v1/branches/:id/working-hours
pub async fn get_working_hours(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Path(id): Path<i64>,
) -> Result<Json<Vec<WorkingHoursResponse>>, AppError> {
    require_permission(&user, "branch.view").map_err(|e| AppError::Forbidden(e.1))?;
    let hours = BranchService::get_working_hours(&state.db, id).await?;
    Ok(Json(hours.into_iter().map(WorkingHoursResponse::from).collect()))
}

/// PUT /api/v1/branches/:id/working-hours
pub async fn update_working_hours(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Path(id): Path<i64>,
    Json(entries): Json<Vec<WorkingHoursEntry>>,
) -> Result<Json<Vec<WorkingHoursResponse>>, AppError> {
    require_permission(&user, "branch.update").map_err(|e| AppError::Forbidden(e.1))?;
    let tuples: Vec<(i32, String, String, bool)> = entries
        .into_iter()
        .map(|e| (e.day_of_week, e.open_time, e.close_time, e.is_closed))
        .collect();
    let hours = BranchService::update_working_hours(&state.db, id, tuples).await?;
    Ok(Json(hours.into_iter().map(WorkingHoursResponse::from).collect()))
}

// ─── Branch Stats ───────────────────────────────────────────────────────

/// GET /api/v1/branches/:id/stats
pub async fn get_branch_stats(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, AppError> {
    require_permission(&user, "branch.view").map_err(|e| AppError::Forbidden(e.1))?;
    let stats = BranchService::get_branch_stats(&state.db, id).await?;
    Ok(Json(stats))
}

// ─── Branch Users ───────────────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct BranchUserResponse {
    pub id: i64,
    pub branch_id: i64,
    pub user_id: i64,
    pub role: String,
}

impl From<crate::modules::branches::domain::entities::branch_user::Model> for BranchUserResponse {
    fn from(u: crate::modules::branches::domain::entities::branch_user::Model) -> Self {
        Self {
            id: u.id,
            branch_id: u.branch_id,
            user_id: u.user_id,
            role: u.role,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct AssignBranchUserRequest {
    pub user_id: i64,
    pub role: String,
}

/// POST /api/v1/branches/:id/users
pub async fn assign_branch_user(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Path(id): Path<i64>,
    Json(req): Json<AssignBranchUserRequest>,
) -> Result<(StatusCode, Json<BranchUserResponse>), AppError> {
    require_permission(&user, "branch.update").map_err(|e| AppError::Forbidden(e.1))?;
    let bu = BranchService::assign_user(&state.db, id, req.user_id, req.role).await?;
    Ok((StatusCode::CREATED, Json(BranchUserResponse::from(bu))))
}

/// DELETE /api/v1/branches/:id/users/:uid
pub async fn remove_branch_user(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Path((id, uid)): Path<(i64, i64)>,
) -> Result<StatusCode, AppError> {
    require_permission(&user, "branch.update").map_err(|e| AppError::Forbidden(e.1))?;
    BranchService::remove_user(&state.db, id, uid).await?;
    Ok(StatusCode::NO_CONTENT)
}
