use sqlx::PgPool;
use crate::modules::document::model::document::DocumentFile;
pub struct DocumentRepository<'a> { pool: &'a PgPool }
impl<'a> DocumentRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self { Self { pool } }
    pub async fn list(&self, entity_type: Option<&str>, entity_id: Option<i64>) -> Result<Vec<DocumentFile>, sqlx::Error> {
        sqlx::query_as::<_, DocumentFile>("SELECT * FROM document_files WHERE ($1::text IS NULL OR entity_type = $1) AND ($2::bigint IS NULL OR entity_id = $2) ORDER BY created_at DESC").bind(entity_type).bind(entity_id).fetch_all(self.pool).await
    }
    pub async fn create(&self, filename: &str, orig: &str, mime: &str, size: i64, bucket: &str, key: &str, uploaded_by: i64) -> Result<DocumentFile, sqlx::Error> {
        sqlx::query_as::<_, DocumentFile>("INSERT INTO document_files (filename, original_filename, mime_type, file_size, storage_bucket, storage_key, uploaded_by) VALUES ($1,$2,$3,$4,$5,$6,$7) RETURNING *").bind(filename).bind(orig).bind(mime).bind(size).bind(bucket).bind(key).bind(uploaded_by).fetch_one(self.pool).await
    }
    pub async fn delete(&self, id: i64) -> Result<bool, sqlx::Error> { let r = sqlx::query("UPDATE document_files SET status = 'deleted' WHERE id = $1").bind(id).execute(self.pool).await?; Ok(r.rows_affected() > 0) }
}
