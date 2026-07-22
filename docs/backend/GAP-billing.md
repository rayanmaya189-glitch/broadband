# Billing Module — ISP Design Gaps

**Module:** `billing`
**Cross-reference:** `DESIGN-GAPS-DEEP-ANALYSIS.md` (ISP-BILL-C01 through ISP-BILL-M03)

---

## Critical Gaps

### ISP-BILL-C01: No Tax Calculation

**Files Affected:**
- `src/modules/billing/application/service.rs`
- `src/modules/billing/domain/entities/invoice.rs`

**Current State:**
```rust
// create_invoice() and auto_generate_invoices() never compute tax
// Tax columns always 0
// tax_config hardcoded: CGST 9%, SGST 9%
```

**Required Implementation:**
```rust
pub fn calculate_gst(
    subtotal: Decimal,
    customer_state: &str,
    company_state: &str,
) -> GstBreakdown {
    if customer_state == company_state {
        // Intra-state: CGST + SGST
        let cgst = subtotal * Decimal::from_str("0.09").unwrap();
        let sgst = subtotal * Decimal::from_str("0.09").unwrap();
        GstBreakdown {
            cgst,
            sgst,
            igst: Decimal::ZERO,
            total_tax: cgst + sgst,
        }
    } else {
        // Inter-state: IGST
        let igst = subtotal * Decimal::from_str("0.18").unwrap();
        GstBreakdown {
            cgst: Decimal::ZERO,
            sgst: Decimal::ZERO,
            igst,
            total_tax: igst,
        }
    }
}
```

**Invoice Line Items Must Include:**
- HSN/SAC code (998421 for broadband services)
- Taxable amount
- CGST rate + amount
- SGST rate + amount
- IGST rate + amount (inter-state)
- Total amount with tax

**GSTIN Validation:**
```rust
pub fn validate_gstin(gstin: &str) -> bool {
    // 15-character format: 2 digit state code + PAN + 1 digit check + Z + 1 digit
    // State code must be valid (01-37, 97, 99)
}
```

---

### ISP-BILL-C02: No Connection Pooling for Device Adapters

**Files Affected:**
- `src/modules/integrations/mikrotik/adapter.rs`
- `src/modules/integrations/huawei/adapter.rs`
- `src/shared/app_state.rs`

**Current State:**
```rust
// Each API call creates new connection:
// MikroTik: new HTTP client per request
// Huawei: new SSH session per execute_cli() call
```

**Required Implementation:**
```rust
// In AppState:
pub struct AppState {
    // ... existing fields ...
    pub mikrotik_pool: Option<Arc<MikrotikPool>>,
    pub huawei_ssh_pool: Option<Arc<SshPool>>,
}

// MikrotikPool:
pub struct MikrotikPool {
    clients: Vec<MikrotikClient>,
    config: MikrotikConfig,
}

impl MikrotikPool {
    pub async fn get_client(&self) -> Result<Arc<MikrotikClient>> {
        // Return idle client or create new if under max
    }
}

// SshPool:
pub struct SshPool {
    sessions: HashMap<IpAddr, SshSession>,
    max_per_host: usize, // OLT typically 8-16
}
```

---

## High Gaps

### ISP-BILL-H01: No Pro-Rata Billing

**Current State:**
- `prorata_adjustments` table exists but is never written to
- Mid-cycle upgrade/downgrade generates no partial invoices

**Required Implementation:**
```rust
pub fn calculate_prorata(
    current_plan_price: Decimal,
    new_plan_price: Decimal,
    billing_period_start: NaiveDate,
    billing_period_end: NaiveDate,
    change_date: NaiveDate,
) -> ProrataAdjustment {
    let total_days = (billing_period_end - billing_period_start).num_days();
    let days_used = (change_date - billing_period_start).num_days();
    let days_remaining = total_days - days_used;

    let current_daily = current_plan_price / Decimal::from(total_days);
    let new_daily = new_plan_price / Decimal::from(total_days);

    // Credit for unused days of old plan
    let credit = current_daily * Decimal::from(days_remaining);
    // Charge for remaining days of new plan
    let charge = new_daily * Decimal::from(days_remaining);

    ProrataAdjustment {
        credit,
        charge,
        net_adjustment: charge - credit,
    }
}
```

**Important:** Pro-rata only applies to monthly billing cycles. Quarterly/annual cycles use full-period pricing.

---

### ISP-BILL-H02: No Invoice PDF Generation

**Required Implementation:**
1. Create HTML template with Handlebars:
```handlebars
<html>
<head>
  <style>
    /* Professional invoice styling */
    .invoice-header { background: #1a1a2e; color: white; }
    .line-item { border-bottom: 1px solid #eee; }
    .tax-row { font-weight: bold; }
  </style>
</head>
<body>
  <div class="invoice-header">
    <h1>AeroXe Broadband</h1>
    <p>GSTIN: {{gstин}}</p>
  </div>
  <div class="invoice-details">
    <p>Invoice #: {{invoice_number}}</p>
    <p>Date: {{invoice_date}}</p>
    <p>Due Date: {{due_date}}</p>
  </div>
  <table class="line-items">
    <tr><th>Description</th><th>Amount</th><th>Tax</th><th>Total</th></tr>
    {{#each line_items}}
    <tr>
      <td>{{this.description}}</td>
      <td>₹{{this.amount}}</td>
      <td>₹{{this.tax_amount}}</td>
      <td>₹{{this.total}}</td>
    </tr>
    {{/each}}
  </table>
  <div class="totals">
    <p>Subtotal: ₹{{subtotal}}</p>
    <p>CGST 9%: ₹{{cgst}}</p>
    <p>SGST 9%: ₹{{sgst}}</p>
    <p>Total: ₹{{total}}</p>
  </div>
</body>
</html>
```

2. Use `printpdf` or `wkhtmltopdf` crate for PDF generation
3. Store PDF in MinIO with presigned URL

---

### ISP-BILL-H03: No Late Fee Application

**Required Worker:**
```rust
pub struct LateFeeWorker {
    db: DatabaseConnection,
}

impl LateFeeWorker {
    pub async fn run(&self) {
        // Daily at 00:00 IST
        let overdue_invoices = self.get_overdue_invoices().await;
        for invoice in overdue_invoices {
            let days_overdue = (Utc::now().date_naive() - invoice.due_date).num_days();
            let rule = self.get_late_fee_rule(&invoice).await?;

            if days_overdue > rule.grace_period_days {
                let late_fee = self.calculate_late_fee(&invoice, &rule, days_overdue);
                self.apply_late_fee(&invoice, late_fee).await?;
                self.create_journal_entry(&invoice, late_fee).await?;
            }
        }
    }
}
```

---

### ISP-BILL-H04: Invoice Number Collision

**Current State:** `timestamp_millis() % 10000` — can collide

**Required Fix:** Use database sequence:
```sql
CREATE SEQUENCE invoice_number_seq START 1;
-- Invoice number: INV-{YYYYMM}-{sequence}
-- Example: INV-202607-000001
```

---

### ISP-BILL-H05: No Partial Payment Support

**Current State:** `record_payment()` marks entire invoice paid

**Required Fix:**
```rust
pub async fn record_payment(
    &self,
    invoice_id: Uuid,
    amount: Decimal,
    payment_method: PaymentMethod,
) -> Result<Payment> {
    let invoice = self.get_invoice(invoice_id).await?;

    let payment = Payment {
        id: Uuid::new_v4(),
        invoice_id,
        amount,
        payment_method,
        status: PaymentStatus::Completed,
        ..
    };

    // Update invoice status based on total paid
    let total_paid = self.get_total_paid(invoice_id).await? + amount;
    let new_status = if total_paid >= invoice.total {
        InvoiceStatus::Paid
    } else if total_paid > Decimal::ZERO {
        InvoiceStatus::PartiallyPaid
    } else {
        invoice.status
    };

    self.update_invoice_status(invoice_id, new_status).await?;
    self.create_payment(payment).await
}
```

---

## Medium Gaps

### ISP-BILL-M01: No GST E-Invoice (IRN)
- Required: Integration with GSTN API for IRN generation
- Invoice must include IRN and QR code before sending to customer

### ISP-BILL-M02: No Payment Reconciliation
- Required: Bank statement import, auto-matching of payments
- Manual adjustment workflow with approval

### ISP-BILL-M03: No TDS Deduction
- Required: TDS handling for enterprise customers (Section 194J/194C)

---

## New Dependencies Required

```toml
# Cargo.toml additions
printpdf = "0.7"       # PDF generation (or wkhtmltopdf)
chrono = "0.4"         # Date math for pro-rata
```

---

## New Route Groups Required

> **WARNING:** These proposed routes must be converted to Protobuf-first before implementation. No GET, no PUT, no path variables, no query strings. See `API-CONVENTIONS.md`.

```
/api/v1/billing/gst/
  POST   /calculate            — Calculate GST for invoice
  POST   /returns              — Generate GSTR-1/3B data
  POST   /hsn-summary          — HSN-wise summary

/api/v1/billing/pdf/
  POST   /invoice/pdf          — Generate/download invoice PDF

/api/v1/billing/reconciliation/
  POST   /import               — Import bank statement
  POST   /matches              — View matched/unmatched payments
  POST   /adjust               — Manual adjustment
```
