# AeroXe Admin Portal — Dashboard Module

> **Req Ref:** §16 Admin Portal Requirements — Dashboard

---

## 1. Overview

The main dashboard provides a real-time overview of business KPIs, operational metrics, and quick actions. Serves as the landing page for all admin users, with role-specific widgets.

## 2. Page Layout

```
┌──────────────────────────────────────────────────────────┐
│  Welcome, {user.name} | Branch: {selectedBranch}         │
├──────────┬──────────┬──────────┬──────────────────────────┤
│ Active   │ Monthly  │ Revenue  │ Open Tickets             │
│ Subs: 847│ Revenue  │ Today    │ 12 (3 critical)          │
│ +12 today│ ₹5.2L    │ ₹42,300  │                          │
├──────────┴──────────┴──────────┴──────────────────────────┤
│                                                           │
│  Revenue Chart (Line - Last 12 months)  │ Device Status   │
│  ████████████████████████████           │ Online: 45      │
│  ████████████████████████████           │ Offline: 3      │
│  ████████████████████████████           │ Degraded: 2     │
│                                         │                  │
├─────────────────────────────────────────┼──────────────────┤
│                                         │                  │
│  New Subscriptions (Bar - Last 30 days) │ Recent Tickets   │
│  ██                                     │ TKT-045: No inet │
│  ████                                   │ TKT-044: Slow    │
│  ██████                                 │ TKT-043: Billing │
│                                         │                  │
├─────────────────────────────────────────┴──────────────────┤
│                                                           │
│  Quick Actions                                            │
│  [+ New Customer] [+ Create Invoice] [+ New Ticket]      │
│  [+ Register Device] [+ Add Lead]                         │
│                                                           │
├───────────────────────────────────────────────────────────┤
│  Recent Activity Feed                                     │
│  • Customer #123 activated (5 min ago)                    │
│  • Invoice INV-0701 paid (12 min ago)                     │
│  • Device OLT-01 status: degraded (20 min ago)            │
│  • Ticket TKT-045 escalated (30 min ago)                  │
└───────────────────────────────────────────────────────────┘
```

## 3. KPI Cards

| Card | Metric | Trend | Color |
|------|--------|-------|-------|
| Active Subscriptions | Total active + delta vs yesterday | ↑/↓ | Green/Red |
| Monthly Revenue | Current month MRR | vs last month | Green/Red |
| Today's Payments | Amount collected today | vs yesterday | Green/Red |
| Open Tickets | Unresolved tickets by priority | critical count | Yellow/Red |
| Online Devices | Devices online / total | percentage | Green/Yellow/Red |
| Leads Pipeline | New leads this week | conversion rate | Blue |

## 4. Charts

### Revenue Trend (Line Chart)
- **X-axis:** Last 12 months
- **Y-axis:** Revenue in ₹
- **Series:** MRR, one-time fees
- **Hover:** Show exact amount, new subscribers, churn

### Subscription Growth (Bar Chart)
- **X-axis:** Last 30 days
- **Y-axis:** New subscriptions
- **Stacked by:** Plan type (Basic, Standard, Premium, Pro, Ultimate)

### Device Health (Donut Chart)
- **Segments:** Online (green), Offline (red), Degraded (yellow), Maintenance (blue)
- **Center text:** Total devices count

### Revenue by Branch (Bar Chart)
- **X-axis:** Branch names
- **Y-axis:** Revenue
- **Filterable by:** Date range

## 5. Real-Time Activity Feed

```typescript
interface ActivityFeedItem {
  id: string;
  type: 'customer' | 'invoice' | 'device' | 'ticket' | 'subscription';
  icon: string;
  title: string;
  description: string;
  timestamp: string;
  link?: string;
}
```

Updates live via WebSocket (`ws:noc:alerts` channel).

## 6. Quick Actions

| Action | Permission | Target |
|--------|-----------|--------|
| + New Customer | `customer.account.create` | `/customers/new` |
| + Create Invoice | `billing.invoice.generate` | `/billing/invoices/new` |
| + New Ticket | `ticket.create` | `/tickets/new` |
| + Register Device | `device.router.register` | `/devices/new` |
| + Add Lead | `lead.create` | `/leads/new` |
| + Send Notification | `notification.send` | Modal |

## 7. Role-Based Dashboard Widgets

| Widget | super_admin | isp_owner | network_admin | noc_engineer | finance_manager | customer_support |
|--------|:-----------:|:---------:|:-------------:|:------------:|:---------------:|:----------------:|
| Revenue KPIs | ✅ | ✅ | ❌ | ❌ | ✅ | ❌ |
| Device Health | ✅ | ✅ | ✅ | ✅ | ❌ | ❌ |
| Open Tickets | ✅ | ✅ | ✅ | ✅ | ❌ | ✅ |
| Subscription Growth | ✅ | ✅ | ❌ | ❌ | ✅ | ❌ |
| Activity Feed | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Quick Actions | ✅ | ✅ | Partial | Partial | Partial | Partial |
| Network Topology | ✅ | ✅ | ✅ | ✅ | ❌ | ❌ |
| Billing Alerts | ✅ | ✅ | ❌ | ❌ | ✅ | ❌ |

## 8. API Endpoints Used

> **API Convention:** Protobuf-first. See `API-CONVENTIONS.md`.

```
POST /api/v1/admin/dashboard/summary/list       → KPI card data
POST /api/v1/admin/dashboard/revenue/list       → Revenue chart data
POST /api/v1/admin/dashboard/subscriptions/list → Subscription growth data
POST /api/v1/admin/dashboard/devices/list       → Device health summary
POST /api/v1/admin/dashboard/tickets/list       → Open tickets summary
POST /api/v1/admin/dashboard/activity/list      → Recent activity feed
POST /api/v1/admin/dashboard/leads/list         → Lead pipeline summary
```

## 9. WebSocket Channels

```
ws:noc:alerts      → Device alerts, SLA breaches
ws:noc:devices     → Device status changes
ws:branch:{id}     → Branch-wide updates (if branch-scoped)
```

## 10. Date Range Filter

Global date range filter affecting all dashboard charts:
- **Preset ranges:** Today, Last 7 days, Last 30 days, This month, Last month, This quarter, This year, Custom
- **Default:** Last 30 days
