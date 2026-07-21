# AeroXe Admin Portal — Leads Module

> **Req Ref:** §2.13 Lead Management, §16 Admin Portal

---

## 1. Overview

Sales lead management — track potential customers through the pipeline from initial contact to conversion. Includes pipeline view, activity logging, and lead-to-customer conversion.

## 2. Pages

### Lead List (`/leads`)

```
┌──────────────────────────────────────────────────────────┐
│  Sales Leads                          [+ New Lead] [Export] │
├──────────────────────────────────────────────────────────┤
│  Status: [All ▼]  Source: [All ▼]  Assigned: [All ▼]    │
├──────────────────────────────────────────────────────────┤
│  Name         │ Phone      │ Source     │ Status      │ Plan    │
│  Sanjay Kumar │ +91984...  │ WhatsApp   │ ● Interested│ Std 100 │
│  Meena Devi   │ +91983...  │ Walk-in    │ ● Quoted    │ Basic 50│
│  Ravi Patel   │ +91982...  │ Referral   │ ● New       │ —       │
│  Suresh Jain  │ +91981...  │ Landing    │ ● Lost      │ —       │
└──────────────────────────────────────────────────────────┘
```

### Pipeline View (`/leads/pipeline`)

```
┌────────────┬────────────┬────────────┬────────────┬────────────┬────────────┐
│    New     │ Contacted  │ Interested │  Surveyed  │  Quoted    │ Converted  │
├────────────┼────────────┼────────────┼────────────┼────────────┼────────────┤
│ Ravi Patel │ Kavita M.  │ Sanjay K.  │ Deepak R.  │ Meena D.   │ Ajay S.    │
│ 📱 Landing │ ☎️ Called  │ 💬 Interst │ 📋 Survey  │ 💰 Quoted  │ ✅ Done    │
│ ₹0 est.   │ ₹600 est.  │ ₹800 est.  │ ₹1000 est. │ ₹600 est.  │ ₹600 rev  │
└────────────┴────────────┴────────────┴────────────┴────────────┴────────────┘
```

### Lead Detail (`/leads/:id`)

```
Lead: Sanjay Kumar (+91984567890)
Status: Interested  │  Source: WhatsApp  │  Branch: Jalgaon
Assigned to: Sales Agent - Rahul
Interested Plan: Standard 100 Mbps (₹600/mo)
Estimated Install: Jul 15, 2026

Activity Timeline:
├── Jul 8, 10:30 — WhatsApp inquiry (auto-created)
├── Jul 8, 11:00 — Called customer (Sales Agent)
├── Jul 8, 11:15 — Interested in Standard plan
└── Jul 8, 14:00 — Site survey scheduled for Jul 10

[Convert to Customer] [Change Status] [Add Activity] [Reassign]
```

## 3. Lead → Customer Conversion

```
1. Staff clicks "Convert to Customer" on lead detail
2. System pre-fills customer form from lead data
3. Staff completes required fields (address, KYC)
4. System creates:
   a. Customer record
   b. Subscription (with interested plan)
   c. Installation order
5. Lead status → "converted"
6. Lead linked to new customer
```

## 4. API Endpoints

> **API Convention:** Protobuf-first. See `API-CONVENTIONS.md`.

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/api/v1/leads/list` | POST | List leads |
| `/api/v1/leads/create` | POST | Create lead |
| `/api/v1/leads/get` | POST | Get lead details |
| `/api/v1/leads/update` | PATCH | Update lead |
| `/api/v1/leads/status/update` | POST | Change lead status |
| `/api/v1/leads/assign` | POST | Assign lead |
| `/api/v1/leads/activities/list` | POST | List activities |
| `/api/v1/leads/activities/create` | POST | Add activity |
| `/api/v1/leads/convert` | POST | Convert to customer |
| `/api/v1/leads/pipeline/list` | POST | Pipeline view data |
| `/api/v1/leads/stats/list` | POST | Lead statistics |

## 5. RBAC

| Action | Required Permission |
|--------|-------------------|
| View leads | `lead.view` |
| Create lead | `lead.create` |
| Update lead | `lead.update` |
| Assign lead | `lead.assign` |
| Convert lead | `lead.convert` |
| Log activity | `lead.activity.create` |
