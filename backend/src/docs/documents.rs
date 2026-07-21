/// OpenAPI schemas and stub handlers for Document endpoints.
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

// ── Request / Response types ─────────────────────────────────────────

#[derive(Debug, Serialize, ToSchema)]
pub struct DocumentResponse {
    /// Document ID
    pub id: i64,
    /// Filename
    pub filename: String,
    /// MIME type
    pub mime_type: String,
    /// File size in bytes
    pub file_size: i64,
    /// Document status
    pub status: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UploadRequest {
    /// Filename
    pub filename: String,
    /// MIME type
    pub mime_type: String,
    /// File size in bytes
    pub file_size: i64,
    /// Storage bucket
    pub storage_bucket: String,
    /// Storage key
    pub storage_key: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct PresignUploadRequest {
    /// Filename
    pub filename: String,
    /// MIME type
    pub mime_type: String,
    /// File size in bytes
    pub file_size: i64,
    /// Storage bucket (optional)
    #[serde(default)]
    pub bucket: Option<String>,
    /// Upload purpose (optional, defaults to "general")
    #[serde(default)]
    pub purpose: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PresignUploadResponse {
    /// Pre-signed upload URL
    pub upload_url: String,
    /// Storage key for the uploaded file
    pub storage_key: String,
    /// Storage bucket
    pub storage_bucket: String,
    /// URL expiry in seconds
    pub expires_in_secs: u64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct DownloadUrlResponse {
    /// Pre-signed download URL
    pub url: String,
    /// URL expiry timestamp
    pub expires_at: String,
}

// ── Stub handler functions (for OpenAPI spec only) ───────────────────

/// List all documents (paginated)
#[utoipa::path(
    get,
    path = "/api/v1/documents",
    tag = "Documents",
    params(("page" = Option<i64>, Query, description = "Page number"),
           ("limit" = Option<i64>, Query, description = "Items per page")),
    responses(
        (status = 200, description = "Paginated list of documents"),
        (status = 403, description = "Forbidden")
    ),
    security(("bearer_auth" = []))
)]
pub async fn list_documents() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Confirm a document upload after direct storage upload
#[utoipa::path(
    post,
    path = "/api/v1/documents/upload/confirm",
    tag = "Documents",
    request_body = UploadRequest,
    responses(
        (status = 201, description = "Document confirmed", body = DocumentResponse),
        (status = 403, description = "Forbidden"),
        (status = 422, description = "Validation error")
    ),
    security(("bearer_auth" = []))
)]
pub async fn confirm_upload() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Generate a pre-signed upload URL
#[utoipa::path(
    post,
    path = "/api/v1/documents/presign",
    tag = "Documents",
    request_body = PresignUploadRequest,
    responses(
        (status = 200, description = "Pre-signed URL generated", body = PresignUploadResponse),
        (status = 403, description = "Forbidden"),
        (status = 500, description = "Storage service not configured")
    ),
    security(("bearer_auth" = []))
)]
pub async fn presign_upload() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Get a single document by ID
#[utoipa::path(
    get,
    path = "/api/v1/documents/{id}",
    tag = "Documents",
    params(("id" = i64, Path, description = "Document ID")),
    responses(
        (status = 200, description = "Document details", body = DocumentResponse),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Document not found")
    ),
    security(("bearer_auth" = []))
)]
pub async fn get_document() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Get a pre-signed download URL for a document
#[utoipa::path(
    get,
    path = "/api/v1/documents/{id}/download",
    tag = "Documents",
    params(("id" = i64, Path, description = "Document ID")),
    responses(
        (status = 200, description = "Download URL", body = DownloadUrlResponse),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Document not found"),
        (status = 500, description = "Storage service not configured")
    ),
    security(("bearer_auth" = []))
)]
pub async fn get_download_url() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// List documents attached to a specific entity
#[utoipa::path(
    get,
    path = "/api/v1/documents/entity/{entity_type}/{entity_id}",
    tag = "Documents",
    params(("entity_type" = String, Path, description = "Entity type (e.g. \"customer\", \"installation\")"),
           ("entity_id" = i64, Path, description = "Entity ID"),
           ("page" = Option<i64>, Query, description = "Page number"),
           ("limit" = Option<i64>, Query, description = "Items per page")),
    responses(
        (status = 200, description = "Paginated list of entity documents"),
        (status = 403, description = "Forbidden")
    ),
    security(("bearer_auth" = []))
)]
pub async fn list_entity_documents() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Soft-delete a document
#[utoipa::path(
    delete,
    path = "/api/v1/documents/{id}",
    tag = "Documents",
    params(("id" = i64, Path, description = "Document ID")),
    responses(
        (status = 204, description = "Document deleted"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Document not found")
    ),
    security(("bearer_auth" = []))
)]
pub async fn delete_document() -> axum::Json<serde_json::Value> {
    unimplemented!()
}
