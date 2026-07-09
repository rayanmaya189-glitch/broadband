use sqlx::PgPool;
use crate::modules::audit::model::audit::AuditLog;

pub struct AuditRepository<'a> { pool: &'a PgPool }
impl<'a> AuditRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self { Self { pool } }

    pub async fn list(&self, user_id: Option<i64>, action: Option<&str>, resource_type: Option<&str>, result: Option<&str>, from: Option<&str>, to: Option<&str>, page: i64, per_page: i64) -> Result<(Vec<AuditLog>, i64), sqlx::Error> {
        let offset = (page - 1) * per_page;
        let count_row: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM audit_logs WHERE ($1::bigint IS NULL OR user_id = $1) AND ($2::text IS NULL OR action LIKE '%' || $2 || '%') AND ($3::text IS NULL OR resource_type = $3) AND ($4::text IS NULL OR result = $4) AND ($5::timestamptz IS NULL OR created_at >= $5) AND ($6::timestamptz IS NULL OR created_at <= $6)"
        ).bind(user_id).bind(action).bind(resource_type).bind(result).bind(from).bind(to).fetch_one(self.pool).await?;

        let logs: Vec<AuditLog> = sqlx::query_as(
            "SELECT id, user_id, user_email, user_role, action, resource_type, resource_id, ip_address, user_agent, result, old_data, new_data, metadata, created_at FROM audit_logs WHERE ($1::bigint IS NULL OR user_id = $1) AND ($2::text IS NULL OR action LIKE '%' || $2 || '%') AND ($3::text IS NULL OR resource_type = $3) AND ($4::text IS NULL OR result = $4) AND ($5::timestamptz IS NULL OR created_at >= $5) AND ($6::timestamptz IS NULL OR created_at <= $6) ORDER BY created_at DESC LIMIT $7 OFFSET $8"
        ).bind(user_id).bind(action).bind(resource_type).bind(result).bind(from).bind(to).bind(per_page).bind(offset).fetch_all(self.pool).await?;

        Ok((logs, count_row.0))
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<AuditLog>, sqlx::Error> {
        sqlx::query_as::<_, AuditLog>(
            "SELECT id, user_id, user_email, user_role, action, resource_type, resource_id, ip_address, user_agent, result, old_data, new_data, metadata, created_at FROM audit_logs WHERE id = $1"
        ).bind(id).fetch_optional(self.pool).await
    }

    pub async fn get_by_resource(&self, resource_type: &str, resource_id: &str) -> Result<Vec<AuditLog>, sqlx::Error> {
        sqlx::query_as::<_, AuditLog>(
            "SELECT id, user_id, user_email, user_role, action, resource_type, resource_id, ip_address, user_agent, result, old_data, new_data, metadata, created_at FROM audit_logs WHERE resource_type = $1 AND resource_id = $2 ORDER BY created_at DESC"
        ).bind(resource_type).bind(resource_id).fetch_all(self.pool).await
    }

    pub async fn insert(&self, user_id: Option<i64>, email: Option<&str>, role: Option<&str>, action: &str, resource_type: Option<&str>, result: &str, old_data: Option<serde_json::Value>, new_data: Option<serde_json::Value>, metadata: Option<serde_json::Value>) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO audit_logs (user_id, user_email, user_role, action, resource_type, result, old_data, new_data, metadata) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9)"
        ).bind(user_id).bind(email).bind(role).bind(action).bind(resource_type).bind(result).bind(old_data).bind(new_data).bind(metadata).execute(self.pool).await?;
        Ok(())
    }

    pub async fn export_csv(&self, user_id: Option<i64>, action: Option<&str>, from: Option<&str>, to: Option<&str>) -> Result<Vec<AuditLog>, sqlx::Error> {
        sqlx::query_as::<_, AuditLog>(
            "SELECT id, user_id, user_email, user_role, action, resource_type, resource_id, ip_address, user_agent, result, old_data, new_data, metadata, created_at FROM audit_logs WHERE ($1::bigint IS NULL OR user_id = $1) AND ($2::text IS NULL OR action LIKE '%' || $2 || '%') AND ($3::timestamptz IS NULL OR created_at >= $3) AND ($4::timestamptz IS NULL OR created_at <= $4) ORDER BY created_at DESC LIMIT 10000"
        ).bind(user_id).bind(action).bind(from).bind(to).fetch_all(self.pool).await
    }
}
