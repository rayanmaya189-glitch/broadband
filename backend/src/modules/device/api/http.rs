use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::modules::device::application::services::DeviceService;
use crate::shared::app_state::AppState;
use crate::shared::errors::AppError;
use crate::shared::middleware::auth::{require_permission, UserContext};
use crate::shared::primitives::PaginationParams;

#[derive(Debug, Serialize)]
pub struct DeviceResponse {
    pub id: i64,
    pub name: String,
    pub serial_number: String,
    pub management_ip: String,
    pub status: String,
    pub health_score: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct RegisterDeviceRequest {
    pub branch_id: i64,
    pub name: String,
    pub device_model_id: i64,
    pub serial_number: String,
    pub management_ip: String,
}

pub async fn list_devices(
    State(state): State<Arc<AppState>>,
    user: UserContext,
) -> Result<Json<Vec<DeviceResponse>>, AppError> {
    require_permission(&user, "device.router.view").map_err(|e| AppError::Forbidden(e.1))?;
    let bid = if user.is_company_wide {
        None
    } else {
        user.branch_id
    };
    let devices = DeviceService::list_devices(&state.db, bid).await?;
    Ok(Json(
        devices
            .into_iter()
            .map(|d| DeviceResponse {
                id: d.id,
                name: d.name,
                serial_number: d.serial_number,
                management_ip: d.management_ip,
                status: d.status,
                health_score: d.health_score,
            })
            .collect(),
    ))
}

pub async fn get_device(
    State(state): State<Arc<AppState>>,
    _user: UserContext,
    Path(id): Path<i64>,
) -> Result<Json<DeviceResponse>, AppError> {
    let d = DeviceService::get_device(&state.db, id).await?;
    Ok(Json(DeviceResponse {
        id: d.id,
        name: d.name,
        serial_number: d.serial_number,
        management_ip: d.management_ip,
        status: d.status,
        health_score: d.health_score,
    }))
}

pub async fn register_device(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Json(req): Json<RegisterDeviceRequest>,
) -> Result<(StatusCode, Json<DeviceResponse>), AppError> {
    require_permission(&user, "device.router.register").map_err(|e| AppError::Forbidden(e.1))?;
    let d = DeviceService::register_device(
        &state.db,
        req.branch_id,
        req.name,
        req.device_model_id,
        req.serial_number,
        req.management_ip,
    )
    .await?;
    // Publish event to outbox
    if let Err(e) = crate::infrastructure::messaging::outbox::insert_outbox_event(
        &state.db,
        "device.registered",
        "device",
        d.id,
        serde_json::json!({"device_id": d.id, "name": d.name, "status": d.status}),
        None,
        Some(user.user_id),
        user.branch_id,
    )
    .await
    {
        tracing::error!(error = %e, "Failed to publish device.registered event");
    }

    Ok((
        StatusCode::CREATED,
        Json(DeviceResponse {
            id: d.id,
            name: d.name,
            serial_number: d.serial_number,
            management_ip: d.management_ip,
            status: d.status,
            health_score: d.health_score,
        }),
    ))
}

pub async fn update_device_status(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Path(id): Path<i64>,
    Json(req): Json<UpdateStatusRequest>,
) -> Result<Json<DeviceResponse>, AppError> {
    require_permission(&user, "device.router.update_status").map_err(|e| AppError::Forbidden(e.1))?;
    let d = DeviceService::update_device_status(&state.db, id, &req.status).await?;
    // Publish event to outbox
    if let Err(e) = crate::infrastructure::messaging::outbox::insert_outbox_event(
        &state.db,
        "device.status.updated",
        "device",
        d.id,
        serde_json::json!({"device_id": d.id, "new_status": d.status}),
        None,
        Some(user.user_id),
        user.branch_id,
    )
    .await
    {
        tracing::error!(error = %e, "Failed to publish device.status.updated event");
    }

    Ok(Json(DeviceResponse {
        id: d.id,
        name: d.name,
        serial_number: d.serial_number,
        management_ip: d.management_ip,
        status: d.status,
        health_score: d.health_score,
    }))
}

#[derive(Debug, Deserialize)]
pub struct UpdateStatusRequest {
    pub status: String,
}

// ─── Device Restart ──────────────────────────────────────────────────────────

/// POST /api/v1/devices/:id/restart
pub async fn restart_device(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, AppError> {
    require_permission(&user, "device.router.restart").map_err(|e| AppError::Forbidden(e.1))?;
    let d = DeviceService::restart_device(&state.db, id).await?;
    if let Err(e) = crate::infrastructure::messaging::outbox::insert_outbox_event(
        &state.db,
        "device.restarted",
        "device",
        d.id,
        serde_json::json!({"device_id": d.id, "name": d.name}),
        None,
        Some(user.user_id),
        user.branch_id,
    )
    .await
    {
        tracing::error!(error = %e, "Failed to publish device.restarted event");
    }
    Ok(Json(serde_json::json!({
        "device_id": d.id,
        "status": "restarting",
        "message": "Device restart initiated",
    })))
}

// ─── Device Shutdown ─────────────────────────────────────────────────────────

/// POST /api/v1/devices/:id/shutdown
pub async fn shutdown_device(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, AppError> {
    require_permission(&user, "device.router.shutdown")
        .map_err(|e| AppError::Forbidden(e.1))?;
    let d = DeviceService::update_device_status(&state.db, id, "offline").await?;
    if let Err(e) = crate::infrastructure::messaging::outbox::insert_outbox_event(
        &state.db,
        "device.shutdown",
        "device",
        d.id,
        serde_json::json!({"device_id": d.id, "name": d.name}),
        None,
        Some(user.user_id),
        user.branch_id,
    )
    .await
    {
        tracing::error!(error = %e, "Failed to publish device.shutdown event");
    }
    Ok(Json(serde_json::json!({
        "device_id": d.id,
        "status": "offline",
        "message": "Device shutdown initiated",
    })))
}

// ─── Device Configure ────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct ConfigureDeviceRequest {
    pub configuration: serde_json::Value,
}

/// PUT /api/v1/devices/:id/configure
pub async fn configure_device(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Path(id): Path<i64>,
    Json(req): Json<ConfigureDeviceRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    require_permission(&user, "device.router.configure")
        .map_err(|e| AppError::Forbidden(e.1))?;
    let d = DeviceService::get_device(&state.db, id).await?;
    if let Err(e) = crate::infrastructure::messaging::outbox::insert_outbox_event(
        &state.db,
        "device.configuration.updated",
        "device",
        d.id,
        serde_json::json!({"device_id": d.id, "configuration": req.configuration}),
        None,
        Some(user.user_id),
        user.branch_id,
    )
    .await
    {
        tracing::error!(error = %e, "Failed to publish device.configuration.updated event");
    }
    Ok(Json(serde_json::json!({
        "device_id": d.id,
        "status": "configured",
        "message": "Device configuration update queued",
    })))
}

// ─── Device Ports ────────────────────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct PortResponse {
    pub id: i64,
    pub device_id: i64,
    pub port_number: i32,
    pub port_name: Option<String>,
    pub port_type: Option<String>,
    pub speed_mbps: Option<i32>,
    pub status: String,
}

/// GET /api/v1/devices/:id/ports
pub async fn list_device_ports(
    State(state): State<Arc<AppState>>,
    _user: UserContext,
    Path(id): Path<i64>,
) -> Result<Json<Vec<PortResponse>>, AppError> {
    let ports = DeviceService::list_ports(&state.db, id).await?;
    Ok(Json(
        ports
            .into_iter()
            .map(|p| PortResponse {
                id: p.id,
                device_id: p.device_id,
                port_number: p.port_number,
                port_name: p.port_name,
                port_type: p.port_type,
                speed_mbps: p.speed_mbps,
                status: p.status,
            })
            .collect(),
    ))
}

#[derive(Debug, Deserialize)]
pub struct UpdatePortRequest {
    pub status: String,
}

/// PUT /api/v1/devices/:id/ports/:pid
pub async fn update_device_port(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Path((_device_id, pid)): Path<(i64, i64)>,
    Json(req): Json<UpdatePortRequest>,
) -> Result<Json<PortResponse>, AppError> {
    require_permission(&user, "device.port.update").map_err(|e| AppError::Forbidden(e.1))?;
    let p = DeviceService::update_port_status(&state.db, pid, &req.status).await?;
    Ok(Json(PortResponse {
        id: p.id,
        device_id: p.device_id,
        port_number: p.port_number,
        port_name: p.port_name,
        port_type: p.port_type,
        speed_mbps: p.speed_mbps,
        status: p.status,
    }))
}

// ─── Device Logs ─────────────────────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct DeviceLogResponse {
    pub id: i64,
    pub device_id: i64,
    pub level: String,
    pub message: String,
    pub source: Option<String>,
    pub created_at: String,
}

/// GET /api/v1/devices/:id/logs
pub async fn list_device_logs(
    State(state): State<Arc<AppState>>,
    _user: UserContext,
    Path(id): Path<i64>,
    Query(p): Query<PaginationParams>,
) -> Result<Json<serde_json::Value>, AppError> {
    let (logs, total) = DeviceService::list_logs(&state.db, id, p.page(), p.limit()).await?;
    let resp: Vec<DeviceLogResponse> = logs
        .into_iter()
        .map(|l| DeviceLogResponse {
            id: l.id,
            device_id: l.device_id,
            level: l.level,
            message: l.message,
            source: l.source,
            created_at: l.created_at.to_rfc3339(),
        })
        .collect();
    Ok(Json(
        serde_json::json!({"items": resp, "total": total, "page": p.page(), "limit": p.limit()}),
    ))
}

// ─── Device Metrics ──────────────────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct DeviceMetricResponse {
    pub id: i64,
    pub device_id: i64,
    pub metric_name: String,
    pub metric_value: String,
    pub unit: Option<String>,
    pub recorded_at: String,
}

/// GET /api/v1/devices/:id/metrics
pub async fn list_device_metrics(
    State(state): State<Arc<AppState>>,
    _user: UserContext,
    Path(id): Path<i64>,
    Query(p): Query<PaginationParams>,
) -> Result<Json<serde_json::Value>, AppError> {
    let (metrics, total) = DeviceService::list_metrics(&state.db, id, p.page(), p.limit()).await?;
    let resp: Vec<DeviceMetricResponse> = metrics
        .into_iter()
        .map(|m| DeviceMetricResponse {
            id: m.id,
            device_id: m.device_id,
            metric_name: m.metric_name,
            metric_value: m.metric_value.to_string(),
            unit: m.unit,
            recorded_at: m.recorded_at.to_rfc3339(),
        })
        .collect();
    Ok(Json(
        serde_json::json!({"items": resp, "total": total, "page": p.page(), "limit": p.limit()}),
    ))
}

// ─── Firmware Update ─────────────────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct FirmwareResponse {
    pub device_id: i64,
    pub firmware_version: Option<String>,
    pub firmware_update_available: Option<String>,
    pub status: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateFirmwareRequest {
    pub to_version: String,
}

/// GET /api/v1/devices/:id/firmware
pub async fn get_firmware_status(
    State(state): State<Arc<AppState>>,
    _user: UserContext,
    Path(id): Path<i64>,
) -> Result<Json<FirmwareResponse>, AppError> {
    let d = DeviceService::get_device(&state.db, id).await?;
    Ok(Json(FirmwareResponse {
        device_id: d.id,
        firmware_version: d.firmware_version,
        firmware_update_available: None,
        status: "current".to_string(),
    }))
}

/// POST /api/v1/devices/:id/firmware/update
pub async fn update_firmware(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Path(id): Path<i64>,
    Json(req): Json<UpdateFirmwareRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    require_permission(&user, "device.router.update_firmware")
        .map_err(|e| AppError::Forbidden(e.1))?;
    if let Err(e) = crate::infrastructure::messaging::outbox::insert_outbox_event(
        &state.db,
        "device.firmware.update.started",
        "device",
        id,
        serde_json::json!({"device_id": id, "to_version": req.to_version}),
        None,
        Some(user.user_id),
        user.branch_id,
    )
    .await
    {
        tracing::error!(error = %e, "Failed to publish device.firmware.update.started event");
    }
    Ok(Json(serde_json::json!({
        "device_id": id,
        "to_version": req.to_version,
        "status": "downloading",
        "message": "Firmware update initiated",
    })))
}
