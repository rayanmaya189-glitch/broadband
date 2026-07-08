# AeroXe Admin Portal — Accounting Module

> **Req Ref:** §8A General Ledger & Double-Entry Accounting, §16 Admin Portal

---

## 1. Overview

Financial accounting dashboard — Chart of Accounts, journal entries, trial balance, financial statements (P&L, Balance Sheet, Cash Flow), and GST filing data generation.

## 2. Pages

### Journal Entries (`/accounting/journal`)

```
┌──────────────────────────────────────────────────────────┐
│  Journal Entries                           [+ New Entry] │
├──────────────────────────────────────────────────────────┤
│  Date: [Range ▼] Account: [All ▼] Status: [All ▼]      │
├──────────────────────────────────────────────────────────┤
│  Entry #        │ Date       │ Description        │ Total  │ Status  │
│  JE-2026-07-001 │ Jul 1      │ Invoice INV-0001   │ ₹708   │ Posted  │
│  JE-2026-07-002 │ Jul 1      │ Invoice INV-0002   │ ₹472   │ Posted  │
│  JE-2026-07-003 │ Jul 5      │ Payment INV-0001   │ ₹708   │ Posted  │
│  JE-2026-07-004 │ Jul 8      │ Refund INV-0003    │ ₹354   │ Draft   │
└──────────────────────────────────────────────────────────┘
```

### Journal Entry Detail

```
Entry JE-2026-07-001 — Invoice Generated
Date: Jul 1, 2026  │  Status: Posted  │  Created by: Admin
─────────────────────────────────────────────────────────
Account                          │ Debit      │ Credit
─────────────────────────────────┼────────────┼──────────
1200 Accounts Receivable         │ ₹708.00    │
4000 Subscription Revenue        │            │ ₹600.00
2100 GST Payable - CGST          │            │ ₹54.00
2100 GST Payable - SGST          │            │ ₹54.00
─────────────────────────────────┼────────────┼──────────
Total                            │ ₹708.00    │ ₹708.00
```

### Trial Balance (`/accounting/trial-balance`)

Period selector → Generated trial balance table:
```
Account Code │ Account Name              │ Debit      │ Credit
─────────────┼───────────────────────────┼────────────┼──────────
1000         │ Cash in Hand              │ ₹15,000    │
1100         │ Bank Accounts             │ ₹2,50,000  │
1200         │ Accounts Receivable       │ ₹45,000    │
2100         │ GST Payable               │            │ ₹32,000
4000         │ Subscription Revenue      │            │ ₹5,20,000
5000         │ Bandwidth Cost            │ ₹1,80,000  │
─────────────┴───────────────────────────┼────────────┼──────────
Total                                     │ ₹5,20,000  │ ₹5,20,000
```

### Financial Statements (`/accounting/statements`)

Tabs: Profit & Loss | Balance Sheet | Cash Flow

### GST Returns (`/accounting/gst`)

GSTR-1 and GSTR-3B data generation with export.

## 3. Chart of Accounts Viewer

```
ASSETS (1xxx)
├── 1000 Cash in Hand               ₹15,000
├── 1100 Bank Accounts              ₹2,50,000
├── 1200 Accounts Receivable        ₹45,000
├── 1300 Prepaid Expenses           ₹8,000
├── 1400 Inventory (Hardware)       ₹1,20,000
└── 1500 Fixed Assets               ₹5,00,000

LIABILITIES (2xxx)
├── 2000 Accounts Payable           ₹25,000
├── 2100 GST Payable                ₹32,000
├── 2200 GST Receivable             ₹12,000
├── 2300 Customer Wallet Balance    ₹5,000
└── 2400 Advance Received           ₹15,000

EQUITY (3xxx)
├── 3000 Owner's Equity             ₹10,00,000
└── 3100 Retained Earnings          ₹2,50,000

REVENUE (4xxx)
├── 4000 Subscription Revenue       ₹5,20,000
├── 4100 Installation Revenue       ₹0
├── 4200 Hardware Sales Revenue     ₹15,000
└── 4400 Late Fee Revenue           ₹2,000

EXPENSES (5xxx)
├── 5000 Bandwidth Cost             ₹1,80,000
├── 5100 Hardware Cost              ₹45,000
├── 5200 Staff Salaries             ₹1,50,000
├── 5300 Marketing Expense          ₹10,000
└── 5500 Payment Gateway Fees       ₹8,000
```

## 4. API Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/api/v1/accounting/accounts` | GET | Chart of accounts |
| `/api/v1/accounting/accounts` | POST | Create account |
| `/api/v1/accounting/journal` | GET | List journal entries |
| `/api/v1/accounting/journal` | POST | Create journal entry |
| `/api/v1/accounting/journal/:id` | GET | Get entry details |
| `/api/v1/accounting/journal/:id/post` | POST | Post entry |
| `/api/v1/accounting/journal/:id/void` | POST | Void entry |
| `/api/v1/accounting/trial-balance` | GET | Generate trial balance |
| `/api/v1/accounting/statements/profit-loss` | GET | P&L statement |
| `/api/v1/accounting/statements/balance-sheet` | GET | Balance sheet |
| `/api/v1/accounting/statements/cash-flow` | GET | Cash flow statement |
| `/api/v1/accounting/gst/:type` | GET | GST return data |

## 5. RBAC

| Action | Required Permission |
|--------|-------------------|
| View journal entries | `accounting.journal.view` |
| Create journal entry | `accounting.journal.create` |
| Post entry | `accounting.journal.post` |
| Void entry | `accounting.journal.void` |
| View accounts | `accounting.accounts.view` |
| Create account | `accounting.accounts.create` |
| View trial balance | `accounting.trial_balance.view` |
| View statements | `accounting.statements.view` |
| View GST data | `accounting.gst.view` |
