# AeroXe Admin Portal — Bandwidth Module

> **Req Ref:** §5 Bandwidth Control System, §16 Admin Portal

---

## 1. Overview

Bandwidth profile management — create, edit, and apply speed profiles to customers. Monitor bandwidth application status and usage tracking.

## 2. Pages

### Bandwidth Profiles (`/bandwidth/profiles`)

```
┌──────────────────────────────────────────────────────────┐
│  Bandwidth Profiles                          [+ Add]     │
├──────────────────────────────────────────────────────────┤
│  Name            │ Download │ Upload │ Plan    │ Status  │
│  Basic 50 Mbps   │ 50 Mbps  │ 25 Mbps│ Basic   │ ● Active│
│  Standard 100    │ 100 Mbps │ 50 Mbps│ Standard│ ● Active│
│  Premium 150     │ 150 Mbps │ 75 Mbps│ Premium │ ● Active│
│  Pro 200         │ 200 Mbps │ 100 Mbps│ Pro    │ ● Active│
│  Ultimate 300    │ 300 Mbps │ 150 Mbps│ Ultimate│ ● Active│
└──────────────────────────────────────────────────────────┘
```

### Profile Detail

```
Profile: Standard 100 Mbps              [Edit] [Apply to All] [Delete]
─────────────────────────────────────────────────────────
Download Limit: 102,400 kbps (100 Mbps)
Upload Limit: 51,200 kbps (50 Mbps)
Burst Download: 153,600 kbps
Burst Upload: 76,800 kbps
Burst Duration: 30 seconds
Priority Queue: 2
QoS Marking: af21
HTB Parent Queue: 1:1
FQ-CoDel: Enabled
Device Type: MikroTik

Plan: Standard 100 Mbps
Active Subscribers: 342

Application Status:
├── Applied: 338
├── Pending: 2
├── Failed: 2
└── [View Applications] [Retry Failed]
```

### Application Status (`/bandwidth/applications`)

```
┌──────────────────────────────────────────────────────────┐
│  Bandwidth Applications                                  │
├──────────────────────────────────────────────────────────┤
│  Customer      │ Profile      │ Device      │ Status  │ Retries │
│  Rahul Sharma  │ Standard 100 │ Router-R01  │ ✅ Applied│ 0     │
│  Priya Patil   │ Basic 50     │ Router-R02  │ ⏳ Pending│ 0     │
│  Amit Deshmukh │ Pro 200      │ Router-R03  │ ❌ Failed│ 3     │
│                │              │             │ Reason: Device unreachable │
│                │              │             │ [Retry] [View Details]     │
└──────────────────────────────────────────────────────────┘
```

## 3. API Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/api/v1/bandwidth/profiles` | GET/POST | List/create profiles |
| `/api/v1/bandwidth/profiles/:id` | GET/PUT/DELETE | CRUD profile |
| `/api/v1/bandwidth/profiles/:id/apply` | POST | Apply to all subscribers |
| `/api/v1/bandwidth/apply/:subscription_id` | POST | Apply to specific subscription |
| `/api/v1/bandwidth/applications` | GET | List application statuses |
| `/api/v1/bandwidth/usage/:subscription_id` | GET | Usage data |

## 4. RBAC

| Action | Required Permission |
|--------|-------------------|
| View profiles | `bandwidth.profile.view` |
| Create/update profile | `bandwidth.profile.update` |
| Apply profile | `bandwidth.profile.apply` |
| Delete profile | `bandwidth.profile.delete` |
