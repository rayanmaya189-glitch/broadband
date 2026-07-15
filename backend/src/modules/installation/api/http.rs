use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::shared::app_state::AppState;
use crate::shared::errors::AppError;
use crate::shared::middleware::auth::UserContext;
use crate::modules::installation::application::services::InstallationService;

#[derive(Debug, Serialize)]
pub struct InstallationResponse { pub id: i64, pub customer_id: i64, pub status: String, pub scheduled_date: Option<String> }

#[derive(Debug, Deserialize)]
pub struct CreateOrderRequest { pub customer_id: i64, pub subscription_id: Option<i64> }

#[derive(Debug, Deserialize)]
pub struct ScheduleRequest { pub scheduled_date: String, pub scheduled_time_slot: Option<String>, pub technician_id: Option<i64> }

pub async fn list_installations(State(state): State<Arc<AppState>>, user: UserContext) -> Result<Json<Vec<InstallationResponse>>, AppError> {
    let bid = if user.is_company_wide { None } else { user.branch_id };
    let orders = InstallationService::list_orders(&state.db, bid).await?;
    Ok(Json(orders.into_iter().map(|o| InstallationResponse { id: o.id, customer_id: o.customer_id, status: o.status, scheduled_date: o.scheduled_date.map(|d| d.to_string()) }).collect()))
}

pub async fn create_installation(State(state): State<Arc<AppState>>, user: UserContext, Json(req): Json<CreateOrderRequest>) -> Result<(StatusCode, Json<InstallationResponse>), AppError> {
    let o = InstallationService::create_order(&state.db, req.customer_id, user.branch_id.unwrap_or(0), req.subscription_id).await?;
    Ok((StatusCode::CREATED, Json(InstallationResponse { id: o.id, customer_id: o.customer_id, status: o.status, scheduled_date: o.scheduled_date.map(|d| d.to_string()) })))
}

pub async fn schedule_installation(State(state): State<Arc<AppState>>, _user: UserContext, Path(id): Path<i64>, Json(req): Json<ScheduleRequest>) -> Result<Json<InstallationResponse>, AppError> {
    let date: chrono::NaiveDate = req.scheduled_date.parse().map_err(|_| AppError::Validation("Invalid date".into()))?;
    let o = InstallationService::schedule_order(&state.db, id, date, req.scheduled_time_slot, req.technician_id).await?;
    Ok(Json(InstallationResponse { id: o.id, customer_id: o.customer_id, status: o.status, scheduled_date: o.scheduled_date.map(|d| d.to_string()) }))
}

pub async fn complete_installation(State(state): State<Arc<AppState>>, _user: UserContext, Path(id): Path<i64>) -> Result<Json<InstallationResponse>, AppError> {
    let o = InstallationService::complete_order(&state.db, id).await?;
    Ok(Json(InstallationResponse { id: o.id, customer_id: o.customer_id, status: o.status, scheduled_date: o.scheduled_date.map(|d| d.to_string()) }))
}

pub async fn cancel_installation(State(state): State<Arc<AppState>>, _user: UserContext, Path(id): Path<i64>) -> Result<StatusCode, AppError> {
    InstallationService::cancel_order(&state.db, id).await?;
    Ok(StatusCode::OK)
}
