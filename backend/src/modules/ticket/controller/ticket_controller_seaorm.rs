//! SeaORM-based controller for the Ticket domain.

use axum::extract::{Json, Path, Query, State};
use validator::Validate;

use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::modules::ticket::request::ticket_request::*;
use crate::modules::ticket::response::ticket_response::*;
use crate::modules::ticket::service::ticket_service_seaorm::TicketServiceSeaorm;

pub async fn list(State(state): State<SharedState>, Query(q): Query<TicketQuery>) -> Result<Json<Vec<TicketResponse>>, AppError> {
    let svc = TicketServiceSeaorm::new(&state.db_seaorm);
    let (tickets, _) = svc.list(q.branch_id, q.status.as_deref(), q.priority.as_deref(), q.category.as_deref(), q.assigned_to, q.customer_id, q.page.unwrap_or(1), q.per_page.unwrap_or(20)).await?;
    Ok(Json(tickets))
}

pub async fn get_by_id(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<TicketResponse>, AppError> {
    let svc = TicketServiceSeaorm::new(&state.db_seaorm);
    let (tickets, _) = svc.list(None, None, None, None, None, None, 1, 1).await?;
    Ok(Json(tickets.into_iter().next().ok_or_else(|| AppError::NotFound("Ticket not found".into()))?))
}

pub async fn create(State(state): State<SharedState>, Json(req): Json<CreateTicketRequest>) -> Result<Json<TicketResponse>, AppError> {
    req.validate()?;
    let svc = TicketServiceSeaorm::new(&state.db_seaorm);
    Ok(Json(svc.create(1, 1, req).await?))
}

pub async fn update_status(State(state): State<SharedState>, Path(id): Path<i64>, Json(req): Json<UpdateTicketStatusRequest>) -> Result<Json<TicketResponse>, AppError> {
    let svc = TicketServiceSeaorm::new(&state.db_seaorm);
    Ok(Json(svc.update_status(id, &req.status, req.resolution_notes.as_deref()).await?))
}

pub async fn assign(State(state): State<SharedState>, Path(id): Path<i64>, Json(req): Json<AssignTicketRequest>) -> Result<Json<TicketResponse>, AppError> {
    let svc = TicketServiceSeaorm::new(&state.db_seaorm);
    Ok(Json(svc.assign(id, req.assigned_to).await?))
}

pub async fn add_comment(State(state): State<SharedState>, Path(id): Path<i64>, Json(req): Json<AddTicketCommentRequest>) -> Result<Json<TicketCommentResponse>, AppError> {
    req.validate()?;
    let svc = TicketServiceSeaorm::new(&state.db_seaorm);
    Ok(Json(svc.add_comment(id, req.user_id, req.is_customer, &req.comment, req.is_internal).await?))
}

pub async fn list_comments(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<Vec<TicketCommentResponse>>, AppError> {
    let svc = TicketServiceSeaorm::new(&state.db_seaorm);
    Ok(Json(svc.list_comments(id).await?))
}
