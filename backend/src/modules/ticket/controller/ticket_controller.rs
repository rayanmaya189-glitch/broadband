//! SeaORM-based controller for the Ticket domain.

use axum::extract::{Json, Path, Query, State};
use validator::Validate;

use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::modules::ticket::request::ticket_request::*;
use crate::modules::ticket::response::ticket_response::*;
use crate::modules::ticket::service::ticket_service::TicketService;

pub async fn list(State(state): State<SharedState>, Query(q): Query<TicketQuery>) -> Result<Json<Vec<TicketResponse>>, AppError> {
    let svc = TicketService::new(&state.db_seaorm);
    let (tickets, _) = svc.list(q.branch_id, q.status.as_deref(), q.priority.as_deref(), q.category.as_deref(), q.assigned_to, q.customer_id, q.page.unwrap_or(1), q.per_page.unwrap_or(20)).await?;
    Ok(Json(tickets))
}

pub async fn get_by_id(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<TicketResponse>, AppError> {
    let _svc = TicketService::new(&state.db_seaorm);
    let repo = crate::modules::ticket::repository::ticket_repository::TicketRepository::new(&state.db_seaorm);
    let t = repo.get_by_id(id).await?.ok_or_else(|| AppError::NotFound("Ticket not found".into()))?;
    Ok(Json(TicketResponse {
        id: t.id, ticket_number: t.ticket_number, branch_id: t.branch_id,
        customer_id: t.customer_id, subscription_id: t.subscription_id,
        created_by: t.created_by, assigned_to: t.assigned_to, escalated_to: t.escalated_to,
        category: t.category, subcategory: t.subcategory, priority: t.priority, status: t.status,
        subject: t.subject, description: t.description, source: t.source,
        resolution_notes: t.resolution_notes, sla_response_at: None, sla_resolution_at: None,
        first_response_at: None, resolved_at: t.resolved_at.map(|v| v.into()),
        closed_at: t.closed_at.map(|v| v.into()),
        reopen_count: t.reopen_count,
        satisfaction_rating: t.satisfaction_rating, satisfaction_feedback: t.satisfaction_feedback,
        created_at: t.created_at.into(), updated_at: t.updated_at.into(),
        creator_name: None, assignee_name: None, branch_name: None, customer_name: None,
    }))
}

pub async fn create(State(state): State<SharedState>, Json(req): Json<CreateTicketRequest>) -> Result<Json<TicketResponse>, AppError> {
    req.validate()?;
    let svc = TicketService::new(&state.db_seaorm);
    Ok(Json(svc.create(1, 1, req).await?))
}

pub async fn update_status(State(state): State<SharedState>, Path(id): Path<i64>, Json(req): Json<UpdateTicketStatusRequest>) -> Result<Json<TicketResponse>, AppError> {
    let svc = TicketService::new(&state.db_seaorm);
    Ok(Json(svc.update_status(id, &req.status, req.resolution_notes.as_deref()).await?))
}

pub async fn assign(State(state): State<SharedState>, Path(id): Path<i64>, Json(req): Json<AssignTicketRequest>) -> Result<Json<TicketResponse>, AppError> {
    let svc = TicketService::new(&state.db_seaorm);
    Ok(Json(svc.assign(id, req.assigned_to).await?))
}

pub async fn add_comment(State(state): State<SharedState>, Path(id): Path<i64>, Json(req): Json<AddTicketCommentRequest>) -> Result<Json<TicketCommentResponse>, AppError> {
    req.validate()?;
    let svc = TicketService::new(&state.db_seaorm);
    Ok(Json(svc.add_comment(id, Some(req.user_id), req.is_customer, &req.comment, req.is_internal.unwrap_or(false)).await?))
}

pub async fn list_comments(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<Vec<TicketCommentResponse>>, AppError> {
    let svc = TicketService::new(&state.db_seaorm);
    Ok(Json(svc.list_comments(id).await?))
}
