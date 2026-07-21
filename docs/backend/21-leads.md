# AeroXe Backend — Leads Module

> **Req Ref:** §2.13 Lead Management

---

## 1. Overview

Sales agents track potential customers through a pipeline from initial contact to conversion. Leads flow through stages: new → contacted → interested → surveyed → quoted → converted (or lost).

## 2. Pipeline Stages

```
new → contacted → interested → surveyed → quoted → converted
                                          ↘ lost
```

## 3. Database Tables

```sql
CREATE TABLE leads (
    id BIGSERIAL PRIMARY KEY,
    branch_id BIGINT NOT NULL REFERENCES branches(id),
    assigned_to BIGINT REFERENCES users(id),
    name VARCHAR(255) NOT NULL,
    phone VARCHAR(20) NOT NULL,
    email VARCHAR(255),
    source VARCHAR(50) NOT NULL
        CHECK (source IN ('landing_page', 'whatsapp', 'referral', 'walk_in',
                          'cold_call', 'social_media', 'field_visit')),
    status VARCHAR(30) DEFAULT 'new'
        CHECK (status IN ('new', 'contacted', 'interested', 'surveyed',
                          'quoted', 'converted', 'lost')),
    interested_plan_id BIGINT REFERENCES plans(id),
    estimated_install_date DATE,
    address TEXT,
    latitude DECIMAL(10, 7),
    longitude DECIMAL(10, 7),
    lost_reason TEXT,
    notes TEXT,
    converted_customer_id BIGINT REFERENCES customers(id),
    converted_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE lead_activities (
    id BIGSERIAL PRIMARY KEY,
    lead_id BIGINT NOT NULL REFERENCES leads(id) ON DELETE CASCADE,
    activity_type VARCHAR(30) NOT NULL
        CHECK (activity_type IN ('call', 'whatsapp', 'visit', 'email', 'note', 'status_change')),
    description TEXT NOT NULL,
    performed_by BIGINT NOT NULL REFERENCES users(id),
    scheduled_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW()
);
```

## 4. API Endpoints

> **API Convention:** Protobuf-first. No GET, no PUT, no path variables, no query strings. See `API-CONVENTIONS.md`.

| Method | Path | Required Role | Description |
|--------|------|--------------|-------------|
| POST | `/api/v1/leads/list` | sales_agent+ | List leads |
| POST | `/api/v1/leads/create` | sales_agent+ | Create lead |
| POST | `/api/v1/leads/get` | sales_agent+ | Get lead details |
| PATCH | `/api/v1/leads/update` | sales_agent+ | Update lead |
| POST | `/api/v1/leads/status` | sales_agent+ | Change lead status |
| POST | `/api/v1/leads/assign` | sales_agent+ | Assign lead |
| POST | `/api/v1/leads/activities/create` | sales_agent+ | Log activity |
| POST | `/api/v1/leads/activities/list` | sales_agent+ | List activities |
| POST | `/api/v1/leads/convert` | sales_agent+ | Convert to customer |
| POST | `/api/v1/leads/pipeline` | sales_agent+ | Pipeline view |
| POST | `/api/v1/leads/stats` | sales_agent+ | Lead statistics |

## 5. Lead → Customer Conversion

```
1. Lead status = 'quoted'
2. Sales agent triggers conversion
3. System creates customer record from lead data
4. Creates subscription with interested plan
5. Creates installation order
6. Updates lead: status = 'converted', converted_customer_id, converted_at
7. Publishes lead.converted event
```

## 6. Events Published

```yaml
lead.created:
  payload: { lead_id, name, phone, source, branch_id }
lead.status.changed:
  payload: { lead_id, old_status, new_status }
lead.converted:
  payload: { lead_id, customer_id, plan_id }
lead.lost:
  payload: { lead_id, reason }
```

## 7. RBAC Permissions

```
lead.view
lead.create
lead.update
lead.assign
lead.convert
lead.activity.create
lead.activity.view
```
