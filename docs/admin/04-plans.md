# AeroXe Admin Portal — Plans Module

> **Req Ref:** §4 Product and Plan Management, §16 Admin Portal

---

## 1. Overview

Manage internet plans, pricing tiers, speed profiles, and service packages. Plans are company-wide (not branch-scoped). Includes checker/maker approval workflow for plan changes.

## 2. Pages

### Plan List (`/plans`)

```
┌──────────────────────────────────────────────────────────┐
│  Internet Plans                              [+ Add Plan]│
├──────────────────────────────────────────────────────────┤
│  Status: [All ▼]  Type: [All ▼]                         │
├──────────────────────────────────────────────────────────┤
│  ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐              │
│  │Basic│ │Stnd │ │Prem │ │Pro  │ │Ult  │              │
│  │50Mb │ │100Mb│ │150Mb│ │200Mb│ │300Mb│              │
│  │₹400 │ │₹600 │ │₹800 │ │₹1000│ │₹1300│              │
│  │ ●   │ │ ★   │ │ ●   │ │ ●   │ │ ●   │              │
│  │Edit │ │Edit │ │Edit │ │Edit │ │Edit │              │
│  └─────┘ └─────┘ └─────┘ └─────┘ └─────┘              │
│                                                          │
│  ★ = Popular  ● = Active  ● = Inactive                  │
└──────────────────────────────────────────────────────────┘
```

### Plan Detail (`/plans/:id`)

```
┌──────────────────────────────────────────────────────────┐
│  Plan: Standard 100 Mbps              [Edit] [Publish]  │
├──────────────────────────────────────────────────────────┤
│  [Overview] [Pricing] [Speed Profile] [Packages] [History] │
├──────────────────────────────────────────────────────────┤
│                                                          │
│  Speed: 100 Mbps ↓ / 50 Mbps ↑ / 150 Mbps burst        │
│  QoS Priority: Standard                                  │
│  SLA Uptime: 99.5%                                       │
│  Data Quota: Unlimited                                   │
│  Status: ● Active  │  Review: ✅ Approved                │
│                                                          │
│  Pricing Table:                                          │
│  ┌──────────┬────────┬──────────┬────────┐              │
│  │ Period   │ Price  │ Savings  │ Per/Mo │              │
│  ├──────────┼────────┼──────────┼────────┤              │
│  │ 1 Month  │ ₹600   │ —        │ ₹600   │              │
│  │ 3 Months │ ₹1,700 │ ₹100     │ ₹567   │              │
│  │ 6 Months │ ₹3,350 │ ₹250     │ ₹558   │              │
│  │ 12 Months│ ₹6,400 │ ₹800     │ ₹533   │              │
│  └──────────┴────────┴──────────┴────────┘              │
│                                                          │
│  Features:                                               │
│  ☑ Unlimited Data  ☑ Free Installation  ☑ 24/7 Support  │
│  ☑ Reliable Connection  ☑ Dual Band WiFi Router Free*   │
│                                                          │
│  Active Subscribers: 342                                 │
│  Monthly Revenue: ₹2,42,160                             │
└──────────────────────────────────────────────────────────┘
```

## 3. Plan Create/Edit Form

```typescript
interface PlanFormData {
  name: string;                    // "Standard 100 Mbps"
  slug: string;                    // "standard-100" (auto-generated)
  description?: string;
  speed_label: string;             // "100 Mbps"
  download_mbps: number;           // 100
  upload_mbps: number;             // 50
  burst_mbps?: number;             // 150
  data_quota: string;              // "unlimited"
  qos_priority: string;            // "standard" | "high" | "critical"
  sla_uptime_percent: number;      // 99.5
  is_popular: boolean;
  is_business: boolean;
  sort_order: number;
  // Pricing
  pricing: {
    1: number;   // Monthly price
    3: number;   // 3-month price
    6: number;   // 6-month price
    12: number;  // 12-month price
  };
  // Features
  features: string[];
}
```

## 4. Speed Profile Editor

Inline editor for speed profile configuration:

```
Speed Profile Configuration
├── Download Limit: [102400] kbps
├── Upload Limit: [51200] kbps
├── Burst Download: [153600] kbps
├── Burst Upload: [76800] kbps
├── Burst Duration: [30] seconds
├── Priority Queue: [2]
├── QoS Marking: [af21 ▼]
├── HTB Parent Queue: [1:1]
├── FQ-CoDel Enabled: [✓]
└── Device Type: [mikrotik ▼]
```

## 5. Checker/Maker Workflow

```
1. Staff creates/edits plan → status: "pending"
2. Reviewer sees plan in pending list
3. Reviewer can:
   a. Approve → plan becomes active
   b. Reject → plan stays pending with notes
   c. Request changes → back to editor
4. Only approved plans are visible to customers
```

## 6. API Endpoints

> **API Convention:** Protobuf-first. See `API-CONVENTIONS.md`.

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/api/v1/plans/list` | POST | List plans (public, active only) |
| `/api/v1/plans/get` | POST | Get plan details |
| `/api/v1/admin/plans/list` | POST | List all plans (including pending) |
| `/api/v1/admin/plans/create` | POST | Create plan |
| `/api/v1/admin/plans/update` | PATCH | Update plan |
| `/api/v1/admin/plans/delete` | DELETE | Soft-delete plan |
| `/api/v1/admin/plans/publish` | POST | Publish (activate) plan |
| `/api/v1/admin/plans/unpublish` | POST | Unpublish plan |
| `/api/v1/admin/plans/clone` | POST | Clone plan |
| `/api/v1/admin/plans/pricing/update` | PATCH | Update pricing |
| `/api/v1/admin/plans/speed-profile/list` | POST | Get speed profile |
| `/api/v1/admin/plans/speed-profile/update` | PATCH | Update speed profile |
| `/api/v1/admin/plans/history/list` | POST | Change history |

## 7. RBAC

| Action | Required Permission |
|--------|-------------------|
| View plans | `plan.view` |
| Create plan | `plan.create` |
| Edit plan | `plan.update` |
| Delete plan | `plan.delete` |
| Publish plan | `plan.publish` |
| Unpublish plan | `plan.unpublish` |
| Clone plan | `plan.clone` |
| Edit speed profile | `plan.speed_profile.update` |
