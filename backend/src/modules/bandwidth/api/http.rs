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
