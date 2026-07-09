use sqlx::PgPool;
use crate::modules::document::model::document::*;

pub struct DocumentRepository<'a> { pool: &'a PgPool }
impl<'a> DocumentRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self { Self { pool } }

    pub async fn list(&self, entity_type: Option<&str>, entity_id: Option<i64>) -> Result<Vec<DocumentFile>, sqlx::Error> {
        sqlx::query_as::<_, DocumentFile>(
            "SELECT id, filename, original_filename, mime_type, file_size, file_hash, storage_bucket, storage_key, storage_url, uploaded_by, entity_type, entity_id, status, metadata, created_at, updated_at FROM document_files WHERE ($1::text IS NULL OR entity_type = $1) AND ($2::bigint IS NULL OR entity_id = $2) AND status != 'deleted' ORDER BY created_at DESC"
        ).bind(entity_type).bind(entity_id).fetch_all(self.pool).await
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<DocumentFile>, sqlx::Error> {
        sqlx::query_as::<_, DocumentFile>(
            "SELECT id, filename, original_filename, mime_type, file_size, file_hash, storage_bucket, storage_key, storage_url, uploaded_by, entity_type, entity_id, status, metadata, created_at, updated_at FROM document_files WHERE id = $1"
        ).bind(id).fetch_optional(self.pool).await
    }

    pub async fn create(&self, filename: &str, orig: &str, mime: &str, size: i64, bucket: &str, key: &str, uploaded_by: i64, entity_type: Option<&str>, entity_id: Option<i64>) -> Result<DocumentFile, sqlx::Error> {
        sqlx::query_as::<_, DocumentFile>(
            "INSERT INTO document_files (filename, original_filename, mime_type, file_size, storage_bucket, storage_key, uploaded_by, entity_type, entity_id, status) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,'pending') RETURNING id, filename, original_filename, mime_type, file_size, file_hash, storage_bucket, storage_key, storage_url, uploaded_by, entity_type, entity_id, status, metadata, created_at, updated_at"
        ).bind(filename).bind(orig).bind(mime).bind(size).bind(bucket).bind(key).bind(uploaded_by).bind(entity_type).bind(entity_id).fetch_one(self.pool).await
    }

    pub async fn confirm_upload(&self, id: i64, file_hash: Option<&str>, storage_url: Option<&str>) -> Result<DocumentFile, sqlx::Error> {
        sqlx::query_as::<_, DocumentFile>(
            "UPDATE document_files SET status = 'active', file_hash = COALESCE($2, file_hash), storage_url = COALESCE($3, storage_url), updated_at = NOW() WHERE id = $1 RETURNING id, filename, original_filename, mime_type, file_size, file_hash, storage_bucket, storage_key, storage_url, uploaded_by, entity_type, entity_id, status, metadata, created_at, updated_at"
        ).bind(id).bind(file_hash).bind(storage_url).fetch_one(self.pool).await
    }

    pub async fn soft_delete(&self, id: i64) -> Result<bool, sqlx::Error> {
        let r = sqlx::query("UPDATE document_files SET status = 'deleted', updated_at = NOW() WHERE id = $1").bind(id).execute(self.pool).await?;
        Ok(r.rows_affected() > 0)
    }

    pub async fn associate_entity(&self, id: i64, entity_type: &str, entity_id: i64) -> Result<bool, sqlx::Error> {
        let r = sqlx::query("UPDATE document_files SET entity_type = $2, entity_id = $3, updated_at = NOW() WHERE id = $1")
            .bind(id).bind(entity_type).bind(entity_id).execute(self.pool).await?;
        Ok(r.rows_affected() > 0)
    }

    // ── Access Logs ─────────────────────────────────────────

    pub async fn log_access(&self, document_id: i64, accessed_by: Option<i64>, access_type: &str, ip_address: Option<&str>, user_agent: Option<&str>) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT INTO document_access_logs (document_id, accessed_by, access_type, ip_address, user_agent) VALUES ($1,$2,$3,$4,$5)")
            .bind(document_id).bind(accessed_by).bind(access_type).bind(ip_address).bind(user_agent).execute(self.pool).await?;
        Ok(())
    }

    pub async fn get_access_logs(&self, document_id: i64) -> Result<Vec<DocumentAccessLog>, sqlx::Error> {
        sqlx::query_as::<_, DocumentAccessLog>(
            "SELECT id, document_id, accessed_by, access_type, ip_address, user_agent, accessed_at FROM document_access_logs WHERE document_id = $1 ORDER BY accessed_at DESC"
        ).bind(document_id).fetch_all(self.pool).await
    }
}
