use axum::extract::{Json, Path, Query, State};
use validator::Validate;

use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::common::middleware::auth_middleware::UserContext;
use crate::modules::ticket::request::ticket_request::*;
use crate::modules::ticket::response::ticket_response::*;
use crate::modules::ticket::service::ticket_service::TicketService;

#[utoipa::path(
    get,
    path = "/api/v1/tickets",
    tag = "Tickets",
    security(("bearer_auth" = [])),
    params(
        ("page" = Option<i64>, Query, description = "Page number"),
        ("limit" = Option<i64>, Query, description = "Items per page"),
        ("status" = Option<String>, Query, description = "Filter by status"),
        ("priority" = Option<String>, Query, description = "Filter by priority"),
        ("category" = Option<String>, Query, description = "Filter by category"),
        ("assigned_to" = Option<i64>, Query, description = "Filter by assignee")
    ),
    responses(
        (status = 200, description = "List of tickets", body = TicketListResponse),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn list_tickets(State(state): State<SharedState>, Query(query): Query<TicketQuery>) -> Result<Json<TicketListResponse>, AppError> {
    let svc = TicketService::new(&state.db);
    Ok(Json(svc.list_tickets(query).await?))
}

#[utoipa::path(
    post,
    path = "/api/v1/tickets",
    tag = "Tickets",
    security(("bearer_auth" = [])),
    request_body = CreateTicketRequest,
    responses(
        (status = 200, description = "Ticket created", body = TicketResponse),
        (status = 422, description = "Validation error")
    )
)]
pub async fn create_ticket(State(state): State<SharedState>, user: UserContext, Json(req): Json<CreateTicketRequest>) -> Result<Json<TicketResponse>, AppError> {
    req.validate()?;
    let svc = TicketService::new(&state.db);
    Ok(Json(svc.create_ticket(req, user.user_id).await?))
}

#[utoipa::path(
    get,
    path = "/api/v1/tickets/{id}",
    tag = "Tickets",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Ticket ID")),
    responses(
        (status = 200, description = "Ticket details", body = TicketResponse),
        (status = 404, description = "Ticket not found")
    )
)]
pub async fn get_ticket(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<TicketResponse>, AppError> {
    let svc = TicketService::new(&state.db);
    Ok(Json(svc.get_ticket(id).await?))
}

#[utoipa::path(
    put,
    path = "/api/v1/tickets/{id}",
    tag = "Tickets",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Ticket ID")),
    request_body = UpdateTicketRequest,
    responses(
        (status = 200, description = "Ticket updated", body = TicketResponse),
        (status = 404, description = "Ticket not found"),
        (status = 422, description = "Validation error")
    )
)]
pub async fn update_ticket(State(state): State<SharedState>, user: UserContext, Path(id): Path<i64>, Json(req): Json<UpdateTicketRequest>) -> Result<Json<TicketResponse>, AppError> {
    let svc = TicketService::new(&state.db);
    Ok(Json(svc.update_ticket(id, user.user_id, req).await?))
}

#[utoipa::path(
    delete,
    path = "/api/v1/tickets/{id}",
    tag = "Tickets",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Ticket ID")),
    responses(
        (status = 200, description = "Ticket deleted", body = MessageResponse),
        (status = 404, description = "Ticket not found")
    )
)]
pub async fn delete_ticket(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<MessageResponse>, AppError> {
    let svc = TicketService::new(&state.db);
    Ok(Json(svc.delete_ticket(id).await?))
}

#[utoipa::path(
    post,
    path = "/api/v1/tickets/{id}/assign",
    tag = "Tickets",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Ticket ID")),
    request_body = AssignTicketRequest,
    responses(
        (status = 200, description = "Ticket assigned", body = TicketResponse),
        (status = 404, description = "Ticket not found")
    )
)]
pub async fn assign_ticket(State(state): State<SharedState>, user: UserContext, Path(id): Path<i64>, Json(req): Json<AssignTicketRequest>) -> Result<Json<TicketResponse>, AppError> {
    let svc = TicketService::new(&state.db);
    Ok(Json(svc.assign_ticket(id, user.user_id, req).await?))
}

#[utoipa::path(
    post,
    path = "/api/v1/tickets/{id}/escalate",
    tag = "Tickets",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Ticket ID")),
    request_body = EscalateTicketRequest,
    responses(
        (status = 200, description = "Ticket escalated", body = TicketResponse),
        (status = 404, description = "Ticket not found")
    )
)]
pub async fn escalate_ticket(State(state): State<SharedState>, user: UserContext, Path(id): Path<i64>, Json(req): Json<EscalateTicketRequest>) -> Result<Json<TicketResponse>, AppError> {
    let svc = TicketService::new(&state.db);
    Ok(Json(svc.escalate_ticket(id, user.user_id, req).await?))
}

#[utoipa::path(
    post,
    path = "/api/v1/tickets/{id}/resolve",
    tag = "Tickets",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Ticket ID")),
    request_body = ResolveTicketRequest,
    responses(
        (status = 200, description = "Ticket resolved", body = TicketResponse),
        (status = 404, description = "Ticket not found")
    )
)]
pub async fn resolve_ticket(State(state): State<SharedState>, user: UserContext, Path(id): Path<i64>, Json(req): Json<ResolveTicketRequest>) -> Result<Json<TicketResponse>, AppError> {
    let svc = TicketService::new(&state.db);
    Ok(Json(svc.resolve_ticket(id, user.user_id, req).await?))
}

#[utoipa::path(
    post,
    path = "/api/v1/tickets/{id}/close",
    tag = "Tickets",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Ticket ID")),
    request_body = CloseTicketRequest,
    responses(
        (status = 200, description = "Ticket closed", body = TicketResponse),
        (status = 404, description = "Ticket not found")
    )
)]
pub async fn close_ticket(State(state): State<SharedState>, user: UserContext, Path(id): Path<i64>, Json(req): Json<CloseTicketRequest>) -> Result<Json<TicketResponse>, AppError> {
    let svc = TicketService::new(&state.db);
    Ok(Json(svc.close_ticket(id, user.user_id, req).await?))
}

#[utoipa::path(
    post,
    path = "/api/v1/tickets/{id}/reopen",
    tag = "Tickets",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Ticket ID")),
    request_body = ReopenTicketRequest,
    responses(
        (status = 200, description = "Ticket reopened", body = TicketResponse),
        (status = 404, description = "Ticket not found")
    )
)]
pub async fn reopen_ticket(State(state): State<SharedState>, user: UserContext, Path(id): Path<i64>, Json(req): Json<ReopenTicketRequest>) -> Result<Json<TicketResponse>, AppError> {
    let svc = TicketService::new(&state.db);
    Ok(Json(svc.reopen_ticket(id, user.user_id, req).await?))
}

#[utoipa::path(
    post,
    path = "/api/v1/tickets/{id}/feedback",
    tag = "Tickets",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Ticket ID")),
    request_body = TicketFeedbackRequest,
    responses(
        (status = 200, description = "Feedback submitted", body = TicketResponse),
        (status = 404, description = "Ticket not found"),
        (status = 422, description = "Validation error")
    )
)]
pub async fn set_feedback(State(state): State<SharedState>, Path(id): Path<i64>, Json(req): Json<TicketFeedbackRequest>) -> Result<Json<TicketResponse>, AppError> {
    req.validate()?;
    let svc = TicketService::new(&state.db);
    Ok(Json(svc.set_feedback(id, req).await?))
}

#[utoipa::path(
    get,
    path = "/api/v1/tickets/{id}/comments",
    tag = "Tickets",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Ticket ID")),
    responses(
        (status = 200, description = "List of comments", body = Vec<TicketCommentResponse>),
        (status = 404, description = "Ticket not found")
    )
)]
pub async fn get_comments(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<Vec<TicketCommentResponse>>, AppError> {
    let svc = TicketService::new(&state.db);
    Ok(Json(svc.get_comments(id).await?))
}

#[utoipa::path(
    post,
    path = "/api/v1/tickets/{id}/comments",
    tag = "Tickets",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Ticket ID")),
    request_body = AddCommentRequest,
    responses(
        (status = 200, description = "Comment added", body = TicketCommentResponse),
        (status = 404, description = "Ticket not found"),
        (status = 422, description = "Validation error")
    )
)]
pub async fn add_comment(State(state): State<SharedState>, user: UserContext, Path(id): Path<i64>, Json(req): Json<AddCommentRequest>) -> Result<Json<TicketCommentResponse>, AppError> {
    req.validate()?;
    let svc = TicketService::new(&state.db);
    Ok(Json(svc.add_comment(id, user.user_id, req).await?))
}

#[utoipa::path(
    get,
    path = "/api/v1/tickets/{id}/escalations",
    tag = "Tickets",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Ticket ID")),
    responses(
        (status = 200, description = "List of escalations", body = Vec<TicketEscalationResponse>),
        (status = 404, description = "Ticket not found")
    )
)]
pub async fn get_escalations(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<Vec<TicketEscalationResponse>>, AppError> {
    let svc = TicketService::new(&state.db);
    Ok(Json(svc.get_escalations(id).await?))
}

#[utoipa::path(
    get,
    path = "/api/v1/tickets/{id}/status-history",
    tag = "Tickets",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Ticket ID")),
    responses(
        (status = 200, description = "Status change history", body = Vec<TicketStatusHistoryResponse>),
        (status = 404, description = "Ticket not found")
    )
)]
pub async fn get_status_history(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<Vec<TicketStatusHistoryResponse>>, AppError> {
    let svc = TicketService::new(&state.db);
    Ok(Json(svc.get_status_history(id).await?))
}

#[utoipa::path(
    get,
    path = "/api/v1/tickets/dashboard",
    tag = "Tickets",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Ticket dashboard stats", body = TicketDashboardResponse),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn get_dashboard(State(state): State<SharedState>) -> Result<Json<TicketDashboardResponse>, AppError> {
    let svc = TicketService::new(&state.db);
    Ok(Json(svc.get_dashboard().await?))
}

#[utoipa::path(
    get,
    path = "/api/v1/tickets/my-assignments",
    tag = "Tickets",
    security(("bearer_auth" = [])),
    params(
        ("page" = Option<i64>, Query, description = "Page number"),
        ("limit" = Option<i64>, Query, description = "Items per page")
    ),
    responses(
        (status = 200, description = "My assigned tickets", body = TicketListResponse),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn get_my_assignments(State(state): State<SharedState>, user: UserContext, Query(q): Query<TicketQuery>) -> Result<Json<TicketListResponse>, AppError> {
    let svc = TicketService::new(&state.db);
    let page = q.page.unwrap_or(1);
    let per_page = q.per_page.unwrap_or(20);
    Ok(Json(svc.get_my_assignments(user.user_id, page, per_page).await?))
}
