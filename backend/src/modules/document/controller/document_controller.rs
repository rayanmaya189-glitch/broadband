use axum::extract::{Json, Path, Query, State};
use validator::Validate;
use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::common::middleware::auth_middleware::UserContext;
use crate::modules::document::request::document_request::*;
use crate::modules::document::response::document_response::*;
use crate::modules::document::service::document_service::DocumentService;

#[utoipa::path(
    get,
    path = "/api/v1/documents",
    tag = "Documents",
    security(("bearer_auth" = [])),
    params(
        ("entity_type" = Option<String>, Query, description = "Filter by entity type"),
        ("entity_id" = Option<i64>, Query, description = "Filter by entity ID")
    ),
    responses(
        (status = 200, description = "List of documents", body = Vec<DocumentResponse>),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn list_documents(State(state): State<SharedState>, Query(q): Query<DocumentQuery>) -> Result<Json<Vec<DocumentResponse>>, AppError> {
    let svc = DocumentService::new(&state.db);
    Ok(Json(svc.list(q).await?))
}

#[utoipa::path(
    get,
    path = "/api/v1/documents/{id}",
    tag = "Documents",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Document ID")),
    responses(
        (status = 200, description = "Document details", body = DocumentResponse),
        (status = 404, description = "Document not found")
    )
)]
pub async fn get_document(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<DocumentResponse>, AppError> {
    let svc = DocumentService::new(&state.db);
    Ok(Json(svc.get_by_id(id).await?))
}

#[utoipa::path(
    post,
    path = "/api/v1/documents/upload-url",
    tag = "Documents",
    security(("bearer_auth" = [])),
    request_body = UploadRequest,
    responses(
        (status = 200, description = "Upload URL generated", body = UploadResponse),
        (status = 422, description = "Validation error")
    )
)]
pub async fn upload_url(State(state): State<SharedState>, user: UserContext, Json(req): Json<UploadRequest>) -> Result<Json<UploadResponse>, AppError> {
    req.validate()?;
    let svc = DocumentService::new(&state.db);
    Ok(Json(svc.upload(req, user.user_id).await?))
}

#[utoipa::path(
    post,
    path = "/api/v1/documents/{id}/confirm",
    tag = "Documents",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Document ID")),
    request_body = ConfirmUploadRequest,
    responses(
        (status = 200, description = "Upload confirmed", body = DocumentResponse),
        (status = 404, description = "Document not found")
    )
)]
pub async fn confirm_upload(State(state): State<SharedState>, Path(id): Path<i64>, Json(req): Json<ConfirmUploadRequest>) -> Result<Json<DocumentResponse>, AppError> {
    let svc = DocumentService::new(&state.db);
    Ok(Json(svc.confirm_upload(id, req).await?))
}

#[utoipa::path(
    put,
    path = "/api/v1/documents/{id}/associate",
    tag = "Documents",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Document ID")),
    request_body = AssociateEntityRequest,
    responses(
        (status = 200, description = "Entity associated", body = DocumentResponse),
        (status = 404, description = "Document not found")
    )
)]
pub async fn associate_entity(State(state): State<SharedState>, Path(id): Path<i64>, Json(req): Json<AssociateEntityRequest>) -> Result<Json<DocumentResponse>, AppError> {
    let svc = DocumentService::new(&state.db);
    Ok(Json(svc.associate_entity(id, req).await?))
}

#[utoipa::path(
    delete,
    path = "/api/v1/documents/{id}",
    tag = "Documents",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Document ID")),
    responses(
        (status = 200, description = "Document deleted"),
        (status = 404, description = "Document not found")
    )
)]
pub async fn delete_document(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<MessageResponse>, AppError> {
    let svc = DocumentService::new(&state.db);
    Ok(Json(svc.delete(id).await?))
}

#[utoipa::path(
    get,
    path = "/api/v1/documents/{id}/access-logs",
    tag = "Documents",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Document ID")),
    responses(
        (status = 200, description = "List of access logs", body = Vec<DocumentAccessLogResponse>),
        (status = 404, description = "Document not found")
    )
)]
pub async fn get_access_logs(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<Vec<DocumentAccessLogResponse>>, AppError> {
    let svc = DocumentService::new(&state.db);
    Ok(Json(svc.get_access_logs(id).await?))
}
