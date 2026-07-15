use crate::modules::discovery::application::services::DiscoveryService;
use crate::shared::app_state::AppState;
use crate::shared::errors::AppError;
use crate::shared::middleware::auth::UserContext;
use axum::extract::{Path, State};
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
    _user: UserContext,
) -> Result<Json<Vec<ScanResponse>>, AppError> {
    let scans = DiscoveryService::list_scans(&state.db).await?;
    Ok(Json(
        scans
            .into_iter()
            .map(|s| ScanResponse {
                id: s.id,
                name: s.name,
                scan_type: s.scan_type,
                is_active: s.is_active,
            })
            .collect(),
    ))
}

pub async fn create_scan(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Json(req): Json<CreateScanRequest>,
) -> Result<(StatusCode, Json<ScanResponse>), AppError> {
    let s = DiscoveryService::create_scan(
        &state.db,
        user.branch_id.unwrap_or(0),
        req.name,
        req.scan_type,
    )
    .await?;
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
    _user: UserContext,
) -> Result<Json<Vec<ResultResponse>>, AppError> {
    let results = DiscoveryService::list_results(&state.db).await?;
    Ok(Json(
        results
            .into_iter()
            .map(|r| ResultResponse {
                id: r.id,
                discovered_ip: r.discovered_ip,
                vendor: r.vendor,
                model: r.model,
                status: r.status,
            })
            .collect(),
    ))
}

pub async fn approve_result(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Path(id): Path<i64>,
) -> Result<StatusCode, AppError> {
    DiscoveryService::approve_result(&state.db, id, user.user_id).await?;
    Ok(StatusCode::OK)
}
