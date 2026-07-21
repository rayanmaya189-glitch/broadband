# AeroXe Admin Portal — Audit Module

> **Req Ref:** §2.10 Audit Tracking, §16 Admin Portal

---

## 1. Overview

Audit log viewer — search, filter, and export the complete audit trail of all system actions. Used for security compliance, incident investigation, and regulatory requirements.

## 2. Pages

### Audit Log Viewer (`/audit`)

```
┌──────────────────────────────────────────────────────────┐
│  Audit Logs                              [Export CSV] [Export JSON] │
├──────────────────────────────────────────────────────────┤
│  User: [________] Action: [________] Resource: [________]│
│  Result: [All ▼] Date: [Range ▼] Branch: [All ▼]       │
├──────────────────────────────────────────────────────────┤
│  Time           │ User          │ Action            │ Result │ IP          │
│  10:30:15 AM    │ admin@aeroxe  │ device.router.    │ ✅ Grant│ 10.0.1.50  │
│                 │               │ restart           │        │             │
│  10:28:42 AM    │ network@aeroxe│ vlan.create       │ ✅ Grant│ 10.0.1.51  │
│  10:25:10 AM    │ unknown       │ auth.login        │ ❌ Deny │ 192.168.1.5│
│  10:20:00 AM    │ support@aeroxe│ ticket.assign     │ ✅ Grant│ 10.0.1.52  │
│  10:15:33 AM    │ billing@aeroxe│ invoice.generate  │ ✅ Grant│ 10.0.1.53  │
└──────────────────────────────────────────────────────────┘
```

### Log Detail (click any row)

```
Audit Entry #100042
─────────────────────────────────────────────────────────
Timestamp: Jul 8, 2026 10:30:15 AM IST
User: admin@aeroxe.com (Network Admin)
IP Address: 10.0.1.50
User Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64)

Action: device.router.restart
Resource: device #42 (Jalgaon-CityCenter-R01)
Result: ✅ Granted

Metadata:
├── device_name: Jalgaon-CityCenter-R01
├── reason: Customer reported connectivity issue
└── duration_ms: 1250
```

## 3. Search Filters

| Filter | Type | Description |
|--------|------|-------------|
| User | Text/email search | Filter by user |
| Action | Text search | Filter by action (supports wildcards) |
| Resource Type | Dropdown | customer, device, invoice, ticket, etc. |
| Resource ID | Text | Specific resource UUID |
| Result | Dropdown | granted, denied, expired |
| Date Range | Date picker | Start and end date |
| Branch | Dropdown | Branch-specific logs |
| IP Address | Text | Filter by source IP |

## 4. Export Options

- **CSV Export:** Filtered results as CSV file
- **JSON Export:** Filtered results as JSON file
- **Date range:** Up to 30 days per export
- **Max records:** 10,000 per export

## 5. API Endpoints

> **API Convention:** Protobuf-first. See `API-CONVENTIONS.md`.

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/api/v1/audit/logs/list` | POST | Search audit logs (paginated) |
| `/api/v1/audit/logs/get` | POST | Get specific log entry |
| `/api/v1/audit/export` | POST | Export logs (CSV/JSON) |
| `/api/v1/audit/user/list` | POST | User activity log |
| `/api/v1/audit/resource/list` | POST | Resource history |

## 6. RBAC

| Action | Required Permission |
|--------|-------------------|
| View audit logs | `audit.log.view` |
| Export audit logs | `audit.log.export` |
| Search audit logs | `audit.log.search` |

## 7. Entity History & Rollback (§8D)

### Entity History Viewer (`/audit/entity-history`)

```
┌──────────────────────────────────────────────────────────┐
│  Entity History                          [Export]        │
├──────────────────────────────────────────────────────────┤
│  Entity: [Customer ▼]  ID: [________]  Action: [All ▼]  │
│  Date: [Range ▼]  Branch: [All ▼]                       │
├──────────────────────────────────────────────────────────┤
│  Time          │ Action          │ User       │ Fields   │
│  Jul 8, 10:30  │ updated         │ admin@aero │ name,    │
│                │                 │            │ email    │
│  Jul 5, 14:20  │ status_changed  │ system     │ active → │
│                │                 │            │ suspended│
│  Jul 1, 09:00  │ created         │ sales@aero │ —        │
└──────────────────────────────────────────────────────────┘
```

### History Detail (click any row)

```
Entity History Entry #H-2026-00042
─────────────────────────────────────────────────────────
Timestamp: Jul 8, 2026 10:30:15 AM IST
Entity: customer #cust_abc123 (Rahul Sharma)
Action: updated
User: admin@aeroxe.com (Super Admin)
IP: 10.0.1.50
Reason: Customer requested address change

── Changed Fields ──────────────────────────────────────
Field        │ Old Value                    │ New Value
─────────────┼──────────────────────────────┼──────────
address.city │ Jalgaon                      │ Pune
address.zip  │ 425001                       │ 411001

── Rollback ────────────────────────────────────────────
[🔄 Rollback to Previous State]

⚠️ Rolling back will restore the old values and create a
new history entry with action = 'rollback'.
─────────────────────────────────────────────────────────
```

### History Tables List

| Entity | History Table | Retention | Compression |
|--------|---------------|-----------|-------------|
| Customers | `customers_history` | 7 years | After 1 year |
| Subscriptions | `subscriptions_history` | 7 years | After 1 year |
| Plans | `plans_history` | 7 years | After 1 year |
| Invoices | `invoices_history` | 7 years | After 1 year |
| Refunds | `refunds_history` | 7 years | After 1 year |
| Journal Entries | `journal_entries_history` | 7 years | After 1 year |
| Manual Payments | `manual_payments_history` | 7 years | After 1 year |
| Network Devices | `network_devices_history` | 3 years | After 6 months |
| Payment Gateways | `payment_gateways_history` | 3 years | After 6 months |
| Discounts | `discounts_history` | 3 years | After 6 months |
| Approval Requests | `approval_requests_history` | 3 years | After 6 months |
| Bandwidth Profiles | `bandwidth_profiles_history` | 2 years | After 6 months |

### History Table Schema

```sql
CREATE TABLE {entity}_history (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    entity_id UUID NOT NULL,
    action VARCHAR(50) NOT NULL, -- created, updated, deleted, status_changed, rollback
    old_data JSONB,              -- snapshot before change
    new_data JSONB,              -- snapshot after change
    changed_fields TEXT[],       -- list of changed field names
    user_id UUID REFERENCES users(id),
    branch_id UUID REFERENCES branches(id),
    ip_address INET,
    user_agent TEXT,
    reason TEXT,                 -- optional reason for change
    rollback_reference UUID,     -- links to original change if rollback
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes
CREATE INDEX idx_{entity}_history_entity ON {entity}_history(entity_id);
CREATE INDEX idx_{entity}_history_action ON {entity}_history(action);
CREATE INDEX idx_{entity}_history_user ON {entity}_history(user_id);
CREATE INDEX idx_{entity}_history_created ON {entity}_history(created_at);
```

### Rollback Workflow

```
1. Admin views entity history
2. Selects a previous state to restore
3. System validates rollback is safe (no downstream dependencies)
4. System applies old_data back to primary table
5. Creates new history entry:
   - action = 'rollback'
   - old_data = current state (before rollback)
   - new_data = restored state
   - rollback_reference = original change ID
6. Publishes entity.{entity_type}.rollback event
7. Notifies affected users
```

### Rollback Safety Checks

| Check | Description |
|-------|-------------|
| Active subscriptions | Cannot rollback customer if active subscription exists |
| Paid invoices | Cannot rollback invoice if payment already processed |
| Deployed devices | Cannot rollback device if currently online |
| Chain dependencies | Cannot rollback if dependent entities exist |
| Approval workflows | Cannot rollback approved items without re-approval |

### History API Endpoints

> **API Convention:** Protobuf-first. See `API-CONVENTIONS.md`.

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/api/v1/audit/entity-history/list` | POST | Search entity history |
| `/api/v1/audit/entity-history/get` | POST | Get history entry detail |
| `/api/v1/audit/entity-history/entity/list` | POST | Get all history for entity |
| `/api/v1/audit/entity-history/rollback` | POST | Rollback to this state |
| `/api/v1/audit/entity-history/compare` | POST | Compare old vs new state |
| `/api/v1/audit/entity-history/export` | POST | Export entity history |

### Entity History RBAC

| Action | Required Permission |
|--------|-------------------|
| View entity history | `audit.entity_history.view` |
| Rollback entity | `audit.entity_history.rollback` |
| Export entity history | `audit.entity_history.export` |

### Rollback Notification

When a rollback occurs, affected users receive:

```
Subject: Entity Changed — Rollback Performed

The {entity_type} ({entity_name}) has been rolled back.

Changed by: {admin_name}
Reason: {reason}
Previous state: {old_values}
Restored state: {new_values}

If you did not expect this change, contact support immediately.
```
