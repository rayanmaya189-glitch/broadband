use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::modules::ticket::application::services::TicketService;
use crate::shared::app_state::AppState;
use crate::shared::errors::AppError;
use crate::shared::middleware::auth::{require_permission, UserContext};
use crate::shared::primitives::PaginationParams;

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
    Query(p): Query<PaginationParams>,
) -> Result<Json<serde_json::Value>, AppError> {
    require_permission(&user, "ticket.view").map_err(|e| AppError::Forbidden(e.1))?;
    let bid = if user.is_company_wide {
        None
    } else {
        user.branch_id
    };
    let (tickets, total) = TicketService::list_tickets(&state.db, bid, p.page(), p.limit()).await?;
    let items: Vec<TicketResponse> = tickets
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
        .collect();
    Ok(Json(
        serde_json::json!({"items": items, "total": total, "page": p.page(), "limit": p.limit()}),
    ))
}

pub async fn create_ticket(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Json(req): Json<CreateTicketRequest>,
) -> Result<(StatusCode, Json<TicketResponse>), AppError> {
    require_permission(&user, "ticket.create").map_err(|e| AppError::Forbidden(e.1))?;
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
    // Publish event to outbox
    if let Err(e) = crate::infrastructure::messaging::outbox::insert_outbox_event(
        &state.db,
        "ticket.created",
        "ticket",
        t.id,
        serde_json::json!({"ticket_id": t.id, "ticket_number": t.ticket_number}),
        None,
        Some(user.user_id),
        user.branch_id,
    )
    .await
    {
        tracing::error!(error = %e, "Failed to publish ticket.created event");
    }

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
    user: UserContext,
    Path(id): Path<i64>,
) -> Result<Json<TicketResponse>, AppError> {
    require_permission(&user, "ticket.view").map_err(|e| AppError::Forbidden(e.1))?;
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
    user: UserContext,
    Path(id): Path<i64>,
    Json(req): Json<AssignRequest>,
) -> Result<StatusCode, AppError> {
    require_permission(&user, "ticket.assign").map_err(|e| AppError::Forbidden(e.1))?;
    TicketService::assign_ticket(&state.db, id, req.assigned_to).await?;
    // Publish event to outbox
    if let Err(e) = crate::infrastructure::messaging::outbox::insert_outbox_event(
        &state.db,
        "ticket.assigned",
        "ticket",
        id,
        serde_json::json!({"ticket_id": id}),
        None,
        Some(user.user_id),
        user.branch_id,
    )
    .await
    {
        tracing::error!(error = %e, "Failed to publish ticket.assigned event");
    }

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
    require_permission(&user, "ticket.resolve").map_err(|e| AppError::Forbidden(e.1))?;
    TicketService::resolve_ticket(&state.db, id, user.user_id, req.resolution_notes).await?;
    // Publish event to outbox
    if let Err(e) = crate::infrastructure::messaging::outbox::insert_outbox_event(
        &state.db,
        "ticket.resolved",
        "ticket",
        id,
        serde_json::json!({"ticket_id": id}),
        None,
        Some(user.user_id),
        user.branch_id,
    )
    .await
    {
        tracing::error!(error = %e, "Failed to publish ticket.resolved event");
    }

    Ok(StatusCode::OK)
}

#[derive(Debug, Deserialize)]
pub struct ResolveRequest {
    #[serde(default)]
    pub resolution_notes: Option<String>,
}
