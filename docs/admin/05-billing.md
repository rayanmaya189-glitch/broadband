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

## 7. Manual Payments & Top-Up (§8C)

### Manual Payments List (`/billing/manual-payments`)

```
┌──────────────────────────────────────────────────────────┐
│  Manual Payments                  [+ Record Payment] [Export] │
├──────────────────────────────────────────────────────────┤
│  Status: [All ▼] Method: [All ▼] Date: [Range ▼]       │
├──────────────────────────────────────────────────────────┤
│  ☐ │ Ref #      │ Customer    │ Amount │ Method    │ Status        │
│  ☐ │ MP-0045    │ Rahul S.    │ ₹708   │ Cash      │ ● Approved   │
│  ☐ │ MP-0044    │ Priya P.    │ ₹472   │ Bank Xfer │ ● Pending    │
│  ☐ │ MP-0043    │ Amit D.     │ ₹1,180 │ Cheque    │ ● Rejected   │
└──────────────────────────────────────────────────────────┘
```

### Record Manual Payment (`/billing/manual-payments/create`)

```
┌──────────────────────────────────────────────────────────┐
│  Record Manual Payment                                   │
├──────────────────────────────────────────────────────────┤
│                                                          │
│  Customer: [Select Customer ▼]                           │
│  Invoice:  [Select Invoice ▼]                            │
│                                                          │
│  Amount: ₹ [________]                                    │
│  Method: ○ Cash  ○ Bank Transfer  ○ Cheque              │
│                                                          │
│  Reference #: [________]  (receipt/transaction #)        │
│  Bank Name:    [________]  (if transfer/cheque)         │
│  Notes:         [________________________]              │
│                                                          │
│  Proof: [📎 Upload Receipt/Screenshot]                   │
│                                                          │
│  ┌─────────────────────────────────────┐                │
│  │        Submit for Approval →        │                │
│  └─────────────────────────────────────┘                │
│                                                          │
│  ⚠️ This will be sent to a finance manager for approval. │
└──────────────────────────────────────────────────────────┘
```

### Manual Payment Detail (`/billing/manual-payments/:id`)

```
┌──────────────────────────────────────────────────────────┐
│  Manual Payment MP-0044                                  │
├──────────────────────────────────────────────────────────┤
│                                                          │
│  Customer: Priya Patil                                   │
│  Invoice:  INV-2026-07-002 (₹472)                       │
│  Amount:   ₹472                                         │
│  Method:   Bank Transfer                                │
│  Reference: TXN-20260708-4521                           │
│  Bank:     State Bank of India                           │
│                                                          │
│  Proof: [📄 receipt.pdf] [🖼️ screenshot.jpg]              │
│                                                          │
│  Status: ● Pending Approval                              │
│  Created by: billing@aeroxe.com (Jul 8, 10:30 AM)       │
│                                                          │
│  ── Approval History ────────────────────────────────── │
│  Jul 8, 10:30 AM — Submitted by billing@aeroxe.com     │
│  Jul 8, 11:00 AM — Pending review by finance_manager    │
│                                                          │
│  [✅ Approve] [❌ Reject] [💬 Add Comment]               │
│                                                          │
│  ⚠️ Approval will mark invoice as paid and generate     │
│  accounting journal entry automatically.                 │
└──────────────────────────────────────────────────────────┘
```

### Maker/Checker Workflow

```
┌──────────┐     ┌──────────────┐     ┌──────────────┐     ┌──────────┐
│  Maker    │────▶│  Pending     │────▶│  Checker     │────▶│ Approved │
│  (staff)  │     │  Approval    │     │  (finance)   │     │          │
└──────────┘     └──────────────┘     └──────┬───────┘     └──────────┘
                                              │
                                              ▼
                                        ┌──────────┐
                                        │ Rejected │
                                        └──────────┘
```

| Step | Actor | Action |
|------|-------|--------|
| 1 | Staff (maker) | Creates payment record, uploads proof |
| 2 | System | Sets status to `pending_approval` |
| 3 | Finance Manager (checker) | Reviews proof and details |
| 4a | Finance Manager | Approves → status: `approved` |
| 4b | Finance Manager | Rejects → status: `rejected` |
| 5 | System | On approval: marks invoice paid, generates journal entry |
| 6 | System | Notifies customer of payment confirmation |

### Customer Top-Up & Wallet

#### Wallet Overview (`/billing/wallets`)

```
┌──────────────────────────────────────────────────────────┐
│  Customer Wallets                                        │
├──────────────────────────────────────────────────────────┤
│  Search: [____] Balance: [Min ▼] [Max ▼]                 │
├──────────────────────────────────────────────────────────┤
│  Customer     │ Balance  │ Total Earned │ Total Used  │
│  Rahul S.     │ ₹1,200   │ ₹1,600      │ ₹400        │
│  Priya P.     │ ₹300     │ ₹500        │ ₹200        │
│  Amit D.      │ ₹0       │ ₹200        │ ₹200        │
└──────────────────────────────────────────────────────────┘
```

#### Wallet Detail (`/billing/wallets/:id`)

```
┌──────────────────────────────────────────────────────────┐
│  Wallet — Rahul Sharma                                   │
├──────────────────────────────────────────────────────────┤
│                                                          │
│  Balance: ₹1,200  │  Earned: ₹1,600  │  Used: ₹400     │
│                                                          │
│  ── Transaction History ────────────────────────────── │
│  Date        │ Type   │ Amount │ Reference     │ Note   │
│  Jul 5       │ Credit │ +₹200  │ Referral      │ Amit D.│
│  Jun 28      │ Debit  │ -₹200  │ INV-2026-06-12│ Auto   │
│  Jun 15      │ Credit │ +₹200  │ Referral      │ Priya P│
│  Jun 1       │ Credit │ +₹200  │ Referral      │ Vikram │
│  May 20      │ Debit  │ -₹200  │ INV-2026-05-08│ Auto   │
│                                                          │
│  [💰 Manual Credit] [📉 Manual Debit] [Export]           │
└──────────────────────────────────────────────────────────┘
```

#### Manual Wallet Adjustment

```
┌──────────────────────────────────────────────────────────┐
│  Wallet Adjustment — Rahul Sharma                        │
├──────────────────────────────────────────────────────────┤
│                                                          │
│  Current Balance: ₹1,200                                 │
│                                                          │
│  Adjustment Type: ○ Credit (+)  ○ Debit (-)             │
│  Amount: ₹ [________]                                    │
│  Reason: [________________________]                      │
│                                                          │
│  ⚠️ This will create a journal entry and notify the     │
│  customer.                                               │
│                                                          │
│  ┌─────────────────────────────────────┐                │
│  │      Apply Adjustment →             │                │
│  └─────────────────────────────────────┘                │
└──────────────────────────────────────────────────────────┘
```

### Top-Up Flow (Customer-Initiated)

```
1. Customer requests top-up (min ₹100) via mobile app
2. System generates payment link (Razorpay)
3. Customer pays via UPI/Card/Net Banking
4. Payment confirmed → credit customer wallet
5. Generate journal entry:
   Dr. Bank/Cash  ₹{amount}
   Cr. Customer Wallet (Liability)  ₹{amount}
6. Notify customer: "₹{amount} added to your wallet"
```

### Wallet Auto-Application

```
When invoice is generated:
1. Check customer wallet balance
2. If balance > 0:
   a. Apply min(balance, invoice_amount) to invoice
   b. Debit wallet
   c. Add line item: "Wallet Credit: -₹{applied}"
   d. Generate journal entry:
      Dr. Customer Wallet (Liability)  ₹{applied}
      Cr. Accounts Receivable  ₹{applied}
3. Excess balance carries forward to next invoice
```

### Manual Payment API Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/api/v1/admin/manual-payments` | GET | List manual payments |
| `/api/v1/admin/manual-payments` | POST | Create manual payment |
| `/api/v1/admin/manual-payments/:id` | GET | Get payment detail |
| `/api/v1/admin/manual-payments/:id/approve` | POST | Approve payment |
| `/api/v1/admin/manual-payments/:id/reject` | POST | Reject payment |
| `/api/v1/admin/wallets` | GET | List customer wallets |
| `/api/v1/admin/wallets/:id` | GET | Get wallet detail |
| `/api/v1/admin/wallets/:id/transactions` | GET | Wallet transactions |
| `/api/v1/admin/wallets/:id/adjust` | POST | Manual wallet adjustment |
| `/api/v1/customer/wallet` | GET | Get own wallet (customer) |
| `/api/v1/customer/wallet/topup` | POST | Initiate top-up (customer) |

### Manual Payment RBAC

| Action | Required Permission |
|--------|-------------------|
| View manual payments | `manual_payment.view` |
| Create manual payment | `manual_payment.create` |
| Approve manual payment | `manual_payment.approve` |
| Reject manual payment | `manual_payment.reject` |
| View wallets | `wallet.view` |
| Credit wallet | `wallet.credit` |
| Debit wallet | `wallet.debit` |
| Adjust wallet | `wallet.adjust` |
