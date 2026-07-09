use axum::extract::{Json, Path, Query, State};
use validator::Validate;
use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::common::middleware::auth_middleware::UserContext;
use crate::modules::document::request::document_request::*;
use crate::modules::document::response::document_response::*;
use crate::modules::document::service::document_service::DocumentService;

pub async fn list_documents(State(state): State<SharedState>, Query(q): Query<DocumentQuery>) -> Result<Json<Vec<DocumentResponse>>, AppError> {
    let svc = DocumentService::new(&state.db);
    Ok(Json(svc.list(q).await?))
}

pub async fn get_document(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<DocumentResponse>, AppError> {
    let svc = DocumentService::new(&state.db);
    Ok(Json(svc.get_by_id(id).await?))
}

pub async fn upload_url(State(state): State<SharedState>, user: UserContext, Json(req): Json<UploadRequest>) -> Result<Json<UploadResponse>, AppError> {
    req.validate()?;
    let svc = DocumentService::new(&state.db);
    Ok(Json(svc.upload(req, user.user_id).await?))
}

pub async fn confirm_upload(State(state): State<SharedState>, Path(id): Path<i64>, Json(req): Json<ConfirmUploadRequest>) -> Result<Json<DocumentResponse>, AppError> {
    let svc = DocumentService::new(&state.db);
    Ok(Json(svc.confirm_upload(id, req).await?))
}

pub async fn associate_entity(State(state): State<SharedState>, Path(id): Path<i64>, Json(req): Json<AssociateEntityRequest>) -> Result<Json<DocumentResponse>, AppError> {
    let svc = DocumentService::new(&state.db);
    Ok(Json(svc.associate_entity(id, req).await?))
}

pub async fn delete_document(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<MessageResponse>, AppError> {
    let svc = DocumentService::new(&state.db);
    Ok(Json(svc.delete(id).await?))
}

pub async fn get_access_logs(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<Vec<DocumentAccessLogResponse>>, AppError> {
    let svc = DocumentService::new(&state.db);
    Ok(Json(svc.get_access_logs(id).await?))
}
