# AeroXe Backend — Billing Module

> **Req Ref:** §8 Billing System, §8.8 Pro-Rata Billing, §8.9 Late Fee Engine

---

## 1. Overview

Manages the complete billing lifecycle: invoice generation, payment processing, refunds, discounts, dunning (overdue payment handling), and late fees. Invoices are generated automatically on subscription renewal and can be created manually.

## 2. Database Tables

```sql
CREATE TABLE invoices (
    id BIGSERIAL PRIMARY KEY,
    invoice_number VARCHAR(20) NOT NULL UNIQUE,
    customer_id BIGINT NOT NULL REFERENCES customers(id),
    branch_id BIGINT NOT NULL REFERENCES branches(id),
    subscription_id BIGINT NOT NULL REFERENCES subscriptions(id),
    billing_period_start DATE NOT NULL,
    billing_period_end DATE NOT NULL,
    subtotal DECIMAL(10,2) NOT NULL,
    discount_amount DECIMAL(10,2) DEFAULT 0,
    tax_amount DECIMAL(10,2) DEFAULT 0,
    total_amount DECIMAL(10,2) NOT NULL,
    currency VARCHAR(3) DEFAULT 'INR',
    status VARCHAR(20) DEFAULT 'draft'
        CHECK (status IN ('draft', 'pending', 'sent', 'paid', 'partial', 'overdue', 'void', 'refunded')),
    due_date DATE NOT NULL,
    paid_at TIMESTAMPTZ,
    payment_method VARCHAR(50),
    payment_reference VARCHAR(255),
    created_by BIGINT REFERENCES users(id),
    reviewed_by BIGINT REFERENCES users(id),
    reviewed_at TIMESTAMPTZ,
    review_status VARCHAR(20) DEFAULT 'pending'
        CHECK (review_status IN ('pending', 'approved', 'rejected')),
    review_notes TEXT,
    approved_by BIGINT REFERENCES users(id),
    approved_at TIMESTAMPTZ,
    notes TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE invoice_line_items (
    id BIGSERIAL PRIMARY KEY,
    invoice_id BIGINT NOT NULL REFERENCES invoices(id) ON DELETE CASCADE,
    description TEXT NOT NULL,
    quantity DECIMAL(10,2) DEFAULT 1,
    unit_price DECIMAL(10,2) NOT NULL,
    amount DECIMAL(10,2) NOT NULL,
    tax_rate DECIMAL(5,2) DEFAULT 0,
    tax_amount DECIMAL(10,2) DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE payments (
    id BIGSERIAL PRIMARY KEY,
    payment_number VARCHAR(20) NOT NULL UNIQUE,
    invoice_id BIGINT NOT NULL REFERENCES invoices(id),
    customer_id BIGINT NOT NULL REFERENCES customers(id),
    branch_id BIGINT NOT NULL REFERENCES branches(id),
    amount DECIMAL(10,2) NOT NULL,
    currency VARCHAR(3) DEFAULT 'INR',
    payment_method VARCHAR(50) NOT NULL,
    payment_gateway VARCHAR(50),
    gateway_transaction_id VARCHAR(255),
    gateway_response JSONB,
    status VARCHAR(20) DEFAULT 'pending'
        CHECK (status IN ('pending', 'processing', 'completed', 'failed', 'refunded')),
    processed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE refunds (
    id BIGSERIAL PRIMARY KEY,
    refund_number VARCHAR(20) NOT NULL UNIQUE,
    payment_id BIGINT NOT NULL REFERENCES payments(id),
    invoice_id BIGINT NOT NULL REFERENCES invoices(id),
    customer_id BIGINT NOT NULL REFERENCES customers(id),
    amount DECIMAL(10,2) NOT NULL,
    reason TEXT NOT NULL,
    requested_by BIGINT REFERENCES users(id),
    approved_by BIGINT REFERENCES users(id),
    approved_at TIMESTAMPTZ,
    review_notes TEXT,
    status VARCHAR(20) DEFAULT 'pending'
        CHECK (status IN ('pending', 'approved', 'processed', 'rejected')),
    processed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE discounts (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    code VARCHAR(50) UNIQUE,
    type VARCHAR(20) NOT NULL CHECK (type IN ('percentage', 'fixed')),
    value DECIMAL(10,2) NOT NULL,
    applicable_plan_ids JSONB,
    applicable_billing_periods INTEGER[],
    max_uses INTEGER,
    current_uses INTEGER DEFAULT 0,
    valid_from DATE NOT NULL,
    valid_until DATE NOT NULL,
    is_active BOOLEAN DEFAULT TRUE,
    created_by BIGINT REFERENCES users(id),
    reviewed_by BIGINT REFERENCES users(id),
    reviewed_at TIMESTAMPTZ,
    review_status VARCHAR(20) DEFAULT 'pending'
        CHECK (review_status IN ('pending', 'approved', 'rejected')),
    approved_by BIGINT REFERENCES users(id),
    approved_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE payment_reminders (
    id BIGSERIAL PRIMARY KEY,
    invoice_id BIGINT NOT NULL REFERENCES invoices(id),
    reminder_type VARCHAR(20) NOT NULL,
    channel VARCHAR(20) NOT NULL,
    sent_at TIMESTAMPTZ NOT NULL,
    status VARCHAR(20) DEFAULT 'sent'
        CHECK (status IN ('sent', 'delivered', 'failed')),
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- History tables
CREATE TABLE invoices_history (
    id BIGSERIAL PRIMARY KEY,
    invoice_id BIGINT NOT NULL,
    action VARCHAR(20) NOT NULL,
    old_data JSONB, new_data JSONB,
    performed_by BIGINT REFERENCES users(id),
    performed_at TIMESTAMPTZ DEFAULT NOW(),
    reason TEXT
);

CREATE TABLE refunds_history (
    id BIGSERIAL PRIMARY KEY,
    refund_id BIGINT NOT NULL,
    action VARCHAR(20) NOT NULL,
    old_data JSONB, new_data JSONB,
    performed_by BIGINT REFERENCES users(id),
    performed_at TIMESTAMPTZ DEFAULT NOW(),
    reason TEXT
);

CREATE TABLE discounts_history (
    id BIGSERIAL PRIMARY KEY,
    discount_id BIGINT NOT NULL,
    action VARCHAR(20) NOT NULL,
    old_data JSONB, new_data JSONB,
    performed_by BIGINT REFERENCES users(id),
    performed_at TIMESTAMPTZ DEFAULT NOW(),
    reason TEXT
);
```

## 3. API Endpoints

> **API Convention:** Protobuf-first. No GET, no PUT, no path variables, no query strings. See `API-CONVENTIONS.md`.

| Method | Path | Required Role | Description |
|--------|------|--------------|-------------|
| POST | `/api/v1/billing/invoices/list` | billing_ops | List invoices |
| POST | `/api/v1/billing/invoices/create` | billing_operator+ | Create invoice |
| POST | `/api/v1/billing/invoices/get` | billing_ops | Get invoice |
| PATCH | `/api/v1/billing/invoices/update` | billing_operator+ | Update invoice |
| POST | `/api/v1/billing/invoices/send` | billing_operator+ | Send invoice to customer |
| POST | `/api/v1/billing/invoices/void` | finance_manager+ | Void invoice |
| POST | `/api/v1/billing/invoices/pdf` | billing_ops | Generate invoice PDF |
| POST | `/api/v1/billing/payments/list` | billing_ops | List payments |
| POST | `/api/v1/billing/payments/create` | billing_operator+ | Record payment |
| POST | `/api/v1/billing/payments/get` | billing_ops | Get payment details |
| POST | `/api/v1/billing/refunds/create` | billing_operator+ | Request refund |
| PATCH | `/api/v1/billing/refunds/approve` | finance_manager+ | Approve refund |
| PATCH | `/api/v1/billing/refunds/reject` | finance_manager+ | Reject refund |
| POST | `/api/v1/billing/discounts/list` | billing_ops | List discounts |
| POST | `/api/v1/billing/discounts/create` | finance_manager+ | Create discount |
| PATCH | `/api/v1/billing/discounts/update` | finance_manager+ | Update discount |
| POST | `/api/v1/billing/dunning/config/get` | finance_manager+ | Get dunning config |
| PATCH | `/api/v1/billing/dunning/config/update` | finance_manager+ | Update dunning config |
| POST | `/api/v1/billing/tax/config/get` | finance_manager+ | Get tax config |
| PATCH | `/api/v1/billing/tax/config/update` | finance_manager+ | Update tax config |

## 4. Invoice Generation Flow

```
1. Trigger: subscription renewal OR manual creation
2. Calculate line items:
   - Plan price for billing period
   - Pro-rata adjustment (if mid-cycle upgrade/downgrade)
   - Late fees (if applicable)
   - Service package add-ons
3. Apply discount (if code provided)
4. Calculate tax (CGST 9% + SGST 9% for Maharashtra)
5. Generate invoice number: INV-{YYYY}-{MM}-{SEQUENCE}
6. Persist invoice + line items
7. Publish invoice.generated event
8. Send invoice to customer via notification
```

## 5. Dunning Flow

```
Day 0:  Invoice due date → status: 'overdue'
Day 3:  First reminder (SMS + Email)
Day 7:  Second reminder (WhatsApp + Email)
Day 10: Subscription suspended
Day 30: Customer terminated + final invoice generated
```

**Dunning config:**
```json
{
  "reminder_days": [3, 7],
  "suspension_day": 10,
  "termination_day": 30,
  "late_fee_percent": 2.0,
  "late_fee_cap_percent": 10.0,
  "channels": ["sms", "email", "whatsapp"]
}
```

## 6. Tax Configuration

```json
{
  "cgst_rate": 9.0,
  "sgst_rate": 9.0,
  "igst_rate": 18.0,
  "applicable_state": "Maharashtra",
  "hsn_code": "998421",
  "sac_code": "998421",
  "tax_name": "GST on Internet Services"
}
```

## 7. Invoice Number Generation

Format: `INV-{YYYY}-{MM}-{SEQUENCE}`

Example: `INV-2026-07-0001`

Uses Redis atomic counter per month for uniqueness.

## 8. Events Published

```yaml
invoice.generated:
  payload: { invoice_id, invoice_number, customer_id, total_amount, due_date }
invoice.sent:
  payload: { invoice_id, customer_id, channel }
invoice.paid:
  payload: { invoice_id, payment_id, amount, payment_method }
invoice.overdue:
  payload: { invoice_id, days_overdue, total_amount }
invoice.voided:
  payload: { invoice_id, reason }
payment.completed:
  payload: { payment_id, invoice_id, amount }
payment.failed:
  payload: { payment_id, invoice_id, reason }
refund.approved:
  payload: { refund_id, invoice_id, amount }
refund.processed:
  payload: { refund_id, invoice_id, amount }
subscription.suspended:
  payload: { customer_id, subscription_id, reason: "payment_overdue" }
```

## 9. RBAC Permissions

```
billing.invoice.view
billing.invoice.generate
billing.invoice.send
billing.invoice.void
billing.invoice.refund
billing.invoice.export
billing.payment.view
billing.payment.process
billing.payment.refund
billing.payment.reconcile
billing.discount.view
billing.discount.create
billing.discount.update
billing.discount.delete
billing.discount.apply
billing.tax.view
billing.tax.configure
billing.dunning.view
billing.dunning.configure
billing.dunning.execute
```

---

## Known Issues & Gap Reference (v2.0)

> **Cross-reference:** `GAP-code-bugs.md` §1, `GAP-security.md`, `DESIGN-GAPS-DEEP-ANALYSIS.md` §9.1

| Bug ID | Severity | Issue | Location |
|--------|----------|-------|----------|
| BUG-BILL-01 | CRITICAL | Pagination `_page`/`_limit` never used — full table loads | `service.rs:15-18` |
| BUG-BILL-02 | CRITICAL | GST always ₹0 — `tax_amount: Set(Decimal::ZERO)` | `service.rs:68-70` |
| BUG-BILL-03 | CRITICAL | Auto-generate ignores tax, discounts, proration | `service.rs:218-221` |
| BUG-BILL-04 | CRITICAL | Invoice number `timestamp_millis() % 10000` — collision possible | `service.rs:56-59` |
| BUG-BILL-05 | HIGH | Invoice delivery is no-op — only flips status, no email/SMS | `service.rs:252-261` |
| BUG-BILL-06 | MEDIUM | Dunning config returns hardcoded values, ignores branch_id | `service.rs:402-417` |
| BUG-BILL-07 | HIGH | Tax config hardcodes Maharashtra GST only — inter-state wrong | `service.rs:421-433` |
| BUG-BILL-08 | CRITICAL | ₹1 payment marks ₹5,000 invoice as "Paid" — revenue leakage | `service.rs:83-120` |
| BUG-BILL-09 | HIGH | Refund approval doesn't process money or reverse accounting | `service.rs:314-333` |
| BUG-BILL-10 | CRITICAL | Payment + invoice update not in DB transaction — double-credit race | `service.rs:111-118` |
| BUG-BILL-11 | HIGH | Domain aggregates bypassed — business rules in entities are dead code | `billing/domain/` |

**Priority:** Fix BILL-02, 04, 08, 10 first (data corruption). See `GAP-IMPLEMENTATION-ROADMAP.md` Phase 1-2.
