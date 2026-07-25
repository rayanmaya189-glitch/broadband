use chrono::{DateTime, NaiveDate, Utc};
/// Entity History & Rollback Module per §32 docs.
/// PostgreSQL trigger-based change tracking with JSONB snapshots.
/// Provides search, diff, and safe rollback to any previous state.
///
/// SECURITY: All entity_type inputs are validated against a whitelist
/// to prevent SQL injection via table name interpolation.
/// All user-provided values use parameterized queries ($1, $2, etc.)
/// to prevent SQL injection via value interpolation.
use sea_orm::{ConnectionTrait, DatabaseConnection, Statement, Value as SeaValue};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::shared::errors::AppError;

/// Whitelist of allowed entity types for history queries.
/// This prevents SQL injection via table name interpolation.
pub const ALLOWED_ENTITY_TYPES: &[&str] = &[
    "customers",
    "subscriptions",
    "plans",
    "invoices",
    "refunds",
    "journal_entries",
    "manual_payments",
    "network_devices",
    "payment_gateways",
    "discounts",
    "approval_requests",
    "bandwidth_profiles",
    "users",
    "tickets",
    "installations",
    "referrals",
    "leads",
    "branches",
    "otp_codes",
    "refresh_tokens",
    "user_sessions",
];

/// Validate that an entity_type is in the allowed whitelist.
fn validate_entity_type(entity_type: &str) -> Result<(), AppError> {
    if ALLOWED_ENTITY_TYPES.contains(&entity_type) {
        Ok(())
    } else {
        Err(AppError::BadRequest(format!(
            "Invalid entity type '{}'. Allowed types: {:?}",
            entity_type, ALLOWED_ENTITY_TYPES
        )))
    }
}

/// A single history entry representing a change to a tracked entity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub id: String,
    pub entity_id: String,
    pub action: String,
    pub old_data: Option<Value>,
    pub new_data: Option<Value>,
    pub changed_fields: Option<Vec<String>>,
    pub user_id: Option<i64>,
    pub user_name: Option<String>,
    pub user_email: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// Paginated result for history queries.
#[derive(Debug, Serialize)]
pub struct PaginatedResult<T> {
    pub items: Vec<T>,
    pub total: i64,
    pub page: i64,
    pub limit: i64,
    pub total_pages: i64,
}

/// A field-level change between two versions.
#[derive(Debug, Serialize)]
pub struct FieldChange {
    pub field: String,
    pub old_value: Value,
    pub new_value: Value,
}

/// Diff between two versions of an entity.
#[derive(Debug, Serialize)]
pub struct HistoryDiff {
    pub entity_type: String,
    pub entity_id: String,
    pub version_a: Value,
    pub version_b: Value,
    pub changes: Vec<FieldChange>,
}

/// Full export of entity history.
#[derive(Debug, Serialize)]
pub struct HistoryExport {
    pub entity_type: String,
    pub entity_id: String,
    pub exported_at: String,
    pub count: usize,
    pub entries: Vec<HistoryEntry>,
}

/// Rollback result after restoring an entity to a previous state.
#[derive(Debug, Serialize)]
pub struct RollbackResult {
    pub history_id: String,
    pub entity_type: String,
    pub entity_id: String,
    pub restored_from: String,
}

/// Service for querying entity history and performing rollbacks.
pub struct EntityHistoryService;

/// Convert a serde_json::Value to a SeaORM Value for parameterized queries.
fn json_to_sea_value(val: &Value) -> SeaValue {
    match val {
        Value::Null => SeaValue::String(None),
        Value::Bool(b) => SeaValue::Bool(Some(*b)),
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                SeaValue::BigInt(Some(i))
            } else if let Some(f) = n.as_f64() {
                SeaValue::Double(Some(f))
            } else {
                SeaValue::String(Some(Box::new(n.to_string())))
            }
        }
        Value::String(s) => SeaValue::String(Some(Box::new(s.clone()))),
        Value::Array(_) | Value::Object(_) => SeaValue::Json(Some(Box::new(val.clone()))),
    }
}

impl EntityHistoryService {
    /// Search history entries for a given entity type with filters.
    pub async fn search_history(
        db: &DatabaseConnection,
        entity_type: &str,
        entity_id: Option<String>,
        action: Option<&str>,
        user_id: Option<i64>,
        from: Option<NaiveDate>,
        to: Option<NaiveDate>,
        page: i64,
        limit: i64,
    ) -> Result<PaginatedResult<HistoryEntry>, AppError> {
        // SECURITY: Validate entity type against whitelist
        validate_entity_type(entity_type)?;

        let table = format!("{}_history", entity_type);

        // Build WHERE conditions and bind parameters using parameterized queries
        let mut conditions = Vec::new();
        let mut params: Vec<SeaValue> = Vec::new();
        let mut param_idx: u32 = 1;

        if let Some(ref eid) = entity_id {
            conditions.push(format!("h.entity_id = ${}", param_idx));
            params.push(SeaValue::String(Some(Box::new(eid.clone()))));
            param_idx += 1;
        }
        if let Some(a) = action {
            // Whitelist allowed actions too
            if !["create", "update", "delete", "rollback"].contains(&a) {
                return Err(AppError::BadRequest(format!(
                    "Invalid action '{}'. Allowed: create, update, delete, rollback",
                    a
                )));
            }
            conditions.push(format!("h.action = ${}", param_idx));
            params.push(SeaValue::String(Some(Box::new(a.to_string()))));
            param_idx += 1;
        }
        if let Some(uid) = user_id {
            conditions.push(format!("h.user_id = ${}", param_idx));
            params.push(SeaValue::BigInt(Some(uid)));
            param_idx += 1;
        }
        if let Some(f) = from {
            conditions.push(format!("h.created_at >= ${}", param_idx));
            params.push(SeaValue::ChronoDateTime(Some(Box::new(
                f.and_hms_opt(0, 0, 0).unwrap_or_default(),
            ))));
            param_idx += 1;
        }
        if let Some(t) = to {
            conditions.push(format!("h.created_at <= ${}", param_idx));
            params.push(SeaValue::ChronoDateTime(Some(Box::new(
                t.and_hms_opt(23, 59, 59).unwrap_or_default(),
            ))));
            param_idx += 1;
        }

        let where_clause = if conditions.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", conditions.join(" AND "))
        };

        // Get total count
        let count_query = format!(
            "SELECT COUNT(*) as count FROM {} h {}",
            table, where_clause
        );
        let count_result = db
            .query_one(Statement::from_sql_and_values(
                db.get_database_backend(),
                &count_query,
                params.clone(),
            ))
            .await?
            .ok_or_else(|| AppError::Internal(anyhow::anyhow!("Count query failed")))?;
        let total: i64 = count_result.try_get("", "count")?;

        // Get paginated results with user join
        let offset = (page - 1) * limit;
        // Add LIMIT and OFFSET as parameters
        let query = format!(
            "SELECT h.*, u.name as user_name, u.email as user_email
             FROM {} h
             LEFT JOIN users u ON h.user_id = u.id
             {}
             ORDER BY h.created_at DESC
             LIMIT ${} OFFSET ${}",
            table,
            where_clause,
            param_idx,
            param_idx + 1
        );
        params.push(SeaValue::BigInt(Some(limit)));
        params.push(SeaValue::BigInt(Some(offset)));

        let results = db
            .query_all(Statement::from_sql_and_values(
                db.get_database_backend(),
                &query,
                params,
            ))
            .await?;

        let entries: Vec<HistoryEntry> = results
            .iter()
            .filter_map(|row| {
                Some(HistoryEntry {
                    id: row.try_get("", "id").ok()?,
                    entity_id: row.try_get("", "entity_id").ok()?,
                    action: row.try_get("", "action").ok()?,
                    old_data: row.try_get("", "old_data").ok(),
                    new_data: row.try_get("", "new_data").ok(),
                    changed_fields: row.try_get("", "changed_fields").ok(),
                    user_id: row.try_get("", "user_id").ok().flatten(),
                    user_name: row.try_get("", "user_name").ok().flatten(),
                    user_email: row.try_get("", "user_email").ok().flatten(),
                    created_at: row.try_get("", "created_at").ok()?,
                })
            })
            .collect();

        Ok(PaginatedResult {
            items: entries,
            total,
            page,
            limit,
            total_pages: ((total as f64) / (limit as f64)).ceil() as i64,
        })
    }

    /// Get a specific history entry by ID.
    pub async fn get_entry(
        db: &DatabaseConnection,
        entity_type: &str,
        history_id: &str,
    ) -> Result<Option<HistoryEntry>, AppError> {
        // SECURITY: Validate entity type against whitelist
        validate_entity_type(entity_type)?;

        let table = format!("{}_history", entity_type);
        let query = format!(
            "SELECT h.*, u.name as user_name, u.email as user_email
             FROM {} h
             LEFT JOIN users u ON h.user_id = u.id
             WHERE h.id = $1
             LIMIT 1",
            table
        );

        let results = db
            .query_all(Statement::from_sql_and_values(
                db.get_database_backend(),
                &query,
                vec![SeaValue::String(Some(Box::new(history_id.to_string())))],
            ))
            .await?;

        Ok(results.first().and_then(|row| {
            Some(HistoryEntry {
                id: row.try_get("", "id").ok()?,
                entity_id: row.try_get("", "entity_id").ok()?,
                action: row.try_get("", "action").ok()?,
                old_data: row.try_get("", "old_data").ok(),
                new_data: row.try_get("", "new_data").ok(),
                changed_fields: row.try_get("", "changed_fields").ok(),
                user_id: row.try_get("", "user_id").ok().flatten(),
                user_name: row.try_get("", "user_name").ok().flatten(),
                user_email: row.try_get("", "user_email").ok().flatten(),
                created_at: row.try_get("", "created_at").ok()?,
            })
        }))
    }

    /// Compare two versions of an entity, returning a field-level diff.
    pub async fn compare_history(
        db: &DatabaseConnection,
        entity_type: &str,
        entity_id: &str,
        version_a: &str,
        version_b: &str,
    ) -> Result<HistoryDiff, AppError> {
        validate_entity_type(entity_type)?;

        let entry_a = Self::get_entry(db, entity_type, version_a)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Version A '{}' not found", version_a)))?;
        let entry_b = Self::get_entry(db, entity_type, version_b)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Version B '{}' not found", version_b)))?;

        if entry_a.entity_id != entity_id || entry_b.entity_id != entity_id {
            return Err(AppError::BadRequest(
                "History entries do not belong to the specified entity".into(),
            ));
        }

        let data_a = entry_a.new_data.unwrap_or_else(|| Value::Object(Default::default()));
        let data_b = entry_b.new_data.unwrap_or_else(|| Value::Object(Default::default()));

        let mut changes = Vec::new();
        if let (Some(obj_a), Some(obj_b)) = (data_a.as_object(), data_b.as_object()) {
            let mut all_keys: Vec<&String> = obj_a.keys().chain(obj_b.keys()).collect();
            all_keys.sort();
            all_keys.dedup();
            for key in all_keys {
                let val_a = obj_a.get(key).cloned().unwrap_or(Value::Null);
                let val_b = obj_b.get(key).cloned().unwrap_or(Value::Null);
                if val_a != val_b {
                    changes.push(FieldChange {
                        field: key.clone(),
                        old_value: val_a,
                        new_value: val_b,
                    });
                }
            }
        }

        Ok(HistoryDiff {
            entity_type: entity_type.to_string(),
            entity_id: entity_id.to_string(),
            version_a: data_a,
            version_b: data_b,
            changes,
        })
    }

    /// Export full history for an entity.
    pub async fn export_history(
        db: &DatabaseConnection,
        entity_type: &str,
        entity_id: &str,
    ) -> Result<HistoryExport, AppError> {
        validate_entity_type(entity_type)?;

        let result = Self::search_history(
            db,
            entity_type,
            Some(entity_id.to_string()),
            None,
            None,
            None,
            None,
            1,
            10000,
        )
        .await?;

        Ok(HistoryExport {
            entity_type: entity_type.to_string(),
            entity_id: entity_id.to_string(),
            exported_at: Utc::now().to_rfc3339(),
            count: result.total as usize,
            entries: result.items,
        })
    }

    /// Rollback an entity to a previous state from history.
    /// Performs safety checks before restoring.
    pub async fn rollback_entity(
        db: &DatabaseConnection,
        entity_type: &str,
        entity_id: &str,
        history_id: &str,
        admin_id: i64,
        reason: &str,
    ) -> Result<RollbackResult, AppError> {
        // SECURITY: Validate entity type against whitelist
        validate_entity_type(entity_type)?;

        // 1. Fetch the history entry
        let entry = Self::get_entry(db, entity_type, history_id)
            .await?
            .ok_or_else(|| AppError::NotFound("History entry not found".into()))?;

        // 2. Validate old_data exists for restoration
        let old_data = entry.old_data.ok_or_else(|| {
            AppError::BadRequest("Cannot rollback: no previous state available".into())
        })?;

        // 3. Safety checks per entity type
        Self::validate_rollback_safety(db, entity_type, entity_id).await?;

        // 4. Build and execute restore query using parameterized values
        if let Some(obj) = old_data.as_object() {
            let mut set_clauses = Vec::new();
            let mut params: Vec<SeaValue> = Vec::new();
            let mut param_idx: u32 = 1;

            for (key, value) in obj {
                if ["id", "created_at"].contains(&key.as_str()) {
                    continue;
                }
                set_clauses.push(format!("{} = ${}", key, param_idx));
                // Convert serde_json::Value to SeaValue
                params.push(json_to_sea_value(value));
                param_idx += 1;
            }

            if !set_clauses.is_empty() {
                // Add entity_id as final parameter
                params.push(SeaValue::String(Some(Box::new(entity_id.to_string()))));
                let query = format!(
                    "UPDATE {} SET {}, updated_at = NOW() WHERE id = ${}",
                    entity_type,
                    set_clauses.join(", "),
                    param_idx
                );
                db.execute(Statement::from_sql_and_values(
                    db.get_database_backend(),
                    &query,
                    params,
                ))
                .await?;
            }
        }

        // 5. Create rollback history entry using parameterized values
        let rollback_query = format!(
            "INSERT INTO {}_history (entity_id, action, old_data, new_data, user_id, reason, rollback_reference, created_at)
             VALUES ($1, 'rollback', $2, $3, $4, $5, $6, NOW())",
            entity_type
        );
        db.execute(Statement::from_sql_and_values(
            db.get_database_backend(),
            &rollback_query,
            vec![
                SeaValue::String(Some(Box::new(entity_id.to_string()))),
                SeaValue::Json(Some(Box::new(
                    entry.new_data.as_ref().cloned().unwrap_or_default(),
                ))),
                SeaValue::Json(Some(Box::new(old_data))),
                SeaValue::BigInt(Some(admin_id)),
                SeaValue::String(Some(Box::new(reason.to_string()))),
                SeaValue::String(Some(Box::new(history_id.to_string()))),
            ],
        ))
        .await?;

        Ok(RollbackResult {
            history_id: history_id.to_string(),
            entity_type: entity_type.to_string(),
            entity_id: entity_id.to_string(),
            restored_from: entry.id,
        })
    }

    /// Safety validation before rollback.
    async fn validate_rollback_safety(
        db: &DatabaseConnection,
        entity_type: &str,
        entity_id: &str,
    ) -> Result<(), AppError> {
        match entity_type {
            "customers" => {
                let count = db
                    .query_one(Statement::from_sql_and_values(
                        db.get_database_backend(),
                        "SELECT COUNT(*) as c FROM subscriptions WHERE customer_id = $1 AND status = 'active'",
                        vec![SeaValue::String(Some(Box::new(entity_id.to_string())))],
                    ))
                    .await?
                    .map(|r| r.try_get::<i64>("", "c").unwrap_or(0))
                    .unwrap_or(0);
                if count > 0 {
                    return Err(AppError::BadRequest(
                        "Cannot rollback customer with active subscription".into(),
                    ));
                }
            }
            "plans" => {
                let count = db
                    .query_one(Statement::from_sql_and_values(
                        db.get_database_backend(),
                        "SELECT COUNT(*) as c FROM subscriptions WHERE plan_id = $1 AND status = 'active'",
                        vec![SeaValue::String(Some(Box::new(entity_id.to_string())))],
                    ))
                    .await?
                    .map(|r| r.try_get::<i64>("", "c").unwrap_or(0))
                    .unwrap_or(0);
                if count > 0 {
                    return Err(AppError::BadRequest(
                        "Cannot rollback plan with active subscribers".into(),
                    ));
                }
            }
            "network_devices" => {
                let count = db
                    .query_one(Statement::from_sql_and_values(
                        db.get_database_backend(),
                        "SELECT COUNT(*) as c FROM network_devices WHERE id = $1 AND status = 'online'",
                        vec![SeaValue::String(Some(Box::new(entity_id.to_string())))],
                    ))
                    .await?
                    .map(|r| r.try_get::<i64>("", "c").unwrap_or(0))
                    .unwrap_or(0);
                if count > 0 {
                    return Err(AppError::BadRequest("Cannot rollback online device".into()));
                }
            }
            _ => {}
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_entity_type_valid() {
        assert!(validate_entity_type("customers").is_ok());
        assert!(validate_entity_type("subscriptions").is_ok());
        assert!(validate_entity_type("plans").is_ok());
    }

    #[test]
    fn test_validate_entity_type_invalid() {
        assert!(validate_entity_type("users; DROP TABLE").is_err());
        assert!(validate_entity_type("nonexistent").is_err());
        assert!(validate_entity_type("audit_logs_history").is_err());
    }

    #[test]
    fn test_allowed_entity_types_count() {
        assert!(
            ALLOWED_ENTITY_TYPES.len() >= 20,
            "Should have at least 20 entity types"
        );
    }
}
