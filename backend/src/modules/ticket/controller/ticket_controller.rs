use axum::extract::{Json, Path, Query, State};
use validator::Validate;

use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::common::middleware::auth_middleware::UserContext;
use crate::modules::ticket::request::ticket_request::*;
use crate::modules::ticket::response::ticket_response::*;
use crate::modules::ticket::service::ticket_service::TicketService;

pub async fn list_tickets(State(state): State<SharedState>, Query(query): Query<TicketQuery>) -> Result<Json<TicketListResponse>, AppError> {
    let svc = TicketService::new(&state.db);
    Ok(Json(svc.list_tickets(query).await?))
}

pub async fn create_ticket(State(state): State<SharedState>, user: UserContext, Json(req): Json<CreateTicketRequest>) -> Result<Json<TicketResponse>, AppError> {
    req.validate()?;
    let svc = TicketService::new(&state.db);
    Ok(Json(svc.create_ticket(req, user.user_id).await?))
}

pub async fn get_ticket(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<TicketResponse>, AppError> {
    let svc = TicketService::new(&state.db);
    Ok(Json(svc.get_ticket(id).await?))
}

pub async fn update_ticket(State(state): State<SharedState>, user: UserContext, Path(id): Path<i64>, Json(req): Json<UpdateTicketRequest>) -> Result<Json<TicketResponse>, AppError> {
    let svc = TicketService::new(&state.db);
    Ok(Json(svc.update_ticket(id, user.user_id, req).await?))
}

pub async fn delete_ticket(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<MessageResponse>, AppError> {
    let svc = TicketService::new(&state.db);
    Ok(Json(svc.delete_ticket(id).await?))
}

pub async fn assign_ticket(State(state): State<SharedState>, user: UserContext, Path(id): Path<i64>, Json(req): Json<AssignTicketRequest>) -> Result<Json<TicketResponse>, AppError> {
    let svc = TicketService::new(&state.db);
    Ok(Json(svc.assign_ticket(id, user.user_id, req).await?))
}

pub async fn escalate_ticket(State(state): State<SharedState>, user: UserContext, Path(id): Path<i64>, Json(req): Json<EscalateTicketRequest>) -> Result<Json<TicketResponse>, AppError> {
    let svc = TicketService::new(&state.db);
    Ok(Json(svc.escalate_ticket(id, user.user_id, req).await?))
}

pub async fn resolve_ticket(State(state): State<SharedState>, user: UserContext, Path(id): Path<i64>, Json(req): Json<ResolveTicketRequest>) -> Result<Json<TicketResponse>, AppError> {
    let svc = TicketService::new(&state.db);
    Ok(Json(svc.resolve_ticket(id, user.user_id, req).await?))
}

pub async fn close_ticket(State(state): State<SharedState>, user: UserContext, Path(id): Path<i64>, Json(req): Json<CloseTicketRequest>) -> Result<Json<TicketResponse>, AppError> {
    let svc = TicketService::new(&state.db);
    Ok(Json(svc.close_ticket(id, user.user_id, req).await?))
}

pub async fn reopen_ticket(State(state): State<SharedState>, user: UserContext, Path(id): Path<i64>, Json(req): Json<ReopenTicketRequest>) -> Result<Json<TicketResponse>, AppError> {
    let svc = TicketService::new(&state.db);
    Ok(Json(svc.reopen_ticket(id, user.user_id, req).await?))
}

pub async fn set_feedback(State(state): State<SharedState>, Path(id): Path<i64>, Json(req): Json<TicketFeedbackRequest>) -> Result<Json<TicketResponse>, AppError> {
    let svc = TicketService::new(&state.db);
    Ok(Json(svc.set_feedback(id, req).await?))
}

pub async fn get_comments(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<Vec<TicketCommentResponse>>, AppError> {
    let svc = TicketService::new(&state.db);
    Ok(Json(svc.get_comments(id).await?))
}

pub async fn add_comment(State(state): State<SharedState>, user: UserContext, Path(id): Path<i64>, Json(req): Json<AddCommentRequest>) -> Result<Json<TicketCommentResponse>, AppError> {
    req.validate()?;
    let svc = TicketService::new(&state.db);
    Ok(Json(svc.add_comment(id, user.user_id, req).await?))
}

pub async fn get_my_assignments(State(state): State<SharedState>, user: UserContext, Query(query): Query<TicketQuery>) -> Result<Json<TicketListResponse>, AppError> {
    let page = query.page.unwrap_or(1);
    let per_page = query.per_page.unwrap_or(20);
    let svc = TicketService::new(&state.db);
    Ok(Json(svc.get_my_assignments(user.user_id, page, per_page).await?))
}

pub async fn get_dashboard(State(state): State<SharedState>) -> Result<Json<TicketDashboardResponse>, AppError> {
    let svc = TicketService::new(&state.db);
    Ok(Json(svc.get_dashboard().await?))
}

// ── Escalation Records ──────────────────────────────────────

pub async fn get_escalations(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<Vec<TicketEscalationResponse>>, AppError> {
    let svc = TicketService::new(&state.db);
    Ok(Json(svc.get_escalations(id).await?))
}

// ── Status History ──────────────────────────────────────────

pub async fn get_status_history(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<Vec<TicketStatusHistoryResponse>>, AppError> {
    let svc = TicketService::new(&state.db);
    Ok(Json(svc.get_status_history(id).await?))
}
