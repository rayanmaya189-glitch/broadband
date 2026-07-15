use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::shared::app_state::AppState;
use crate::shared::errors::AppError;
use crate::shared::middleware::auth::UserContext;
use crate::modules::lead::application::services::LeadService;

#[derive(Debug, Serialize)]
pub struct LeadResponse { pub id: i64, pub name: String, pub phone: String, pub status: String, pub source: String }

#[derive(Debug, Deserialize)]
pub struct CreateLeadRequest { pub name: String, pub phone: String, pub email: Option<String>, pub source: String }

pub async fn list_leads(State(state): State<Arc<AppState>>, user: UserContext) -> Result<Json<Vec<LeadResponse>>, AppError> {
    let bid = if user.is_company_wide { None } else { user.branch_id };
    let leads = LeadService::list_leads(&state.db, bid).await?;
    Ok(Json(leads.into_iter().map(|l| LeadResponse { id: l.id, name: l.name, phone: l.phone, status: l.status, source: l.source }).collect()))
}

pub async fn create_lead(State(state): State<Arc<AppState>>, user: UserContext, Json(req): Json<CreateLeadRequest>) -> Result<(StatusCode, Json<LeadResponse>), AppError> {
    let l = LeadService::create_lead(&state.db, user.branch_id.unwrap_or(0), req.name, req.phone, req.email, req.source).await?;
    Ok((StatusCode::CREATED, Json(LeadResponse { id: l.id, name: l.name, phone: l.phone, status: l.status, source: l.source })))
}

pub async fn update_lead_status(State(state): State<Arc<AppState>>, _user: UserContext, Path(id): Path<i64>, Json(req): Json<UpdateStatusRequest>) -> Result<Json<LeadResponse>, AppError> {
    let l = LeadService::update_lead_status(&state.db, id, &req.status).await?;
    Ok(Json(LeadResponse { id: l.id, name: l.name, phone: l.phone, status: l.status, source: l.source }))
}

#[derive(Debug, Deserialize)]
pub struct UpdateStatusRequest { pub status: String }
