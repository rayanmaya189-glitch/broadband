# AeroXe Admin Portal — Subscriptions Module

> **Req Ref:** §3.6 Subscriptions, §16 Admin Portal

---

## 1. Overview

Manages customer subscriptions — view, create, upgrade, downgrade, suspend, reactivate, and cancel. Shows subscription status, billing period, and pro-rata adjustments.

## 2. Pages

### Subscription List (`/subscriptions`)

```
┌──────────────────────────────────────────────────────────┐
│  Subscriptions                                [+ Add] [Export] │
├──────────────────────────────────────────────────────────┤
│  Search: [____] Status: [All ▼] Plan: [All ▼] Branch: [All ▼] │
├──────────────────────────────────────────────────────────┤
│  ☐ │ Customer      │ Plan       │ Status │ Period │ Next Bill │ Amount │
│  ☐ │ Rahul Sharma  │ Std 100    │ ● Active│ Monthly│ Aug 1    │ ₹708   │
│  ☐ │ Priya Patil   │ Basic 50   │ ● Active│ 12-mo  │ Jul 2027 │ ₹4300  │
│  ☐ │ Amit Deshmukh │ Pro 200    │ ⏸ Susp.│ Monthly│ Overdue  │ ₹1180  │
└──────────────────────────────────────────────────────────┘
```

### Subscription Detail (`/subscriptions/:id`)

Shows:
- Customer info (linked)
- Plan details with speed profile
- Billing period & next billing date
- Payment history
- Pro-rata adjustment calculator
- Status transition buttons
- Subscription history (all changes)

## 3. Key Features

### Plan Upgrade/Downgrade
```
1. Staff selects new plan
2. System calculates pro-rata adjustment:
   - Credit for remaining old plan days
   - Charge for remaining new plan days
   - Net adjustment displayed
3. Staff confirms → subscription updated
4. Invoice generated for adjustment amount
5. Speed profile applied to device
6. Event published: subscription.upgraded/downgraded
```

### Suspend/Reactivate
```
Suspend:
1. Staff selects reason (payment_overdue, manual, violation)
2. Subscription status → suspended
3. Bandwidth profile removed from device
4. Customer notified
5. Invoice for remaining balance generated

Reactivate:
1. Staff confirms reactivation
2. Subscription status → active
3. Speed profile re-applied
4. Customer notified
```

## 4. API Endpoints

> **API Convention:** Protobuf-first. See `API-CONVENTIONS.md`.

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/api/v1/subscriptions/list` | POST | List subscriptions |
| `/api/v1/subscriptions/create` | POST | Create subscription |
| `/api/v1/subscriptions/get` | POST | Get subscription details |
| `/api/v1/subscriptions/update` | PATCH | Update subscription |
| `/api/v1/subscriptions/upgrade` | POST | Upgrade plan |
| `/api/v1/subscriptions/downgrade` | POST | Downgrade plan |
| `/api/v1/subscriptions/suspend` | POST | Suspend subscription |
| `/api/v1/subscriptions/reactivate` | POST | Reactivate subscription |
| `/api/v1/subscriptions/cancel` | POST | Cancel subscription |
| `/api/v1/subscriptions/history/list` | POST | Change history |

## 5. RBAC

| Action | Required Permission |
|--------|-------------------|
| View subscriptions | `customer.subscription.view` |
| Create subscription | `customer.subscription.create` |
| Upgrade plan | `customer.subscription.upgrade` |
| Downgrade plan | `customer.subscription.downgrade` |
| Suspend subscription | `customer.subscription.suspend` |
| Reactivate subscription | `customer.subscription.reactivate` |
| Cancel subscription | `customer.subscription.cancel` |
