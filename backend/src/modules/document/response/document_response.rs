use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct DocumentResponse { pub id: i64, pub filename: String, pub original_filename: String, pub mime_type: String, pub file_size: i64, pub status: String, pub created_at: DateTime<Utc> }
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UploadResponse { pub document_id: i64, pub upload_url: String, pub expires_in: i64 }
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct MessageResponse { pub message: String }
