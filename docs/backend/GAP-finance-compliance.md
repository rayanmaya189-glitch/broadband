# AeroXe Broadband — Indian Finance & Regulatory Compliance Gap Analysis v3.0

**Date:** 2026-07-21
**Author:** Backend Architecture Team
**Scope:** Billing, accounting, payment, tax, and regulatory compliance from Indian ISP perspective
**Previous:** v1.0 (84 gaps), v2.0 (68 gaps) — this is v3.0 (25 finance/regulatory gaps)

---

## Executive Summary

The billing and accounting modules provide solid CRUD coverage but **lack Indian-specific financial compliance**. GST is never calculated on invoices, TDS is not handled, security deposits are untracked, and the accounting module has an incomplete chart of accounts. For a Jalgaon-based ISP, these gaps create immediate regulatory risk.

**Combined gap count:** 215 total (84 + 68 + 63)

---

## F-01: GST Calculation Never Executed

- **File:** `billing/application/service.rs:68-70`
- **What exists:** `tax_amount: Set(Decimal::ZERO)` — hardcoded to zero
- **What's missing:**
  - `create_invoice()` never computes GST
  - `auto_generate_invoices()` never adds tax to invoice total
  - No place-of-supply logic (CGST+SGST vs IGST)
  - No HSN/SAC code assignment per line item
  - No GSTIN validation on customer
- **Legal requirement:** Section 9 CGST Act — all taxable supplies attract GST
- **Impact:** Every invoice issued is non-compliant. Penalties up to 100% of tax amount under Section 122.
- **Fix:**
```rust
// billing/application/service.rs — create_invoice()
let tax_config = get_tax_config(customer_state_code, provider_state_code)?;
let cgst = if tax_config.is_intra_state {
    subtotal * Decimal::from_str("0.09").unwrap()
} else { Decimal::ZERO };
let sgst = if tax_config.is_intra_state {
    subtotal * Decimal::from_str("0.09").unwrap()
} else { Decimal::ZERO };
let igst = if !tax_config.is_intra_state {
    subtotal * Decimal::from_str("0.18").unwrap()
} else { Decimal::ZERO };
```

---

## F-02: No Place-of-Supply Logic (Inter-State vs Intra-State)

- **File:** `billing/application/service.rs:421-433`
- **What exists:** `get_tax_config()` hardcodes Maharashtra (state_code = 27)
- **What's missing:**
  - No `customer.state_code` field
  - No comparison between provider state and customer state
  - When expanding to MP/Mumbai, wrong GST type applied
- **Legal requirement:** Section 5 IGST Act — inter-state supplies attract IGST, not CGST+SGST
- **Impact:** Wrong GST on ~30-50% of invoices once expansion begins
- **Fix:** Add `state_code` to `customers` table. Compare with provider state_code. Apply IGST for inter-state.

---

## F-03: No Security Deposit Ledger

- **File:** No entity exists for security deposit tracking
- **What's missing:**
  - No `security_deposits` table
  - No workflow: collect → hold → refund/forfeit
  - No journal entry on deposit collection (double-entry violation)
  - ₹500-2000 deposits collected but never tracked in accounting
- **Legal requirement:** Security deposits are refundable liabilities under Ind AS 109
- **Impact:** Balance sheet misstatement. Deposits appear as revenue. Refund disputes unresolvable.
- **Fix:**
```sql
CREATE TABLE security_deposits (
    id BIGSERIAL PRIMARY KEY,
    customer_id BIGINT NOT NULL,
    amount DECIMAL(10,2) NOT NULL,
    collected_at TIMESTAMPTZ DEFAULT NOW(),
    refund_amount DECIMAL(10,2),
    refund_at TIMESTAMPTZ,
    status VARCHAR(20) DEFAULT 'held'
        CHECK (status IN ('held', 'refunded', 'forfeited')),
    forfeiture_reason TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW()
);
```

---

## F-04: No GST on Late Fees

- **File:** `billing/workers/late_fee_worker.rs` (doesn't exist yet)
- **What exists:** `late_fee_percent: "2.0"` defined but never applied
- **What's missing:**
  - Late fees attract 18% GST per Circular 178/10/2022 (RBI clarification)
  - No GST calculation on late fee amount
  - Late fees appear as separate line item without tax
- **Legal requirement:** Circular 178/10/2022 — GST on late payment charges
- **Impact:** Under-payment of GST on late fee component
- **Fix:** When applying late fee, add GST: `late_fee × 1.18` (inclusive) or `late_fee + (late_fee × 0.18)` as separate tax line.

---

## F-05: No Credit Notes / Debit Notes

- **File:** No entity exists
- **What's missing:**
  - No `credit_notes` or `debit_notes` tables
  - No workflow for post-invoice corrections
  - No GSTR-1 credit note reporting
- **Legal requirement:** Section 34 CGST Act — mandatory for any post-invoice value reduction
- **Impact:** Cannot issue credit notes for overbilled amounts, plan downgrades, or service credits. GSTR-1 non-compliant.
- **Fix:**
```sql
CREATE TABLE credit_notes (
    id BIGSERIAL PRIMARY KEY,
    credit_note_number VARCHAR(30) NOT NULL UNIQUE,
    original_invoice_id BIGINT NOT NULL REFERENCES invoices(id),
    customer_id BIGINT NOT NULL,
    reason VARCHAR(50) NOT NULL,
    taxable_amount DECIMAL(12,2) NOT NULL,
    cgst DECIMAL(12,2) DEFAULT 0,
    sgst DECIMAL(12,2) DEFAULT 0,
    igst DECIMAL(12,2) DEFAULT 0,
    total_amount DECIMAL(12,2) NOT NULL,
    status VARCHAR(20) DEFAULT 'draft',
    issued_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW()
);
```

---

## F-06: No Revenue Recognition Framework (Ind AS 115)

- **File:** No implementation
- **What's missing:**
  - Annual plan ₹14,000 recognized as month-1 revenue
  - No deferred revenue calculation for prepaid plans
  - No monthly revenue recognition schedule
  - No `deferred_revenue` liability account
- **Legal requirement:** Ind AS 115 — revenue recognized over time as service is delivered
- **Impact:** Financial misstatement. Revenue overstatement in month of collection.
- **Fix:** Create `revenue_recognition_schedule` table. Monthly journal entry: `Dr Deferred Revenue, Cr Revenue` for 1/12 of annual amount.

---

## F-07: No Advance Payment Tracking

- **File:** No implementation
- **What's missing:**
  - Annual/quarterly prepayments not deferred
  - No `advance_payments` table
  - No automatic monthly recognition
- **Impact:** Cash basis accounting instead of accrual. Audit qualification risk.

---

## F-08: No HSN/SAC per Line Item

- **File:** `billing/domain/entities/invoice.rs`
- **What exists:** `hsn_sac_code: Option<String>` field exists but always None
- **What's missing:**
  - Broadband internet → SAC 998421 (mandatory for GST e-invoice)
  - Router rental → SAC 998314
  - Late fee → SAC 997159
  - Different items need different HSN/SAC codes
- **Legal requirement:** Rule 46 CGST Rules — HSN/SAC mandatory on all invoices above ₹5 crore turnover
- **Impact:** GSTR-1 HSN summary will be wrong
- **Fix:** Define `hsn_sac_mapping` config: `{"broadband": "998421", "router_rental": "998314", "late_fee": "997159"}`

---

## F-09: No Reverse Charge Mechanism (RCM)

- **File:** No implementation
- **What's missing:**
  - Legal services, security services, director services attract RCM
  - No RCM tracking on vendor payments
  - No input tax credit on RCM paid
- **Legal requirement:** Section 9(3) & 9(4) CGST Act
- **Impact:** GST underpayment on RCM-applicable services

---

## F-10: No GST E-Invoice (IRN Generation)

- **File:** No implementation
- **What's missing:**
  - No GSTN API integration for IRN generation
  - No QR code on invoices
  - Mandatory above ₹5 crore turnover (as of Aug 2023)
- **Legal requirement:** Notification 17/2022 — mandatory e-invoicing
- **Impact:** Invalid invoices for B2B customers once turnover threshold crossed

---

## F-11: No Payment Reconciliation

- **File:** No implementation
- **What's missing:**
  - No bank statement import (CSV/OFX)
  - No UTR/auto-matching logic
  - No reconciliation status tracking
  - Manual reconciliation required
- **Impact:** Revenue leakage from unmatched payments. ~2-5% revenue at risk.

---

## F-12: No UPI Autopay / e-Mandate Management

- **File:** No implementation
- **What's missing:**
  - No `upi_mandates` table
  - No e-mandate lifecycle (create → authenticate → activate → pause → revoke)
  - No auto-debit scheduling
- **Impact:** Cannot offer recurring payments for 70%+ of Indian customers who prefer UPI

---

## F-13: No Gateway Settlement Cycle Tracking

- **File:** No implementation
- **What's missing:**
  - No tracking of T+1/T+2 settlement between gateway and bank
  - No `settlement_batches` table
  - Cash flow inaccuracy — payment received ≠ bank credit
- **Impact:** Cash flow forecasting inaccurate. Cannot reconcile gateway fees.

---

## F-14: No MDR (Merchant Discount Rate) Tracking

- **File:** No implementation
- **What's missing:**
  - No tracking of gateway fees per transaction (2% for cards, 0% for UPI, 1.5% for wallets)
  - No expense category for MDR
  - ₹37,800/month in gateway fees (at ₹1.8M monthly revenue) unreconciled
- **Impact:** Expense leakage. P&L understatement.

---

## F-15: No Bad Debt Provisioning (ECL Model)

- **File:** No implementation
- **What's missing:**
  - No Expected Credit Loss calculation for receivables
  - No aging bucket classification (30/60/90/180+ days)
  - No provision journal entries
- **Legal requirement:** Ind AS 109 — expected credit loss model for financial assets
- **Impact:** Balance sheet overstatement of receivables

---

## F-16: No Mid-Month Join/Leave Pro-Rata

- **File:** `billing/domain/primitives.rs`
- **What exists:** `prorata_adjustments` table exists but is never written to
- **What's missing:**
  - First invoice for mid-month join not pro-rated
  - Final invoice for mid-month leave not pro-rated
  - Uses `period_start + 30 days` instead of actual month end
- **Impact:** Revenue disputes. Customer overpayment on leaving.

---

## F-17: No Grandfathered Plan Pricing

- **File:** No implementation
- **What's missing:**
  - When plan price changes, all existing subscriptions silently get new price
  - No `effective_date` on plan changes
  - No grandfathering mechanism for existing customers
- **Impact:** Customer disputes. Regulatory risk under Consumer Protection Act.

---

## F-18: No Enterprise Billing Features

- **File:** No implementation
- **What's missing:**
  - No consolidated invoicing for multi-connection enterprise accounts
  - No PO (Purchase Order) reference on invoices
  - No credit terms (Net 30/60/90)
  - No TDS deduction handling
- **Impact:** Enterprise customers (₹50K+ MRR) cannot be invoiced properly

---

## F-19: Incomplete Chart of Accounts

- **File:** `13-accounting.md`
- **What exists:** Basic accounts (cash, receivables, revenue, expenses)
- **What's missing (15+ accounts):**
  - 2005: TDS Receivable (asset)
  - 2006: Security Deposits Held (liability)
  - 2007: Deferred Revenue (liability)
  - 2008: RCM GST Payable (liability)
  - 2009: Provision for Bad Debts (contra-asset)
  - 4002: Late Fee Revenue
  - 4003: Router Rental Revenue
  - 5003: Depreciation Expense
  - 5004: MDR/Gateway Fee Expense
  - 5005: Bandwidth Upstream Cost
  - 5006: RCM GST Paid
  - 5007: Bad Debt Expense
  - 5008: Provision for Doubtful Debts
  - 5009: Legal & Professional Fees
  - 5010: Bank Charges
- **Impact:** Audit qualification. Incomplete financial statements.

---

## F-20: No Tax Invoice Compliance (Rule 46)

- **File:** `billing/application/service.rs`
- **What's missing — mandatory fields per Rule 46 CGST Rules:**
  - Supplier GSTIN
  - Supplier state code
  - Customer GSTIN (for B2B)
  - Invoice type (regular / revised)
  - HSN/SAC summary
  - Place of supply with state code
  - Reverse charge flag (Y/N)
  - Signature/digital signature
- **Impact:** Invoice is not a valid tax document. Input tax credit denied to customers.

---

## F-21: No GST on Discounted Amounts

- **File:** No implementation
- **What's missing:**
  - Pre-invoice discount: GST on reduced amount (Section 15(3) CGST Act)
  - Post-invoice discount: credit note required
  - No discount handling at all in billing
- **Impact:** Wrong GST calculation when discounts are offered

---

## F-22: No Cash Collection by Field Agents

- **File:** No implementation
- **What's missing:**
  - 30-50% of Jalgaon customers pay cash
  - No field agent cash collection workflow
  - No cash-in-transit tracking
  - No reconciliation against agent collections
- **Impact:** Cash-in-transit risk. Revenue leakage. Agent fraud potential.

---

## F-23: No EMI Options

- **File:** No implementation
- **What's missing:**
  - ₹14,000 annual plans need EMI for adoption
  - No EMI calculation
  - No EMI collection schedule
- **Revenue impact:** Lower annual plan adoption. Higher churn.

---

## F-24: No Wallet Withdrawal

- **File:** No implementation
- **What's missing:**
  - Terminated customers can't get wallet balance refunded
  - No wallet withdrawal workflow
- **Legal requirement:** Consumer Protection Act — refund of unused balances
- **Impact:** Consumer protection violation

---

## F-25: No TRAI-Compliant Dunning Process

- **File:** No implementation
- **What's missing:**
  - TRAI requires 2 written notices before disconnection
  - 30-day notice period for disconnection
  - No notice tracking
  - No compliance with TRAI QoS Regulations 2024
- **Legal requirement:** TRAI QoS Regulations — customer must receive 2 written notices
- **Impact:** Regulatory non-compliance. Customer complaints to TRAI.

---

## Implementation Priority

| Priority | Gaps | Est. Effort |
|----------|------|-------------|
| **P0 (Immediate)** | F-01, F-02, F-04, F-05, F-08, F-20 | 2 weeks |
| **P1 (Pre-launch)** | F-03, F-06, F-07, F-16, F-17, F-18, F-19, F-25 | 3 weeks |
| **P2 (Post-launch)** | F-09, F-10, F-11, F-12, F-13, F-14, F-15 | 3 weeks |
| **P3 (Enhancement)** | F-21, F-22, F-23, F-24 | 2 weeks |

**Total estimated effort:** 10 weeks

---

*Document version: 3.0 — 2026-07-21*
*Combined total: 215 gaps (84 v1.0 + 68 v2.0 + 63 v3.0)*
*See also: DESIGN-GAPS-DEEP-ANALYSIS.md, GAP-IMPLEMENTATION-ROADMAP.md*
