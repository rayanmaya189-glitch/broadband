use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::shared::app_state::AppState;
use crate::shared::errors::AppError;
use crate::shared::middleware::auth::UserContext;
use crate::modules::document::application::services::DocumentService;

#[derive(Debug, Serialize)]
pub struct DocumentResponse { pub id: i64, pub filename: String, pub mime_type: String, pub file_size: i64, pub status: String }

#[derive(Debug, Deserialize)]
pub struct UploadRequest { pub filename: String, pub mime_type: String, pub file_size: i64, pub storage_bucket: String, pub storage_key: String }

pub async fn list_documents(State(state): State<Arc<AppState>>, _user: UserContext) -> Result<Json<Vec<DocumentResponse>>, AppError> {
    let docs = DocumentService::list_documents(&state.db, None).await?;
    Ok(Json(docs.into_iter().map(|d| DocumentResponse { id: d.id, filename: d.filename, mime_type: d.mime_type, file_size: d.file_size, status: d.status }).collect()))
}

pub async fn confirm_upload(State(state): State<Arc<AppState>>, user: UserContext, Json(req): Json<UploadRequest>) -> Result<(StatusCode, Json<DocumentResponse>), AppError> {
    let d = DocumentService::create_document(&state.db, req.filename.clone(), req.filename, req.mime_type.clone(), req.file_size, req.storage_bucket, req.storage_key, user.user_id).await?;
    Ok((StatusCode::CREATED, Json(DocumentResponse { id: d.id, filename: d.filename, mime_type: d.mime_type, file_size: d.file_size, status: d.status })))
}

pub async fn delete_document(State(state): State<Arc<AppState>>, _user: UserContext, Path(id): Path<i64>) -> Result<StatusCode, AppError> {
    DocumentService::delete_document(&state.db, id).await?;
    Ok(StatusCode::NO_CONTENT)
}
