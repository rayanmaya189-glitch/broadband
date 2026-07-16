use crate::modules::discovery::application::services::DiscoveryService;
use crate::shared::app_state::AppState;
use crate::shared::errors::AppError;
use crate::shared::middleware::auth::{require_permission, UserContext};
use crate::shared::primitives::PaginationParams;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Serialize)]
pub struct ScanResponse {
    pub id: i64,
    pub name: String,
    pub scan_type: String,
    pub is_active: bool,
}

#[derive(Debug, Deserialize)]
pub struct CreateScanRequest {
    pub name: String,
    pub scan_type: String,
}

#[derive(Debug, Serialize)]
pub struct ResultResponse {
    pub id: i64,
    pub discovered_ip: String,
    pub vendor: Option<String>,
    pub model: Option<String>,
    pub status: String,
}

pub async fn list_scans(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Query(p): Query<PaginationParams>,
) -> Result<Json<serde_json::Value>, AppError> {
    require_permission(&user, "discovery.scan.view").map_err(|e| AppError::Forbidden(e.1))?;
    let (scans, total) = DiscoveryService::list_scans(&state.db, p.page(), p.limit()).await?;
    let items: Vec<ScanResponse> = scans
            .into_iter()
            .map(|s| ScanResponse {
                id: s.id,
                name: s.name,
                scan_type: s.scan_type,
                is_active: s.is_active,
            })
            .collect();
    Ok(Json(serde_json::json!({"items": items, "total": total, "page": p.page(), "limit": p.limit()})))
}

pub async fn create_scan(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Json(req): Json<CreateScanRequest>,
) -> Result<(StatusCode, Json<ScanResponse>), AppError> {
    require_permission(&user, "discovery.scan.create").map_err(|e| AppError::Forbidden(e.1))?;
    let s = DiscoveryService::create_scan(
        &state.db,
        user.branch_id.unwrap_or(0),
        req.name,
        req.scan_type,
    )
    .await?;
    if let Err(e) = crate::infrastructure::messaging::outbox::insert_outbox_event(
        &state.db, "discovery.scan.created", "discovery_scan", s.id,
        serde_json::json!({"scan_id": s.id, "name": s.name}), None,
        Some(user.user_id), user.branch_id,
    ).await {
        tracing::error!(error = %e, "Failed to publish discovery.scan.created event");
    }
    Ok((
        StatusCode::CREATED,
        Json(ScanResponse {
            id: s.id,
            name: s.name,
            scan_type: s.scan_type,
            is_active: s.is_active,
        }),
    ))
}

pub async fn list_results(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Query(p): Query<PaginationParams>,
) -> Result<Json<serde_json::Value>, AppError> {
    require_permission(&user, "discovery.result.view").map_err(|e| AppError::Forbidden(e.1))?;
    let (results, total) = DiscoveryService::list_results(&state.db, p.page(), p.limit()).await?;
    let items: Vec<ResultResponse> = results
            .into_iter()
            .map(|r| ResultResponse {
                id: r.id,
                discovered_ip: r.discovered_ip,
                vendor: r.vendor,
                model: r.model,
                status: r.status,
            })
            .collect();
    Ok(Json(serde_json::json!({"items": items, "total": total, "page": p.page(), "limit": p.limit()})))
}

pub async fn approve_result(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Path(id): Path<i64>,
) -> Result<StatusCode, AppError> {
    require_permission(&user, "discovery.result.approve").map_err(|e| AppError::Forbidden(e.1))?;
    DiscoveryService::approve_result(&state.db, id, user.user_id).await?;
    if let Err(e) = crate::infrastructure::messaging::outbox::insert_outbox_event(
        &state.db, "discovery.result.approved", "discovery_result", id,
        serde_json::json!({"result_id": id}), None,
        Some(user.user_id), user.branch_id,
    ).await {
        tracing::error!(error = %e, "Failed to publish discovery.result.approved event");
    }
    Ok(StatusCode::OK)
}
