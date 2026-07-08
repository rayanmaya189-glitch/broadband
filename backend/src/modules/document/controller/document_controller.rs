use axum::extract::{Json, Path, Query, State};
use validator::Validate;
use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::modules::document::request::document_request::*;
use crate::modules::document::response::document_response::*;
use crate::modules::document::service::document_service::DocumentService;

pub async fn list_documents(State(state): State<SharedState>, Query(q): Query<DocumentQuery>) -> Result<Json<Vec<DocumentResponse>>, AppError> {
    let svc = DocumentService::new(&state.db);
    Ok(Json(svc.list(q).await?))
}

pub async fn upload_url(State(state): State<SharedState>, Json(req): Json<UploadRequest>) -> Result<Json<UploadResponse>, AppError> {
    req.validate()?;
    let svc = DocumentService::new(&state.db);
    Ok(Json(svc.upload(req, 1).await?))
}

pub async fn delete_document(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<MessageResponse>, AppError> {
    let svc = DocumentService::new(&state.db);
    Ok(Json(svc.delete(id).await?))
}
