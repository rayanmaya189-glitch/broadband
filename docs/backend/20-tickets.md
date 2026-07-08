# AeroXe Backend — Tickets Module

> **Req Ref:** §7A Support Ticketing System

---

## 1. Overview

Manages the full ticket lifecycle from creation through assignment, escalation, resolution, and closure. Includes SLA tracking, priority-based routing, customer satisfaction ratings, and automatic escalation on SLA breaches.

## 2. Ticket Lifecycle

```
open → assigned → in_progress → waiting_customer → resolved → closed
                                                         ↗
                                  escalated ↗
closed → reopened → open
```

## 3. Priority & SLA

| Priority | Response SLA | Resolution SLA | Example |
|----------|-------------|----------------|---------|
| critical | 15 minutes | 2 hours | Total outage |
| high | 30 minutes | 4 hours | Business customer down |
| medium | 2 hours | 24 hours | Intermittent issues |
| low | 8 hours | 72 hours | General queries |

## 4. Categories

| Category | Subcategories |
|----------|---------------|
| connectivity | no_internet, slow_speed, intermittent, dns_resolution |
| installation | new_installation, relocation, disconnection, router_issue |
| billing | payment_issue, invoice_query, refund_request, plan_change |
| hardware | router_replacement, ont_issue, cable_damage, fiber_cut |
| account | kyc_update, password_reset, profile_change, suspension_query |
| other | general_inquiry, feedback, complaint |

## 5. Database Tables

```sql
CREATE TABLE tickets (
    id BIGSERIAL PRIMARY KEY,
    ticket_number VARCHAR(20) NOT NULL UNIQUE,
    branch_id BIGINT NOT NULL REFERENCES branches(id),
    customer_id BIGINT REFERENCES customers(id),
    subscription_id BIGINT REFERENCES subscriptions(id),
    created_by BIGINT NOT NULL REFERENCES users(id),
    assigned_to BIGINT REFERENCES users(id),
    escalated_to BIGINT REFERENCES users(id),
    category VARCHAR(50) NOT NULL,
    subcategory VARCHAR(50),
    priority VARCHAR(10) DEFAULT 'medium'
        CHECK (priority IN ('critical', 'high', 'medium', 'low')),
    status VARCHAR(30) DEFAULT 'open'
        CHECK (status IN ('open', 'assigned', 'in_progress', 'waiting_customer',
                          'escalated', 'resolved', 'closed', 'reopened')),
    subject VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    source VARCHAR(20) DEFAULT 'customer'
        CHECK (source IN ('customer', 'phone', 'email', 'whatsapp', 'agent', 'system')),
    resolution_notes TEXT,
    sla_response_at TIMESTAMPTZ,
    sla_resolution_at TIMESTAMPTZ,
    first_response_at TIMESTAMPTZ,
    resolved_at TIMESTAMPTZ,
    closed_at TIMESTAMPTZ,
    reopen_count INTEGER DEFAULT 0,
    satisfaction_rating INTEGER CHECK (satisfaction_rating BETWEEN 1 AND 5),
    satisfaction_feedback TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE ticket_comments (
    id BIGSERIAL PRIMARY KEY,
    ticket_id BIGINT NOT NULL REFERENCES tickets(id) ON DELETE CASCADE,
    user_id BIGINT REFERENCES users(id),
    is_customer BOOLEAN DEFAULT FALSE,
    comment TEXT NOT NULL,
    is_internal BOOLEAN DEFAULT FALSE,
    attachments JSONB,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE ticket_escalations (
    id BIGSERIAL PRIMARY KEY,
    ticket_id BIGINT NOT NULL REFERENCES tickets(id),
    from_user_id BIGINT NOT NULL REFERENCES users(id),
    to_user_id BIGINT NOT NULL REFERENCES users(id),
    from_priority VARCHAR(10),
    to_priority VARCHAR(10),
    reason TEXT NOT NULL,
    escalated_at TIMESTAMPTZ DEFAULT NOW(),
    acknowledged_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE ticket_attachments (
    id BIGSERIAL PRIMARY KEY,
    ticket_id BIGINT NOT NULL REFERENCES tickets(id) ON DELETE CASCADE,
    comment_id BIGINT REFERENCES ticket_comments(id) ON DELETE SET NULL,
    document_id BIGINT NOT NULL REFERENCES document_files(id),
    uploaded_by BIGINT NOT NULL REFERENCES users(id),
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE ticket_status_history (
    id BIGSERIAL PRIMARY KEY,
    ticket_id BIGINT NOT NULL REFERENCES tickets(id) ON DELETE CASCADE,
    old_status VARCHAR(30),
    new_status VARCHAR(30) NOT NULL,
    changed_by BIGINT NOT NULL REFERENCES users(id),
    reason TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE tickets_history (
    id BIGSERIAL PRIMARY KEY,
    ticket_id BIGINT NOT NULL,
    action VARCHAR(20) NOT NULL,
    old_data JSONB, new_data JSONB,
    performed_by BIGINT REFERENCES users(id),
    performed_at TIMESTAMPTZ DEFAULT NOW(),
    reason TEXT
);
```

## 6. API Endpoints

| Method | Path | Required Role | Description |
|--------|------|--------------|-------------|
| GET | `/api/v1/tickets` | ticket.view | List tickets |
| POST | `/api/v1/tickets` | ticket.create | Create ticket |
| GET | `/api/v1/tickets/:id` | ticket.view | Get ticket |
| PUT | `/api/v1/tickets/:id` | ticket.update | Update ticket |
| POST | `/api/v1/tickets/:id/assign` | ticket.assign | Assign ticket |
| POST | `/api/v1/tickets/:id/escalate` | ticket.escalate | Escalate ticket |
| POST | `/api/v1/tickets/:id/resolve` | ticket.resolve | Resolve ticket |
| POST | `/api/v1/tickets/:id/close` | ticket.close | Close ticket |
| POST | `/api/v1/tickets/:id/reopen` | customer (own) | Reopen ticket |
| GET | `/api/v1/tickets/:id/comments` | ticket.view | List comments |
| POST | `/api/v1/tickets/:id/comments` | ticket.create | Add comment |
| POST | `/api/v1/tickets/:id/satisfaction` | customer (own) | Rate ticket |
| GET | `/api/v1/tickets/my-assignments` | ticket.view | My assigned tickets |
| GET | `/api/v1/tickets/dashboard` | ticket.view | Ticket dashboard stats |

## 7. Ticket Number Generation

Format: `TKT-{YYYY}-{MM}-{SEQUENCE}`

Example: `TKT-2026-07-0001`

## 8. Auto-Escalation Job

Runs every 5 minutes:

```rust
async fn check_sla_breaches(state: &AppState) {
    // Find tickets approaching SLA breach
    let approaching = db.query(
        "SELECT * FROM tickets
         WHERE status IN ('open', 'assigned', 'in_progress')
         AND sla_resolution_at < NOW() + INTERVAL '30 minutes'
         AND escalated_to IS NULL"
    ).await?;

    for ticket in approaching {
        // Auto-escalate to next tier
        escalate_ticket(&state, &ticket, "SLA breach approaching").await;
        // Send notification
    }
}
```

## 9. Events Published

```yaml
ticket.created:
  payload: { ticket_id, ticket_number, branch_id, category, priority, subject, source }
ticket.assigned:
  payload: { ticket_id, assigned_to, assigned_by }
ticket.escalated:
  payload: { ticket_id, from_user_id, to_user_id, reason, new_priority }
ticket.resolved:
  payload: { ticket_id, resolved_by, resolution_notes, resolution_time_minutes }
ticket.reopened:
  payload: { ticket_id, reopen_count, reopened_by }
ticket.closed:
  payload: { ticket_id, satisfaction_rating }
sla.breach.warning:
  payload: { ticket_id, breach_type, sla_at, current_time }
```

## 10. RBAC Permissions

```
ticket.view
ticket.create
ticket.assign
ticket.update
ticket.resolve
ticket.close
ticket.escalate
ticket.reopen
ticket.comment.view
ticket.comment.create
ticket.comment.update
ticket.comment.delete
```
