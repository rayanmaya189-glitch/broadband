# AeroXe Backend — Referrals Module

> **Req Ref:** §3.7 Referral Program

---

## 1. Overview

Customers earn rewards for referring new subscribers. The system tracks referral codes, monitors the referral funnel (pending → registered → activated → rewarded), and automatically credits both referrer and referee upon activation.

## 2. Referral Flow

```
1. Referrer shares unique referral code
2. New customer registers with code → status: 'registered'
3. New customer activates service → status: 'activated'
4. System credits referrer wallet + applies referee discount
5. status: 'rewarded'
6. Journal entry: Dr. Marketing Expense, Cr. Prepaid Expenses
```

## 3. Database Tables

```sql
CREATE TABLE referral_programs (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    referrer_reward_type VARCHAR(20) NOT NULL
        CHECK (referrer_reward_type IN ('credit', 'free_days', 'plan_upgrade')),
    referrer_reward_value DECIMAL(10,2) NOT NULL,
    referee_reward_type VARCHAR(20) NOT NULL
        CHECK (referee_reward_type IN ('credit', 'free_days', 'discount')),
    referee_reward_value DECIMAL(10,2) NOT NULL,
    max_referrals_per_customer INTEGER,
    valid_from DATE NOT NULL,
    valid_until DATE NOT NULL,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE referral_tracking (
    id BIGSERIAL PRIMARY KEY,
    program_id BIGINT NOT NULL REFERENCES referral_programs(id),
    referrer_id BIGINT NOT NULL REFERENCES customers(id),
    referee_id BIGINT REFERENCES customers(id),
    referral_code VARCHAR(20) NOT NULL,
    status VARCHAR(20) DEFAULT 'pending'
        CHECK (status IN ('pending', 'registered', 'activated', 'rewarded')),
    referrer_reward_amount DECIMAL(10,2),
    referee_reward_amount DECIMAL(10,2),
    referrer_reward_applied BOOLEAN DEFAULT FALSE,
    referee_reward_applied BOOLEAN DEFAULT FALSE,
    registered_at TIMESTAMPTZ,
    activated_at TIMESTAMPTZ,
    rewarded_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW()
);
```

## 4. API Endpoints

| Method | Path | Required Role | Description |
|--------|------|--------------|-------------|
| GET | `/api/v1/referrals/my-code` | customer (own) | Get my referral code |
| GET | `/api/v1/referrals/my-referrals` | customer (own) | List my referrals |
| GET | `/api/v1/referrals/stats` | finance_manager+ | Referral statistics |
| GET | `/api/v1/admin/referral-programs` | finance_manager+ | List programs |
| POST | `/api/v1/admin/referral-programs` | finance_manager+ | Create program |
| PUT | `/api/v1/admin/referral-programs/:id` | finance_manager+ | Update program |

## 5. Referral Code Generation

```rust
fn generate_referral_code(customer_name: &str) -> String {
    let prefix = customer_name.chars().take(4).collect::<String>().to_uppercase();
    let suffix = rand::thread_rng().gen_range(1000..9999);
    format!("{}{}", prefix, suffix)  // e.g., "RAHU2485"
}
```

## 6. Events Published

```yaml
referral.created:
  payload: { referral_id, referrer_id, referral_code }
referral.registered:
  payload: { referral_id, referee_id }
referral.activated:
  payload: { referral_id, referrer_id, referee_id }
referral.rewarded:
  payload: { referral_id, referrer_reward, referee_reward }
```

## 7. RBAC Permissions

```
referral.view
referral.program.create
referral.program.update
referral.program.delete
referral.reward.process
```
