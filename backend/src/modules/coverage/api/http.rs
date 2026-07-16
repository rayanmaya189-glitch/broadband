use crate::modules::coverage::application::services::CoverageService;
use crate::shared::app_state::AppState;
use crate::shared::errors::AppError;
use crate::shared::middleware::auth::{require_permission, UserContext};
use crate::shared::primitives::PaginationParams;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Serialize)]
pub struct CoverageAreaResponse {
    pub id: i64,
    pub name: String,
    pub area_type: String,
    pub is_active: bool,
}

pub async fn list_coverage_areas(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Query(p): Query<PaginationParams>,
) -> Result<Json<serde_json::Value>, AppError> {
    require_permission(&user, "coverage.view").map_err(|e| AppError::Forbidden(e.1))?;
    let bid = if user.is_company_wide {
        None
    } else {
        user.branch_id
    };
    let (areas, total) = CoverageService::list_areas(&state.db, bid, p.page(), p.limit()).await?;
    let items: Vec<CoverageAreaResponse> = areas
        .into_iter()
        .map(|a| CoverageAreaResponse {
            id: a.id,
            name: a.name,
            area_type: a.area_type,
            is_active: a.is_active,
        })
        .collect();
    Ok(Json(serde_json::json!({"items": items, "total": total, "page": p.page(), "limit": p.limit()})))
}

#[derive(Debug, Deserialize)]
pub struct CheckPincodeRequest {
    pub pincode: String,
}

#[derive(Debug, Serialize)]
pub struct AvailabilityResponse {
    pub available: bool,
    pub area_name: Option<String>,
    pub estimated_days: Option<i32>,
}

pub async fn check_availability(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CheckPincodeRequest>,
) -> Result<Json<AvailabilityResponse>, AppError> {
    match CoverageService::check_pincode(&state.db, &req.pincode).await? {
        Some(area) => Ok(Json(AvailabilityResponse {
            available: true,
            area_name: Some(area.name),
            estimated_days: area.estimated_installation_days,
        })),
        None => Ok(Json(AvailabilityResponse {
            available: false,
            area_name: None,
            estimated_days: None,
        })),
    }
}

pub async fn create_coverage_area(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Json(req): Json<CreateAreaRequest>,
) -> Result<(StatusCode, Json<CoverageAreaResponse>), AppError> {
    require_permission(&user, "coverage.create").map_err(|e| AppError::Forbidden(e.1))?;
    let a = CoverageService::create_area(
        &state.db,
        user.branch_id.unwrap_or(0),
        req.name,
        req.area_type,
    )
    .await?;
    if let Err(e) = crate::infrastructure::messaging::outbox::insert_outbox_event(
        &state.db, "coverage.area.created", "coverage_area", a.id,
        serde_json::json!({"area_id": a.id, "name": a.name}), None,
        Some(user.user_id), user.branch_id,
    ).await {
        tracing::error!(error = %e, "Failed to publish coverage.area.created event");
    }
    Ok((
        StatusCode::CREATED,
        Json(CoverageAreaResponse {
            id: a.id,
            name: a.name,
            area_type: a.area_type,
            is_active: a.is_active,
        }),
    ))
}

#[derive(Debug, Deserialize)]
pub struct CreateAreaRequest {
    pub name: String,
    pub area_type: String,
}
