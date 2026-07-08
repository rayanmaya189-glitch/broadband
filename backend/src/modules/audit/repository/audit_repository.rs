use sqlx::PgPool;
use crate::modules::audit::model::audit::AuditLog;

pub struct AuditRepository<'a> { pool: &'a PgPool }
impl<'a> AuditRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self { Self { pool } }

    pub async fn list(&self, user_id: Option<i64>, action: Option<&str>, resource_type: Option<&str>, page: i64, per_page: i64) -> Result<(Vec<AuditLog>, i64), sqlx::Error> {
        let offset = (page - 1) * per_page;
        let count_row: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM audit_logs WHERE ($1::bigint IS NULL OR user_id = $1) AND ($2::text IS NULL OR action LIKE '%' || $2 || '%') AND ($3::text IS NULL OR resource_type = $3)"
        ).bind(user_id).bind(action).bind(resource_type).fetch_one(self.pool).await?;

        let logs: Vec<AuditLog> = sqlx::query_as(
            "SELECT * FROM audit_logs WHERE ($1::bigint IS NULL OR user_id = $1) AND ($2::text IS NULL OR action LIKE '%' || $2 || '%') AND ($3::text IS NULL OR resource_type = $3) ORDER BY created_at DESC LIMIT $4 OFFSET $5"
        ).bind(user_id).bind(action).bind(resource_type).bind(per_page).bind(offset).fetch_all(self.pool).await?;

        Ok((logs, count_row.0))
    }

    pub async fn insert(&self, user_id: Option<i64>, email: Option<&str>, role: Option<&str>, action: &str, resource_type: Option<&str>, result: &str) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO audit_logs (user_id, user_email, user_role, action, resource_type, result) VALUES ($1,$2,$3,$4,$5,$6)"
        ).bind(user_id).bind(email).bind(role).bind(action).bind(resource_type).bind(result).execute(self.pool).await?;
        Ok(())
    }
}
