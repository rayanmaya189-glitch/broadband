# AeroXe Admin Portal — Referrals Module

## Overview

The Referrals module allows administrators to create, manage, and monitor referral programs. Admins can configure reward rules, track referral performance, and view conversion analytics. The module integrates with the billing system for automatic reward processing.

---

## Pages

| Page | Route | Description |
|------|-------|-------------|
| Referral Programs | `/referrals/programs` | List all referral programs with status |
| Program Detail | `/referrals/programs/:id` | Program config, stats, and referral list |
| Referral Tracking | `/referrals/tracking` | All individual referrals across programs |
| Referral Analytics | `/referrals/analytics` | Funnel, conversion, and revenue impact |

---

## Referral Programs List

```
┌─────────────────────────────────────────────────────────────────┐
│  AeroXe Admin                                    👤 Admin  🔔   │
├─────────────────────────────────────────────────────────────────┤
│  📊 Dashboard  👥 Customers  💳 Billing  🎁 Referrals  ⚙️ Settings│
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  Referral Programs                    [+ Create Program]        │
│                                                                 │
│  ┌─ Filter ──────────────────────────────────────────────┐     │
│  │ Status: [All ▼]  Search: [________________] 🔍        │     │
│  └──────────────────────────────────────────────────────┘     │
│                                                                 │
│  ┌────────────────────────────────────────────────────────┐   │
│  │ Name          │ Status   │ Referrals │ Conv. │ Rewards │   │
│  ├────────────────────────────────────────────────────────┤   │
│  │ Launch 2026   │ Active   │ 1,247     │ 34%   │ ₹45,000│   │
│  │ Refer & Earn  │ Active   │ 892       │ 28%   │ ₹32,000│   │
│  │ Monsoon Offer │ Paused   │ 156       │ 42%   │ ₹8,500 │   │
│  │ Holiday 2025  │ Ended    │ 2,103     │ 31%   │ ₹89,000│   │
│  └────────────────────────────────────────────────────────┘   │
│                                                                 │
│  Showing 1-4 of 4 programs                                     │
└─────────────────────────────────────────────────────────────────┘
```

---

## Program Detail Page

```
┌─────────────────────────────────────────────────────────────────┐
│  ← Referral Programs / Launch 2026                              │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  ┌─ Program Info ──────────────────────────────────────────┐   │
│  │ Name: Launch 2026                                       │   │
│  │ Status: ● Active                                        │   │
│  │ Created: Jan 15, 2026                                   │   │
│  │ Validity: Jan 15 - Dec 31, 2026                         │   │
│  │ [Edit] [Pause] [End Program]                             │   │
│  └────────────────────────────────────────────────────────┘   │
│                                                                 │
│  ┌─ Reward Configuration ─────────────────────────────────┐   │
│  │                                                         │   │
│  │ Referrer Rewards:                                        │   │
│  │   Type: Credit          Amount: ₹200                    │   │
│  │   Max Referrals: 10 per customer                        │   │
│  │                                                         │   │
│  │ Referee Rewards:                                         │   │
│  │   Type: Discount        Amount: 10% off first month    │   │
│  │                                                         │   │
│  │ [Edit Configuration]                                     │   │
│  └────────────────────────────────────────────────────────┘   │
│                                                                 │
│  ┌─ Funnel ───────────────────────────────────────────────┐   │
│  │                                                         │   │
│  │  Total Referrals: 1,247                                 │   │
│  │  ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓ 100%               │   │
│  │                                                         │   │
│  │  Registered: 892                                        │   │
│  │  ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓░░░░░░░  72%               │   │
│  │                                                         │   │
│  │  Activated: 424                                         │   │
│  │  ▓▓▓▓▓▓▓▓▓▓▓▓░░░░░░░░░░░░░░░░░░░░  34%               │   │
│  │                                                         │   │
│  │  Rewarded: 424                                          │   │
│  │  ▓▓▓▓▓▓▓▓▓▓▓▓░░░░░░░░░░░░░░░░░░░░  34%               │   │
│  └────────────────────────────────────────────────────────┘   │
│                                                                 │
│  ┌─ Top Referrers ────────────────────────────────────────┐   │
│  │ #  Customer        Referrals  Converted  Rewards       │   │
│  │ 1  Rahul Patil     23         8          ₹1,600       │   │
│  │ 2  Priya Sharma    18         6          ₹1,200       │   │
│  │ 3  Amit Deshmukh   15         5          ₹1,000       │   │
│  │ [View All →]                                             │   │
│  └────────────────────────────────────────────────────────┘   │
│                                                                 │
│  ┌─ Referral List ────────────────────────────────────────┐   │
│  │ Referral Code │ Referrer     │ Referee     │ Status     │   │
│  │ RAHU2485      │ Rahul Patil  │ +9198765432 │ Activated  │   │
│  │ PRIY1823      │ Priya Sharma │ +9198765433 │ Registered │   │
│  │ AMIT1567      │ Amit Desh.   │ +9198765434 │ Pending    │   │
│  │ [Load More →]                                             │   │
│  └────────────────────────────────────────────────────────┘   │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## Referral Tracking Page

```
┌─────────────────────────────────────────────────────────────────┐
│  ← Referral Tracking                                            │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  ┌─ Filters ──────────────────────────────────────────────┐   │
│  │ Program: [All ▼]  Status: [All ▼]  Date: [Range 📅]    │   │
│  │ Search: [Referral code or phone] 🔍                     │   │
│  └──────────────────────────────────────────────────────┘     │
│                                                                 │
│  Status Summary:                                                │
│  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐         │
│  │ Pending  │ │Registered│ │Activated │ │ Rewarded │         │
│  │   355    │ │   468    │ │   424    │ │   424    │         │
│  └──────────┘ └──────────┘ └──────────┘ └──────────┘         │
│                                                                 │
│  ┌────────────────────────────────────────────────────────┐   │
│  │ Code     │ Program     │ Referrer   │ Referee  │ Status│   │
│  ├────────────────────────────────────────────────────────┤   │
│  │ RAHU2485 │ Launch 2026 │ Rahul P.   │ +9198..  │ Active│   │
│  │ PRIY1823 │ Launch 2026 │ Priya S.   │ +9198..  │ Reg.  │   │
│  │ AMIT1567 │ Launch 2026 │ Amit D.    │ +9198..  │ Pend. │   │
│  └────────────────────────────────────────────────────────┘   │
│                                                                 │
│  [Export CSV] [Export PDF]                                      │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## Referral Analytics Page

```
┌─────────────────────────────────────────────────────────────────┐
│  ← Referral Analytics                                           │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  ┌─ KPIs ────────────────────────────────────────────────┐    │
│  │                                                        │    │
│  │ Total Referrals    Active Programs   Avg. Conv. Rate  │    │
│  │     4,398              2                33%           │    │
│  │                                                        │    │
│  │ Total Rewards      Revenue Impact    Cost per Acq.    │    │
│  │   ₹174,500          +₹2,100,000         ₹412          │    │
│  │                                                        │    │
│  └────────────────────────────────────────────────────────┘   │
│                                                                 │
│  ── Conversion Funnel ─────────────────────────────────────    │
│                                                                 │
│  ┌────────────────────────────────────────────────────────┐   │
│  │  📊 Funnel Chart                                        │   │
│  │                                                         │   │
│  │  Shared    ████████████████████████████ 4,398 (100%)   │   │
│  │  Clicked   ███████████████████░░░░░░░░░ 3,200 (73%)   │   │
│  │  Registered████████████████░░░░░░░░░░░░ 2,600 (59%)   │   │
│  │  Activated ██████████░░░░░░░░░░░░░░░░░░ 1,450 (33%)   │   │
│  │  Rewarded  ██████████░░░░░░░░░░░░░░░░░░ 1,450 (33%)   │   │
│  │                                                         │   │
│  └────────────────────────────────────────────────────────┘   │
│                                                                 │
│  ── Monthly Trend ─────────────────────────────────────────    │
│                                                                 │
│  ┌────────────────────────────────────────────────────────┐   │
│  │  📈 Line Chart                                          │   │
│  │                                                         │   │
│  │  Referrals  ▏         ╱╲                                │   │
│  │  Activated  ▏      ╱╲╱  ╲╱╲                            │   │
│  │  Rewarded   ▏   ╱╲╱        ╲                           │   │
│  │             ──╱────────────────────                     │   │
│  │             Jan Feb Mar Apr May Jun                     │   │
│  │                                                         │   │
│  └────────────────────────────────────────────────────────┘   │
│                                                                 │
│  ── Revenue Impact ────────────────────────────────────────    │
│                                                                 │
│  ┌────────────────────────────────────────────────────────┐   │
│  │  New MRR from Referrals: ₹175,000/month                │   │
│  │  Customer Acquisition Cost (Referral): ₹412            │   │
│  │  Customer Acquisition Cost (Paid Ads): ₹1,200          │   │
│  │  Savings vs Paid Ads: ₹1,156,400/year                  │   │
│  └────────────────────────────────────────────────────────┘   │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## API Endpoints

### List Referral Programs
```
GET /api/v1/admin/referral-programs?page=1&limit=20&status=active

Response 200:
{
  "programs": [
    {
      "id": "prog_abc123",
      "name": "Launch 2026",
      "status": "active",
      "referrer_reward_type": "credit",
      "referrer_reward_value": 200,
      "referee_reward_type": "discount",
      "referee_reward_value": 10,
      "max_referrals_per_customer": 10,
      "start_date": "2026-01-15",
      "end_date": "2026-12-31",
      "stats": {
        "total_referrals": 1247,
        "registered": 892,
        "activated": 424,
        "rewarded": 424,
        "conversion_rate": 34.0,
        "total_rewards_given": 45000
      }
    }
  ],
  "pagination": { "page": 1, "limit": 20, "total": 4 }
}
```

### Create Referral Program
```
POST /api/v1/admin/referral-programs

Request:
{
  "name": "Monsoon Offer 2026",
  "referrer_reward_type": "credit",
  "referrer_reward_value": 300,
  "referee_reward_type": "discount",
  "referee_reward_value": 15,
  "max_referrals_per_customer": 5,
  "start_date": "2026-06-01",
  "end_date": "2026-08-31"
}

Response 201:
{
  "program": {
    "id": "prog_def456",
    "status": "active",
    "created_at": "2026-06-01T00:00:00Z"
  }
}
```

### Update Referral Program
```
PUT /api/v1/admin/referral-programs/:id

Request:
{
  "status": "paused",
  "referrer_reward_value": 250
}

Response 200:
{
  "program": { "id": "prog_abc123", "status": "paused", "updated_at": "..." }
}
```

### Get Referral Analytics
```
GET /api/v1/admin/referrals/analytics?program_id=prog_abc123&from=2026-01-01&to=2026-07-08

Response 200:
{
  "summary": {
    "total_referrals": 4398,
    "total_activated": 1450,
    "conversion_rate": 33.0,
    "total_rewards_given": 174500,
    "new_mrr_from_referrals": 175000,
    "cost_per_acquisition": 412
  },
  "funnel": {
    "shared": 4398,
    "clicked": 3200,
    "registered": 2600,
    "activated": 1450,
    "rewarded": 1450
  },
  "monthly_trend": [
    { "month": "2026-01", "referrals": 320, "activated": 105 },
    { "month": "2026-02", "referrals": 410, "activated": 142 },
    { "month": "2026-03", "referrals": 580, "activated": 198 }
  ],
  "top_referrers": [
    { "customer_id": "cust_001", "name": "Rahul Patil", "referrals": 23, "activated": 8, "rewards": 1600 }
  ]
}
```

### List All Referrals
```
GET /api/v1/admin/referrals?page=1&limit=50&program_id=prog_abc123&status=activated

Response 200:
{
  "referrals": [
    {
      "id": "ref_xyz789",
      "referral_code": "RAHU2485",
      "program": "Launch 2026",
      "referrer": { "id": "cust_001", "name": "Rahul Patil" },
      "referee": { "phone": "+919876543210", "name": null },
      "status": "activated",
      "referrer_reward_status": "credited",
      "referrer_reward_amount": 200,
      "referee_reward_status": "applied",
      "created_at": "2026-03-15T10:00:00Z",
      "activated_at": "2026-03-20T14:30:00Z"
    }
  ],
  "pagination": { "page": 1, "limit": 50, "total": 1450 }
}
```

### Export Referrals
```
GET /api/v1/admin/referrals/export?program_id=prog_abc123&format=csv

Response 200: CSV file download
```

---

## RBAC Permissions

| Permission | Roles |
|------------|-------|
| `referral.view` | finance_manager, super_admin |
| `referral.program.create` | finance_manager, super_admin |
| `referral.program.update` | finance_manager, super_admin |
| `referral.program.delete` | super_admin only |
| `referral.export` | finance_manager, super_admin |

---

## Referral Status Flow

```
┌──────────┐     ┌──────────────┐     ┌──────────────┐     ┌──────────┐
│  Pending  │────▶│  Registered  │────▶│  Activated   │────▶│ Rewarded │
│  (shared) │     │  (signed up) │     │  (subscribed)│     │          │
└──────────┘     └──────────────┘     └──────────────┘     └──────────┘
```

---

## Reward Types

| Type | Referrer | Referee | Example |
|------|----------|---------|---------|
| `credit` | Account credit | — | ₹200 credit to referrer account |
| `free_days` | Free subscription days | — | 7 days free for referrer |
| `plan_upgrade` | Temporary plan upgrade | — | Upgrade to next tier for 1 month |
| `discount` | — | Discount on first bill | 10% off first month for referee |

---

## Integration Points

| System | Integration |
|--------|-------------|
| **Billing** | Auto-credit referrer account on activation |
| **Subscriptions** | Apply discount to referee's first invoice |
| **Accounting** | Journal entries for reward liabilities |
| **Notifications** | Send reward notifications to referrer and referee |
| **Events** | Publish `referral.created`, `activated`, `rewarded` |
