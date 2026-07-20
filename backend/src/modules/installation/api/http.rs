use crate::modules::installation::application::services::InstallationService;
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
pub struct InstallationResponse {
    pub id: i64,
    pub customer_id: i64,
    pub status: String,
    pub scheduled_date: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateOrderRequest {
    pub customer_id: i64,
    #[serde(default)]
    pub subscription_id: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct ScheduleRequest {
    pub scheduled_date: String,
    #[serde(default)]
    pub scheduled_time_slot: Option<String>,
    #[serde(default)]
    pub technician_id: Option<i64>,
}

pub async fn list_installations(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Query(p): Query<PaginationParams>,
) -> Result<Json<serde_json::Value>, AppError> {
    require_permission(&user, "installation.order.view").map_err(|e| AppError::Forbidden(e.1))?;
    let bid = if user.is_company_wide {
        None
    } else {
        user.branch_id
    };
    let (orders, total) =
        InstallationService::list_orders(&state.db, bid, p.page(), p.limit()).await?;
    let items: Vec<InstallationResponse> = orders
        .into_iter()
        .map(|o| InstallationResponse {
            id: o.id,
            customer_id: o.customer_id,
            status: o.status,
            scheduled_date: o.scheduled_date.map(|d| d.to_string()),
        })
        .collect();
    Ok(Json(
        serde_json::json!({"items": items, "total": total, "page": p.page(), "limit": p.limit()}),
    ))
}

pub async fn create_installation(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Json(req): Json<CreateOrderRequest>,
) -> Result<(StatusCode, Json<InstallationResponse>), AppError> {
    require_permission(&user, "installation.order.create").map_err(|e| AppError::Forbidden(e.1))?;
    let o = InstallationService::create_order(
        &state.db,
        req.customer_id,
        user.branch_id.unwrap_or(0),
        req.subscription_id,
    )
    .await?;
    if let Err(e) = crate::infrastructure::messaging::outbox::insert_outbox_event(
        &state.db,
        "installation.created",
        "installation_order",
        o.id,
        serde_json::json!({"order_id": o.id}),
        None,
        Some(user.user_id),
        user.branch_id,
    )
    .await
    {
        tracing::error!(error = %e, "Failed to publish installation.created event");
    }
    Ok((
        StatusCode::CREATED,
        Json(InstallationResponse {
            id: o.id,
            customer_id: o.customer_id,
            status: o.status,
            scheduled_date: o.scheduled_date.map(|d| d.to_string()),
        }),
    ))
}

pub async fn schedule_installation(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Path(id): Path<i64>,
    Json(req): Json<ScheduleRequest>,
) -> Result<Json<InstallationResponse>, AppError> {
    require_permission(&user, "installation.order.schedule")
        .map_err(|e| AppError::Forbidden(e.1))?;
    let date: chrono::NaiveDate = req
        .scheduled_date
        .parse()
        .map_err(|_| AppError::Validation("Invalid date".into()))?;
    let o = InstallationService::schedule_order(
        &state.db,
        id,
        date,
        req.scheduled_time_slot,
        req.technician_id,
    )
    .await?;
    if let Err(e) = crate::infrastructure::messaging::outbox::insert_outbox_event(
        &state.db,
        "installation.scheduled",
        "installation_order",
        o.id,
        serde_json::json!({"order_id": o.id, "status": o.status}),
        None,
        Some(user.user_id),
        user.branch_id,
    )
    .await
    {
        tracing::error!(error = %e, "Failed to publish installation.scheduled event");
    }
    Ok(Json(InstallationResponse {
        id: o.id,
        customer_id: o.customer_id,
        status: o.status,
        scheduled_date: o.scheduled_date.map(|d| d.to_string()),
    }))
}

pub async fn complete_installation(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Path(id): Path<i64>,
) -> Result<Json<InstallationResponse>, AppError> {
    require_permission(&user, "installation.order.complete")
        .map_err(|e| AppError::Forbidden(e.1))?;
    let o = InstallationService::complete_order(&state.db, id).await?;
    if let Err(e) = crate::infrastructure::messaging::outbox::insert_outbox_event(
        &state.db,
        "installation.completed",
        "installation_order",
        o.id,
        serde_json::json!({"order_id": o.id, "status": o.status}),
        None,
        Some(user.user_id),
        user.branch_id,
    )
    .await
    {
        tracing::error!(error = %e, "Failed to publish installation.completed event");
    }
    Ok(Json(InstallationResponse {
        id: o.id,
        customer_id: o.customer_id,
        status: o.status,
        scheduled_date: o.scheduled_date.map(|d| d.to_string()),
    }))
}

pub async fn cancel_installation(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Path(id): Path<i64>,
) -> Result<StatusCode, AppError> {
    require_permission(&user, "installation.order.cancel").map_err(|e| AppError::Forbidden(e.1))?;
    InstallationService::cancel_order(&state.db, id).await?;
    if let Err(e) = crate::infrastructure::messaging::outbox::insert_outbox_event(
        &state.db,
        "installation.cancelled",
        "installation_order",
        id,
        serde_json::json!({"order_id": id}),
        None,
        Some(user.user_id),
        user.branch_id,
    )
    .await
    {
        tracing::error!(error = %e, "Failed to publish installation.cancelled event");
    }
    Ok(StatusCode::OK)
}

// ─── Equipment Tracking ────────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct EquipmentResponse {
    pub id: i64,
    pub installation_order_id: i64,
    pub equipment_type: String,
    pub model_name: Option<String>,
    pub serial_number: Option<String>,
    pub quantity: i32,
    pub status: String,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AddEquipmentRequest {
    pub equipment_type: String,
    #[serde(default)]
    pub model_name: Option<String>,
    #[serde(default)]
    pub serial_number: Option<String>,
    #[serde(default)]
    pub quantity: Option<i32>,
    #[serde(default)]
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateEquipmentStatusRequest {
    pub status: String,
}

/// GET /api/v1/installations/:id/equipment
pub async fn list_equipment(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Path(id): Path<i64>,
) -> Result<Json<Vec<EquipmentResponse>>, AppError> {
    require_permission(&user, "installation.order.view").map_err(|e| AppError::Forbidden(e.1))?;
    let items = InstallationService::list_equipment(&state.db, id).await?;
    Ok(Json(items.into_iter().map(|e| EquipmentResponse {
        id: e.id,
        installation_order_id: e.installation_order_id,
        equipment_type: e.equipment_type,
        model_name: e.model_name,
        serial_number: e.serial_number,
        quantity: e.quantity,
        status: e.status,
        notes: e.notes,
    }).collect()))
}

/// POST /api/v1/installations/:id/equipment
pub async fn add_equipment(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Path(id): Path<i64>,
    Json(req): Json<AddEquipmentRequest>,
) -> Result<(StatusCode, Json<EquipmentResponse>), AppError> {
    require_permission(&user, "installation.order.create").map_err(|e| AppError::Forbidden(e.1))?;
    let item = InstallationService::add_equipment(
        &state.db,
        id,
        req.equipment_type,
        req.model_name,
        req.serial_number,
        req.quantity.unwrap_or(1),
        req.notes,
    )
    .await?;
    Ok((
        StatusCode::CREATED,
        Json(EquipmentResponse {
            id: item.id,
            installation_order_id: item.installation_order_id,
            equipment_type: item.equipment_type,
            model_name: item.model_name,
            serial_number: item.serial_number,
            quantity: item.quantity,
            status: item.status,
            notes: item.notes,
        }),
    ))
}

/// PUT /api/v1/installations/equipment/:equipment_id/status
pub async fn update_equipment_status(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Path(equipment_id): Path<i64>,
    Json(req): Json<UpdateEquipmentStatusRequest>,
) -> Result<Json<EquipmentResponse>, AppError> {
    require_permission(&user, "installation.order.update").map_err(|e| AppError::Forbidden(e.1))?;
    let item = InstallationService::update_equipment_status(&state.db, equipment_id, &req.status)
        .await?;
    Ok(Json(EquipmentResponse {
        id: item.id,
        installation_order_id: item.installation_order_id,
        equipment_type: item.equipment_type,
        model_name: item.model_name,
        serial_number: item.serial_number,
        quantity: item.quantity,
        status: item.status,
        notes: item.notes,
    }))
}

// ─── Get Single Installation ───────────────────────────────────────────

/// GET /api/v1/installations/:id
pub async fn get_installation(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Path(id): Path<i64>,
) -> Result<Json<InstallationResponse>, AppError> {
    require_permission(&user, "installation.order.view").map_err(|e| AppError::Forbidden(e.1))?;
    let o = InstallationService::get_order(&state.db, id).await?;
    Ok(Json(InstallationResponse {
        id: o.id,
        customer_id: o.customer_id,
        status: o.status,
        scheduled_date: o.scheduled_date.map(|d| d.to_string()),
    }))
}

// ─── Reschedule ────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct RescheduleRequest {
    pub new_date: String,
    #[serde(default)]
    pub new_time_slot: Option<String>,
}

/// POST /api/v1/installations/:id/reschedule
pub async fn reschedule_installation(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Path(id): Path<i64>,
    Json(req): Json<RescheduleRequest>,
) -> Result<Json<InstallationResponse>, AppError> {
    require_permission(&user, "installation.order.schedule")
        .map_err(|e| AppError::Forbidden(e.1))?;
    let date: chrono::NaiveDate = req
        .new_date
        .parse()
        .map_err(|_| AppError::Validation("Invalid date".into()))?;
    let o = InstallationService::reschedule_order(&state.db, id, date, req.new_time_slot).await?;
    Ok(Json(InstallationResponse {
        id: o.id,
        customer_id: o.customer_id,
        status: o.status,
        scheduled_date: o.scheduled_date.map(|d| d.to_string()),
    }))
}

// ─── Start Installation ────────────────────────────────────────────────

/// POST /api/v1/installations/:id/start
pub async fn start_installation(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Path(id): Path<i64>,
) -> Result<Json<InstallationResponse>, AppError> {
    require_permission(&user, "installation.order.complete")
        .map_err(|e| AppError::Forbidden(e.1))?;
    let o = InstallationService::start_order(&state.db, id).await?;
    Ok(Json(InstallationResponse {
        id: o.id,
        customer_id: o.customer_id,
        status: o.status,
        scheduled_date: o.scheduled_date.map(|d| d.to_string()),
    }))
}

// ─── Add Photo ─────────────────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct PhotoResponse {
    pub id: i64,
    pub installation_order_id: i64,
    pub storage_key: String,
    pub storage_bucket: String,
    pub photo_type: String,
    pub uploaded_by: Option<i64>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AddPhotoRequest {
    pub storage_key: String,
    pub storage_bucket: String,
    pub photo_type: String,
    #[serde(default)]
    pub notes: Option<String>,
}

/// POST /api/v1/installations/:id/photos
pub async fn add_installation_photo(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Path(id): Path<i64>,
    Json(req): Json<AddPhotoRequest>,
) -> Result<(StatusCode, Json<PhotoResponse>), AppError> {
    require_permission(&user, "installation.order.create").map_err(|e| AppError::Forbidden(e.1))?;
    let photo = InstallationService::add_photo(
        &state.db,
        id,
        req.storage_key,
        req.storage_bucket,
        req.photo_type,
        Some(user.user_id),
        req.notes,
    )
    .await?;
    Ok((
        StatusCode::CREATED,
        Json(PhotoResponse {
            id: photo.id,
            installation_order_id: photo.installation_order_id,
            storage_key: photo.storage_key,
            storage_bucket: photo.storage_bucket,
            photo_type: photo.photo_type,
            uploaded_by: photo.uploaded_by,
            notes: photo.notes,
        }),
    ))
}

// ─── My Assignments ────────────────────────────────────────────────────

/// GET /api/v1/installations/my-assignments
pub async fn list_my_installation_assignments(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Query(p): Query<PaginationParams>,
) -> Result<Json<serde_json::Value>, AppError> {
    let (orders, total) =
        InstallationService::list_my_assignments(&state.db, user.user_id, p.page(), p.limit())
            .await?;
    let items: Vec<InstallationResponse> = orders
        .into_iter()
        .map(|o| InstallationResponse {
            id: o.id,
            customer_id: o.customer_id,
            status: o.status,
            scheduled_date: o.scheduled_date.map(|d| d.to_string()),
        })
        .collect();
    Ok(Json(
        serde_json::json!({"items": items, "total": total, "page": p.page(), "limit": p.limit()}),
    ))
}
