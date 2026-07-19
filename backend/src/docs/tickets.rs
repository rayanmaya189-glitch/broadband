/// OpenAPI schemas and stub handlers for Ticket endpoints.
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

// ── Request / Response types ─────────────────────────────────────────

#[derive(Debug, Serialize, ToSchema)]
pub struct TicketResponse {
    pub id: i64,
    pub ticket_number: String,
    pub customer_id: Option<i64>,
    pub subject: String,
    pub description: String,
    pub category: String,
    pub priority: String,
    pub status: String,
    pub assigned_to: Option<i64>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateTicketRequest {
    #[serde(default)]
    pub customer_id: Option<i64>,
    pub category: String,
    pub priority: String,
    pub subject: String,
    pub description: String,
    pub source: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct AssignTicketRequest {
    pub assigned_to: i64,
    #[serde(default)]
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct TicketListParams {
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub status: Option<String>,
    pub priority: Option<String>,
    pub category: Option<String>,
}

// ── Stub handler functions ───────────────────────────────────────────

/// List all support tickets
#[utoipa::path(
    get,
    path = "/api/v1/tickets",
    tag = "Tickets",
    params(TicketListParams),
    responses(
        (status = 200, description = "List of tickets"),
        (status = 403, description = "Insufficient permissions")
    ),
    security(("bearer_auth" = []))
)]
pub async fn list_tickets() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Create a new support ticket
#[utoipa::path(
    post,
    path = "/api/v1/tickets",
    tag = "Tickets",
    request_body = CreateTicketRequest,
    responses(
        (status = 201, description = "Ticket created", body = TicketResponse),
        (status = 400, description = "Validation error")
    ),
    security(("bearer_auth" = []))
)]
pub async fn create_ticket() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Get a ticket by ID
#[utoipa::path(
    get,
    path = "/api/v1/tickets/{id}",
    tag = "Tickets",
    params(("id" = i64, Path, description = "Ticket ID")),
    responses(
        (status = 200, description = "Ticket details", body = TicketResponse),
        (status = 404, description = "Ticket not found")
    ),
    security(("bearer_auth" = []))
)]
pub async fn get_ticket() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Assign a ticket to a support agent
#[utoipa::path(
    post,
    path = "/api/v1/tickets/{id}/assign",
    tag = "Tickets",
    params(("id" = i64, Path, description = "Ticket ID")),
    request_body = AssignTicketRequest,
    responses(
        (status = 200, description = "Ticket assigned"),
        (status = 404, description = "Ticket not found")
    ),
    security(("bearer_auth" = []))
)]
pub async fn assign_ticket() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Resolve a support ticket
#[utoipa::path(
    post,
    path = "/api/v1/tickets/{id}/resolve",
    tag = "Tickets",
    params(("id" = i64, Path, description = "Ticket ID")),
    responses(
        (status = 200, description = "Ticket resolved"),
        (status = 404, description = "Ticket not found")
    ),
    security(("bearer_auth" = []))
)]
pub async fn resolve_ticket() -> axum::Json<serde_json::Value> {
    unimplemented!()
}
