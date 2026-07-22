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
use sea_orm::{ConnectionTrait, DatabaseConnection, Statement};
use uuid::Uuid;

pub async fn set_history_context(
    db: &DatabaseConnection,
    user_id: Uuid,
    branch_id: Option<Uuid>,
    ip_address: Option<String>,
    user_agent: Option<String>,
) -> Result<()> {
    db.execute(Statement::from_string(
        db.get_database_backend(),
        format!("SELECT set_config('app.current_user_id', '{}', true)", user_id),
    ))
    .await?;

    if let Some(branch) = branch_id {
        db.execute(Statement::from_string(
            db.get_database_backend(),
            format!("SELECT set_config('app.current_branch_id', '{}', true)", branch),
        ))
        .await?;
    }

    if let Some(ip) = ip_address {
        db.execute(Statement::from_string(
            db.get_database_backend(),
            format!("SELECT set_config('app.current_ip_address', '{}', true)", ip),
        ))
        .await?;
    }

    if let Some(ua) = user_agent {
        db.execute(Statement::from_string(
            db.get_database_backend(),
            format!("SELECT set_config('app.current_user_agent', '{}', true)", ua),
        ))
        .await?;
    }

    Ok(())
}
```

---

## Rollback Logic

### Safety Checks Before Rollback

```rust
// src/services/rollback.rs
use sea_orm::{ActiveModelTrait, ColumnTrait, ConnectionTrait, DatabaseConnection, DatabaseTransaction, EntityTrait, QueryFilter, Set, Statement, TransactionTrait};
use chrono::Utc;
use uuid::Uuid;

pub struct RollbackService {
    db: DatabaseConnection,
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

        // 4. Execute restore and rollback entry creation outside closure
        let update_query = self.build_restore_query(entity_type, &old_data);
        self.db.execute(Statement::from_string(
            self.db.get_database_backend(),
            update_query,
        )).await?;

        // 5. Create rollback history entry
        let rollback_entry = self.create_rollback_entry(
            entity_type,
            entity_id,
            &history,
            admin_id,
            &reason,
        ).await?;

        // 6. Publish rollback event
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
        old_data: &prost::bytes::Bytes,
    ) -> Result<()> {
        match entity_type {
            "customers" => {
                // Cannot rollback customer if they have active subscription
                let has_active = subscription::Entity::find()
                    .filter(subscription::Column::CustomerId.eq(entity_id))
                    .filter(subscription::Column::Status.eq("active"))
                    .count(&self.db)
                    .await? > 0;

                if has_active {
                    return Err(AppError::BadRequest(
                        "Cannot rollback customer with active subscription".into()
                    ));
                }
            }
            "invoices" => {
                // Cannot rollback invoice if payment already processed
                let has_payment = payment::Entity::find()
                    .filter(payment::Column::InvoiceId.eq(entity_id))
                    .filter(payment::Column::Status.eq("completed"))
                    .count(&self.db)
                    .await? > 0;

                if has_payment {
                    return Err(AppError::BadRequest(
                        "Cannot rollback invoice with completed payment".into()
                    ));
                }
            }
            "network_devices" => {
                // Cannot rollback device if currently online
                let is_online = network_device::Entity::find()
                    .filter(network_device::Column::Id.eq(entity_id))
                    .filter(network_device::Column::Status.eq("online"))
                    .count(&self.db)
                    .await? > 0;

                if is_online {
                    return Err(AppError::BadRequest(
                        "Cannot rollback online device".into()
                    ));
                }
            }
            "plans" => {
                // Cannot rollback plan if active subscriptions exist
                let has_subscribers = subscription::Entity::find()
                    .filter(subscription::Column::PlanId.eq(entity_id))
                    .filter(subscription::Column::Status.eq("active"))
                    .count(&self.db)
                    .await? > 0;

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

    // Note: restore_entity logic is now inline in rollback_entity
    // to avoid self-capture issues in async closures.

    fn build_restore_query(&self, entity_type: &str, entity_id: Uuid, old_data: &prost::bytes::Bytes) -> String {
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
            "UPDATE {} SET {}, updated_at = NOW() WHERE id = '{}'",
            entity_type,
            set_clauses.join(", "),
            entity_id
        )
    }

    async fn create_rollback_entry(
        &self,
        entity_type: &str,
        entity_id: Uuid,
        original_history: &HistoryEntry,
        admin_id: Uuid,
        reason: &str,
    ) -> Result<HistoryEntry> {
        // Use raw SQL for dynamic entity_type table insertion
        // Note: For production, consider using a generic history entity model
        // or a trait-based approach for type-safe rollback entries.
        let query = format!(
            "INSERT INTO {}_history (entity_id, action, old_data, new_data, changed_fields, user_id, reason, rollback_reference)
             VALUES ('{}', 'rollback', '{}', '{}', '{}', '{}', '{}', '{}')
             RETURNING *",
            entity_type,
            entity_id,
            original_history.new_data.map(|d| String::from_utf8_lossy(&d).to_string()).unwrap_or_default(),
            original_history.old_data.map(|d| String::from_utf8_lossy(&d).to_string()).unwrap_or_default(),
            original_history.changed_fields.map(|d| String::from_utf8_lossy(&d).to_string()).unwrap_or_default(),
            admin_id,
            reason,
            original_history.id
        );

        self.db.execute(Statement::from_string(
            self.db.get_database_backend(),
            query,
        )).await?;

        Ok(original_history.clone())
    }
}
```

### Rollback API Endpoint

```rust
// src/api/audit/rollback.rs
pub async fn rollback_entity(
    State(state): State<AppState>,
    body: RollbackRequest,
) -> Result<RollbackResult> {
    // Check RBAC permission
    require_permission(&state, &body.user_id, "audit.entity_history.rollback").await?;

    let result = state.rollback_service.rollback_entity(
        &body.entity_type,
        body.entity_id,
        body.history_id,
        body.user_id,
        body.reason,
    ).await?;

    Ok(proto_response(result))
}

#[derive(prost::Message)]
pub struct RollbackRequest {
    #[prost(string, tag = "1")]
    pub entity_id: String,
    #[prost(string, tag = "2")]
    pub reason: String,
}
```

---

## History Query Service

### Search History

```rust
use sea_orm::{ConnectionTrait, DatabaseConnection, Statement, FromJson, QueryResult};
use chrono::NaiveDate;
use uuid::Uuid;

#[derive(Debug, Clone, FromJson, serde::Deserialize, serde::Serialize)]
pub struct HistoryEntry {
    pub id: Uuid,
    pub entity_id: Uuid,
    pub action: String,
    pub old_data: Option<prost::bytes::Bytes>,
    pub new_data: Option<prost::bytes::Bytes>,
    pub changed_fields: Option<Vec<String>>,
    pub user_id: Option<Uuid>,
    pub user_name: Option<String>,
    pub user_email: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

pub struct HistoryQueryService {
    db: DatabaseConnection,
}

impl HistoryQueryService {
    pub async fn search_history(
        &self,
        entity_type: &str,
        entity_id: Option<Uuid>,
        action: Option<&str>,
        user_id: Option<Uuid>,
        from: Option<NaiveDate>,
        to: Option<NaiveDate>,
        page: i64,
        limit: i64,
    ) -> Result<PaginatedResult<HistoryEntry>> {
        let table = format!("{}_history", entity_type);

        // Build WHERE conditions dynamically
        let mut conditions = Vec::new();
        let mut params: Vec<String> = Vec::new();

        if let Some(eid) = entity_id {
            conditions.push(format!("h.entity_id = '{}'", eid));
        }
        if let Some(a) = action {
            conditions.push(format!("h.action = '{}'", a));
        }
        if let Some(uid) = user_id {
            conditions.push(format!("h.user_id = '{}'", uid));
        }
        if let Some(f) = from {
            conditions.push(format!("h.created_at >= '{}'", f));
        }
        if let Some(t) = to {
            conditions.push(format!("h.created_at <= '{}'", t));
        }

        let where_clause = if conditions.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", conditions.join(" AND "))
        };

        // Get total count
        let count_query = format!("SELECT COUNT(*) as count FROM {} h {}", table, where_clause);
        let count_result = self.db.query_one(Statement::from_string(
            self.db.get_database_backend(),
            count_query,
        )).await?.ok_or_else(|| AppError::Internal(anyhow::anyhow!("Count query failed")))?;
        let total: i64 = count_result.try_get("", "count")?;

        // Get paginated results with user join
        let offset = (page - 1) * limit;
        let query = format!(
            "SELECT h.*, u.name as user_name, u.email as user_email
             FROM {} h
             LEFT JOIN users u ON h.user_id = u.id
             {}
             ORDER BY h.created_at DESC
             LIMIT {} OFFSET {}",
            table, where_clause, limit, offset
        );

        let results = self.db.query_all(Statement::from_string(
            self.db.get_database_backend(),
            query,
        )).await?;

        let entries: Vec<HistoryEntry> = results.iter().map(|row| {
            HistoryEntry {
                id: row.try_get("", "id").unwrap_or_default(),
                entity_id: row.try_get("", "entity_id").unwrap_or_default(),
                action: row.try_get("", "action").unwrap_or_default(),
                old_data: row.try_get("", "old_data").ok(),
                new_data: row.try_get("", "new_data").ok(),
                changed_fields: row.try_get("", "changed_fields").ok(),
                user_id: row.try_get("", "user_id").ok(),
                user_name: row.try_get("", "user_name").ok(),
                user_email: row.try_get("", "user_email").ok(),
                created_at: row.try_get("", "created_at").unwrap_or_default(),
            }
        }).collect();

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
use sea_orm::{ConnectionTrait, DatabaseConnection, Statement};

pub async fn run_history_cleanup(db: &DatabaseConnection) -> Result<()> {
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

        let result = db.execute(Statement::from_string(
            db.get_database_backend(),
            query,
        )).await?;

        tracing::info!(
            "Cleaned up {} history: {} rows affected",
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
use sea_orm::{ConnectionTrait, DatabaseConnection, Statement};
use chrono::Utc;

pub async fn create_monthly_partitions(db: &DatabaseConnection) -> Result<()> {
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

    let next_month = Utc::now()
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

        db.execute(Statement::from_string(
            db.get_database_backend(),
            query,
        )).await?;
        tracing::info!("Created partition: {}", partition_name);
    }

    Ok(())
}
```

### Compress Old Partitions

```rust
// src/jobs/partition_compression.rs
use sea_orm::{ConnectionTrait, DatabaseConnection, Statement};

pub async fn compress_old_partitions(db: &DatabaseConnection) -> Result<()> {
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
    ];        for (table, _compress_after_days) in tables_to_compress {
        // Note: VACUUM cannot use WHERE clause or run inside a transaction.
        // Run VACUUM FULL outside of SeaORM transactions, typically via
        // a separate maintenance script or pg_cron job:
        //   VACUUM FULL (FREEZE) {table};
        // This is a DDL-level operation that must be executed directly.
        tracing::info!(
            "Compress old partition: {} (run VACUUM FULL outside app)",
            table
        );
    }

    Ok(())
}
```

---

## API Endpoints

> **API Convention:** Protobuf-first. No GET, no PUT, no path variables, no query strings. See `API-CONVENTIONS.md`.

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/api/v1/audit/entity-history/list` | `POST` | Search entity history (paginated) |
| `/api/v1/audit/entity-history/get` | `POST` | Get specific history entry |
| `/api/v1/audit/entity-history/entity-list` | `POST` | Get all history for entity |
| `/api/v1/audit/entity-history/rollback` | `POST` | Rollback to this state |
| `/api/v1/audit/entity-history/compare` | `POST` | Compare old vs new state |
| `/api/v1/audit/entity-history/export` | `POST` | Export entity history |

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
