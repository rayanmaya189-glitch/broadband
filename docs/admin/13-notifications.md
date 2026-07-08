# AeroXe Admin Portal — Notifications Module

> **Req Ref:** §9 Notification Platform, §16 Admin Portal

---

## 1. Overview

Notification management — view sent notifications, manage templates, configure channels (email, SMS, WhatsApp), and retry failed deliveries.

## 2. Pages

### Notification List (`/notifications`)

```
┌──────────────────────────────────────────────────────────┐
│  Notifications                          [+ Send Manual]  │
├──────────────────────────────────────────────────────────┤
│  Channel: [All ▼]  Status: [All ▼]  Date: [Range ▼]     │
├──────────────────────────────────────────────────────────┤
│  Time       │ Channel │ Recipient    │ Subject          │ Status   │
│  10:30 AM   │ Email   │ Rahul S.     │ Invoice INV-001  │ ✅ Deliv │
│  10:15 AM   │ SMS     │ Priya P.     │ OTP: 847291      │ ✅ Sent  │
│  09:45 AM   │ WhatsApp│ Amit D.      │ Payment reminder │ ❌ Failed│
│  09:30 AM   │ Email   │ 123 customers│ Monthly newsletter│ ✅ Sent │
└──────────────────────────────────────────────────────────┘
```

### Template Management (`/notifications/templates`)

```
┌──────────────────────────────────────────────────────────┐
│  Notification Templates                    [+ Add]       │
├──────────────────────────────────────────────────────────┤
│  Name              │ Channel │ Last Used │ Status        │
│  invoice_email     │ Email   │ 847 times │ ● Active      │
│  payment_reminder  │ SMS     │ 234 times │ ● Active      │
│  installation_notify│ WhatsApp│ 89 times  │ ● Active      │
│  ticket_confirm    │ Email   │ 156 times │ ● Active      │
│  welcome_email     │ Email   │ 120 times │ ● Active      │
└──────────────────────────────────────────────────────────┘
```

### Template Editor

```
Template: invoice_email
Channel: Email
Subject: Payment Reminder - Invoice {{invoice_number}}

Body (Handlebars):
─────────────────────────────────────────
Dear {{customer_name}},

Your invoice {{invoice_number}} for ₹{{total_amount}} is due on {{due_date}}.

Payment Link: {{payment_url}}

Thank you,
AeroXe Broadband Team
─────────────────────────────────────────

Variables: customer_name, invoice_number, total_amount, due_date, payment_url
Preview: [Send Test] [Save] [Publish]
```

### Channel Configuration (`/notifications/channels`)

```
┌──────────────────────────────────────────────────────────┐
│  Notification Channels                                   │
├──────────────────────────────────────────────────────────┤
│  Channel  │ Provider    │ Status  │ Rate Limit │ Actions │
│  Email    │ AWS SES     │ ● Active│ 200/sec    │ [Config]│
│  SMS      │ MSG91       │ ● Active│ 10/sec     │ [Config]│
│  WhatsApp │ Business API│ ● Active│ 80/min     │ [Config]│
│  Push     │ FCM         │ ○ Inactive│ —        │ [Config]│
│  In-App   │ WebSocket   │ ● Active│ —          │ —       │
└──────────────────────────────────────────────────────────┘
```

## 3. API Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/api/v1/notifications` | GET | List notifications |
| `/api/v1/notifications/send` | POST | Send manual notification |
| `/api/v1/notifications/templates` | GET/POST | List/create templates |
| `/api/v1/notifications/templates/:id` | PUT/DELETE | Update/delete template |
| `/api/v1/notifications/channels` | GET | List channels |
| `/api/v1/notifications/channels/:id` | PUT | Configure channel |
| `/api/v1/notifications/:id/retry` | POST | Retry failed notification |
| `/api/v1/notifications/history` | GET | Delivery history |

## 4. RBAC

| Action | Required Permission |
|--------|-------------------|
| View notifications | `notification.send` |
| Send notification | `notification.send` |
| Manage templates | `notification.template.manage` |
| Configure channels | `notification.channel.configure` |
