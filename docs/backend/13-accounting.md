# AeroXe Backend — Accounting Module

> **Req Ref:** §8A General Ledger & Double-Entry Accounting

---

## 1. Overview

Implements double-entry accounting with a Chart of Accounts, journal entries, financial statements, and GST filing data generation. Every financial transaction in the billing module creates corresponding journal entries.

## 2. Database Tables

```sql
CREATE TABLE chart_of_accounts (
    id BIGSERIAL PRIMARY KEY,
    code VARCHAR(20) NOT NULL UNIQUE,
    name VARCHAR(255) NOT NULL,
    account_type VARCHAR(30) NOT NULL
        CHECK (account_type IN ('asset', 'liability', 'equity', 'revenue', 'expense')),
    parent_id BIGINT REFERENCES chart_of_accounts(id),
    is_group BOOLEAN DEFAULT FALSE,
    is_active BOOLEAN DEFAULT TRUE,
    description TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE journal_entries (
    id BIGSERIAL PRIMARY KEY,
    entry_number VARCHAR(30) NOT NULL UNIQUE,
    entry_date DATE NOT NULL,
    description TEXT NOT NULL,
    reference_type VARCHAR(50),
    reference_id BIGINT,
    total_debit DECIMAL(12,2) NOT NULL,
    total_credit DECIMAL(12,2) NOT NULL,
    status VARCHAR(20) DEFAULT 'draft'
        CHECK (status IN ('draft', 'posted', 'voided')),
    created_by BIGINT REFERENCES users(id),
    reviewed_by BIGINT REFERENCES users(id),
    reviewed_at TIMESTAMPTZ,
    posted_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE journal_entry_lines (
    id BIGSERIAL PRIMARY KEY,
    journal_entry_id BIGINT NOT NULL REFERENCES journal_entries(id) ON DELETE CASCADE,
    account_id BIGINT NOT NULL REFERENCES chart_of_accounts(id),
    debit DECIMAL(12,2) DEFAULT 0,
    credit DECIMAL(12,2) DEFAULT 0,
    description TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    CHECK (debit >= 0 AND credit >= 0),
    CHECK (debit > 0 OR credit > 0)
);

CREATE TABLE trial_balances (
    id BIGSERIAL PRIMARY KEY,
    period_start DATE NOT NULL,
    period_end DATE NOT NULL,
    account_id BIGINT NOT NULL REFERENCES chart_of_accounts(id),
    opening_balance DECIMAL(12,2) DEFAULT 0,
    total_debit DECIMAL(12,2) DEFAULT 0,
    total_credit DECIMAL(12,2) DEFAULT 0,
    closing_balance DECIMAL(12,2) DEFAULT 0,
    generated_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(period_start, period_end, account_id)
);

CREATE TABLE gst_returns (
    id BIGSERIAL PRIMARY KEY,
    return_type VARCHAR(10) NOT NULL CHECK (return_type IN ('GSTR1', 'GSTR3B')),
    period_month INTEGER NOT NULL,
    period_year INTEGER NOT NULL,
    total_taxable_value DECIMAL(12,2) DEFAULT 0,
    total_cgst DECIMAL(12,2) DEFAULT 0,
    total_sgst DECIMAL(12,2) DEFAULT 0,
    total_igst DECIMAL(12,2) DEFAULT 0,
    status VARCHAR(20) DEFAULT 'draft',
    filed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW()
);
```

## 3. Chart of Accounts (ISP-Specific)

```
ASSETS (1xxx)
├── 1000  Cash in Hand
├── 1100  Bank Accounts
├── 1200  Accounts Receivable
├── 1300  Prepaid Expenses
├── 1400  Inventory (Hardware)
└── 1500  Fixed Assets

LIABILITIES (2xxx)
├── 2000  Accounts Payable
├── 2100  GST Payable (Output)
├── 2200  GST Receivable (Input)
├── 2300  Customer Wallet Balance
└── 2400  Advance Received

EQUITY (3xxx)
├── 3000  Owner's Equity
└── 3100  Retained Earnings

REVENUE (4xxx)
├── 4000  Subscription Revenue
├── 4100  Installation Revenue
├── 4200  Hardware Sales Revenue
├── 4300  Priority Support Revenue
└── 4400  Late Fee Revenue

EXPENSES (5xxx)
├── 5000  Bandwidth Cost
├── 5100  Hardware Cost
├── 5200  Staff Salaries
├── 5300  Marketing Expense
├── 5400  Referral Rewards
├── 5500  Payment Gateway Fees
├── 5600  Depreciation
└── 5700  Utilities
```

## 4. Journal Entry Examples

### Invoice Generated
```
Dr. Accounts Receivable (1200)    ₹708.00
    Cr. Subscription Revenue (4000)      ₹600.00
    Cr. GST Payable - CGST (2100)       ₹54.00
    Cr. GST Payable - SGST (2100)       ₹54.00
```

### Payment Received
```
Dr. Bank Account (1100)           ₹708.00
    Cr. Accounts Receivable (1200)       ₹708.00
```

### Refund Processed
```
Dr. Accounts Receivable (1200)    ₹708.00
    Cr. Bank Account (1100)              ₹708.00
```

### Referral Reward
```
Dr. Marketing Expense (5300)      ₹100.00
    Cr. Customer Wallet Balance (2300)   ₹100.00
```

## 5. API Endpoints

| Method | Path | Required Role | Description |
|--------|------|--------------|-------------|
| GET | `/api/v1/accounting/accounts` | finance_manager+ | Chart of accounts |
| POST | `/api/v1/accounting/accounts` | finance_manager+ | Create account |
| GET | `/api/v1/accounting/journal` | finance_manager+ | List journal entries |
| POST | `/api/v1/accounting/journal` | billing_operator+ | Create journal entry |
| POST | `/api/v1/accounting/journal/:id/post` | finance_manager+ | Post entry |
| POST | `/api/v1/accounting/journal/:id/void` | finance_manager+ | Void entry |
| GET | `/api/v1/accounting/trial-balance` | finance_manager+ | Generate trial balance |
| GET | `/api/v1/accounting/statements/profit-loss` | finance_manager+ | P&L statement |
| GET | `/api/v1/accounting/statements/balance-sheet` | finance_manager+ | Balance sheet |
| GET | `/api/v1/accounting/statements/cash-flow` | finance_manager+ | Cash flow statement |
| GET | `/api/v1/accounting/gst/:type` | finance_manager+ | GST return data |
| GET | `/api/v1/accounting/reports/revenue` | finance_manager+ | Revenue report |

## 6. GST Filing Data

### GSTR-1 (Outward Supplies)
```sql
SELECT
    c.gstin,
    i.invoice_number,
    i.invoice_date,
    i.taxable_value,
    i.cgst_amount,
    i.sgst_amount,
    i.igst_amount
FROM invoices i
JOIN customers c ON i.customer_id = c.id
WHERE i.billing_period_start >= $1 AND i.billing_period_end <= $2
  AND i.status = 'paid';
```

### GSTR-3B (Summary Return)
```sql
SELECT
    SUM(taxable_value) as total_taxable,
    SUM(cgst) as total_cgst,
    SUM(sgst) as total_sgst,
    SUM(igst) as total_igst
FROM invoices
WHERE billing_period_month = $1 AND billing_period_year = $2
  AND status = 'paid';
```

## 7. RBAC Permissions

```
accounting.journal.view
accounting.journal.create
accounting.journal.post
accounting.journal.void
accounting.accounts.view
accounting.accounts.create
accounting.trial_balance.view
accounting.statements.view
accounting.gst.view
accounting.gst.file
```
