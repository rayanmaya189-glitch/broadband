use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::modules::bandwidth::application::services::BandwidthService;
use crate::shared::app_state::AppState;
use crate::shared::errors::AppError;
use crate::shared::middleware::auth::{require_permission, UserContext};
use crate::shared::primitives::PaginationParams;

#[derive(Debug, Serialize)]
pub struct BandwidthProfileResponse {
    pub id: i64,
    pub name: String,
    pub download_kbps: i32,
    pub upload_kbps: i32,
    pub is_active: bool,
}

#[derive(Debug, Deserialize)]
pub struct CreateProfileRequest {
    pub name: String,
    pub download_kbps: i32,
    pub upload_kbps: i32,
}

pub async fn list_profiles(
    State(state): State<Arc<AppState>>,
    _user: UserContext,
    Query(p): Query<PaginationParams>,
) -> Result<Json<serde_json::Value>, AppError> {
    let (profiles, total) = BandwidthService::list_profiles(&state.db, p.page(), p.limit()).await?;
    let items: Vec<BandwidthProfileResponse> = profiles
        .into_iter()
        .map(|p| BandwidthProfileResponse {
            id: p.id,
            name: p.name,
            download_kbps: p.download_kbps,
            upload_kbps: p.upload_kbps,
            is_active: p.is_active,
        })
        .collect();
    Ok(Json(
        serde_json::json!({"items": items, "total": total, "page": p.page(), "limit": p.limit()}),
    ))
}

pub async fn create_profile(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Json(req): Json<CreateProfileRequest>,
) -> Result<(StatusCode, Json<BandwidthProfileResponse>), AppError> {
    require_permission(&user, "bandwidth.profile.create").map_err(|e| AppError::Forbidden(e.1))?;
    let p =
        BandwidthService::create_profile(&state.db, req.name, req.download_kbps, req.upload_kbps)
            .await?;
    if let Err(e) = crate::infrastructure::messaging::outbox::insert_outbox_event(
        &state.db,
        "bandwidth.profile.created",
        "bandwidth_profile",
        p.id,
        serde_json::json!({"profile_id": p.id, "name": p.name}),
        None,
        Some(user.user_id),
        user.branch_id,
    )
    .await
    {
        tracing::error!(error = %e, "Failed to publish bandwidth.profile.created event");
    }
    Ok((
        StatusCode::CREATED,
        Json(BandwidthProfileResponse {
            id: p.id,
            name: p.name,
            download_kbps: p.download_kbps,
            upload_kbps: p.upload_kbps,
            is_active: p.is_active,
        }),
    ))
}

pub async fn update_profile(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Path(id): Path<i64>,
    Json(req): Json<UpdateProfileRequest>,
) -> Result<Json<BandwidthProfileResponse>, AppError> {
    require_permission(&user, "bandwidth.profile.update").map_err(|e| AppError::Forbidden(e.1))?;
    let p = BandwidthService::update_profile(
        &state.db,
        id,
        req.name,
        req.download_kbps,
        req.upload_kbps,
    )
    .await?;
    if let Err(e) = crate::infrastructure::messaging::outbox::insert_outbox_event(
        &state.db,
        "bandwidth.profile.updated",
        "bandwidth_profile",
        p.id,
        serde_json::json!({"profile_id": p.id}),
        None,
        Some(user.user_id),
        user.branch_id,
    )
    .await
    {
        tracing::error!(error = %e, "Failed to publish bandwidth.profile.updated event");
    }
    Ok(Json(BandwidthProfileResponse {
        id: p.id,
        name: p.name,
        download_kbps: p.download_kbps,
        upload_kbps: p.upload_kbps,
        is_active: p.is_active,
    }))
}

#[derive(Debug, Deserialize)]
pub struct UpdateProfileRequest {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub download_kbps: Option<i32>,
    #[serde(default)]
    pub upload_kbps: Option<i32>,
}

pub async fn delete_profile(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Path(id): Path<i64>,
) -> Result<StatusCode, AppError> {
    require_permission(&user, "bandwidth.profile.delete").map_err(|e| AppError::Forbidden(e.1))?;
    BandwidthService::delete_profile(&state.db, id).await?;
    if let Err(e) = crate::infrastructure::messaging::outbox::insert_outbox_event(
        &state.db,
        "bandwidth.profile.deleted",
        "bandwidth_profile",
        id,
        serde_json::json!({"profile_id": id}),
        None,
        Some(user.user_id),
        user.branch_id,
    )
    .await
    {
        tracing::error!(error = %e, "Failed to publish bandwidth.profile.deleted event");
    }
    Ok(StatusCode::NO_CONTENT)
}

// ─── Bandwidth Policies ────────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct BandwidthPolicyResponse {
    pub id: i64,
    pub name: String,
    pub policy_type: String,
    pub config: serde_json::Value,
    pub priority: i32,
    pub is_active: bool,
}

#[derive(Debug, Deserialize)]
pub struct CreatePolicyRequest {
    pub name: String,
    pub policy_type: String,
    pub config: serde_json::Value,
    #[serde(default)]
    pub priority: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePolicyRequest {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub config: Option<serde_json::Value>,
    #[serde(default)]
    pub priority: Option<i32>,
    #[serde(default)]
    pub is_active: Option<bool>,
}

/// GET /api/v1/bandwidth/policies
pub async fn list_policies(
    State(state): State<Arc<AppState>>,
    _user: UserContext,
) -> Result<Json<Vec<BandwidthPolicyResponse>>, AppError> {
    let items = BandwidthService::list_policies(&state.db).await?;
    Ok(Json(items.into_iter().map(|p| BandwidthPolicyResponse {
        id: p.id,
        name: p.name,
        policy_type: p.policy_type,
        config: p.config,
        priority: p.priority,
        is_active: p.is_active,
    }).collect()))
}

/// POST /api/v1/bandwidth/policies
pub async fn create_policy(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Json(req): Json<CreatePolicyRequest>,
) -> Result<(StatusCode, Json<BandwidthPolicyResponse>), AppError> {
    require_permission(&user, "bandwidth.profile.create").map_err(|e| AppError::Forbidden(e.1))?;
    let p = BandwidthService::create_policy(
        &state.db,
        req.name,
        req.policy_type,
        req.config,
        req.priority.unwrap_or(0),
    )
    .await?;
    Ok((
        StatusCode::CREATED,
        Json(BandwidthPolicyResponse {
            id: p.id,
            name: p.name,
            policy_type: p.policy_type,
            config: p.config,
            priority: p.priority,
            is_active: p.is_active,
        }),
    ))
}

/// PUT /api/v1/bandwidth/policies/:id
pub async fn update_policy(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Path(id): Path<i64>,
    Json(req): Json<UpdatePolicyRequest>,
) -> Result<Json<BandwidthPolicyResponse>, AppError> {
    require_permission(&user, "bandwidth.profile.update").map_err(|e| AppError::Forbidden(e.1))?;
    let p = BandwidthService::update_policy(
        &state.db,
        id,
        req.name,
        req.config,
        req.priority,
        req.is_active,
    )
    .await?;
    Ok(Json(BandwidthPolicyResponse {
        id: p.id,
        name: p.name,
        policy_type: p.policy_type,
        config: p.config,
        priority: p.priority,
        is_active: p.is_active,
    }))
}

/// DELETE /api/v1/bandwidth/policies/:id
pub async fn delete_policy(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Path(id): Path<i64>,
) -> Result<StatusCode, AppError> {
    require_permission(&user, "bandwidth.profile.delete").map_err(|e| AppError::Forbidden(e.1))?;
    BandwidthService::delete_policy(&state.db, id).await?;
    Ok(StatusCode::NO_CONTENT)
}

// ─── Get Single Profile ─────────────────────────────────────────────────

/// GET /api/v1/bandwidth/profiles/:id
pub async fn get_profile(
    State(state): State<Arc<AppState>>,
    _user: UserContext,
    Path(id): Path<i64>,
) -> Result<Json<BandwidthProfileResponse>, AppError> {
    let p = BandwidthService::get_profile(&state.db, id).await?;
    Ok(Json(BandwidthProfileResponse {
        id: p.id,
        name: p.name,
        download_kbps: p.download_kbps,
        upload_kbps: p.upload_kbps,
        is_active: p.is_active,
    }))
}

// ─── Apply Profile to All ──────────────────────────────────────────────

/// POST /api/v1/bandwidth/profiles/:id/apply
pub async fn apply_profile_to_all(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, AppError> {
    require_permission(&user, "bandwidth.profile.update").map_err(|e| AppError::Forbidden(e.1))?;
    let count = BandwidthService::apply_profile(&state.db, id).await?;
    Ok(Json(serde_json::json!({
        "profile_id": id,
        "updated_count": count,
    })))
}

// ─── Apply Profile to Subscription ─────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct ApplyProfileRequest {
    pub profile_id: i64,
}

/// POST /api/v1/bandwidth/apply/:subscription_id
pub async fn apply_to_subscription(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Path(subscription_id): Path<i64>,
    Json(req): Json<ApplyProfileRequest>,
) -> Result<(StatusCode, Json<serde_json::Value>), AppError> {
    require_permission(&user, "bandwidth.profile.update").map_err(|e| AppError::Forbidden(e.1))?;
    let app = BandwidthService::apply_to_subscription(&state.db, subscription_id, req.profile_id)
        .await?;
    Ok((
        StatusCode::CREATED,
        Json(serde_json::json!({
            "id": app.id,
            "subscription_id": app.subscription_id,
            "profile_id": app.profile_id,
            "status": app.status,
        })),
    ))
}

// ─── List Bandwidth Applications ───────────────────────────────────────

/// GET /api/v1/bandwidth/applications
pub async fn list_bandwidth_applications(
    State(state): State<Arc<AppState>>,
    _user: UserContext,
) -> Result<Json<Vec<serde_json::Value>>, AppError> {
    let apps = BandwidthService::list_applications(&state.db).await?;
    Ok(Json(
        apps.into_iter()
            .map(|a| {
                serde_json::json!({
                    "id": a.id,
                    "profile_id": a.profile_id,
                    "subscription_id": a.subscription_id,
                    "status": a.status,
                    "applied_at": a.applied_at,
                    "retry_count": a.retry_count,
                    "created_at": a.created_at,
                })
            })
            .collect(),
    ))
}

// ─── Bandwidth Usage ───────────────────────────────────────────────────

/// GET /api/v1/bandwidth/usage/:subscription_id
pub async fn get_bandwidth_usage(
    State(state): State<Arc<AppState>>,
    _user: UserContext,
    Path(subscription_id): Path<i64>,
) -> Result<Json<serde_json::Value>, AppError> {
    let usage = BandwidthService::get_usage(&state.db, subscription_id).await?;
    Ok(Json(usage))
}
