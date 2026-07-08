# AeroXe Backend — Subscriptions Module

> **Req Ref:** §3.6 Subscriptions, §4 Plan Management

---

## 1. Overview

Manages customer subscriptions to plans, including lifecycle (active → suspended → cancelled → expired), billing period tracking, plan upgrades/downgrades with pro-rata billing, and auto-renewal.

## 2. Database Tables

```sql
CREATE TABLE subscriptions (
    id BIGSERIAL PRIMARY KEY,
    customer_id BIGINT NOT NULL REFERENCES customers(id),
    branch_id BIGINT NOT NULL REFERENCES branches(id),
    plan_id BIGINT NOT NULL REFERENCES plans(id),
    status VARCHAR(20) DEFAULT 'active'
        CHECK (status IN ('active', 'suspended', 'cancelled', 'expired')),
    billing_period_months INTEGER NOT NULL DEFAULT 1,
    start_date DATE NOT NULL,
    end_date DATE,
    next_billing_date DATE,
    auto_renew BOOLEAN DEFAULT TRUE,
    pppoe_session_id BIGINT REFERENCES pppoe_sessions(id),
    mac_address MACADDR,
    ip_address INET,
    vlan_id INTEGER,
    created_by BIGINT REFERENCES users(id),
    reviewed_by BIGINT REFERENCES users(id),
    reviewed_at TIMESTAMPTZ,
    review_status VARCHAR(20) DEFAULT 'pending'
        CHECK (review_status IN ('pending', 'approved', 'rejected')),
    review_notes TEXT,
    approved_by BIGINT REFERENCES users(id),
    approved_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE subscriptions_history (
    id BIGSERIAL PRIMARY KEY,
    subscription_id BIGINT NOT NULL,
    action VARCHAR(20) NOT NULL,
    old_data JSONB,
    new_data JSONB,
    performed_by BIGINT REFERENCES users(id),
    performed_at TIMESTAMPTZ DEFAULT NOW(),
    reason TEXT
);

CREATE TABLE service_accounts (
    id BIGSERIAL PRIMARY KEY,
    subscription_id BIGINT NOT NULL REFERENCES subscriptions(id),
    service_type VARCHAR(50) NOT NULL,
    status VARCHAR(20) DEFAULT 'active',
    config JSONB,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
```

## 3. API Endpoints

| Method | Path | Required Role | Description |
|--------|------|--------------|-------------|
| GET | `/api/v1/subscriptions` | customer_ops | List subscriptions |
| POST | `/api/v1/subscriptions` | sales_agent+ | Create subscription |
| GET | `/api/v1/subscriptions/:id` | customer_ops | Get subscription |
| PUT | `/api/v1/subscriptions/:id` | customer_support+ | Update subscription |
| POST | `/api/v1/subscriptions/:id/upgrade` | customer_support+ | Upgrade plan |
| POST | `/api/v1/subscriptions/:id/downgrade` | customer_support+ | Downgrade plan |
| POST | `/api/v1/subscriptions/:id/cancel` | customer_support+ | Cancel subscription |
| POST | `/api/v1/subscriptions/:id/suspend` | billing_operator+ | Suspend subscription |
| POST | `/api/v1/subscriptions/:id/reactivate` | billing_operator+ | Reactivate subscription |
| POST | `/api/v1/subscriptions/:id/renew` | system (auto) | Auto-renew subscription |
| GET | `/api/v1/subscriptions/:id/history` | customer_ops | View change history |

## 4. Subscription Lifecycle

```
created (pending approval) → approved (active)
active → suspended (payment overdue / manual)
suspended → active (payment received / manual reactivation)
active → cancelled (customer request)
suspended → cancelled (exceeded suspension period)
active/expired → expired (end_date reached, no renewal)
```

## 5. Pro-Rata Billing

When upgrading or downgrading mid-cycle:

```rust
pub fn calculate_pro_rata(
    old_plan_price: Decimal,
    new_plan_price: Decimal,
    billing_period_days: i32,
    days_used: i32,
) -> ProRataAdjustment {
    let remaining_days = billing_period_days - days_used;
    let old_daily = old_plan_price / billing_period_days;
    let new_daily = new_plan_price / billing_period_days;
    let credit = old_daily * remaining_days;
    let charge = new_daily * remaining_days;
    let adjustment = charge - credit;

    ProRataAdjustment {
        old_plan_credit: credit,
        new_plan_charge: charge,
        adjustment,  // positive = additional charge, negative = credit
    }
}
```

## 6. Auto-Renewal Job

Runs daily at 1:00 AM IST:

```rust
async fn auto_renew_subscriptions(state: &AppState) {
    let expiring = db.query(
        "SELECT * FROM subscriptions WHERE next_billing_date = CURRENT_DATE
         AND auto_renew = TRUE AND status = 'active'"
    ).await?;

    for sub in expiring {
        // 1. Generate invoice (§12-billing)
        // 2. Send payment reminder
        // 3. Update next_billing_date
        // 4. Publish subscription.renewed event
    }
}
```

## 7. Events Published

```yaml
subscription.created:
  payload: { subscription_id, customer_id, plan_id, billing_period_months }
subscription.renewed:
  payload: { subscription_id, customer_id, next_billing_date }
subscription.suspended:
  payload: { subscription_id, customer_id, reason }
subscription.reactivated:
  payload: { subscription_id, customer_id }
subscription.cancelled:
  payload: { subscription_id, customer_id, reason }
subscription.upgraded:
  payload: { subscription_id, old_plan_id, new_plan_id, pro_rata_adjustment }
subscription.downgraded:
  payload: { subscription_id, old_plan_id, new_plan_id, pro_rata_adjustment }
```

## 8. RBAC Permissions

```
subscription.view
subscription.create
subscription.update
subscription.upgrade
subscription.downgrade
subscription.cancel
subscription.suspend
subscription.reactivate
```
