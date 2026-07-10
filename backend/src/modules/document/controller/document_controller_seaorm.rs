//! SeaORM-based controller for the Document domain.

use axum::extract::{Json, Path, Query, State};
use validator::Validate;

use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::modules::document::request::document_request::*;
use crate::modules::document::response::document_response::*;
use crate::modules::document::service::document_service_seaorm::DocumentServiceSeaorm;

pub async fn list(State(state): State<SharedState>, Query(q): Query<DocumentQuery>) -> Result<Json<Vec<DocumentFileResponse>>, AppError> {
    let svc = DocumentServiceSeaorm::new(&state.db_seaorm);
    Ok(Json(svc.list(q.entity_type.as_deref(), q.entity_id).await?))
}

pub async fn get_by_id(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<DocumentFileResponse>, AppError> {
    let svc = DocumentServiceSeaorm::new(&state.db_seaorm);
    Ok(Json(svc.get_by_id(id).await?))
}

pub async fn soft_delete(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<MessageResponse>, AppError> {
    let svc = DocumentServiceSeaorm::new(&state.db_seaorm);
    Ok(Json(svc.soft_delete(id).await?))
}

pub async fn confirm_upload(State(state): State<SharedState>, Path(id): Path<i64>, Json(req): Json<ConfirmUploadRequest>) -> Result<Json<DocumentFileResponse>, AppError> {
    let svc = DocumentServiceSeaorm::new(&state.db_seaorm);
    Ok(Json(svc.confirm_upload(id, req.file_hash.as_deref(), req.storage_url.as_deref()).await?))
}
