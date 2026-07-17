use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
    response::IntoResponse,
};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, QueryOrder, QuerySelect, Set};
use serde::{Deserialize, Serialize};
use crate::shared::app_state::SharedState;
use crate::shared::errors::AppError;
use crate::modules::monitoring::domain::entities::{metric_record, monitoring_alert};

/// Query parameters for listing metrics
#[derive(Debug, Deserialize)]
pub struct MetricsQuery {
    pub device_id: Option<i64>,
    pub metric_name: Option<String>,
    pub limit: Option<i64>,
}

/// Query parameters for listing alerts
#[derive(Debug, Deserialize)]
pub struct AlertsQuery {
    pub severity: Option<String>,
    pub status: Option<String>,
    pub branch_id: Option<i64>,
}

/// Request body for creating an alert
#[derive(Debug, Deserialize)]
pub struct CreateAlertRequest {
    pub device_id: i64,
    pub branch_id: i64,
    pub severity: String,
    pub title: String,
    pub message: String,
}

/// Request body for acknowledging an alert
#[derive(Debug, Deserialize)]
pub struct AcknowledgeAlertRequest {
    pub user_id: i64,
}

/// Request body for resolving an alert
#[derive(Debug, Deserialize)]
pub struct ResolveAlertRequest {
    pub user_id: i64,
    pub notes: Option<String>,
}

/// Response for metrics
#[derive(Debug, Serialize)]
pub struct MetricsResponse {
    pub metrics: Vec<serde_json::Value>,
    pub total: usize,
}

/// Response for alerts
#[derive(Debug, Serialize)]
pub struct AlertsResponse {
    pub alerts: Vec<serde_json::Value>,
    pub total: usize,
}

/// Response for alert statistics
#[derive(Debug, Serialize)]
pub struct AlertStatsResponse {
    pub total_active: i64,
    pub critical: i64,
    pub high: i64,
    pub medium: i64,
    pub low: i64,
    pub by_branch: std::collections::HashMap<i64, i64>,
}

/// GET /api/v1/monitoring/metrics
pub async fn list_metrics(
    State(state): State<SharedState>,
    Query(query): Query<MetricsQuery>,
) -> Result<impl IntoResponse, AppError> {
    let mut q = metric_record::Entity::find();
    if let Some(did) = query.device_id {
        q = q.filter(metric_record::Column::DeviceId.eq(did));
    }
    if let Some(ref name) = query.metric_name {
        q = q.filter(metric_record::Column::MetricName.eq(name.as_str()));
    }
    let limit = query.limit.unwrap_or(100).min(500);
    let records = q
        .order_by_desc(metric_record::Column::RecordedAt)
        .limit(limit as u64)
        .all(&state.db)
        .await?;
    let total = records.len();
    let metrics: Vec<serde_json::Value> = records
        .into_iter()
        .map(|r| serde_json::json!({
            "id": r.id,
            "device_id": r.device_id,
            "branch_id": r.branch_id,
            "metric_name": r.metric_name,
            "metric_value": r.metric_value,
            "unit": r.unit,
            "recorded_at": r.recorded_at,
        }))
        .collect();
    Ok(Json(MetricsResponse { metrics, total }))
}

/// GET /api/v1/monitoring/metrics/:device_id
pub async fn get_device_metrics(
    State(state): State<SharedState>,
    Path(device_id): Path<i64>,
    Query(query): Query<MetricsQuery>,
) -> Result<impl IntoResponse, AppError> {
    let mut q = metric_record::Entity::find()
        .filter(metric_record::Column::DeviceId.eq(device_id));
    if let Some(ref name) = query.metric_name {
        q = q.filter(metric_record::Column::MetricName.eq(name.as_str()));
    }
    let limit = query.limit.unwrap_or(100).min(500);
    let records = q
        .order_by_desc(metric_record::Column::RecordedAt)
        .limit(limit as u64)
        .all(&state.db)
        .await?;
    let total = records.len();
    let metrics: Vec<serde_json::Value> = records
        .into_iter()
        .map(|r| serde_json::json!({
            "id": r.id,
            "device_id": r.device_id,
            "metric_name": r.metric_name,
            "metric_value": r.metric_value,
            "unit": r.unit,
            "recorded_at": r.recorded_at,
        }))
        .collect();
    Ok(Json(MetricsResponse { metrics, total }))
}

/// GET /api/v1/monitoring/alerts
pub async fn list_alerts(
    State(state): State<SharedState>,
    Query(query): Query<AlertsQuery>,
) -> Result<impl IntoResponse, AppError> {
    let mut q = monitoring_alert::Entity::find();
    if let Some(ref sev) = query.severity {
        q = q.filter(monitoring_alert::Column::Severity.eq(sev.as_str()));
    }
    if let Some(ref st) = query.status {
        q = q.filter(monitoring_alert::Column::Status.eq(st.as_str()));
    }
    if let Some(bid) = query.branch_id {
        q = q.filter(monitoring_alert::Column::BranchId.eq(bid));
    }
    let records = q
        .order_by_desc(monitoring_alert::Column::CreatedAt)
        .limit(200)
        .all(&state.db)
        .await?;
    let total = records.len();
    let alerts: Vec<serde_json::Value> = records
        .into_iter()
        .map(|r| serde_json::json!({
            "id": r.id,
            "device_id": r.device_id,
            "branch_id": r.branch_id,
            "severity": r.severity,
            "status": r.status,
            "title": r.title,
            "message": r.message,
            "acknowledged_by": r.acknowledged_by,
            "resolved_by": r.resolved_by,
            "resolved_at": r.resolved_at,
            "created_at": r.created_at,
        }))
        .collect();
    Ok(Json(AlertsResponse { alerts, total }))
}

/// GET /api/v1/monitoring/alerts/stats
pub async fn get_alert_stats(
    State(state): State<SharedState>,
) -> Result<impl IntoResponse, AppError> {
    let active = monitoring_alert::Entity::find()
        .filter(monitoring_alert::Column::Status.is_in(vec!["firing", "acknowledged"]))
        .all(&state.db)
        .await?;
    let total_active = active.len() as i64;
    let critical = active.iter().filter(|a| a.severity == "critical").count() as i64;
    let high = active.iter().filter(|a| a.severity == "high").count() as i64;
    let medium = active.iter().filter(|a| a.severity == "medium").count() as i64;
    let low = active.iter().filter(|a| a.severity == "low").count() as i64;
    let mut by_branch = std::collections::HashMap::new();
    for alert in &active {
        *by_branch.entry(alert.branch_id).or_insert(0) += 1;
    }
    Ok(Json(AlertStatsResponse { total_active, critical, high, medium, low, by_branch }))
}

/// POST /api/v1/monitoring/alerts
pub async fn create_alert(
    State(state): State<SharedState>,
    Json(request): Json<CreateAlertRequest>,
) -> Result<impl IntoResponse, AppError> {
    let now = chrono::Utc::now();
    let active = monitoring_alert::ActiveModel {
        device_id: Set(request.device_id),
        branch_id: Set(request.branch_id),
        severity: Set(request.severity),
        title: Set(request.title),
        message: Set(request.message),
        status: Set("firing".to_string()),
        alert_type: Set("manual".to_string()),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    };
    let result = active.insert(&state.db).await?;
    Ok((StatusCode::CREATED, Json(serde_json::json!({
        "id": result.id,
        "status": result.status,
        "created_at": result.created_at,
    }))))
}

/// POST /api/v1/monitoring/alerts/:id/acknowledge
pub async fn acknowledge_alert(
    State(state): State<SharedState>,
    Path(alert_id): Path<i64>,
    Json(request): Json<AcknowledgeAlertRequest>,
) -> Result<impl IntoResponse, AppError> {
    let alert = monitoring_alert::Entity::find_by_id(alert_id)
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Alert {} not found", alert_id)))?;
    let mut active: monitoring_alert::ActiveModel = alert.into();
    active.status = Set("acknowledged".to_string());
    active.acknowledged_by = Set(Some(request.user_id));
    active.acknowledged_at = Set(Some(chrono::Utc::now()));
    active.updated_at = Set(chrono::Utc::now());
    active.update(&state.db).await?;
    Ok(StatusCode::OK)
}

/// POST /api/v1/monitoring/alerts/:id/resolve
pub async fn resolve_alert(
    State(state): State<SharedState>,
    Path(alert_id): Path<i64>,
    Json(request): Json<ResolveAlertRequest>,
) -> Result<impl IntoResponse, AppError> {
    let alert = monitoring_alert::Entity::find_by_id(alert_id)
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Alert {} not found", alert_id)))?;
    let mut active: monitoring_alert::ActiveModel = alert.into();
    active.status = Set("resolved".to_string());
    active.resolved_by = Set(Some(request.user_id));
    active.resolved_at = Set(Some(chrono::Utc::now()));
    active.resolution_notes = Set(request.notes);
    active.updated_at = Set(chrono::Utc::now());
    active.update(&state.db).await?;
    Ok(StatusCode::OK)
}
