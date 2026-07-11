use axum::extract::{Json, Path, Query, State};

use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::common::middleware::auth_middleware::UserContext;
use crate::modules::ticket::request::ticket_request::*;
use crate::modules::ticket::response::ticket_response::*;
use crate::modules::ticket::service::ticket_service::TicketService;

/// List all tickets (admin: full filtering, all tickets).
pub async fn list(
    State(state): State<SharedState>,
    Query(query): Query<TicketQuery>,
) -> Result<Json<(Vec<TicketResponse>, i64)>, AppError> {
    let svc = TicketService::new(&state.db_seaorm);
    let (tickets, total) = svc.list(
        query.branch_id, query.status.as_deref(), query.priority.as_deref(),
        query.category.as_deref(), query.assigned_to, query.customer_id,
        query.page.unwrap_or(1), query.per_page.unwrap_or(20),
    ).await?;
    Ok(Json((tickets, total)))
}

/// Create a ticket on behalf of a customer (admin).
pub async fn create(
    State(state): State<SharedState>,
    user: UserContext,
    Json(req): Json<CreateTicketRequest>,
) -> Result<Json<TicketResponse>, AppError> {
    let svc = TicketService::new(&state.db_seaorm);
    Ok(Json(svc.create(req.branch_id, user.user_id, req).await?))
}

/// Get ticket by ID (admin: any ticket).
pub async fn get_by_id(
    State(state): State<SharedState>,
    Path(id): Path<i64>,
) -> Result<Json<TicketResponse>, AppError> {
    let svc = TicketService::new(&state.db_seaorm);
    Ok(Json(svc.get_by_id(id).await?))
}

/// Update ticket (admin).
pub async fn update(
    State(state): State<SharedState>,
    Path(id): Path<i64>,
    Json(req): Json<UpdateTicketRequest>,
) -> Result<Json<TicketResponse>, AppError> {
    let svc = TicketService::new(&state.db_seaorm);
    Ok(Json(svc.update(id, &req).await?))
}

/// Delete ticket (admin).
pub async fn delete(
    State(state): State<SharedState>,
    Path(id): Path<i64>,
) -> Result<Json<MessageResponse>, AppError> {
    let svc = TicketService::new(&state.db_seaorm);
    Ok(Json(svc.delete(id).await?))
}

/// Update ticket status (admin).
pub async fn update_status(
    State(state): State<SharedState>,
    Path(id): Path<i64>,
    Json(req): Json<UpdateTicketStatusRequest>,
) -> Result<Json<TicketResponse>, AppError> {
    let svc = TicketService::new(&state.db_seaorm);
    Ok(Json(svc.update_status(id, &req.status, req.resolution_notes.as_deref()).await?))
}

/// Assign ticket to a staff member (admin).
pub async fn assign(
    State(state): State<SharedState>,
    Path(id): Path<i64>,
    Json(req): Json<AssignTicketRequest>,
) -> Result<Json<TicketResponse>, AppError> {
    let svc = TicketService::new(&state.db_seaorm);
    Ok(Json(svc.assign(id, req.assigned_to).await?))
}

/// Escalate ticket (admin).
pub async fn escalate(
    State(state): State<SharedState>,
    Path(id): Path<i64>,
    Json(req): Json<EscalateTicketRequest>,
) -> Result<Json<TicketResponse>, AppError> {
    let svc = TicketService::new(&state.db_seaorm);
    Ok(Json(svc.escalate(id, req.escalated_to, req.new_priority.as_deref()).await?))
}

/// Resolve ticket (admin).
pub async fn resolve(
    State(state): State<SharedState>,
    Path(id): Path<i64>,
    Json(req): Json<ResolveTicketRequest>,
) -> Result<Json<TicketResponse>, AppError> {
    let svc = TicketService::new(&state.db_seaorm);
    Ok(Json(svc.update_status(id, "resolved", Some(&req.resolution_notes)).await?))
}

/// Close ticket (admin).
pub async fn close(
    State(state): State<SharedState>,
    Path(id): Path<i64>,
) -> Result<Json<TicketResponse>, AppError> {
    let svc = TicketService::new(&state.db_seaorm);
    Ok(Json(svc.update_status(id, "closed", None).await?))
}

/// Reopen ticket (admin).
pub async fn reopen(
    State(state): State<SharedState>,
    Path(id): Path<i64>,
) -> Result<Json<TicketResponse>, AppError> {
    let svc = TicketService::new(&state.db_seaorm);
    Ok(Json(svc.reopen(id).await?))
}

/// Get dashboard stats (admin).
pub async fn get_dashboard(
    State(state): State<SharedState>,
) -> Result<Json<TicketDashboardResponse>, AppError> {
    let svc = TicketService::new(&state.db_seaorm);
    Ok(Json(svc.get_dashboard().await?))
}

/// Get my assigned tickets (admin).
pub async fn get_my_assignments(
    State(state): State<SharedState>,
    user: UserContext,
) -> Result<Json<Vec<TicketResponse>>, AppError> {
    let svc = TicketService::new(&state.db_seaorm);
    Ok(Json(svc.get_my_assignments(user.user_id).await?))
}

/// List escalations for a ticket (admin).
pub async fn get_escalations(
    State(state): State<SharedState>,
    Path(id): Path<i64>,
) -> Result<Json<Vec<TicketEscalationResponse>>, AppError> {
    let svc = TicketService::new(&state.db_seaorm);
    Ok(Json(svc.get_escalations(id).await?))
}

/// Get status history (admin).
pub async fn get_status_history(
    State(state): State<SharedState>,
    Path(id): Path<i64>,
) -> Result<Json<Vec<TicketStatusHistoryResponse>>, AppError> {
    let svc = TicketService::new(&state.db_seaorm);
    Ok(Json(svc.get_status_history(id).await?))
}
