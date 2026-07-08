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

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/api/v1/audit/logs` | GET | Search audit logs (paginated) |
| `/api/v1/audit/logs/:id` | GET | Get specific log entry |
| `/api/v1/audit/export` | GET | Export logs (CSV/JSON) |
| `/api/v1/audit/user/:id` | GET | User activity log |
| `/api/v1/audit/resource/:type/:id` | GET | Resource history |

## 6. RBAC

| Action | Required Permission |
|--------|-------------------|
| View audit logs | `audit.log.view` |
| Export audit logs | `audit.log.export` |
| Search audit logs | `audit.log.search` |
