# AeroXe Backend — Notifications Module

> **Req Ref:** §9 Notification Platform

---

## 1. Overview

Multi-channel notification system supporting Email, SMS, WhatsApp, Push, and In-App notifications. Uses a template engine (Handlebars), queue-based delivery with retry logic, and exponential backoff for failures.

## 2. Notification Channels

| Channel | Provider | Use Case |
|---------|----------|----------|
| Email | SMTP (AWS SES / SendGrid) | Invoices, account updates |
| SMS | MSG91 / Twilio | OTPs, payment reminders |
| WhatsApp | WhatsApp Business API | Installation updates, support |
| Push | Firebase Cloud Messaging | Mobile app notifications |
| In-App | WebSocket | Dashboard alerts |

## 3. Database Tables

```sql
CREATE TABLE notification_templates (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL UNIQUE,
    channel VARCHAR(20) NOT NULL
        CHECK (channel IN ('email', 'sms', 'whatsapp', 'push', 'in_app')),
    subject_template TEXT,
    body_template TEXT NOT NULL,
    variables JSONB,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE notification_channels (
    id BIGSERIAL PRIMARY KEY,
    channel VARCHAR(20) NOT NULL UNIQUE,
    provider VARCHAR(50) NOT NULL,
    config JSONB NOT NULL,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE notifications (
    id BIGSERIAL PRIMARY KEY,
    template_id BIGINT REFERENCES notification_templates(id),
    channel VARCHAR(20) NOT NULL,
    recipient_type VARCHAR(20) NOT NULL,
    recipient_id BIGINT NOT NULL,
    recipient_address VARCHAR(255) NOT NULL,
    subject TEXT,
    body TEXT NOT NULL,
    variables JSONB,
    status VARCHAR(20) DEFAULT 'queued'
        CHECK (status IN ('queued', 'sent', 'delivered', 'failed', 'retrying')),
    retry_count INTEGER DEFAULT 0,
    max_retries INTEGER DEFAULT 3,
    last_error TEXT,
    sent_at TIMESTAMPTZ,
    delivered_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW()
) PARTITION BY RANGE (created_at);

CREATE TABLE notification_history (
    id BIGSERIAL PRIMARY KEY,
    notification_id BIGINT NOT NULL,
    event VARCHAR(50) NOT NULL,
    details JSONB,
    recorded_at TIMESTAMPTZ DEFAULT NOW()
) PARTITION BY RANGE (recorded_at);
```

## 4. Template Engine (Handlebars)

```handlebars
<!-- invoice_reminder template -->
Subject: Payment Reminder - Invoice {{invoice_number}}

Dear {{customer_name}},

This is a friendly reminder that your invoice {{invoice_number}} for
₹{{total_amount}} is due on {{due_date}}.

Please make the payment to avoid service interruption.

Payment Link: {{payment_url}}

Thank you,
AeroXe Broadband Team
```

## 5. Queue Architecture

```
Event Published (NATS)
    ↓
Notification Service
    ↓
Template Rendering (Handlebars)
    ↓
Queue (Redis List)
    ↓
Worker Pool
    ├── Email Worker → SMTP/API
    ├── SMS Worker → MSG91/Twilio
    ├── WhatsApp Worker → Business API
    ├── Push Worker → FCM
    └── In-App Worker → WebSocket
    ↓
Delivery Confirmation
    ↓
Update notification status
    ↓
Log to notification_history
```

## 6. Retry Strategy

```rust
pub fn calculate_retry_delay(retry_count: u32) -> Duration {
    // Exponential backoff: 10s, 30s, 90s
    let base_delay = 10;
    let multiplier = 3u64.pow(retry_count);
    Duration::from_secs(base_delay * multiplier)
}
```

## 7. API Endpoints

> **API Convention:** Protobuf-first. No GET, no PUT, no path variables, no query strings. See `API-CONVENTIONS.md`.

| Method | Path | Required Role | Description |
|--------|------|--------------|-------------|
| `POST` | `/api/v1/notifications/list` | notification.send | List notifications |
| `POST` | `/api/v1/notifications/send` | notification.send | Send notification |
| `POST` | `/api/v1/notifications/templates/list` | notification.template.manage | List templates |
| `POST` | `/api/v1/notifications/templates/create` | notification.template.manage | Create template |
| `PATCH` | `/api/v1/notifications/templates/update` | notification.template.manage | Update template |
| `POST` | `/api/v1/notifications/channels/list` | notification.channel.view | List channels |
| `PATCH` | `/api/v1/notifications/channels/update` | notification.channel.configure | Configure channel |
| `POST` | `/api/v1/notifications/history/list` | notification.send | Delivery history |
| `POST` | `/api/v1/notifications/retry` | notification.send | Retry failed notification |

## 8. Template Variables by Event

| Event | Template | Key Variables |
|-------|----------|--------------|
| invoice.generated | invoice_email | invoice_number, total_amount, due_date, payment_url |
| invoice.overdue | payment_reminder | invoice_number, days_overdue, total_amount |
| customer.activated | welcome_email | customer_name, plan_name, pppoe_username |
| installation.scheduled | installation_notify | scheduled_date, scheduled_time_slot, technician_name |
| ticket.created | ticket_confirm | ticket_number, subject, category |
| ticket.escalated | ticket_escalate | ticket_number, new_priority, reason |
| referral.rewarded | referral_reward | referrer_name, reward_amount, reward_type |

## 9. RBAC Permissions

```
notification.template.view
notification.template.create
notification.template.update
notification.template.delete
notification.channel.view
notification.channel.configure
notification.send
notification.retry
```
