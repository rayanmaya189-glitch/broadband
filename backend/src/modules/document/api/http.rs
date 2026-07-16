use crate::modules::document::application::services::DocumentService;
use crate::shared::app_state::AppState;
use crate::shared::errors::AppError;
use crate::shared::middleware::auth::{require_permission, UserContext};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use sea_orm::EntityTrait;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Serialize)]
pub struct DocumentResponse {
    pub id: i64,
    pub filename: String,
    pub mime_type: String,
    pub file_size: i64,
    pub status: String,
}

#[derive(Debug, Deserialize)]
pub struct UploadRequest {
    pub filename: String,
    pub mime_type: String,
    pub file_size: i64,
    pub storage_bucket: String,
    pub storage_key: String,
}

#[derive(Debug, Deserialize)]
pub struct PresignUploadRequest {
    pub filename: String,
    pub mime_type: String,
    pub file_size: i64,
    #[serde(default)]
    pub bucket: Option<String>,
    #[serde(default)]
    pub purpose: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PresignUploadResponse {
    pub upload_url: String,
    pub storage_key: String,
    pub storage_bucket: String,
    pub expires_in_secs: u64,
}

pub async fn list_documents(
    State(state): State<Arc<AppState>>,
    user: UserContext,
) -> Result<Json<Vec<DocumentResponse>>, AppError> {
    require_permission(&user, "document.view").map_err(|e| AppError::Forbidden(e.1))?;
    let docs = DocumentService::list_documents(&state.db, None).await?;
    Ok(Json(
        docs.into_iter()
            .map(|d| DocumentResponse {
                id: d.id,
                filename: d.filename,
                mime_type: d.mime_type,
                file_size: d.file_size,
                status: d.status,
            })
            .collect(),
    ))
}

pub async fn confirm_upload(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Json(req): Json<UploadRequest>,
) -> Result<(StatusCode, Json<DocumentResponse>), AppError> {
    require_permission(&user, "document.upload").map_err(|e| AppError::Forbidden(e.1))?;
    let d = DocumentService::create_document(
        &state.db,
        req.filename.clone(),
        req.filename,
        req.mime_type.clone(),
        req.file_size,
        req.storage_bucket,
        req.storage_key,
        user.user_id,
    )
    .await?;
    Ok((
        StatusCode::CREATED,
        Json(DocumentResponse {
            id: d.id,
            filename: d.filename,
            mime_type: d.mime_type,
            file_size: d.file_size,
            status: d.status,
        }),
    ))
}

pub async fn presign_upload(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Json(req): Json<PresignUploadRequest>,
) -> Result<(StatusCode, Json<PresignUploadResponse>), AppError> {
    require_permission(&user, "document.upload").map_err(|e| AppError::Forbidden(e.1))?;
    let storage = state
        .storage
        .as_ref()
        .ok_or_else(|| AppError::Internal(anyhow::anyhow!("Storage service not configured")))?;

    let bucket = req.bucket.as_deref();
    let purpose = req.purpose.as_deref().unwrap_or("general");
    let now = chrono::Utc::now();
    let ext = req.filename.rsplit('.').next().unwrap_or("bin");
    let storage_key = format!(
        "{}/{}/{}/{}/{}.{}",
        purpose,
        now.format("%Y/%m/%d"),
        user.user_id,
        uuid::Uuid::new_v4(),
        req.filename
            .replace(|c: char| !c.is_alphanumeric() && c != '.', "_"),
        ext
    );

    let upload_url = storage
        .presign_upload(bucket, &storage_key, &req.mime_type, 3600)
        .await?;

    Ok((
        StatusCode::OK,
        Json(PresignUploadResponse {
            upload_url,
            storage_key,
            storage_bucket: bucket.unwrap_or("aeroxe-documents").to_string(),
            expires_in_secs: 3600,
        }),
    ))
}

pub async fn delete_document(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Path(id): Path<i64>,
) -> Result<StatusCode, AppError> {
    require_permission(&user, "document.delete").map_err(|e| AppError::Forbidden(e.1))?;
    // Get document info before deletion
    let doc = crate::modules::document::domain::entities::DocumentFile::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Document {} not found", id)))?;

    // Delete from storage if available
    if let Some(storage) = &state.storage {
        if let Err(e) = storage
            .delete_object(Some(&doc.storage_bucket), &doc.storage_key)
            .await
        {
            tracing::warn!(error = %e, doc_id = id, "Failed to delete file from storage");
        }
    }

    // Soft-delete in database
    DocumentService::delete_document(&state.db, id).await?;
    Ok(StatusCode::NO_CONTENT)
}
