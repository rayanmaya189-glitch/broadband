# AeroXe Backend — Installations Module

> **Req Ref:** §3.4 Installation Workflow

---

## 1. Overview

Manages the end-to-end installation workflow from order creation through scheduling, field technician assignment, on-site installation, and service activation. Tracks equipment issued and installation quality metrics.

## 2. Database Tables

```sql
CREATE TABLE installation_orders (
    id BIGSERIAL PRIMARY KEY,
    customer_id BIGINT NOT NULL REFERENCES customers(id),
    branch_id BIGINT NOT NULL REFERENCES branches(id),
    subscription_id BIGINT REFERENCES subscriptions(id),
    assigned_technician_id BIGINT REFERENCES users(id),
    status VARCHAR(30) DEFAULT 'pending'
        CHECK (status IN ('pending', 'scheduled', 'in_progress', 'completed', 'cancelled', 'rescheduled')),
    scheduled_date DATE,
    scheduled_time_slot VARCHAR(20),
    completed_at TIMESTAMPTZ,
    installation_type VARCHAR(20) DEFAULT 'new',
    equipment_issued JSONB,
    fiber_drop_length_meters INTEGER,
    onu_power_dbm DECIMAL(5,2),
    notes TEXT,
    photos TEXT[],
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
```

## 3. API Endpoints

> **API Convention:** Protobuf-first. No GET, no PUT, no path variables, no query strings. See `API-CONVENTIONS.md`.

| Method | Path | Required Role | Description |
|--------|------|--------------|-------------|
| POST | `/api/v1/installations/list` | field_technician+ | List installation orders |
| POST | `/api/v1/installations/create` | sales_agent+ | Create installation order |
| POST | `/api/v1/installations/get` | field_technician+ | Get installation details |
| PATCH | `/api/v1/installations/schedule` | customer_support+ | Schedule installation |
| PATCH | `/api/v1/installations/reschedule` | field_technician+ | Reschedule |
| PATCH | `/api/v1/installations/start` | field_technician | Start installation |
| PATCH | `/api/v1/installations/complete` | field_technician | Complete installation |
| PATCH | `/api/v1/installations/cancel` | customer_support+ | Cancel installation |
| POST | `/api/v1/installations/photos` | field_technician | Upload installation photos |
| POST | `/api/v1/installations/my-assignments/list` | field_technician | Get my assignments |

## 4. Installation State Machine

```
pending → scheduled → in_progress → completed
                ↓            ↓
            cancelled    rescheduled → scheduled
```

## 5. Installation Completion Flow

```
1. Technician marks installation as 'in_progress'
2. Technician physically installs:
   - Fiber drop to customer premises
   - ONT (Optical Network Terminal)
   - Router (if included with plan)
3. Technician records:
   - Fiber drop length (meters)
   - ONU power level (dBm)
   - Equipment serial numbers
   - Installation photos
4. On completion:
   a. Update installation status → 'completed'
   b. Update customer status → 'active'
   c. Create/update PPPoE session credentials
   d. Apply bandwidth speed profile to device
   e. Activate subscription
   f. Generate first invoice
   g. Publish customer.activated event
   h. Send welcome notification to customer
```

## 6. Equipment Tracking

```json
{
  "equipment_issued": [
    { "type": "ont", "model": "Huawei HG8245H", "serial": "ONT-001" },
    { "type": "router", "model": "TP-Link Archer C6", "serial": "RTR-001" }
  ]
}
```

Equipment is linked to the inventory module (§18-inventory.md).

## 7. Events Published

```yaml
installation.scheduled:
  payload: { installation_id, customer_id, technician_id, scheduled_date }
installation.started:
  payload: { installation_id, customer_id, technician_id }
installation.completed:
  payload: { installation_id, customer_id, equipment_issued }
installation.cancelled:
  payload: { installation_id, customer_id, reason }
```

## 8. RBAC Permissions

```
installation.view
installation.create
installation.schedule
installation.complete
installation.cancel
installation.reschedule
```
