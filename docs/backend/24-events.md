# AeroXe Backend — Events Module

> **Req Ref:** §12 Event Sourcing Design, §15 NATS Event Architecture

---

## 1. Overview

Implements event sourcing using NATS JetStream for durable, exactly-once message delivery. All inter-module communication happens through events. The event store provides a complete audit trail of every state change in the system.

> **v3.0 Gap Reference:** `GAP-architecture-patterns.md` §P-07 (no API/webhook retry+DLQ for external HTTP calls — outbox exists for domain events but not API ops), §P-15 (no worker job DLQ — bad record blocks entire queue). See also `24-events.md` §8 for full outbox pattern docs.

## 2. Architecture

```
Module publishes event → NATS JetStream (durable stream)
    ↓
Event Store (persisted to PostgreSQL)
    ↓
Subscribers (other modules, background workers)
    ├── Bandwidth Engine
    ├── Notification Service
    ├── Billing Engine
    ├── Audit Logger
    └── WebSocket Broadcaster
```

## 3. Database Tables

```sql
CREATE TABLE events (
    id BIGSERIAL PRIMARY KEY,
    event_id UUID NOT NULL DEFAULT gen_random_uuid(),
    event_type VARCHAR(100) NOT NULL,
    aggregate_type VARCHAR(50) NOT NULL,
    aggregate_id BIGINT NOT NULL,
    payload JSONB NOT NULL,
    metadata JSONB,
    caused_by_user_id BIGINT REFERENCES users(id),
    caused_by_branch_id BIGINT REFERENCES branches(id),
    sequence_number BIGSERIAL,
    published_at TIMESTAMPTZ DEFAULT NOW(),
    processed BOOLEAN DEFAULT FALSE
) PARTITION BY RANGE (published_at);

CREATE TABLE event_subscriptions (
    id BIGSERIAL PRIMARY KEY,
    subscriber_name VARCHAR(100) NOT NULL,
    event_type VARCHAR(100) NOT NULL,
    last_processed_id BIGINT DEFAULT 0,
    last_processed_at TIMESTAMPTZ,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(subscriber_name, event_type)
);
```

## 4. NATS JetStream Configuration

```rust
// Stream configuration
pub struct StreamConfig {
    pub name: String,           // "EVENTS"
    pub subjects: Vec<String>,  // ["events.>"]
    pub retention: String,      // "limits"
    pub max_msgs: i64,          // 1_000_000
    pub max_bytes: i64,         // 1GB
    pub storage: String,        // "file"
    pub discard: String,        // "old"
    pub max_age: i64,           // 30 days (in nanos)
}
```

## 5. Event Publishing

```rust
pub struct EventPublisher {
    nats: async_nats::Client,
    db: DatabaseConnection,
}

impl EventPublisher {
    pub async fn publish(
        &self,
        event_type: &str,
        aggregate_type: &str,
        aggregate_id: i64,
        payload: prost::bytes::Bytes,
        user_id: Option<i64>,
        branch_id: Option<i64>,
    ) -> Result<()> {
        // 1. Persist to event store (outbox pattern)
        let event = self.db.insert_event(InsertEvent {
            event_type: event_type.to_string(),
            aggregate_type: aggregate_type.to_string(),
            aggregate_id,
            payload: payload.clone(),
            caused_by_user_id: user_id,
            caused_by_branch_id: branch_id,
        }).await?;

        // 2. Publish to NATS
        let subject = format!("events.{}", event_type);
        self.nats.publish(subject, payload.to_string().into()).await?;

        Ok(())
    }
}
```

## 6. Event Types Catalog

### Customer Events
- `customer.created`
- `customer.activated`
- `customer.suspended`
- `customer.reactivated`
- `customer.terminated`
- `customer.kyc.submitted`
- `customer.kyc.verified`
- `customer.installation.scheduled`

### Billing Events
- `invoice.generated`
- `invoice.sent`
- `invoice.paid`
- `invoice.overdue`
- `invoice.voided`
- `payment.completed`
- `payment.failed`
- `refund.approved`
- `refund.processed`

### Subscription Events
- `subscription.created`
- `subscription.renewed`
- `subscription.suspended`
- `subscription.reactivated`
- `subscription.cancelled`
- `subscription.upgraded`
- `subscription.downgraded`

### Device Events
- `device.registered`
- `device.status.changed`
- `device.discovered`
- `device.auto_registered`
- `device.rejected`
- `device.ont.discovered`
- `device.firmware.update.started`
- `device.firmware.update.completed`

### Network Events
- `vlan.created`
- `vlan.deleted`
- `ippool.exhausted`
- `ippool.warning`
- `pppoe.session.started`
- `pppoe.session.ended`
- `customer.session.connected`
- `customer.session.disconnected`

### Ticket Events
- `ticket.created`
- `ticket.assigned`
- `ticket.escalated`
- `ticket.resolved`
- `ticket.reopened`

### Bandwidth Events
- `bandwidth.profile.created`
- `bandwidth.profile.updated`
- `bandwidth.profile.applied`
- `bandwidth.profile.failed`

### Other Events
- `lead.created`
- `lead.converted`
- `referral.created`
- `referral.activated`
- `referral.rewarded`
- `sla.breach.warning`

## 7. Event Schema

```json
{
  "event_id": "uuid-v4",
  "event_type": "invoice.generated",
  "aggregate_type": "invoice",
  "aggregate_id": 12345,
  "payload": {
    "invoice_id": 12345,
    "invoice_number": "INV-2026-07-0001",
    "customer_id": 678,
    "total_amount": 708.00,
    "due_date": "2026-07-10"
  },
  "metadata": {
    "caused_by_user_id": 42,
    "caused_by_branch_id": 1,
    "ip_address": "10.0.1.50",
    "user_agent": "Mozilla/5.0..."
  },
  "sequence_number": 100001,
  "published_at": "2026-07-08T14:30:00Z"
}
```

## 8. Outbox Pattern (Transactional Outbox)

Ensures reliable event publishing with at-least-once delivery guarantee.

### 8.1 Core Flow

```
1. Start database transaction
2. Perform business logic (insert/update entity)
3. Insert event into outbox_events table (same transaction)
4. Commit transaction
5. OutboxWorker polls for unpublished events
6. Publish to NATS JetStream
7. Mark event as published
8. Cleanup published events after retention period
```

This guarantees **atomicity** — the event is only published if the business transaction commits. No lost events.

### 8.2 Outbox Worker

Spawned in `main.rs` at startup. Runs continuously:

```rust
// In main.rs
let outbox_worker = OutboxWorker::new(
    Arc::new(outbox_db),
    outbox_publisher,
);
// Spawned with graceful shutdown
tokio::select! {
    _ = outbox_worker.run() => {},
    _ = shutdown_rx.recv() => { /* graceful shutdown */ }
}
```

Worker polls `outbox_events` table every N seconds for unpublished events, publishes to NATS, marks as published.

### 8.3 Dead-Letter Queue (DLQ)

When event publishing fails after max retries, the event moves to the dead-letter queue:

```rust
// In outbox.rs
const MAX_RETRIES: i32 = 5;

pub async fn record_publish_failure(db: &DbPool, event_id: Uuid) -> Result<()> {
    let event = find_event(db, &event_id).await?;

    if event.retry_count >= MAX_RETRIES {
        // Move to dead-letter queue
        let mut active = event.into_active_model();
        active.dead_letter = Set(true);
        active.dead_letter_at = Set(Some(Utc::now()));
        active.update(db).await?;
        return Ok(());
    }

    // Increment retry count
    let mut active = event.into_active_model();
    active.retry_count = Set(event.retry_count + 1);
    active.update(db).await?;
}
```

### 8.4 Dead-Letter Event Management

Separate `dead_letter_events` table for failed events:

```sql
CREATE TABLE dead_letter_events (
    id BIGSERIAL PRIMARY KEY,
    event_id UUID NOT NULL,
    event_type VARCHAR(100) NOT NULL,
    aggregate_type VARCHAR(50) NOT NULL,
    aggregate_id BIGINT NOT NULL,
    payload JSONB NOT NULL,
    error_message TEXT,
    retry_count INTEGER DEFAULT 0,
    max_retries INTEGER DEFAULT 5,
    status VARCHAR(20) DEFAULT 'failed'
        CHECK (status IN ('failed', 'replayed', 'discarded')),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    last_retry_at TIMESTAMPTZ
);
```

**Management operations:**
| Operation | Function | Purpose |
|-----------|----------|---------|
| List failed | `list_dead_letters()` | View all DLQ events with error messages |
| Replay | `replay_dead_letter(id)` | Reset retry count, re-queue for publishing |
| Discard | `discard_dead_letter(id)` | Mark as discarded, no retry |
| Cleanup | `cleanup_dead_letters(days)` | Delete old replayed/discarded events |

### 8.5 Outbox Schema

```sql
CREATE TABLE outbox_events (
    id BIGSERIAL PRIMARY KEY,
    event_id UUID NOT NULL DEFAULT gen_random_uuid(),
    event_type VARCHAR(100) NOT NULL,
    aggregate_type VARCHAR(50) NOT NULL,
    aggregate_id BIGINT NOT NULL,
    payload JSONB NOT NULL,
    published BOOLEAN DEFAULT FALSE,
    published_at TIMESTAMPTZ,
    retry_count INTEGER DEFAULT 0,
    dead_letter BOOLEAN DEFAULT FALSE,
    dead_letter_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW()
);
```

### 8.6 Event Lifecycle

```
INSERT (in transaction)
    ↓
UNPUBLISHED (outbox worker polls)
    ↓
PUBLISHING (worker sends to NATS)
    ↓ ┌─── SUCCESS ──→ PUBLISHED → CLEANUP (after retention)
    │
    └─── FAILURE ──→ RETRY (retry_count++)
                        ↓
                    MAX_RETRIES EXCEEDED
                        ↓
                    DEAD-LETTER QUEUE
                        ↓ ┌─── REPLAY ──→ UNPUBLISHED (retry from scratch)
                        │
                        └─── DISCARD ──→ Final state
```

### 8.7 Idempotency Requirement

Subscribers **MUST** be idempotent. The outbox guarantees at-least-once delivery — events may be delivered more than once during retries or replays.

```rust
// Subscriber idempotency pattern
async fn handle_event(&self, event: &Event) -> Result<()> {
    // Check if already processed
    if self.is_already_processed(&event.event_id).await? {
        return Ok(()); // Skip duplicate
    }

    // Process event
    self.process(event).await?;

    // Mark as processed
    self.mark_processed(&event.event_id).await?;

    Ok(())
}
```

## 9. Subscriber Configuration

```rust
pub struct SubscriberConfig {
    pub name: String,           // "bandwidth-engine"
    pub event_types: Vec<String>, // ["bandwidth.profile.updated", "customer.activated"]
    pub handler: Box<dyn EventHandler>,
    pub concurrency: usize,     // Number of concurrent message handlers
}
```

## 10. RBAC Permissions

```
event.view
event.export
event.replay
```
