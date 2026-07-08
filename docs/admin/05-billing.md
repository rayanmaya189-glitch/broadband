# AeroXe Admin Portal — Billing Module

> **Req Ref:** §8 Billing System, §16 Admin Portal

---

## 1. Overview

Complete billing management — invoices, payments, refunds, discounts, dunning configuration, and tax settings. Includes invoice PDF generation and payment link creation.

## 2. Pages

### Invoice List (`/billing/invoices`)

```
┌──────────────────────────────────────────────────────────┐
│  Invoices                    [+ Create] [Export] [Bulk Send] │
├──────────────────────────────────────────────────────────┤
│  Search: [____] Status: [All ▼] Date: [Range ▼] Branch: [All ▼] │
├──────────────────────────────────────────────────────────┤
│  ☐ │ Invoice #      │ Customer    │ Amount │ Due Date │ Status  │
│  ☐ │ INV-2026-07-001│ Rahul S.    │ ₹708   │ Jul 10   │ ● Paid  │
│  ☐ │ INV-2026-07-002│ Priya P.    │ ₹472   │ Jul 10   │ ● Overdue│
│  ☐ │ INV-2026-07-003│ Amit D.     │ ₹1,180 │ Jul 15   │ ● Pending│
└──────────────────────────────────────────────────────────┘
```

### Invoice Detail (`/billing/invoices/:id`)

```
┌──────────────────────────────────────────────────────────┐
│  Invoice INV-2026-07-0001           [PDF] [Send] [Record Payment] │
├──────────────────────────────────────────────────────────┤
│                                                          │
│  ┌──────────────────────────────────────────────────┐   │
│  │  AeroXe Broadband                                │   │
│  │  Aeroxe Enterprises Pvt. Ltd.                    │   │
│  │  Jalgaon, Maharashtra                            │   │
│  │                                                  │   │
│  │  Bill To: Rahul Sharma                           │   │
│  │  42, Shivaji Nagar, Jalgaon 425001              │   │
│  │                                                  │   │
│  │  Invoice #: INV-2026-07-0001                    │   │
│  │  Date: Jul 1, 2026                               │   │
│  │  Due Date: Jul 10, 2026                          │   │
│  │                                                  │   │
│  │  ┌─────────────────────┬─────┬───────┬────────┐ │   │
│  │  │ Description         │ Qty │ Rate  │ Amount │ │   │
│  │  ├─────────────────────┼─────┼───────┼────────┤ │   │
│  │  │ Standard 100 Mbps   │  1  │ ₹600  │ ₹600   │ │   │
│  │  │ — July 2026         │     │       │        │ │   │
│  │  ├─────────────────────┼─────┼───────┼────────┤ │   │
│  │  │ Subtotal            │     │       │ ₹600   │ │   │
│  │  │ CGST (9%)           │     │       │ ₹54    │ │   │
│  │  │ SGST (9%)           │     │       │ ₹54    │ │   │
│  │  │ Total               │     │       │ ₹708   │ │   │
│  │  └─────────────────────┴─────┴───────┴────────┘ │   │
│  └──────────────────────────────────────────────────┘   │
│                                                          │
│  Status: ● Paid  │  Paid: Jul 5, 2026                   │
│  Payment Method: UPI  │  Reference: pay_ABC123           │
└──────────────────────────────────────────────────────────┘
```

### Payment List (`/billing/payments`)

Shows all payments with filtering by status, method, date, customer.

### Refund List (`/billing/refunds`)

Shows refund requests with approval workflow.

### Discount Management (`/billing/discounts`)

CRUD for discount codes with validation rules.

## 3. Key Features

### Create Invoice
```
1. Select customer → auto-fill subscription details
2. System calculates line items from plan pricing
3. Apply discount (if code provided)
4. Calculate tax (CGST + SGST for Maharashtra)
5. Preview invoice → Confirm → Generate PDF
6. Send to customer via email/WhatsApp
```

### Record Payment
```
1. Staff selects invoice
2. Records payment method (UPI, cash, card, net banking)
3. Enters amount, reference number
4. System marks invoice as "paid"
5. Generates journal entry (accounting module)
6. Publishes invoice.paid event
```

### Refund Workflow
```
1. Staff requests refund with reason
2. System checks: amount > ₹5,000 requires approval
3. If approval needed → sends to finance_manager
4. Finance manager approves/rejects
5. If approved → processes refund via original payment method
6. Generates reverse journal entry
7. Notifies customer
```

### Dunning (Overdue Handling)
```
Dashboard shows overdue invoices:
- Day 1-3: Reminder badge (yellow)
- Day 3-7: Warning badge (orange)
- Day 7-10: Critical badge (red) — suspension pending
- Day 10+: Auto-suspended

Quick actions: Send Reminder, Suspend, Reactivate
```

## 4. Invoice Number Format

`INV-{YYYY}-{MM}-{SEQUENCE}` (e.g., `INV-2026-07-0001`)

Auto-generated on creation using Redis atomic counter.

## 5. API Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/api/v1/billing/invoices` | GET | List invoices |
| `/api/v1/billing/invoices` | POST | Create invoice |
| `/api/v1/billing/invoices/:id` | GET | Get invoice details |
| `/api/v1/billing/invoices/:id` | PUT | Update invoice |
| `/api/v1/billing/invoices/:id/send` | POST | Send to customer |
| `/api/v1/billing/invoices/:id/void` | POST | Void invoice |
| `/api/v1/billing/invoices/:id/pdf` | GET | Download PDF |
| `/api/v1/billing/payments` | GET | List payments |
| `/api/v1/billing/payments` | POST | Record payment |
| `/api/v1/billing/refunds` | POST | Request refund |
| `/api/v1/billing/refunds/:id/approve` | POST | Approve refund |
| `/api/v1/billing/refunds/:id/reject` | POST | Reject refund |
| `/api/v1/billing/discounts` | GET/POST | List/create discounts |
| `/api/v1/billing/discounts/:id` | PUT/DELETE | Update/delete discount |
| `/api/v1/billing/payment-link` | POST | Generate payment link |

## 6. RBAC

| Action | Required Permission |
|--------|-------------------|
| View invoices | `billing.invoice.view` |
| Create invoice | `billing.invoice.generate` |
| Send invoice | `billing.invoice.send` |
| Void invoice | `billing.invoice.void` |
| View payments | `billing.payment.view` |
| Record payment | `billing.payment.process` |
| Request refund | `billing.payment.refund` |
| Approve refund > ₹5K | `billing.invoice.refund` + approval |
| Manage discounts | `billing.discount.create` |
| Configure dunning | `billing.dunning.configure` |
