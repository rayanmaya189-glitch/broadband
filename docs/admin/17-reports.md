# AeroXe Admin Portal — Reports Module

> **Req Ref:** §16 Admin Portal Requirements — Reports & Analytics

---

## 1. Overview

Business intelligence and reporting — revenue reports, customer analytics, network utilization, and operational metrics. Exportable to CSV/PDF with scheduled report generation.

## 2. Pages

### Reports Dashboard (`/reports`)

```
┌──────────────────────────────────────────────────────────┐
│  Reports & Analytics                                     │
├──────────────────────────────────────────────────────────┤
│  [Revenue] [Customers] [Network] [Operations] [Custom]   │
├──────────────────────────────────────────────────────────┤
│                                                          │
│  Quick Stats (This Month)                                │
│  ├── MRR: ₹5,20,000 (+8% vs last month)                │
│  ├── Total Revenue: ₹5,42,300                           │
│  ├── New Subscribers: 47                                │
│  ├── Churned: 12 (churn rate: 1.4%)                    │
│  ├── ARPU: ₹614                                        │
│  └── LTV: ₹7,368                                       │
│                                                          │
│  Revenue by Plan (Pie Chart)                             │
│  ├── Basic: ₹48,000 (9.2%)                             │
│  ├── Standard: ₹2,04,000 (39.2%)                       │
│  ├── Premium: ₹1,36,000 (26.2%)                        │
│  ├── Pro: ₹80,000 (15.4%)                              │
│  └── Ultimate: ₹52,000 (10.0%)                         │
│                                                          │
│  Revenue by Branch (Bar Chart)                           │
│  ├── Jalgaon: ₹4,20,000                                │
│  ├── Bhusawal: ₹85,000                                 │
│  └── Mumbai: ₹15,000                                   │
└──────────────────────────────────────────────────────────┘
```

### Revenue Report (`/reports/revenue`)

```
┌──────────────────────────────────────────────────────────┐
│  Revenue Report                     Date: [Range ▼]      │
│                                   Branch: [All ▼]        │
│                                   [Export CSV] [Export PDF]│
├──────────────────────────────────────────────────────────┤
│                                                          │
│  Revenue Trend (Line Chart — Last 12 months)            │
│  ████████████████████████████████████                    │
│  ████████████████████████████████████                    │
│                                                          │
│  Revenue Breakdown Table:                                │
│  ┌──────────────┬──────────┬──────────┬──────────┐      │
│  │ Month        │ MRR      │ One-Time │ Total    │      │
│  ├──────────────┼──────────┼──────────┼──────────┤      │
│  │ Jul 2026     │ ₹5,20,000│ ₹22,300 │ ₹5,42,300│     │
│  │ Jun 2026     │ ₹4,82,000│ ₹18,500 │ ₹5,00,500│     │
│  │ May 2026     │ ₹4,45,000│ ₹15,000 │ ₹4,60,000│     │
│  └──────────────┴──────────┴──────────┴──────────┘      │
└──────────────────────────────────────────────────────────┘
```

### Customer Report (`/reports/customers`)

- Subscriber growth over time
- Customer distribution by plan
- Churn analysis
- Customer lifetime value (LTV)
- Acquisition source breakdown
- Geographic distribution

### Network Report (`/reports/network`)

- Device uptime/availability
- Bandwidth utilization trends
- IP pool utilization
- PPPoE session statistics
- Network health score

### Operations Report

- Ticket resolution times
- SLA compliance rates
- Installation turnaround times
- Technician performance

## 3. Report Types

| Report | Frequency | Format | Recipients |
|--------|-----------|--------|------------|
| Daily Revenue Summary | Daily 8 AM | Email | finance_manager |
| Weekly Operations | Weekly Monday | Email | isp_owner, network_admin |
| Monthly Financial | Monthly 1st | PDF | finance_manager, isp_owner |
| SLA Compliance | Weekly | Email | noc_engineer, customer_support |
| Device Health | Daily | Dashboard | noc_engineer |

## 4. Custom Report Builder

```
Custom Report Builder:
├── Data Source: [Customers ▼]
├── Metrics: [Count, Revenue, Avg ARPU ▼]
├── Dimensions: [Plan, Branch, Month ▼]
├── Filters: [Status = Active ▼]
├── Date Range: [Last 12 months ▼]
├── [Generate Report]
└── [Save as Template] [Schedule]
```

## 5. API Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/api/v1/reports/summary` | GET | Dashboard summary |
| `/api/v1/reports/revenue` | GET | Revenue report |
| `/api/v1/reports/customers` | GET | Customer report |
| `/api/v1/reports/network` | GET | Network report |
| `/api/v1/reports/operations` | GET | Operations report |
| `/api/v1/reports/custom` | POST | Custom report query |
| `/api/v1/reports/export/:type` | GET | Export report (CSV/PDF) |
| `/api/v1/reports/schedule` | POST/GET | Schedule recurring reports |

## 6. RBAC

| Action | Required Permission |
|--------|-------------------|
| View reports | `report.view` |
| Generate report | `report.generate` |
| Export report | `report.export` |
| Schedule report | `report.schedule` |
