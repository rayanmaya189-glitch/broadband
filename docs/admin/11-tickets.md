# AeroXe Admin Portal — Tickets Module

> **Req Ref:** §7A Support Ticketing System, §16 Admin Portal

---

## 1. Overview

Support ticketing management — create, assign, escalate, resolve, and close tickets. SLA tracking with real-time breach warnings. Kanban board view for workflow management.

## 2. Pages

### Ticket List (`/tickets`)

```
┌──────────────────────────────────────────────────────────┐
│  Support Tickets                    [+ New Ticket] [Export] │
├──────────────────────────────────────────────────────────┤
│  View: [List ▼]  Status: [All ▼]  Priority: [All ▼]     │
│  Category: [All ▼]  Assigned: [All ▼]  Branch: [All ▼]  │
├──────────────────────────────────────────────────────────┤
│  Ticket #    │ Subject          │ Customer │ Priority │ Status     │ SLA    │
│  TKT-2026-07-001 │ No internet  │ Rahul S. │ 🔴 High  │ ● Open     │ 25 min │
│  TKT-2026-07-002 │ Slow speed   │ Priya P. │ 🟡 Med   │ ● In Prog  │ 18 hrs │
│  TKT-2026-07-003 │ Billing Q    │ Amit D.  │ 🟢 Low   │ ● Resolved │ ✅     │
└──────────────────────────────────────────────────────────┘
```

### Kanban View (`/tickets` — toggle to Board)

```
┌────────────┬────────────┬────────────┬────────────┬────────────┐
│   Open     │  Assigned  │ In Progress│  Waiting   │  Resolved  │
├────────────┼────────────┼────────────┼────────────┼────────────┤
│ TKT-001    │ TKT-004    │ TKT-002    │ TKT-005    │ TKT-003    │
│ 🔴 High    │ 🔴 High    │ 🟡 Med     │ 🟡 Med     │ 🟢 Low     │
│ No internet│ Fiber cut  │ Slow speed │ Info needed│ Billing Q  │
├────────────┤            ├────────────┤            ├────────────┤
│ TKT-006    │            │ TKT-007    │            │            │
│ 🟡 Med     │            │ 🔴 High    │            │            │
│ Router issue│           │ Outage     │            │            │
└────────────┴────────────┴────────────┴────────────┴────────────┘
```

### Ticket Detail (`/tickets/:id`)

```
┌──────────────────────────────────────────────────────────┐
│  TKT-2026-07-001 — No internet since morning   [Assign] │
│  Customer: Rahul Sharma  │  Priority: High  │  Status: Open │
│  SLA: Response by 10:45 (25 min left)                   │
├──────────────────────────────────────────────────────────┤
│  [Details] [Comments] [History] [Escalate] [Resolve]    │
├──────────────────────────────────────────────────────────┤
│                                                          │
│  Description:                                            │
│  "No internet since morning. Router shows red light."    │
│                                                          │
│  Conversation:                                           │
│  ┌──────────────────────────────────────────────────┐   │
│  │ [Support] Rahul: Can you check the ONT status?   │   │
│  │ [Customer] 10:32 AM: ONT shows red light too     │   │
│  │ [Support] Rahul: We're dispatching a technician   │   │
│  └──────────────────────────────────────────────────┘   │
│                                                          │
│  [Add Comment]  [Internal Note]  [Attach File]          │
└──────────────────────────────────────────────────────────┘
```

## 3. Key Features

### SLA Timer
- Real-time countdown to SLA breach
- Color coding: Green (>2h) → Yellow (30m-2h) → Red (<30m)
- Auto-escalation on SLA breach

### Quick Actions
- **Assign:** Select support agent from dropdown
- **Escalate:** Choose escalation target + reason
- **Resolve:** Enter resolution notes
- **Close:** After resolution (auto-closes after 7 days)

## 4. API Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/api/v1/tickets` | GET/POST | List/create tickets |
| `/api/v1/tickets/:id` | GET/PUT | Get/update ticket |
| `/api/v1/tickets/:id/assign` | POST | Assign ticket |
| `/api/v1/tickets/:id/escalate` | POST | Escalate ticket |
| `/api/v1/tickets/:id/resolve` | POST | Resolve ticket |
| `/api/v1/tickets/:id/close` | POST | Close ticket |
| `/api/v1/tickets/:id/comments` | GET/POST | List/add comments |
| `/api/v1/tickets/:id/satisfaction` | POST | Rate satisfaction |
| `/api/v1/tickets/my-assignments` | GET | My assigned tickets |
| `/api/v1/tickets/dashboard` | GET | Dashboard stats |

## 5. RBAC

| Action | Required Permission |
|--------|-------------------|
| View tickets | `ticket.view` |
| Create ticket | `ticket.create` |
| Assign ticket | `ticket.assign` |
| Update ticket | `ticket.update` |
| Resolve ticket | `ticket.resolve` |
| Close ticket | `ticket.close` |
| Escalate ticket | `ticket.escalate` |
| Reopen ticket | `ticket.reopen` |
