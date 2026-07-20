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

#[derive(Debug, Deserialize)]
pub struct EscalateRequest {
    pub escalated_to: i64,
    #[serde(default)]
    pub reason: Option<String>,
}

pub async fn escalate_ticket(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Path(id): Path<i64>,
    Json(req): Json<EscalateRequest>,
) -> Result<StatusCode, AppError> {
    require_permission(&user, "ticket.escalate").map_err(|e| AppError::Forbidden(e.1))?;
    TicketService::escalate_ticket(&state.db, id, req.escalated_to, req.reason).await?;
    if let Err(e) = crate::infrastructure::messaging::outbox::insert_outbox_event(
        &state.db,
        "ticket.escalated",
        "ticket",
        id,
        serde_json::json!({"ticket_id": id}),
        None,
        Some(user.user_id),
        user.branch_id,
    )
    .await
    {
        tracing::error!(error = %e, "Failed to publish ticket.escalated event");
    }

    Ok(StatusCode::OK)
}

#[derive(Debug, Deserialize)]
pub struct CloseRequest {
    #[serde(default)]
    pub closure_notes: Option<String>,
}

pub async fn close_ticket(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Path(id): Path<i64>,
    Json(req): Json<CloseRequest>,
) -> Result<StatusCode, AppError> {
    require_permission(&user, "ticket.close").map_err(|e| AppError::Forbidden(e.1))?;
    TicketService::close_ticket(&state.db, id, req.closure_notes).await?;
    if let Err(e) = crate::infrastructure::messaging::outbox::insert_outbox_event(
        &state.db,
        "ticket.closed",
        "ticket",
        id,
        serde_json::json!({"ticket_id": id}),
        None,
        Some(user.user_id),
        user.branch_id,
    )
    .await
    {
        tracing::error!(error = %e, "Failed to publish ticket.closed event");
    }

    Ok(StatusCode::OK)
}

#[derive(Debug, Deserialize)]
pub struct ReopenRequest {
    #[serde(default)]
    pub reopen_reason: Option<String>,
}

pub async fn reopen_ticket(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Path(id): Path<i64>,
    Json(req): Json<ReopenRequest>,
) -> Result<StatusCode, AppError> {
    require_permission(&user, "ticket.update").map_err(|e| AppError::Forbidden(e.1))?;
    TicketService::reopen_ticket(&state.db, id, req.reopen_reason).await?;
    if let Err(e) = crate::infrastructure::messaging::outbox::insert_outbox_event(
        &state.db,
        "ticket.reopened",
        "ticket",
        id,
        serde_json::json!({"ticket_id": id}),
        None,
        Some(user.user_id),
        user.branch_id,
    )
    .await
    {
        tracing::error!(error = %e, "Failed to publish ticket.reopened event");
    }

    Ok(StatusCode::OK)
}

#[derive(Debug, Deserialize)]
pub struct UpdateTicketRequest {
    pub subject: String,
    pub priority: String,
    pub category: String,
}

pub async fn update_ticket(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Path(id): Path<i64>,
    Json(req): Json<UpdateTicketRequest>,
) -> Result<Json<TicketResponse>, AppError> {
    require_permission(&user, "ticket.update").map_err(|e| AppError::Forbidden(e.1))?;
    let t = TicketService::update_ticket(&state.db, id, req.subject, req.priority, req.category).await?;
    if let Err(e) = crate::infrastructure::messaging::outbox::insert_outbox_event(
        &state.db,
        "ticket.updated",
        "ticket",
        t.id,
        serde_json::json!({"ticket_id": t.id}),
        None,
        Some(user.user_id),
        user.branch_id,
    )
    .await
    {
        tracing::error!(error = %e, "Failed to publish ticket.updated event");
    }
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

pub async fn list_ticket_comments(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, AppError> {
    require_permission(&user, "ticket.view").map_err(|e| AppError::Forbidden(e.1))?;
    let comments = TicketService::get_comments(&state.db, id).await?;
    Ok(Json(serde_json::json!({"items": comments})))
}

#[derive(Debug, Deserialize)]
pub struct AddCommentRequest {
    pub comment: String,
}

pub async fn add_ticket_comment(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Path(id): Path<i64>,
    Json(req): Json<AddCommentRequest>,
) -> Result<(StatusCode, Json<serde_json::Value>), AppError> {
    require_permission(&user, "ticket.comment").map_err(|e| AppError::Forbidden(e.1))?;
    let c = TicketService::add_comment(&state.db, id, user.user_id, req.comment).await?;
    Ok((StatusCode::CREATED, Json(serde_json::json!(c))))
}

#[derive(Debug, Deserialize)]
pub struct SatisfactionRequest {
    pub rating: i32,
    #[serde(default)]
    pub feedback: Option<String>,
}

pub async fn rate_ticket_satisfaction(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Path(id): Path<i64>,
    Json(req): Json<SatisfactionRequest>,
) -> Result<StatusCode, AppError> {
    require_permission(&user, "ticket.resolve").map_err(|e| AppError::Forbidden(e.1))?;
    TicketService::rate_satisfaction(&state.db, id, req.rating, req.feedback).await?;
    if let Err(e) = crate::infrastructure::messaging::outbox::insert_outbox_event(
        &state.db,
        "ticket.satisfaction.rated",
        "ticket",
        id,
        serde_json::json!({"ticket_id": id, "rating": req.rating}),
        None,
        Some(user.user_id),
        user.branch_id,
    )
    .await
    {
        tracing::error!(error = %e, "Failed to publish ticket.satisfaction.rated event");
    }
    Ok(StatusCode::OK)
}

pub async fn list_my_assignments(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Query(p): Query<PaginationParams>,
) -> Result<Json<serde_json::Value>, AppError> {
    require_permission(&user, "ticket.view").map_err(|e| AppError::Forbidden(e.1))?;
    let (tickets, total) = TicketService::list_my_assignments(&state.db, user.user_id, p.page(), p.limit()).await?;
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

pub async fn get_ticket_metrics(
    State(state): State<Arc<AppState>>,
    user: UserContext,
) -> Result<Json<serde_json::Value>, AppError> {
    require_permission(&user, "ticket.view").map_err(|e| AppError::Forbidden(e.1))?;
    let bid = if user.is_company_wide {
        None
    } else {
        user.branch_id
    };
    let metrics = TicketService::get_dashboard_metrics(&state.db, bid).await?;
    Ok(Json(metrics))
}
