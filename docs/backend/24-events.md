# AeroXe Backend — Events Module

> **Req Ref:** §12 Event Sourcing Design, §15 NATS Event Architecture

---

## 1. Overview

Implements event sourcing using NATS JetStream for durable, exactly-once message delivery. All inter-module communication happens through events. The event store provides a complete audit trail of every state change in the system.

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
        payload: serde_json::Value,
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

## 8. Outbox Pattern

Ensures reliable event publishing:

```
1. Start database transaction
2. Perform business logic (insert/update entity)
3. Insert event into events table (same transaction)
4. Commit transaction
5. Background worker polls for unpublished events
6. Publish to NATS
7. Mark event as published
```

This guarantees at-least-once delivery. Subscribers must be idempotent.

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
