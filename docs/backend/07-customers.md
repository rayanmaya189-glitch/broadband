# AeroXe Backend — Customers Module

> **Req Ref:** §3 Customer Management Module

---

## 1. Overview

Manages the full customer lifecycle from prospect → registered → KYC → installation → active → suspended → terminated. Includes KYC document management, profiles, addresses, and history tracking.

## 2. Database Tables

```sql
CREATE TABLE customers (
    id BIGSERIAL PRIMARY KEY,
    customer_code VARCHAR(20) NOT NULL UNIQUE,
    branch_id BIGINT NOT NULL REFERENCES branches(id),
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255),
    phone VARCHAR(20) NOT NULL,
    alternate_phone VARCHAR(20),
    status VARCHAR(30) NOT NULL DEFAULT 'registered'
        CHECK (status IN ('registered', 'kyc_pending', 'kyc_verified',
                          'installation_scheduled', 'installation_in_progress',
                          'active', 'suspended', 'terminated')),
    referral_code VARCHAR(20) UNIQUE,
    referred_by BIGINT REFERENCES customers(id),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    deleted_at TIMESTAMPTZ
);

CREATE TABLE customers_history (
    id BIGSERIAL PRIMARY KEY,
    customer_id BIGINT NOT NULL,
    action VARCHAR(20) NOT NULL,
    old_data JSONB,
    new_data JSONB,
    performed_by BIGINT REFERENCES users(id),
    performed_at TIMESTAMPTZ DEFAULT NOW(),
    reason TEXT
);

CREATE TABLE customer_profiles (
    id BIGSERIAL PRIMARY KEY,
    customer_id BIGINT NOT NULL REFERENCES customers(id) ON DELETE CASCADE,
    aadhaar_hash VARCHAR(255),
    pan_hash VARCHAR(255),
    gender VARCHAR(10),
    date_of_birth DATE,
    occupation VARCHAR(100),
    kyc_status VARCHAR(20) DEFAULT 'pending'
        CHECK (kyc_status IN ('pending', 'submitted', 'verified', 'rejected')),
    kyc_verified_at TIMESTAMPTZ,
    kyc_verified_by BIGINT REFERENCES users(id),
    kyc_rejection_reason TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE kyc_documents (
    id BIGSERIAL PRIMARY KEY,
    customer_id BIGINT NOT NULL REFERENCES customers(id) ON DELETE CASCADE,
    document_type VARCHAR(50) NOT NULL,
    file_url TEXT NOT NULL,
    file_hash VARCHAR(255),
    status VARCHAR(20) DEFAULT 'pending'
        CHECK (status IN ('pending', 'approved', 'rejected')),
    reviewed_by BIGINT REFERENCES users(id),
    reviewed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE addresses (
    id BIGSERIAL PRIMARY KEY,
    customer_id BIGINT NOT NULL REFERENCES customers(id) ON DELETE CASCADE,
    address_type VARCHAR(20) DEFAULT 'installation'
        CHECK (address_type IN ('installation', 'billing', 'correspondence')),
    line1 VARCHAR(255) NOT NULL,
    line2 VARCHAR(255),
    area VARCHAR(100),
    city VARCHAR(100) NOT NULL,
    state VARCHAR(100) NOT NULL,
    pincode VARCHAR(10) NOT NULL,
    country VARCHAR(50) DEFAULT 'India',
    latitude DECIMAL(10, 7),
    longitude DECIMAL(10, 7),
    landmark VARCHAR(255),
    is_primary BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
```

## 3. API Endpoints

| Method | Path | Required Role | Description |
|--------|------|--------------|-------------|
| GET | `/api/v1/customers` | customer_ops | List customers (branch-scoped) |
| POST | `/api/v1/customers` | sales_agent+ | Create customer |
| GET | `/api/v1/customers/:id` | customer_ops | Get customer details |
| PUT | `/api/v1/customers/:id` | customer_ops | Update customer |
| DELETE | `/api/v1/customers/:id` | finance_manager+ | Soft-delete customer |
| PUT | `/api/v1/customers/:id/status` | customer_support+ | Change customer status |
| GET | `/api/v1/customers/:id/profile` | customer_ops | Get customer profile |
| PUT | `/api/v1/customers/:id/profile` | customer_ops | Update profile |
| POST | `/api/v1/customers/:id/kyc/submit` | customer_ops | Submit KYC documents |
| POST | `/api/v1/customers/:id/kyc/verify` | finance_manager+ | Verify KYC |
| POST | `/api/v1/customers/:id/kyc/reject` | finance_manager+ | Reject KYC |
| GET | `/api/v1/customers/:id/addresses` | customer_ops | List addresses |
| POST | `/api/v1/customers/:id/addresses` | customer_ops | Add address |
| PUT | `/api/v1/customers/:id/addresses/:aid` | customer_ops | Update address |
| DELETE | `/api/v1/customers/:id/addresses/:aid` | customer_ops | Delete address |
| GET | `/api/v1/customers/:id/history` | customer_ops | View change history |
| GET | `/api/v1/customers/:id/subscriptions` | customer_ops | List subscriptions |
| GET | `/api/v1/customers/:id/tickets` | customer_ops | List tickets |

## 4. Customer Lifecycle State Machine

```
registered → kyc_pending → kyc_verified → installation_scheduled
→ installation_in_progress → active → suspended → terminated
```

**State transition rules:**
| From | To | Allowed Roles |
|------|----|---------------|
| registered | kyc_pending | sales_agent, customer (self) |
| kyc_pending | kyc_verified | finance_manager, customer_support |
| kyc_pending | registered | finance_manager (reject) |
| kyc_verified | installation_scheduled | field_technician, customer_support |
| installation_scheduled | installation_in_progress | field_technician |
| installation_in_progress | active | field_technician |
| active | suspended | billing_operator, finance_manager |
| suspended | active | billing_operator, finance_manager |
| active | terminated | customer_support, finance_manager |
| suspended | terminated | finance_manager |

## 5. Customer Code Generation

Format: `AX-{BRANCH_CODE}-{YYYYMM}-{SEQUENCE}`

Example: `AX-JLG-202607-0001`

```rust
async fn generate_customer_code(db: &Db, branch_code: &str) -> String {
    let month = Utc::now().format("%Y%m");
    let key = format!("customer_code:{}:{}", branch_code, month);
    let seq = redis.incr(&key, 1).await?;
    redis.expire(&key, 31 * 86400).await?;
    format!("AX-{}-{}-{:04}", branch_code, month, seq)
}
```

## 6. Events Published

```yaml
customer.created:
  payload: { customer_id, name, phone, email, referred_by, source }
customer.activated:
  payload: { customer_id, subscription_id, plan_id, pppoe_username }
customer.suspended:
  payload: { customer_id, subscription_id, reason, suspended_by }
customer.reactivated:
  payload: { customer_id, subscription_id, reactivated_by }
customer.terminated:
  payload: { customer_id, subscription_id, reason, terminated_by }
customer.kyc.submitted:
  payload: { customer_id, document_types }
customer.kyc.verified:
  payload: { customer_id, verified_by }
```

## 7. RBAC Permissions

```
customer.account.view
customer.account.create
customer.account.update
customer.account.delete
customer.account.disable
customer.account.enable
customer.account.suspend
customer.account.reactivate
customer.subscription.view
customer.subscription.create
customer.subscription.upgrade
customer.subscription.downgrade
customer.subscription.cancel
customer.subscription.suspend
customer.subscription.reactivate
customer.profile.view
customer.profile.update
customer.profile.verify_kyc
customer.address.view
customer.address.create
customer.address.update
customer.address.delete
customer.installation.view
customer.installation.create
customer.installation.schedule
customer.installation.complete
customer.installation.cancel
customer.installation.reschedule
```

---

## Known Issues & Gap Reference (v2.0)

> **Cross-reference:** `GAP-code-bugs.md` §3, `GAP-customer.md`, `DESIGN-GAPS-DEEP-ANALYSIS.md` §9.3

| Bug ID | Severity | Issue | Location |
|--------|----------|-------|----------|
| BUG-CUST-01 | CRITICAL | Phone uniqueness check has race condition — duplicate accounts possible | `service.rs:52-60` |
| BUG-CUST-02 | MEDIUM | No email uniqueness check — multiple accounts with same email | `service.rs:44-77` |
| BUG-CUST-03 | CRITICAL | No status transition validation — any status → any status allowed | `service.rs:80-89` |
| BUG-CUST-04 | MEDIUM | No email/phone format validation — invalid data enters system | `service.rs:142-165` |
| BUG-CUST-05 | HIGH | Customer search uses `LIKE '%query%'` — full table scan, slow at scale | `service.rs:168-186` |
| BUG-CUST-06 | LOW | `add_address` always sets `is_primary=true` — overwrites existing primary | `service.rs:125` |
| BUG-CUST-07 | MEDIUM | `get_customer` doesn't filter soft-deletes — deleted customers accessible | `service.rs:34-42` |

**Priority:** Fix CUST-01, 03 first (data corruption). See `GAP-IMPLEMENTATION-ROADMAP.md` Phase 1, 4.
