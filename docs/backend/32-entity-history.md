# AeroXe Backend — Entity History & Rollback Module

## Overview

Critical entities use a companion `_history` table to track all changes (created, updated, deleted, status_changed) with JSONB snapshots of old and new data. Supports safe rollback to any previous state. History tables are partitioned and have configurable retention policies with compression and archival.

---

## Database Tables

### History Table Schema (Template)

Every tracked entity gets a companion table following this pattern:

```sql
CREATE TABLE {entity}_history (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    entity_id UUID NOT NULL,
    action VARCHAR(50) NOT NULL,
    -- 'created', 'updated', 'deleted', 'status_changed', 'rollback'
    old_data JSONB,              -- snapshot before change (null on create)
    new_data JSONB,              -- snapshot after change (null on delete)
    changed_fields TEXT[],       -- array of changed field names
    user_id UUID REFERENCES users(id),
    branch_id UUID REFERENCES branches(id),
    ip_address INET,
    user_agent TEXT,
    reason TEXT,                 -- optional reason for change
    rollback_reference UUID,     -- links to original change if this is a rollback
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes
CREATE INDEX idx_{entity}_history_entity ON {entity}_history(entity_id);
CREATE INDEX idx_{entity}_history_action ON {entity}_history(action);
CREATE INDEX idx_{entity}_history_user ON {entity}_history(user_id);
CREATE INDEX idx_{entity}_history_created ON {entity}_history(created_at);
```

### Complete History Tables List

| # | Entity | History Table | Retention | Compress After | Partition |
|---|--------|---------------|-----------|----------------|-----------|
| 1 | Customers | `customers_history` | 7 years | 1 year | Monthly |
| 2 | Subscriptions | `subscriptions_history` | 7 years | 1 year | Monthly |
| 3 | Plans | `plans_history` | 7 years | 1 year | Monthly |
| 4 | Invoices | `invoices_history` | 7 years | 1 year | Monthly |
| 5 | Refunds | `refunds_history` | 7 years | 1 year | Monthly |
| 6 | Journal Entries | `journal_entries_history` | 7 years | 1 year | Monthly |
| 7 | Manual Payments | `manual_payments_history` | 7 years | 1 year | Monthly |
| 8 | Network Devices | `network_devices_history` | 3 years | 6 months | Monthly |
| 9 | Payment Gateways | `payment_gateways_history` | 3 years | 6 months | Monthly |
| 10 | Discounts | `discounts_history` | 3 years | 6 months | Monthly |
| 11 | Approval Requests | `approval_requests_history` | 3 years | 6 months | Monthly |
| 12 | Bandwidth Profiles | `bandwidth_profiles_history` | 2 years | 6 months | Monthly |

### Partitioned Table Example (Customers)

```sql
-- Main table is partitioned by created_at
CREATE TABLE customers_history (
    id UUID DEFAULT gen_random_uuid(),
    entity_id UUID NOT NULL,
    action VARCHAR(50) NOT NULL,
    old_data JSONB,
    new_data JSONB,
    changed_fields TEXT[],
    user_id UUID,
    branch_id UUID,
    ip_address INET,
    user_agent TEXT,
    reason TEXT,
    rollback_reference UUID,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (id, created_at)
) PARTITION BY RANGE (created_at);

-- Monthly partitions
CREATE TABLE customers_history_2026_01 PARTITION OF customers_history
    FOR VALUES FROM ('2026-01-01') TO ('2026-02-01');
CREATE TABLE customers_history_2026_02 PARTITION OF customers_history
    FOR VALUES FROM ('2026-02-01') TO ('2026-03-01');
-- ... auto-created by pg_cron
```

---

## PostgreSQL Trigger Function

### Generic History Trigger

```sql
-- Create a reusable function that captures changes
CREATE OR REPLACE FUNCTION fn_capture_history()
RETURNS TRIGGER AS $$
DECLARE
    v_old_data JSONB;
    v_new_data JSONB;
    v_action TEXT;
    v_changed_fields TEXT[];
    v_entity_id UUID;
BEGIN
    -- Determine action and data
    IF TG_OP = 'INSERT' THEN
        v_action := 'created';
        v_old_data := NULL;
        v_new_data := to_jsonb(NEW);
        v_entity_id := NEW.id;
    ELSIF TG_OP = 'UPDATE' THEN
        v_action := 'updated';
        v_old_data := to_jsonb(OLD);
        v_new_data := to_jsonb(NEW);
        v_entity_id := NEW.id;

        -- Detect status changes
        IF OLD.status IS DISTINCT FROM NEW.status THEN
            v_action := 'status_changed';
        END IF;

        -- Compute changed fields
        SELECT array_agg(key) INTO v_changed_fields
        FROM jsonb_each(v_new_data) new_data
        WHERE new_data.value IS DISTINCT FROM (v_old_data ->> new_data.key)::jsonb;
    ELSIF TG_OP = 'DELETE' THEN
        v_action := 'deleted';
        v_old_data := to_jsonb(OLD);
        v_new_data := NULL;
        v_entity_id := OLD.id;
    END IF;

    -- Insert into the history table
    EXECUTE format(
        'INSERT INTO %I (entity_id, action, old_data, new_data, changed_fields, user_id, branch_id, ip_address, user_agent, reason)
         VALUES ($1, $2, $3, $4, $5, current_setting(''app.current_user_id'')::uuid,
                 current_setting(''app.current_branch_id'')::uuid,
                 inet_client_addr(),
                 current_setting(''app.current_user_agent''),
                 current_setting(''app.current_reason''))',
        TG_TABLE_NAME || '_history'
    )
    USING v_entity_id, v_action, v_old_data, v_new_data, v_changed_fields;

    RETURN COALESCE(NEW, OLD);
END;
$$ LANGUAGE plpgsql;
```

### Apply Trigger to Each Table

```sql
-- Customers
CREATE TRIGGER trg_customers_history
    AFTER INSERT OR UPDATE OR DELETE ON customers
    FOR EACH ROW EXECUTE FUNCTION fn_capture_history();

-- Subscriptions
CREATE TRIGGER trg_subscriptions_history
    AFTER INSERT OR UPDATE OR DELETE ON subscriptions
    FOR EACH ROW EXECUTE FUNCTION fn_capture_history();

-- Plans
CREATE TRIGGER trg_plans_history
    AFTER INSERT OR UPDATE OR DELETE ON plans
    FOR EACH ROW EXECUTE FUNCTION fn_capture_history();

-- Invoices
CREATE TRIGGER trg_invoices_history
    AFTER INSERT OR UPDATE OR DELETE ON invoices
    FOR EACH ROW EXECUTE FUNCTION fn_capture_history();

-- Refunds
CREATE TRIGGER trg_refunds_history
    AFTER INSERT OR UPDATE OR DELETE ON refunds
    FOR EACH ROW EXECUTE FUNCTION fn_capture_history();

-- Journal Entries
CREATE TRIGGER trg_journal_entries_history
    AFTER INSERT OR UPDATE OR DELETE ON journal_entries
    FOR EACH ROW EXECUTE FUNCTION fn_capture_history();

-- Network Devices
CREATE TRIGGER trg_network_devices_history
    AFTER INSERT OR UPDATE OR DELETE ON network_devices
    FOR EACH ROW EXECUTE FUNCTION fn_capture_history();

-- (Apply to remaining 5 tables similarly)
```

### Setting Context Variables (Rust Axum)

Before any database operation, set the context variables so the trigger can capture who made the change:

```rust
// src/middleware/history_context.rs
use axum::extract::Request;
use tower_http::trace::MakeSpan;

pub async fn set_history_context(
    pool: &PgPool,
    user_id: Uuid,
    branch_id: Option<Uuid>,
    ip_address: Option<String>,
    user_agent: Option<String>,
) -> Result<()> {
    sqlx::query("SELECT set_config('app.current_user_id', $1, true)")
        .bind(user_id.to_string())
        .execute(pool).await?;

    if let Some(branch) = branch_id {
        sqlx::query("SELECT set_config('app.current_branch_id', $1, true)")
            .bind(branch.to_string())
            .execute(pool).await?;
    }

    if let Some(ip) = ip_address {
        sqlx::query("SELECT set_config('app.current_ip_address', $1, true)")
            .bind(ip)
            .execute(pool).await?;
    }

    if let Some(ua) = user_agent {
        sqlx::query("SELECT set_config('app.current_user_agent', $1, true)")
            .bind(ua)
            .execute(pool).await?;
    }

    Ok(())
}
```

---

## Rollback Logic

### Safety Checks Before Rollback

```rust
// src/services/rollback.rs
pub struct RollbackService {
    pool: PgPool,
}

impl RollbackService {
    pub async fn rollback_entity(
        &self,
        entity_type: &str,
        entity_id: Uuid,
        history_id: Uuid,
        admin_id: Uuid,
        reason: String,
    ) -> Result<RollbackResult> {
        // 1. Fetch the history entry
        let history = self.fetch_history_entry(entity_type, history_id).await?;

        // 2. Validate the entry has old_data to restore
        let old_data = history.old_data.ok_or_else(|| {
            AppError::BadRequest("Cannot rollback: no previous state available".into())
        })?;

        // 3. Run safety checks
        self.validate_rollback_safety(entity_type, entity_id, &old_data).await?;

        // 4. Begin transaction
        let mut tx = self.pool.begin().await?;

        // 5. Apply old_data back to primary table
        self.restore_entity(&mut tx, entity_type, entity_id, &old_data).await?;

        // 6. Create rollback history entry
        let rollback_entry = self.create_rollback_entry(
            &mut tx,
            entity_type,
            entity_id,
            &history,
            admin_id,
            &reason,
        ).await?;

        // 7. Commit
        tx.commit().await?;

        // 8. Publish rollback event
        self.publish_rollback_event(entity_type, entity_id, &rollback_entry).await?;

        Ok(RollbackResult {
            history_id: rollback_entry.id,
            entity_type: entity_type.to_string(),
            entity_id,
            restored_from: history_id,
        })
    }

    async fn validate_rollback_safety(
        &self,
        entity_type: &str,
        entity_id: Uuid,
        old_data: &serde_json::Value,
    ) -> Result<()> {
        match entity_type {
            "customers" => {
                // Cannot rollback customer if they have active subscription
                let has_active = sqlx::query_scalar::<_, bool>(
                    "SELECT EXISTS(SELECT 1 FROM subscriptions WHERE customer_id = $1 AND status = 'active')"
                )
                .bind(entity_id)
                .fetch_one(&self.pool).await?;

                if has_active {
                    return Err(AppError::BadRequest(
                        "Cannot rollback customer with active subscription".into()
                    ));
                }
            }
            "invoices" => {
                // Cannot rollback invoice if payment already processed
                let has_payment = sqlx::query_scalar::<_, bool>(
                    "SELECT EXISTS(SELECT 1 FROM payments WHERE invoice_id = $1 AND status = 'completed')"
                )
                .bind(entity_id)
                .fetch_one(&self.pool).await?;

                if has_payment {
                    return Err(AppError::BadRequest(
                        "Cannot rollback invoice with completed payment".into()
                    ));
                }
            }
            "network_devices" => {
                // Cannot rollback device if currently online
                let is_online = sqlx::query_scalar::<_, bool>(
                    "SELECT EXISTS(SELECT 1 FROM network_devices WHERE id = $1 AND status = 'online')"
                )
                .bind(entity_id)
                .fetch_one(&self.pool).await?;

                if is_online {
                    return Err(AppError::BadRequest(
                        "Cannot rollback online device".into()
                    ));
                }
            }
            "plans" => {
                // Cannot rollback plan if active subscriptions exist
                let has_subscribers = sqlx::query_scalar::<_, bool>(
                    "SELECT EXISTS(SELECT 1 FROM subscriptions WHERE plan_id = $1 AND status = 'active')"
                )
                .bind(entity_id)
                .fetch_one(&self.pool).await?;

                if has_subscribers {
                    return Err(AppError::BadRequest(
                        "Cannot rollback plan with active subscribers".into()
                    ));
                }
            }
            _ => {}
        }

        Ok(())
    }

    async fn restore_entity(
        &self,
        tx: &mut PgTransaction<'_>,
        entity_type: &str,
        entity_id: Uuid,
        old_data: &serde_json::Value,
    ) -> Result<()> {
        // Build dynamic UPDATE query from old_data JSONB
        let update_query = self.build_restore_query(entity_type, old_data);

        sqlx::query(&update_query)
            .bind(entity_id)
            .execute(&mut **tx).await?;

        Ok(())
    }

    fn build_restore_query(&self, entity_type: &str, old_data: &serde_json::Value) -> String {
        // Dynamically build UPDATE SET clause from JSONB keys
        let mut set_clauses = Vec::new();

        if let Some(obj) = old_data.as_object() {
            for (key, value) in obj {
                if key == "id" || key == "created_at" || key == "updated_at" {
                    continue; // Skip immutable fields
                }
                set_clauses.push(format!("{} = '{}'", key, value));
            }
        }

        format!(
            "UPDATE {} SET {}, updated_at = NOW() WHERE id = $1",
            entity_type,
            set_clauses.join(", ")
        )
    }

    async fn create_rollback_entry(
        &self,
        tx: &mut PgTransaction<'_>,
        entity_type: &str,
        entity_id: Uuid,
        original_history: &HistoryEntry,
        admin_id: Uuid,
        reason: &str,
    ) -> Result<HistoryEntry> {
        let entry = sqlx::query_as::<_, HistoryEntry>(&format!(
            "INSERT INTO {}_history (entity_id, action, old_data, new_data, changed_fields, user_id, reason, rollback_reference)
             VALUES ($1, 'rollback', $2, $3, $4, $5, $6, $7)
             RETURNING *",
            entity_type
        ))
        .bind(entity_id)
        .bind(&original_history.new_data) // Current state (before rollback)
        .bind(&original_history.old_data) // Restored state
        .bind(&original_history.changed_fields)
        .bind(admin_id)
        .bind(reason)
        .bind(original_history.id)
        .fetch_one(&mut **tx).await?;

        Ok(entry)
    }
}
```

### Rollback API Endpoint

```rust
// src/api/audit/rollback.rs
pub async fn rollback_entity(
    State(state): State<AppState>,
    Path((entity_type, history_id)): Path<(String, Uuid)>,
    Json(payload): Json<RollbackRequest>,
) -> Result<Json<RollbackResult>> {
    // Check RBAC permission
    require_permission(&state, &payload.user_id, "audit.entity_history.rollback").await?;

    let result = state.rollback_service.rollback_entity(
        &entity_type,
        payload.entity_id,
        history_id,
        payload.user_id,
        payload.reason,
    ).await?;

    Ok(Json(result))
}

#[derive(Deserialize)]
pub struct RollbackRequest {
    pub entity_id: Uuid,
    pub reason: String,
}
```

---

## History Query Service

### Search History

```rust
pub struct HistoryQueryService {
    pool: PgPool,
}

impl HistoryQueryService {
    pub async fn search_history(
        &self,
        entity_type: &str,
        entity_id: Option<Uuid>,
        action: Option<&str>,
        user_id: Option<Uuid>,
        from: Option<chrono::NaiveDate>,
        to: Option<chrono::NaiveDate>,
        page: i64,
        limit: i64,
    ) -> Result<PaginatedResult<HistoryEntry>> {
        let table = format!("{}_history", entity_type);

        let mut query = format!(
            "SELECT h.*, u.name as user_name, u.email as user_email
             FROM {} h
             LEFT JOIN users u ON h.user_id = u.id
             WHERE 1=1",
            table
        );

        let mut count_query = format!("SELECT COUNT(*) FROM {} WHERE 1=1", table);

        let mut params: Vec<Box<dyn PgArguments + Send + Sync>> = Vec::new();
        let mut param_index = 1;

        if let Some(eid) = entity_id {
            query.push_str(&format!(" AND h.entity_id = ${}", param_index));
            count_query.push_str(&format!(" AND entity_id = ${}", param_index));
            params.push(Box::new(eid));
            param_index += 1;
        }

        if let Some(a) = action {
            query.push_str(&format!(" AND h.action = ${}", param_index));
            count_query.push_str(&format!(" AND action = ${}", param_index));
            params.push(Box::new(a.to_string()));
            param_index += 1;
        }

        if let Some(uid) = user_id {
            query.push_str(&format!(" AND h.user_id = ${}", param_index));
            count_query.push_str(&format!(" AND user_id = ${}", param_index));
            params.push(Box::new(uid));
            param_index += 1;
        }

        if let Some(f) = from {
            query.push_str(&format!(" AND h.created_at >= ${}", param_index));
            count_query.push_str(&format!(" AND created_at >= ${}", param_index));
            params.push(Box::new(f));
            param_index += 1;
        }

        if let Some(t) = to {
            query.push_str(&format!(" AND h.created_at <= ${}", param_index));
            count_query.push_str(&format!(" AND created_at <= ${}", param_index));
            params.push(Box::new(t));
            param_index += 1;
        }

        // Get total count
        let total: i64 = sqlx::query_scalar(&count_query)
            .bind_all(&params)
            .fetch_one(&self.pool).await?;

        // Get paginated results
        query.push_str(&format!(" ORDER BY h.created_at DESC LIMIT {} OFFSET {}", limit, (page - 1) * limit));

        let entries = sqlx::query_as::<_, HistoryEntry>(&query)
            .bind_all(&params)
            .fetch_all(&self.pool).await?;

        Ok(PaginatedResult {
            items: entries,
            total,
            page,
            limit,
            total_pages: (total as f64 / limit as f64).ceil() as i64,
        })
    }
}
```

---

## Retention Jobs

### Cleanup Job (Daily at 2 AM)

```rust
// src/jobs/history_cleanup.rs
pub async fn run_history_cleanup(pool: &PgPool) -> Result<()> {
    let retention_policies = vec![
        ("customers_history", 2555),           // 7 years
        ("subscriptions_history", 2555),       // 7 years
        ("plans_history", 2555),               // 7 years
        ("invoices_history", 2555),            // 7 years
        ("refunds_history", 2555),             // 7 years
        ("journal_entries_history", 2555),     // 7 years
        ("manual_payments_history", 2555),     // 7 years
        ("network_devices_history", 1095),     // 3 years
        ("payment_gateways_history", 1095),    // 3 years
        ("discounts_history", 1095),           // 3 years
        ("approval_requests_history", 1095),   // 3 years
        ("bandwidth_profiles_history", 730),   // 2 years
    ];

    for (table, retention_days) in retention_policies {
        let query = format!(
            "DELETE FROM {} WHERE created_at < NOW() - INTERVAL '{} days'",
            table, retention_days
        );

        let result = sqlx::query(&query).execute(pool).await?;
        tracing::info!(
            "Cleaned up {} history: {} rows deleted",
            table,
            result.rows_affected()
        );
    }

    Ok(())
}
```

### Partition Management (Monthly)

```rust
// src/jobs/partition_maintenance.rs
pub async fn create_monthly_partitions(pool: &PgPool) -> Result<()> {
    let tables = vec![
        "customers_history",
        "subscriptions_history",
        "plans_history",
        "invoices_history",
        "refunds_history",
        "journal_entries_history",
        "manual_payments_history",
        "network_devices_history",
        "payment_gateways_history",
        "discounts_history",
        "approval_requests_history",
        "bandwidth_profiles_history",
    ];

    let next_month = chrono::Utc::now()
        .date_naive()
        .with_day(1)
        .unwrap()
        .checked_add_months(chrono::Months::new(1))
        .unwrap();

    let partition_name_suffix = next_month.format("%Y_%m");
    let partition_start = next_month.format("%Y-%m-01");
    let next_next_month = next_month
        .checked_add_months(chrono::Months::new(1))
        .unwrap();
    let partition_end = next_next_month.format("%Y-%m-01");

    for table in tables {
        let partition_name = format!("{}_{}", table, partition_name_suffix);

        let query = format!(
            "CREATE TABLE IF NOT EXISTS {} PARTITION OF {} FOR VALUES FROM ('{}') TO ('{}')",
            partition_name, table, partition_start, partition_end
        );

        sqlx::query(&query).execute(pool).await?;
        tracing::info!("Created partition: {}", partition_name);
    }

    Ok(())
}
```

### Compress Old Partitions

```rust
pub async fn compress_old_partitions(pool: &PgPool) -> Result<()> {
    // Compress partitions older than compression threshold
    // Using TOAST compression (automatic in PostgreSQL)

    let tables_to_compress = vec![
        ("customers_history", 365),            // 1 year
        ("subscriptions_history", 365),
        ("plans_history", 365),
        ("invoices_history", 365),
        ("refunds_history", 365),
        ("journal_entries_history", 365),
        ("manual_payments_history", 365),
        ("network_devices_history", 180),      // 6 months
        ("payment_gateways_history", 180),
        ("discounts_history", 180),
        ("approval_requests_history", 180),
        ("bandwidth_profiles_history", 180),
    ];

    for (table, compress_after_days) in tables_to_compress {
        // VACUUM FULL on old partitions for compression
        let query = format!(
            "VACUUM FULL (FREEZE) {} WHERE created_at < NOW() - INTERVAL '{} days'",
            table, compress_after_days
        );

        // Note: VACUUM cannot be run inside a transaction
        sqlx::raw_sql(&query).execute(pool).await?;
        tracing::info!("Compressed old partition: {}", table);
    }

    Ok(())
}
```

---

## API Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/api/v1/audit/entity-history` | GET | Search entity history (paginated) |
| `/api/v1/audit/entity-history/:id` | GET | Get specific history entry |
| `/api/v1/audit/entity-history/:entity/:id` | GET | Get all history for entity |
| `/api/v1/audit/entity-history/:id/rollback` | POST | Rollback to this state |
| `/api/v1/audit/entity-history/compare` | GET | Compare old vs new state |
| `/api/v1/audit/entity-history/export` | GET | Export entity history |

---

## RBAC Permissions

| Permission | Roles | Description |
|------------|-------|-------------|
| `audit.entity_history.view` | finance_manager, super_admin | View entity history |
| `audit.entity_history.rollback` | super_admin only | Rollback entity to previous state |
| `audit.entity_history.export` | finance_manager, super_admin | Export entity history |

---

## Events

| Event | Payload | Trigger |
|-------|---------|---------|
| `entity.{type}.created` | `{ entity_id, new_data }` | Entity created |
| `entity.{type}.updated` | `{ entity_id, old_data, new_data, changed_fields }` | Entity updated |
| `entity.{type}.deleted` | `{ entity_id, old_data }` | Entity deleted |
| `entity.{type}.status_changed` | `{ entity_id, old_status, new_status }` | Status changed |
| `entity.{type}.rollback` | `{ entity_id, restored_from, history_id, admin_id }` | Rollback performed |
