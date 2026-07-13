use axum::extract::{Json, Path, Query, State};
use validator::Validate;

use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::common::middleware::auth_middleware::UserContext;
use crate::modules::lead::request::lead_request::*;
use crate::modules::lead::response::lead_response::*;
use crate::modules::lead::service::lead_service::LeadService;

#[utoipa::path(
    get,
    path = "/api/v1/leads",
    tag = "Leads",
    security(("bearer_auth" = [])),
    params(
        ("page" = Option<i64>, Query, description = "Page number"),
        ("limit" = Option<i64>, Query, description = "Items per page"),
        ("status" = Option<String>, Query, description = "Filter by status"),
        ("assigned_to" = Option<i64>, Query, description = "Filter by assignee")
    ),
    responses(
        (status = 200, description = "List of leads"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn list_leads(State(state): State<SharedState>, Query(query): Query<LeadQuery>) -> Result<Json<LeadListResponse>, AppError> {
    let svc = LeadService::new(&state.db);
    Ok(Json(svc.list_leads(query).await?))
}

#[utoipa::path(
    post,
    path = "/api/v1/leads",
    tag = "Leads",
    security(("bearer_auth" = [])),
    request_body = CreateLeadRequest,
    responses(
        (status = 200, description = "Lead created", body = LeadResponse),
        (status = 422, description = "Validation error")
    )
)]
pub async fn create_lead(State(state): State<SharedState>, Json(req): Json<CreateLeadRequest>) -> Result<Json<LeadResponse>, AppError> {
    req.validate()?;
    let svc = LeadService::new(&state.db);
    Ok(Json(svc.create_lead(req).await?))
}

#[utoipa::path(
    get,
    path = "/api/v1/leads/{id}",
    tag = "Leads",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Lead ID")),
    responses(
        (status = 200, description = "Lead details", body = LeadResponse),
        (status = 404, description = "Lead not found")
    )
)]
pub async fn get_lead(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<LeadResponse>, AppError> {
    let svc = LeadService::new(&state.db);
    Ok(Json(svc.get_lead(id).await?))
}

#[utoipa::path(
    put,
    path = "/api/v1/leads/{id}",
    tag = "Leads",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Lead ID")),
    request_body = UpdateLeadRequest,
    responses(
        (status = 200, description = "Lead updated", body = LeadResponse),
        (status = 404, description = "Lead not found")
    )
)]
pub async fn update_lead(State(state): State<SharedState>, Path(id): Path<i64>, Json(req): Json<UpdateLeadRequest>) -> Result<Json<LeadResponse>, AppError> {
    let svc = LeadService::new(&state.db);
    Ok(Json(svc.update_lead(id, req).await?))
}

#[utoipa::path(
    post,
    path = "/api/v1/leads/{id}/status",
    tag = "Leads",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Lead ID")),
    request_body = LeadStatusRequest,
    responses(
        (status = 200, description = "Status updated", body = LeadResponse),
        (status = 404, description = "Lead not found")
    )
)]
pub async fn update_status(State(state): State<SharedState>, Path(id): Path<i64>, Json(req): Json<LeadStatusRequest>) -> Result<Json<LeadResponse>, AppError> {
    let svc = LeadService::new(&state.db);
    Ok(Json(svc.update_status(id, req).await?))
}

#[utoipa::path(
    post,
    path = "/api/v1/leads/{id}/assign",
    tag = "Leads",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Lead ID")),
    request_body = AssignLeadRequest,
    responses(
        (status = 200, description = "Lead assigned", body = LeadResponse),
        (status = 404, description = "Lead not found")
    )
)]
pub async fn assign_lead(State(state): State<SharedState>, Path(id): Path<i64>, Json(req): Json<AssignLeadRequest>) -> Result<Json<LeadResponse>, AppError> {
    let svc = LeadService::new(&state.db);
    Ok(Json(svc.assign_lead(id, req).await?))
}

#[utoipa::path(
    post,
    path = "/api/v1/leads/{id}/activities",
    tag = "Leads",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Lead ID")),
    request_body = AddActivityRequest,
    responses(
        (status = 200, description = "Activity added", body = LeadActivityResponse),
        (status = 404, description = "Lead not found"),
        (status = 422, description = "Validation error")
    )
)]
pub async fn add_activity(State(state): State<SharedState>, user: UserContext, Path(id): Path<i64>, Json(req): Json<AddActivityRequest>) -> Result<Json<LeadActivityResponse>, AppError> {
    req.validate()?;
    let svc = LeadService::new(&state.db);
    Ok(Json(svc.add_activity(id, user.user_id, req).await?))
}

#[utoipa::path(
    get,
    path = "/api/v1/leads/{id}/activities",
    tag = "Leads",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Lead ID")),
    responses(
        (status = 200, description = "List of activities", body = Vec<LeadActivityResponse>),
        (status = 404, description = "Lead not found")
    )
)]
pub async fn get_activities(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<Vec<LeadActivityResponse>>, AppError> {
    let svc = LeadService::new(&state.db);
    Ok(Json(svc.get_activities(id).await?))
}

#[utoipa::path(
    post,
    path = "/api/v1/leads/{id}/convert",
    tag = "Leads",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Lead ID")),
    request_body = ConvertLeadRequest,
    responses(
        (status = 200, description = "Lead converted to customer", body = LeadResponse),
        (status = 404, description = "Lead not found")
    )
)]
pub async fn convert_lead(State(state): State<SharedState>, user: UserContext, Path(id): Path<i64>, Json(req): Json<ConvertLeadRequest>) -> Result<Json<LeadResponse>, AppError> {
    let svc = LeadService::new(&state.db);
    Ok(Json(svc.convert_lead(id, &state.db, user.user_id, req).await?))
}

#[utoipa::path(
    delete,
    path = "/api/v1/leads/{id}",
    tag = "Leads",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Lead ID")),
    responses(
        (status = 200, description = "Lead deleted"),
        (status = 404, description = "Lead not found")
    )
)]
pub async fn delete_lead(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<MessageResponse>, AppError> {
    let svc = LeadService::new(&state.db);
    Ok(Json(svc.delete_lead(id).await?))
}

#[utoipa::path(
    get,
    path = "/api/v1/leads/pipeline",
    tag = "Leads",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Lead pipeline view", body = LeadPipelineResponse),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn get_pipeline(State(state): State<SharedState>) -> Result<Json<LeadPipelineResponse>, AppError> {
    let svc = LeadService::new(&state.db);
    Ok(Json(svc.get_pipeline().await?))
}

#[utoipa::path(
    get,
    path = "/api/v1/leads/stats",
    tag = "Leads",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Lead statistics", body = LeadStatsResponse),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn get_stats(State(state): State<SharedState>) -> Result<Json<LeadStatsResponse>, AppError> {
    let svc = LeadService::new(&state.db);
    Ok(Json(svc.get_stats().await?))
}
