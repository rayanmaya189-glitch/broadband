use axum::extract::{Json, State};

use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::modules::realtime::response::realtime_response::*;

/// Health check endpoint
#[utoipa::path(
    get,
    path = "/api/v1/realtime/health",
    tag = "Realtime",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Health check", body = HealthResponse),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn health(State(state): State<SharedState>) -> Result<Json<HealthResponse>, AppError> {
    let connections = state.ws_manager.total_connections().await;
    Ok(Json(HealthResponse {
        status: "ok".into(),
        connections,
    }))
}

/// List available WebSocket channels
#[utoipa::path(
    get,
    path = "/api/v1/realtime/channels",
    tag = "Realtime",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "List of channels", body = Vec<ChannelInfo>),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn channels(
    State(state): State<SharedState>,
) -> Result<Json<Vec<ChannelInfo>>, AppError> {
    let connections = state.ws_manager.total_connections().await;
    let channels = vec![
        ChannelInfo {
            name: "ws:noc:alerts".into(),
            description: "NOC dashboard alerts - device alerts, SLA breaches, outages".into(),
            subscribers: connections,
        },
        ChannelInfo {
            name: "ws:noc:devices".into(),
            description: "NOC device monitoring - status changes, health scores".into(),
            subscribers: connections,
        },
        ChannelInfo {
            name: "ws:noc:sessions".into(),
            description: "NOC session monitoring - customer online/offline status".into(),
            subscribers: connections,
        },
        ChannelInfo {
            name: "ws:noc:discovery".into(),
            description: "NOC device discovery - new device discoveries".into(),
            subscribers: connections,
        },
        ChannelInfo {
            name: "ws:admin:metrics".into(),
            description: "Admin metrics - real-time KPIs, revenue, subscriber count".into(),
            subscribers: connections,
        },
        ChannelInfo {
            name: "ws:customer:{id}".into(),
            description: "Customer portal - invoice, ticket, subscription updates".into(),
            subscribers: 0,
        },
        ChannelInfo {
            name: "ws:branch:{id}".into(),
            description: "Branch-wide updates - alerts, ticket escalations".into(),
            subscribers: 0,
        },
    ];
    Ok(Json(channels))
}

/// Get connection statistics
#[utoipa::path(
    get,
    path = "/api/v1/realtime/stats",
    tag = "Realtime",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Connection stats"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn stats(
    State(state): State<SharedState>,
) -> Result<Json<serde_json::Value>, AppError> {
    let total = state.ws_manager.total_connections().await;
    Ok(Json(serde_json::json!({
        "total_connections": total,
        "status": "healthy",
    })))
}
