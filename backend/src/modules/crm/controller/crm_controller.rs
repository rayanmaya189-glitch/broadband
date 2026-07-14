use axum::extract::{Json, Path, Query, State};
use validator::Validate;

use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::modules::crm::request::crm_request::*;
use crate::modules::crm::response::crm_response::*;
use crate::modules::crm::service::crm_service::CrmService;

pub async fn list_interactions(State(state): State<SharedState>, Query(q): Query<InteractionQuery>) -> Result<Json<Vec<InteractionResponse>>, AppError> {
    let svc = CrmService::new(&state.db);
    let (items, _) = svc.list_interactions(q.customer_id, q.branch_id, q.interaction_type.as_deref(), q.page.unwrap_or(1), q.per_page.unwrap_or(20)).await?;
    Ok(Json(items))
}

pub async fn create_interaction(State(state): State<SharedState>, Json(req): Json<CreateInteractionRequest>) -> Result<Json<InteractionResponse>, AppError> {
    req.validate()?;
    let svc = CrmService::new(&state.db);
    Ok(Json(svc.create_interaction(req.customer_id, req.branch_id, 0, req).await?))
}

pub async fn list_notes(State(state): State<SharedState>, Path(customer_id): Path<i64>, Query(q): Query<PageQuery>) -> Result<Json<Vec<NoteResponse>>, AppError> {
    let svc = CrmService::new(&state.db);
    let (items, _) = svc.list_notes(customer_id, q.page.unwrap_or(1), q.per_page.unwrap_or(20)).await?;
    Ok(Json(items))
}

pub async fn create_note(State(state): State<SharedState>, Path(customer_id): Path<i64>, Json(req): Json<CreateNoteRequest>) -> Result<Json<NoteResponse>, AppError> {
    req.validate()?;
    let svc = CrmService::new(&state.db);
    Ok(Json(svc.create_note(customer_id, 0, 0, req).await?))
}

pub async fn list_tags(State(state): State<SharedState>, Query(q): Query<TagQuery>) -> Result<Json<Vec<TagResponse>>, AppError> {
    let svc = CrmService::new(&state.db);
    Ok(Json(svc.list_tags(q.branch_id, q.category.as_deref()).await?))
}

pub async fn create_tag(State(state): State<SharedState>, Json(req): Json<CreateTagRequest>) -> Result<Json<TagResponse>, AppError> {
    req.validate()?;
    let svc = CrmService::new(&state.db);
    Ok(Json(svc.create_tag(None, req).await?))
}

pub async fn assign_tag(State(state): State<SharedState>, Path((customer_id, tag_id)): Path<(i64, i64)>) -> Result<Json<MessageResponse>, AppError> {
    let svc = CrmService::new(&state.db);
    Ok(Json(svc.assign_tag(customer_id, tag_id).await?))
}

pub async fn list_segments(State(state): State<SharedState>) -> Result<Json<Vec<SegmentResponse>>, AppError> {
    let svc = CrmService::new(&state.db);
    Ok(Json(svc.list_segments(None).await?))
}

pub async fn create_segment(State(state): State<SharedState>, Json(req): Json<CreateSegmentRequest>) -> Result<Json<SegmentResponse>, AppError> {
    req.validate()?;
    let svc = CrmService::new(&state.db);
    Ok(Json(svc.create_segment(None, req).await?))
}
