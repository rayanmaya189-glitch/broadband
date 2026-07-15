use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::modules::ticket::application::services::TicketService;
use crate::shared::app_state::AppState;
use crate::shared::errors::AppError;
use crate::shared::middleware::auth::UserContext;

#[derive(Debug, Serialize)]
pub struct TicketResponse {
    pub id: i64,
    pub ticket_number: String,
    pub subject: String,
    pub category: String,
    pub priority: String,
    pub status: String,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateTicketRequest {
    pub category: String,
    pub priority: String,
    pub subject: String,
    pub description: String,
    pub source: String,
    #[serde(default)]
    pub customer_id: Option<i64>,
}

pub async fn list_tickets(
    State(state): State<Arc<AppState>>,
    user: UserContext,
) -> Result<Json<Vec<TicketResponse>>, AppError> {
    let bid = if user.is_company_wide {
        None
    } else {
        user.branch_id
    };
    let tickets = TicketService::list_tickets(&state.db, bid).await?;
    Ok(Json(
        tickets
            .into_iter()
            .map(|t| TicketResponse {
                id: t.id,
                ticket_number: t.ticket_number,
                subject: t.subject,
                category: t.category,
                priority: t.priority,
                status: t.status,
                created_at: t.created_at.to_rfc3339(),
            })
            .collect(),
    ))
}

pub async fn create_ticket(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Json(req): Json<CreateTicketRequest>,
) -> Result<(StatusCode, Json<TicketResponse>), AppError> {
    let t = TicketService::create_ticket(
        &state.db,
        user.branch_id.unwrap_or(0),
        user.user_id,
        req.category,
        req.priority,
        req.subject,
        req.description,
        req.source,
        req.customer_id,
    )
    .await?;
    Ok((
        StatusCode::CREATED,
        Json(TicketResponse {
            id: t.id,
            ticket_number: t.ticket_number,
            subject: t.subject,
            category: t.category,
            priority: t.priority,
            status: t.status,
            created_at: t.created_at.to_rfc3339(),
        }),
    ))
}

pub async fn get_ticket(
    State(state): State<Arc<AppState>>,
    _user: UserContext,
    Path(id): Path<i64>,
) -> Result<Json<TicketResponse>, AppError> {
    let t = TicketService::get_ticket(&state.db, id).await?;
    Ok(Json(TicketResponse {
        id: t.id,
        ticket_number: t.ticket_number,
        subject: t.subject,
        category: t.category,
        priority: t.priority,
        status: t.status,
        created_at: t.created_at.to_rfc3339(),
    }))
}

pub async fn assign_ticket(
    State(state): State<Arc<AppState>>,
    _user: UserContext,
    Path(id): Path<i64>,
    Json(req): Json<AssignRequest>,
) -> Result<StatusCode, AppError> {
    TicketService::assign_ticket(&state.db, id, req.assigned_to).await?;
    Ok(StatusCode::OK)
}

#[derive(Debug, Deserialize)]
pub struct AssignRequest {
    pub assigned_to: i64,
}

pub async fn resolve_ticket(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Path(id): Path<i64>,
    Json(req): Json<ResolveRequest>,
) -> Result<StatusCode, AppError> {
    TicketService::resolve_ticket(&state.db, id, user.user_id, req.resolution_notes).await?;
    Ok(StatusCode::OK)
}

#[derive(Debug, Deserialize)]
pub struct ResolveRequest {
    #[serde(default)]
    pub resolution_notes: Option<String>,
}
