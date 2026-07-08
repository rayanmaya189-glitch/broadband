# AeroXe Admin Portal — Settings Module

> **Req Ref:** §16 Admin Portal Requirements

---

## 1. Overview

System-wide configuration — billing settings, notification channels, tax configuration, dunning rules, and general platform settings.

## 2. Pages

### General Settings (`/settings`)

```
┌──────────────────────────────────────────────────────────┐
│  System Settings                                         │
├──────────────────────────────────────────────────────────┤
│  [General] [Billing] [Notifications] [Security] [API]    │
├──────────────────────────────────────────────────────────┤
│                                                          │
│  General Settings                                        │
│  ├── Company Name: AeroXe Broadband                     │
│  ├── Legal Name: Aeroxe Enterprises Pvt. Ltd.           │
│  ├── Domain: aeroxebroadband.com                         │
│  ├── Support Phone: +91 77700 33326                     │
│  ├── Support Email: support@aeroxe.com                  │
│  ├── Default Timezone: Asia/Kolkata                      │
│  ├── Default Currency: INR                               │
│  └── [Save Changes]                                      │
└──────────────────────────────────────────────────────────┘
```

### Billing Settings (`/settings/billing`)

```
┌──────────────────────────────────────────────────────────┐
│  Billing Settings                                        │
├──────────────────────────────────────────────────────────┤
│                                                          │
│  Tax Configuration                                       │
│  ├── CGST Rate: [9.0] %                                 │
│  ├── SGST Rate: [9.0] %                                 │
│  ├── IGST Rate: [18.0] %                                │
│  ├── HSN/SAC Code: [998421]                              │
│  └── Applicable State: [Maharashtra ▼]                   │
│                                                          │
│  Dunning Configuration                                   │
│  ├── Reminder Days: [3] [7]                              │
│  ├── Suspension Day: [10]                                │
│  ├── Termination Day: [30]                               │
│  ├── Late Fee Percent: [2.0] %                           │
│  ├── Late Fee Cap: [10.0] %                              │
│  └── Reminder Channels: [✓ SMS] [✓ Email] [✓ WhatsApp]  │
│                                                          │
│  Invoice Settings                                        │
│  ├── Invoice Prefix: [INV]                               │
│  ├── Payment Due Days: [10]                              │
│  ├── Auto-send Invoices: [✓ Enabled]                     │
│  └── Invoice Footer: [________]                          │
│                                                          │
│  [Save Changes]                                          │
└──────────────────────────────────────────────────────────┘
```

### Notification Settings (`/settings/notifications`)

```
┌──────────────────────────────────────────────────────────┐
│  Notification Settings                                   │
├──────────────────────────────────────────────────────────┤
│                                                          │
│  Email (AWS SES)                                         │
│  ├── SMTP Host: [email-smtp.ap-south-1.amazonaws.com]   │
│  ├── SMTP Port: [587]                                    │
│  ├── Username: [****]                                    │
│  ├── Password: [****]                                    │
│  ├── From Name: [AeroXe Broadband]                       │
│  ├── From Email: [noreply@aeroxebroadband.com]          │
│  └── [Test Send] [Save]                                  │
│                                                          │
│  SMS (MSG91)                                             │
│  ├── API Key: [****]                                     │
│  ├── Sender ID: [AEROXE]                                 │
│  ├── Route: [Transactional ▼]                            │
│  └── [Test Send] [Save]                                  │
│                                                          │
│  WhatsApp (Business API)                                 │
│  ├── API Token: [****]                                   │
│  ├── Phone Number ID: [****]                             │
│  └── [Test Send] [Save]                                  │
└──────────────────────────────────────────────────────────┘
```

### Security Settings (`/settings/security`)

```
┌──────────────────────────────────────────────────────────┐
│  Security Settings                                       │
├──────────────────────────────────────────────────────────┤
│                                                          │
│  Authentication                                          │
│  ├── Enforce 2FA for Admin: [✓ Enabled]                  │
│  ├── Password Min Length: [12]                           │
│  ├── Session Timeout: [24] hours                         │
│  ├── Max Login Attempts: [5]                             │
│  └── Lockout Duration: [30] minutes                      │
│                                                          │
│  API Security                                            │
│  ├── Rate Limit (Default): [100] req/min                 │
│  ├── Rate Limit (Auth): [5] req/min                      │
│  ├── CORS Origins: [aeroxebroadband.com, admin.aeroxe..] │
│  └── API Key Rotation: [90] days                         │
│                                                          │
│  [Save Changes]                                          │
└──────────────────────────────────────────────────────────┘
```

## 3. API Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/api/v1/admin/settings` | GET/PUT | Get/update general settings |
| `/api/v1/admin/settings/billing` | GET/PUT | Billing settings |
| `/api/v1/admin/settings/notifications` | GET/PUT | Notification settings |
| `/api/v1/admin/settings/security` | GET/PUT | Security settings |
| `/api/v1/admin/settings/test-email` | POST | Send test email |
| `/api/v1/admin/settings/test-sms` | POST | Send test SMS |

## 4. RBAC

| Action | Required Permission |
|--------|-------------------|
| View settings | `settings.view` |
| Update settings | `settings.update` |
| Security settings | `security.config.update` |
