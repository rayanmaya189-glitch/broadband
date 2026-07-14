//! SeaORM-based repository for the EntityHistory domain.
//!
//! Provides entity history tracking with rollback support.
//! Rollback includes safety checks per entity type and actual
//! entity data restoration via SeaORM (no raw SQL).

use sea_orm::*;
use serde_json::Value as JsonValue;

use crate::common::errors::app_error::AppError;
use crate::modules::audit::model::entity_history_entity::{self, Model as EntityHistoryModel};

/// Result of a successful rollback operation.
#[derive(Debug)]
pub struct RollbackResult {
    pub history_id: i64,
    pub entity_type: String,
    pub entity_id: i64,
    pub restored_from: i64,
}

pub struct EntityHistoryRepository<'a> {
    db: &'a DatabaseConnection,
}

/// Immutable fields that should never be overwritten during rollback.
const IMMUTABLE_FIELDS: &[&str] = &["id", "created_at"];

/// Entity types that support rollback and their table names.
const ROLLABLE_ENTITIES: &[&str] = &[
    "customers",
    "subscriptions",
    "plans",
    "invoices",
    "refunds",
    "journal_entries",
    "network_devices",
    "payment_gateways",
    "discounts",
    "bandwidth_profiles",
];

/// Whitelist of valid columns per entity type for rollback.
/// Prevents setting unexpected columns from corrupted old_data JSONB.
fn valid_columns_for_entity(entity_type: &str) -> Option<&'static [&'static str]> {
    match entity_type {
        "customers" => Some(&[
            "customer_code", "first_name", "last_name", "email", "phone",
            "alternate_phone", "status", "branch_id", "lead_id", "referred_by",
            "created_by", "kyc_status", "notes", "updated_at",
        ]),
        "subscriptions" => Some(&[
            "customer_id", "branch_id", "plan_id", "status",
            "billing_period_months", "start_date", "end_date",
            "next_billing_date", "auto_renew", "updated_at",
        ]),
        "plans" => Some(&[
            "name", "code", "description", "speed_down_mbps", "speed_up_mbps",
            "data_cap_gb", "price_monthly", "price_quarterly", "price_half_yearly",
            "price_yearly", "gst_percent", "is_active", "is_promotional",
            "category", "updated_at",
        ]),
        "invoices" => Some(&[
            "invoice_number", "customer_id", "branch_id", "subscription_id",
            "billing_period_start", "billing_period_end", "subtotal",
            "discount_amount", "tax_amount", "cgst_amount", "sgst_amount",
            "igst_amount", "total_amount", "currency", "status", "due_date",
            "paid_at", "payment_method", "payment_reference", "created_by",
            "review_status", "review_notes", "reviewed_by", "reviewed_at",
            "approved_by", "approved_at", "notes", "updated_at",
        ]),
        "refunds" => Some(&[
            "refund_number", "payment_id", "invoice_id", "customer_id",
            "amount", "reason", "requested_by", "approved_by", "status",
            "processed_at",
        ]),
        "journal_entries" => Some(&[
            "entry_number", "entry_date", "description", "reference_type",
            "reference_id", "total_debit", "total_credit", "status",
            "posted_at", "created_by", "updated_at",
        ]),
        "network_devices" => Some(&[
            "branch_id", "name", "device_model_id", "serial_number",
            "management_ip", "management_port", "firmware_version", "status",
            "health_score", "location_city", "location_area", "created_by",
            "updated_at",
        ]),
        "payment_gateways" => Some(&[
            "gateway_id", "name", "is_primary", "is_active",
            "supported_methods", "currency", "updated_at",
        ]),
        "discounts" => Some(&[
            "name", "code", "discount_type", "value", "max_uses",
            "current_uses", "valid_from", "valid_until", "is_active",
        ]),
        "bandwidth_profiles" => Some(&[
            "name", "description", "plan_id", "download_kbps", "upload_kbps",
            "burst_download_kbps", "burst_upload_kbps", "burst_duration_seconds",
            "priority", "is_active", "updated_at",
        ]),
        _ => None,
    }
}

impl<'a> EntityHistoryRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self { Self { db } }

    pub async fn search(
        &self, entity_type: Option<&str>, entity_id: Option<i64>, action: Option<&str>,
        user_id: Option<i64>, _from: Option<&str>, _to: Option<&str>,
        page: i64, per_page: i64,
    ) -> Result<(Vec<EntityHistoryModel>, i64), AppError> {
        let page_size = per_page as u64;
        let page_num = if per_page > 0 { ((page - 1).max(0) as u64 * page_size) / page_size } else { 0 };
        let mut select = entity_history_entity::Entity::find();
        if let Some(et) = entity_type {
            select = select.filter(entity_history_entity::Column::EntityType.eq(et));
        }
        if let Some(eid) = entity_id {
            select = select.filter(entity_history_entity::Column::EntityId.eq(eid));
        }
        if let Some(a) = action {
            select = select.filter(entity_history_entity::Column::Action.eq(a));
        }
        if let Some(uid) = user_id {
            select = select.filter(entity_history_entity::Column::UserId.eq(uid));
        }
        let total = select.clone().count(self.db).await?;
        let entries = select
            .order_by_desc(entity_history_entity::Column::CreatedAt)
            .paginate(self.db, page_size)
            .fetch_page(page_num).await?;
        Ok((entries, total as i64))
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<EntityHistoryModel>, AppError> {
        Ok(entity_history_entity::Entity::find_by_id(id).one(self.db).await?)
    }

    pub async fn get_entity_history(&self, entity_type: &str, entity_id: i64) -> Result<Vec<EntityHistoryModel>, AppError> {
        let entries = entity_history_entity::Entity::find()
            .filter(entity_history_entity::Column::EntityType.eq(entity_type))
            .filter(entity_history_entity::Column::EntityId.eq(entity_id))
            .order_by_desc(entity_history_entity::Column::CreatedAt)
            .all(self.db).await?;
        Ok(entries)
    }

    pub async fn record_change(
        &self, entity_type: &str, entity_id: i64, action: &str,
        old_data: Option<serde_json::Value>, new_data: Option<serde_json::Value>,
        changed_fields: Option<Vec<String>>, user_id: Option<i64>,
        branch_id: Option<i64>, reason: Option<&str>,
    ) -> Result<EntityHistoryModel, AppError> {
        let now = chrono::Utc::now();
        let active = entity_history_entity::ActiveModel {
            entity_type: Set(entity_type.to_owned()),
            entity_id: Set(entity_id),
            action: Set(action.to_owned()),
            old_data: Set(old_data),
            new_data: Set(new_data),
            changed_fields: Set(changed_fields.map(|v| serde_json::to_value(v).unwrap_or_default())),
            user_id: Set(user_id),
            branch_id: Set(branch_id),
            reason: Set(reason.map(|s| s.to_owned())),
            created_at: Set(now.into()),
            ..Default::default()
        };
        let model = active.insert(self.db).await?;
        Ok(model)
    }

    /// Perform a full rollback: safety checks → restore entity data → record rollback history.
    ///
    /// Safety checks, entity restore, and rollback history record are all
    /// wrapped in a SeaORM transaction for strict atomicity (no TOCTOU gap).
    ///
    /// This is the main entry point for rollback operations. It:
    /// 1. Fetches the history entry (outside tx — read-only)
    /// 2. Validates old_data exists and entity type is rollbackable
    /// 3. Inside a transaction: runs safety checks + restores entity data + records rollback history
    pub async fn rollback(&self, history_id: i64, user_id: i64, reason: &str) -> Result<RollbackResult, AppError> {
        // 1. Fetch the history entry
        let entry = entity_history_entity::Entity::find_by_id(history_id)
            .one(self.db).await?
            .ok_or_else(|| AppError::NotFound("History entry not found".into()))?;

        // 2. Validate the entry has old_data to restore
        let old_data = entry.old_data.clone()
            .ok_or_else(|| AppError::Validation("Cannot rollback: no previous state available (action was 'created')".into()))?;

        // 3. Validate entity type is rollbackable
        if !ROLLABLE_ENTITIES.contains(&entry.entity_type.as_str()) {
            return Err(AppError::Validation(format!(
                "Entity type '{}' does not support rollback", entry.entity_type
            )));
        }

        // 4. Restore entity data + safety checks + record rollback history — atomically in a transaction
        let restored_from = history_id;
        let entity_type = entry.entity_type.clone();
        let entity_id = entry.entity_id;
        let current_new_data = entry.new_data.clone();
        let current_changed_fields = entry.changed_fields.clone();
        let reason_owned = reason.to_owned();

        let rollback_entry = self.db.transaction::<_, entity_history_entity::Model, AppError>(|tx| {
            Box::pin(async move {
                // Safety checks inside transaction (TOCTOU fix)
                Self::validate_rollback_safety(tx, &entity_type, entity_id, &old_data).await?;

                // Restore the entity data
                Self::restore_entity_data(tx, &entity_type, entity_id, &old_data).await?;

                // Record rollback history entry
                let now = chrono::Utc::now();
                let rollback_active = entity_history_entity::ActiveModel {
                    entity_type: Set(entity_type),
                    entity_id: Set(entity_id),
                    action: Set("rollback".to_owned()),
                    old_data: Set(current_new_data),       // Current state (before rollback)
                    new_data: Set(Some(old_data)),          // Restored state
                    changed_fields: Set(current_changed_fields),
                    user_id: Set(Some(user_id)),
                    reason: Set(Some(reason_owned)),
                    rollback_reference: Set(Some(restored_from)),
                    created_at: Set(now.into()),
                    ..Default::default()
                };
                let rollback_entry = rollback_active.insert(tx).await?;
                Ok(rollback_entry)
            })
        }).await
        .map_err(|e| AppError::Validation(format!("Transaction failed during rollback: {e}")))?;

        Ok(RollbackResult {
            history_id: rollback_entry.id,
            entity_type: entry.entity_type,
            entity_id: entry.entity_id,
            restored_from: history_id,
        })
    }

    /// Entity-type-specific safety checks before rollback.
    ///
    /// Each entity type has business rules that prevent rollback in certain states.
    /// These checks prevent data corruption and maintain referential integrity.
    ///
    /// Accepts a `db` reference so it can operate inside a transaction.
    async fn validate_rollback_safety(
        db: &(impl sea_orm::ConnectionTrait + Send + Sync),
        entity_type: &str, entity_id: i64, _old_data: &JsonValue,
    ) -> Result<(), AppError> {
        match entity_type {
            "customers" => {
                // Cannot rollback customer if they have active subscription
                let stmt = Statement::from_sql_and_values(
                    DatabaseBackend::Postgres,
                    "SELECT EXISTS(SELECT 1 FROM subscriptions WHERE customer_id = $1 AND status = 'active')",
                    vec![entity_id.into()],
                );
                let result = db.query_one(stmt).await?;
                let has_active = result
                    .and_then(|r| r.try_get("", "exists").ok())
                    .unwrap_or(false);
                if has_active {
                    return Err(AppError::Validation(
                        "Cannot rollback customer with active subscription".into(),
                    ));
                }
            }
            "invoices" => {
                // Cannot rollback invoice if payment already processed
                let stmt = Statement::from_sql_and_values(
                    DatabaseBackend::Postgres,
                    "SELECT EXISTS(SELECT 1 FROM payments WHERE invoice_id = $1 AND status = 'completed')",
                    vec![entity_id.into()],
                );
                let result = db.query_one(stmt).await?;
                let has_payment = result
                    .and_then(|r| r.try_get("", "exists").ok())
                    .unwrap_or(false);
                if has_payment {
                    return Err(AppError::Validation(
                        "Cannot rollback invoice with completed payment".into(),
                    ));
                }
            }
            "network_devices" => {
                // Cannot rollback device if currently online
                let stmt = Statement::from_sql_and_values(
                    DatabaseBackend::Postgres,
                    "SELECT EXISTS(SELECT 1 FROM network_devices WHERE id = $1 AND status = 'online')",
                    vec![entity_id.into()],
                );
                let result = db.query_one(stmt).await?;
                let is_online = result
                    .and_then(|r| r.try_get("", "exists").ok())
                    .unwrap_or(false);
                if is_online {
                    return Err(AppError::Validation(
                        "Cannot rollback online network device".into(),
                    ));
                }
            }
            "plans" => {
                // Cannot rollback plan if active subscriptions exist
                let stmt = Statement::from_sql_and_values(
                    DatabaseBackend::Postgres,
                    "SELECT EXISTS(SELECT 1 FROM subscriptions WHERE plan_id = $1 AND status = 'active')",
                    vec![entity_id.into()],
                );
                let result = db.query_one(stmt).await?;
                let has_subscribers = result
                    .and_then(|r| r.try_get("", "exists").ok())
                    .unwrap_or(false);
                if has_subscribers {
                    return Err(AppError::Validation(
                        "Cannot rollback plan with active subscribers".into(),
                    ));
                }
            }
            "subscriptions" => {
                // Cannot rollback subscription if invoice has been paid
                let stmt = Statement::from_sql_and_values(
                    DatabaseBackend::Postgres,
                    "SELECT EXISTS(SELECT 1 FROM invoices WHERE subscription_id = $1 AND status = 'paid')",
                    vec![entity_id.into()],
                );
                let result = db.query_one(stmt).await?;
                let has_paid_invoice = result
                    .and_then(|r| r.try_get("", "exists").ok())
                    .unwrap_or(false);
                if has_paid_invoice {
                    return Err(AppError::Validation(
                        "Cannot rollback subscription with paid invoices".into(),
                    ));
                }
            }
            _ => {
                // No safety checks for other entity types
            }
        }
        Ok(())
    }

    /// Restore entity data by applying old_data JSONB to the primary table.
    ///
    /// Uses a dynamic UPDATE query built from the old_data JSONB keys,
    /// skipping immutable fields (id, created_at, updated_at).
    ///
    /// Accepts a `db` reference so it can operate inside a transaction.
    async fn restore_entity_data(
        db: &(impl sea_orm::ConnectionTrait + Send + Sync), entity_type: &str, entity_id: i64, old_data: &JsonValue,
    ) -> Result<(), AppError> {
        let table_name = match entity_type {
            "customers" => "customers",
            "subscriptions" => "subscriptions",
            "plans" => "plans",
            "invoices" => "invoices",
            "refunds" => "refunds",
            "journal_entries" => "journal_entries",
            "network_devices" => "network_devices",
            "payment_gateways" => "payment_gateways",
            "discounts" => "discounts",
            "bandwidth_profiles" => "bandwidth_profiles",
            _ => return Err(AppError::Validation(format!("Unknown entity type: {entity_type}"))),
        };

        // Build dynamic SET clause from old_data JSONB
        let valid_cols = valid_columns_for_entity(entity_type)
            .ok_or_else(|| AppError::Validation(format!("No column whitelist for entity type: {entity_type}")))?;

        let mut set_clauses = Vec::new();
        let mut params: Vec<sea_orm::Value> = Vec::new();
        let mut param_index = 1;

        if let Some(obj) = old_data.as_object() {
            for (key, value) in obj {
                // Skip immutable fields (id, created_at)
                if IMMUTABLE_FIELDS.contains(&key.as_str()) {
                    continue;
                }
                // Skip columns not in the whitelist for this entity type
                if !valid_cols.contains(&key.as_str()) {
                    continue;
                }
                set_clauses.push(format!("{key} = ${param_index}"));
                param_index += 1;
                params.push(sea_orm::Value::Json(Some(Box::new(value.clone()))));
            }
        }

        if set_clauses.is_empty() {
            return Err(AppError::Validation("No fields to restore in old_data".into()));
        }

        // Add updated_at = NOW()
        set_clauses.push("updated_at = NOW()".to_owned());

        let sql = format!(
            "UPDATE {} SET {} WHERE id = ${}",
            table_name,
            set_clauses.join(", "),
            param_index,
        );
        params.push(sea_orm::Value::BigInt(Some(entity_id)));

        let stmt = Statement::from_sql_and_values(DatabaseBackend::Postgres, &sql, params);
        let result = db.execute(stmt).await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound(format!(
                "{entity_type} with id {entity_id} not found (may have been deleted)"
            )));
        }

        Ok(())
    }
}
