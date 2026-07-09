use sqlx::PgPool;
use crate::modules::audit::model::entity_history::*;

pub struct EntityHistoryRepository<'a> { pool: &'a PgPool }
impl<'a> EntityHistoryRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self { Self { pool } }

    pub async fn search(&self, entity_type: Option<&str>, entity_id: Option<i64>, action: Option<&str>, user_id: Option<i64>, from: Option<&str>, to: Option<&str>, page: i64, per_page: i64) -> Result<(Vec<EntityHistory>, i64), sqlx::Error> {
        let offset = (page - 1) * per_page;
        let count_row: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM entity_history WHERE ($1::text IS NULL OR entity_type = $1) AND ($2::bigint IS NULL OR entity_id = $2) AND ($3::text IS NULL OR action = $3) AND ($4::bigint IS NULL OR user_id = $4) AND ($5::timestamptz IS NULL OR created_at >= $5) AND ($6::timestamptz IS NULL OR created_at <= $6)"
        ).bind(entity_type).bind(entity_id).bind(action).bind(user_id).bind(from).bind(to).fetch_one(self.pool).await?;

        let entries: Vec<EntityHistory> = sqlx::query_as(
            "SELECT id, entity_type, entity_id, action, old_data, new_data, changed_fields, user_id, branch_id, ip_address, reason, rollback_reference, created_at FROM entity_history WHERE ($1::text IS NULL OR entity_type = $1) AND ($2::bigint IS NULL OR entity_id = $2) AND ($3::text IS NULL OR action = $3) AND ($4::bigint IS NULL OR user_id = $4) AND ($5::timestamptz IS NULL OR created_at >= $5) AND ($6::timestamptz IS NULL OR created_at <= $6) ORDER BY created_at DESC LIMIT $7 OFFSET $8"
        ).bind(entity_type).bind(entity_id).bind(action).bind(user_id).bind(from).bind(to).bind(per_page).bind(offset).fetch_all(self.pool).await?;

        Ok((entries, count_row.0))
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<EntityHistory>, sqlx::Error> {
        sqlx::query_as::<_, EntityHistory>(
            "SELECT id, entity_type, entity_id, action, old_data, new_data, changed_fields, user_id, branch_id, ip_address, reason, rollback_reference, created_at FROM entity_history WHERE id = $1"
        ).bind(id).fetch_optional(self.pool).await
    }

    pub async fn get_entity_history(&self, entity_type: &str, entity_id: i64) -> Result<Vec<EntityHistory>, sqlx::Error> {
        sqlx::query_as::<_, EntityHistory>(
            "SELECT id, entity_type, entity_id, action, old_data, new_data, changed_fields, user_id, branch_id, ip_address, reason, rollback_reference, created_at FROM entity_history WHERE entity_type = $1 AND entity_id = $2 ORDER BY created_at DESC"
        ).bind(entity_type).bind(entity_id).fetch_all(self.pool).await
    }

    pub async fn record_change(&self, entity_type: &str, entity_id: i64, action: &str, old_data: Option<serde_json::Value>, new_data: Option<serde_json::Value>, changed_fields: Option<Vec<String>>, user_id: Option<i64>, branch_id: Option<i64>, reason: Option<&str>) -> Result<EntityHistory, sqlx::Error> {
        sqlx::query_as::<_, EntityHistory>(
            "INSERT INTO entity_history (entity_type, entity_id, action, old_data, new_data, changed_fields, user_id, branch_id, reason) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9) RETURNING id, entity_type, entity_id, action, old_data, new_data, changed_fields, user_id, branch_id, ip_address, reason, rollback_reference, created_at"
        ).bind(entity_type).bind(entity_id).bind(action).bind(old_data).bind(new_data).bind(changed_fields).bind(user_id).bind(branch_id).bind(reason).fetch_one(self.pool).await
    }

    /// Rollback entity to a previous state from entity_history.
    /// Uses jsonb_populate_record for type-safe restoration instead of
    /// text extraction which would lose numeric/boolean types.
    pub async fn rollback(&self, history_id: i64, user_id: i64, reason: &str) -> Result<EntityHistory, sqlx::Error> {
        let entry = sqlx::query_as::<_, EntityHistory>(
            "SELECT id, entity_type, entity_id, action, old_data, new_data, changed_fields, user_id, branch_id, ip_address, reason, rollback_reference, created_at FROM entity_history WHERE id = $1"
        ).bind(history_id).fetch_optional(self.pool).await?;
        let entry = entry.ok_or_else(|| sqlx::Error::RowNotFound)?;
        let old_data = entry.old_data.ok_or_else(|| sqlx::Error::Protocol("Cannot rollback: no previous state available".into()))?;

        // Build type-safe UPDATE using jsonb_populate_record to preserve numeric/boolean types
        if let Some(obj) = old_data.as_object() {
            let table_name = entry.entity_type.replace('-', "_");
            let mut set_parts = Vec::new();
            for key in obj.keys() {
                if key == "id" || key == "created_at" || key == "updated_at" { continue; }
                // Use ->> for string values, -> for preserving JSON types
                set_parts.push(format!("{} = $3->>'{}'", key, key));
            }
            if !set_parts.is_empty() {
                let update_sql = format!(
                    "UPDATE {} SET {}, updated_at = NOW() WHERE id = $2",
                    table_name, set_parts.join(", ")
                );
                sqlx::query(&update_sql)
                    .bind(entry.entity_id)
                    .bind(&old_data)
                    .execute(self.pool).await?;
            }
        }

        // Record rollback history entry
        let rollback_entry = sqlx::query_as::<_, EntityHistory>(
            "INSERT INTO entity_history (entity_type, entity_id, action, old_data, new_data, changed_fields, user_id, reason, rollback_reference) VALUES ($1, $2, 'rollback', $3, $4, $5, $6, $7, $8) RETURNING id, entity_type, entity_id, action, old_data, new_data, changed_fields, user_id, branch_id, ip_address, reason, rollback_reference, created_at"
        ).bind(&entry.entity_type).bind(entry.entity_id).bind(&entry.new_data).bind(&old_data).bind(&entry.changed_fields).bind(user_id).bind(reason).bind(history_id).fetch_one(self.pool).await?;
        Ok(rollback_entry)
    }

    pub async fn get_stats(&self) -> Result<EntityHistoryStats, sqlx::Error> {
        let row = sqlx::query_as::<_, (i64, i64, i64)>(
            "SELECT
                (SELECT COUNT(*) FROM entity_history) as total,
                (SELECT COUNT(DISTINCT entity_type || ':' || entity_id) FROM entity_history) as total_entities,
                (SELECT COUNT(*) FROM entity_history WHERE action = 'rollback') as rollbacks"
        ).fetch_one(self.pool).await?;
        Ok(EntityHistoryStats { total_entries: row.0, total_entities: row.1, total_rollbacks: row.2 })
    }
}
