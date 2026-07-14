//! SeaORM-based controller for the Ticket domain.

use axum::extract::{Json, Path, Query, State};
use validator::Validate;

use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::modules::ticket::request::ticket_request::*;
use crate::modules::ticket::response::ticket_response::*;
use crate::modules::ticket::service::ticket_service::TicketService;

pub async fn list(State(state): State<SharedState>, Query(q): Query<TicketQuery>) -> Result<Json<Vec<TicketResponse>>, AppError> {
    let svc = TicketService::new(&state.db);
    let (tickets, _) = svc.list(q.branch_id, q.status.as_deref(), q.priority.as_deref(), q.category.as_deref(), q.assigned_to, q.customer_id, q.page.unwrap_or(1), q.per_page.unwrap_or(20)).await?;
    Ok(Json(tickets))
}

pub async fn get_by_id(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<TicketResponse>, AppError> {
    let svc = TicketService::new(&state.db);
    let t = svc.get_by_id(id).await?;
    Ok(Json(t))
}

pub async fn create(State(state): State<SharedState>, Json(req): Json<CreateTicketRequest>) -> Result<Json<TicketResponse>, AppError> {
    req.validate()?;
    let svc = TicketService::new(&state.db);
    Ok(Json(svc.create(1, 1, req).await?))
}

pub async fn update_status(State(state): State<SharedState>, Path(id): Path<i64>, Json(req): Json<UpdateTicketStatusRequest>) -> Result<Json<TicketResponse>, AppError> {
    let svc = TicketService::new(&state.db);
    Ok(Json(svc.update_status(id, &req.status, req.resolution_notes.as_deref()).await?))
}

pub async fn update(State(state): State<SharedState>, Path(id): Path<i64>, Json(req): Json<UpdateTicketRequest>) -> Result<Json<TicketResponse>, AppError> {
    let svc = TicketService::new(&state.db);
    Ok(Json(svc.update(id, &req).await?))
}

pub async fn delete(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<MessageResponse>, AppError> {
    let svc = TicketService::new(&state.db);
    Ok(Json(svc.delete(id).await?))
}

pub async fn assign(State(state): State<SharedState>, Path(id): Path<i64>, Json(req): Json<AssignTicketRequest>) -> Result<Json<TicketResponse>, AppError> {
    let svc = TicketService::new(&state.db);
    Ok(Json(svc.assign(id, req.assigned_to).await?))
}

pub async fn escalate(State(state): State<SharedState>, Path(id): Path<i64>, Json(req): Json<EscalateTicketRequest>) -> Result<Json<TicketResponse>, AppError> {
    req.validate()?;
    let svc = TicketService::new(&state.db);
    Ok(Json(svc.escalate(id, req.escalated_to, req.new_priority.as_deref()).await?))
}

pub async fn resolve(State(state): State<SharedState>, Path(id): Path<i64>, Json(req): Json<ResolveTicketRequest>) -> Result<Json<TicketResponse>, AppError> {
    req.validate()?;
    let svc = TicketService::new(&state.db);
    Ok(Json(svc.update_status(id, "resolved", Some(&req.resolution_notes)).await?))
}

pub async fn close(State(state): State<SharedState>, Path(id): Path<i64>, Json(req): Json<CloseTicketRequest>) -> Result<Json<TicketResponse>, AppError> {
    let svc = TicketService::new(&state.db);
    Ok(Json(svc.update_status(id, "closed", req.closure_notes.as_deref()).await?))
}

pub async fn reopen(State(state): State<SharedState>, Path(id): Path<i64>, Json(req): Json<ReopenTicketRequest>) -> Result<Json<TicketResponse>, AppError> {
    req.validate()?;
    let svc = TicketService::new(&state.db);
    Ok(Json(svc.reopen(id).await?))
}

pub async fn set_feedback(State(state): State<SharedState>, Path(id): Path<i64>, Json(req): Json<TicketFeedbackRequest>) -> Result<Json<TicketResponse>, AppError> {
    let svc = TicketService::new(&state.db);
    Ok(Json(svc.set_feedback(id, req.satisfaction_rating, req.satisfaction_feedback.as_deref()).await?))
}

pub async fn add_comment(State(state): State<SharedState>, Path(id): Path<i64>, Json(req): Json<AddTicketCommentRequest>) -> Result<Json<TicketCommentResponse>, AppError> {
    req.validate()?;
    let svc = TicketService::new(&state.db);
    Ok(Json(svc.add_comment(id, Some(req.user_id), req.is_customer, &req.comment, req.is_internal.unwrap_or(false)).await?))
}

pub async fn list_comments(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<Vec<TicketCommentResponse>>, AppError> {
    let svc = TicketService::new(&state.db);
    Ok(Json(svc.list_comments(id).await?))
}

pub async fn get_escalations(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<Vec<TicketEscalationResponse>>, AppError> {
    let svc = TicketService::new(&state.db);
    Ok(Json(svc.get_escalations(id).await?))
}

pub async fn get_status_history(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<Vec<TicketStatusHistoryResponse>>, AppError> {
    let svc = TicketService::new(&state.db);
    Ok(Json(svc.get_status_history(id).await?))
}

pub async fn get_my_assignments(State(state): State<SharedState>, Path(technician_id): Path<i64>) -> Result<Json<Vec<TicketResponse>>, AppError> {
    let svc = TicketService::new(&state.db);
    Ok(Json(svc.get_my_assignments(technician_id).await?))
}

pub async fn get_dashboard(State(state): State<SharedState>) -> Result<Json<TicketDashboardResponse>, AppError> {
    let svc = TicketService::new(&state.db);
    Ok(Json(svc.get_dashboard().await?))
}
