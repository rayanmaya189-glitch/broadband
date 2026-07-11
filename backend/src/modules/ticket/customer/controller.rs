use axum::extract::{Json, Path, State};

use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::common::middleware::auth_middleware::UserContext;
use crate::modules::ticket::request::ticket_request::*;
use crate::modules::ticket::response::ticket_response::*;
use crate::modules::ticket::service::ticket_service::TicketService;

/// Create a ticket as a customer.
pub async fn create(
    State(state): State<SharedState>,
    user: UserContext,
    Json(req): Json<CreateTicketRequest>,
) -> Result<Json<TicketResponse>, AppError> {
    let svc = TicketService::new(&state.db_seaorm);
    Ok(Json(svc.create(req.branch_id, user.user_id, req).await?))
}

/// Get customer's own tickets.
pub async fn get_my_tickets(
    State(state): State<SharedState>,
    user: UserContext,
) -> Result<Json<Vec<TicketResponse>>, AppError> {
    let svc = TicketService::new(&state.db_seaorm);
    let (tickets, _) = svc.list(None, None, None, None, None, Some(user.user_id), 1, 100).await?;
    Ok(Json(tickets))
}

/// Get a specific ticket (customer: only own).
pub async fn get_by_id(
    State(state): State<SharedState>,
    user: UserContext,
    Path(id): Path<i64>,
) -> Result<Json<TicketResponse>, AppError> {
    let svc = TicketService::new(&state.db_seaorm);
    let ticket = svc.get_by_id(id).await?;
    if ticket.customer_id != Some(user.user_id) {
        return Err(AppError::Forbidden("Access denied".into()));
    }
    Ok(Json(ticket))
}

/// Add comment to own ticket.
pub async fn add_comment(
    State(state): State<SharedState>,
    user: UserContext,
    Path(id): Path<i64>,
    Json(req): Json<AddCommentRequest>,
) -> Result<Json<TicketCommentResponse>, AppError> {
    let svc = TicketService::new(&state.db_seaorm);
    let ticket = svc.get_by_id(id).await?;
    if ticket.customer_id != Some(user.user_id) {
        return Err(AppError::Forbidden("Access denied".into()));
    }
    Ok(Json(svc.add_comment(id, Some(user.user_id), true, &req.comment, false).await?))
}

/// View comments on own ticket.
pub async fn list_comments(
    State(state): State<SharedState>,
    user: UserContext,
    Path(id): Path<i64>,
) -> Result<Json<Vec<TicketCommentResponse>>, AppError> {
    let svc = TicketService::new(&state.db_seaorm);
    let ticket = svc.get_by_id(id).await?;
    if ticket.customer_id != Some(user.user_id) {
        return Err(AppError::Forbidden("Access denied".into()));
    }
    Ok(Json(svc.list_comments(id).await?))
}

/// Set feedback on resolved ticket.
pub async fn set_feedback(
    State(state): State<SharedState>,
    user: UserContext,
    Path(id): Path<i64>,
    Json(req): Json<TicketFeedbackRequest>,
) -> Result<Json<TicketResponse>, AppError> {
    let svc = TicketService::new(&state.db_seaorm);
    let ticket = svc.get_by_id(id).await?;
    if ticket.customer_id != Some(user.user_id) {
        return Err(AppError::Forbidden("Access denied".into()));
    }
    Ok(Json(svc.set_feedback(id, req.satisfaction_rating, req.satisfaction_feedback.as_deref()).await?))
}
