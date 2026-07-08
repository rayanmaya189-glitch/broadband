# AeroXe Backend — Referrals Module

## Overview

The Referral Program module manages customer referral incentives, tracking them through a funnel from sharing to reward application. Customers share unique referral codes, and both referrer and referee receive rewards when the referred customer subscribes. The program is fully managed by admins.

---

## Data Models

### referral_programs

| Column | Type | Description |
|--------|------|-------------|
| `id` | UUID (PK) | Program identifier |
| `name` | VARCHAR(100) | Program name |
| `status` | ENUM | `active`, `paused`, `ended` |
| `referrer_reward_type` | ENUM | `credit`, `free_days`, `plan_upgrade` |
| `referrer_reward_value` | DECIMAL(10,2) | Reward amount/value |
| `referee_reward_type` | ENUM | `discount`, `free_days` |
| `referee_reward_value` | DECIMAL(10,2) | Reward amount/value |
| `max_referrals_per_customer` | INT | Limit per customer |
| `start_date` | DATE | Program start |
| `end_date` | DATE | Program end |
| `terms` | JSONB | Program terms and conditions |
| `created_by` | UUID (FK → users) | Admin who created |
| `created_at` | TIMESTAMPTZ | Creation timestamp |
| `updated_at` | TIMESTAMPTZ | Last update |

### referral_tracking

| Column | Type | Description |
|--------|------|-------------|
| `id` | UUID (PK) | Referral identifier |
| `program_id` | UUID (FK → referral_programs) | Associated program |
| `referrer_id` | UUID (FK → customers) | Customer who shared |
| `referee_id` | UUID (FK → customers, nullable) | Customer who was referred |
| `referral_code` | VARCHAR(10) | Unique referral code |
| `referee_phone` | VARCHAR(15) | Phone number shared |
| `status` | ENUM | `pending`, `registered`, `activated`, `rewarded` |
| `referrer_reward_status` | ENUM | `pending`, `credited`, `applied`, `failed` |
| `referrer_reward_amount` | DECIMAL(10,2) | Actual reward amount |
| `referee_reward_status` | ENUM | `pending`, `applied`, `failed` |
| `referee_reward_amount` | DECIMAL(10,2) | Actual reward amount |
| `wallet_credit_id` | UUID (FK → customer_wallets, nullable) | Associated wallet credit |
| `invoice_discount_id` | UUID (FK → invoice_discounts, nullable) | Associated discount |
| `shared_at` | TIMESTAMPTZ | When code was shared |
| `registered_at` | TIMESTAMPTZ | When referee registered |
| `activated_at` | TIMESTAMPTZ | When referee subscribed |
| `rewarded_at` | TIMESTAMPTZ | When rewards were applied |

### customer_wallets

| Column | Type | Description |
|--------|------|-------------|
| `id` | UUID (PK) | Wallet identifier |
| `customer_id` | UUID (FK → customers) | Owner |
| `balance` | DECIMAL(10,2) | Available balance |
| `total_earned` | DECIMAL(10,2) | Total rewards earned |
| `total_used` | DECIMAL(10,2) | Total rewards redeemed |
| `currency` | VARCHAR(3) | INR |
| `created_at` | TIMESTAMPTZ | Creation timestamp |
| `updated_at` | TIMESTAMPTZ | Last update |

### wallet_transactions

| Column | Type | Description |
|--------|------|-------------|
| `id` | UUID (PK) | Transaction identifier |
| `wallet_id` | UUID (FK → customer_wallets) | Associated wallet |
| `type` | ENUM | `credit`, `debit`, `adjustment` |
| `amount` | DECIMAL(10,2) | Transaction amount |
| `reference_type` | VARCHAR(50) | `referral`, `invoice`, `manual` |
| `reference_id` | UUID | Associated entity ID |
| `description` | TEXT | Transaction description |
| `created_at` | TIMESTAMPTZ | Transaction timestamp |

---

## API Endpoints

### Customer-Facing APIs

| Method | Path | Required Role | Description |
|--------|------|---------------|-------------|
| `GET` | `/api/v1/customer/referrals/my-code` | customer | Get my referral code and stats |
| `GET` | `/api/v1/customer/referrals/my-referrals` | customer | List my referrals |
| `GET` | `/api/v1/customer/referrals/stats` | customer | Get referral stats and rewards |
| `GET` | `/api/v1/customer/referrals/program` | customer | Get active program details |
| `POST` | `/api/v1/customer/referrals/share` | customer | Record a referral share |
| `GET` | `/api/v1/customer/wallet` | customer | Get wallet balance and transactions |

### Admin APIs

| Method | Path | Required Role | Description |
|--------|------|---------------|-------------|
| `GET` | `/api/v1/admin/referral-programs` | finance_manager+ | List all programs |
| `POST` | `/api/v1/admin/referral-programs` | finance_manager+ | Create a new program |
| `PUT` | `/api/v1/admin/referral-programs/:id` | finance_manager+ | Update program |
| `DELETE` | `/api/v1/admin/referral-programs/:id` | super_admin | Delete program |
| `GET` | `/api/v1/admin/referrals` | finance_manager+ | List all referrals |
| `GET` | `/api/v1/admin/referrals/analytics` | finance_manager+ | Referral analytics |
| `GET` | `/api/v1/admin/referrals/export` | finance_manager+ | Export referral data |
| `GET` | `/api/v1/admin/wallets` | finance_manager+ | List all wallets |
| `POST` | `/api/v1/admin/wallets/:id/adjust` | finance_manager+ | Manual wallet adjustment |

---

## Business Logic

### Referral Code Generation

```rust
fn generate_referral_code(customer_name: &str) -> String {
    let prefix = customer_name
        .chars()
        .filter(|c| c.is_alphabetic())
        .take(4)
        .collect::<String>()
        .to_uppercase();
    
    let suffix = rand::thread_rng().gen_range(1000..9999);
    format!("{}{}", prefix, suffix)
    // Example: "RAHU2485"
}
```

### Referral Flow

```
1. Customer shares code → POST /api/v1/customer/referrals/share
   → Creates referral_tracking with status: "pending"

2. Friend registers → POST /api/v1/auth/otp/verify
   → Updates status: "registered"
   → Publishes event: referral.registered

3. Friend subscribes → POST /api/v1/customer/subscription
   → Updates status: "activated"
   → Publishes event: referral.activated

4. System processes rewards → Event handler
   → Credits referrer wallet
   → Applies discount to referee's first invoice
   → Updates status: "rewarded"
   → Publishes event: referral.rewarded
```

### Reward Processing

```rust
async fn process_referral_reward(
    pool: &PgPool,
    referral_id: Uuid,
    program: &ReferralProgram,
) -> Result<()> {
    let mut tx = pool.begin().await?;
    
    // 1. Credit referrer wallet
    if program.referrer_reward_type == "credit" {
        sqlx::query(
            "UPDATE customer_wallets SET balance = balance + $1, total_earned = total_earned + $1 WHERE customer_id = $2"
        )
        .bind(program.referrer_reward_value)
        .bind(referral.referrer_id)
        .execute(&mut *tx).await?;
        
        // Create wallet transaction
        sqlx::query(
            "INSERT INTO wallet_transactions (wallet_id, type, amount, reference_type, reference_id, description) VALUES ($1, 'credit', $2, 'referral', $3, $4)"
        )
        .bind(wallet.id)
        .bind(program.referrer_reward_value)
        .bind(referral_id)
        .bind(format!("Referral reward for {}", referral.referee_name))
        .execute(&mut *tx).await?;
    }
    
    // 2. Apply discount to referee's first invoice
    if program.referee_reward_type == "discount" {
        sqlx::query(
            "INSERT INTO invoice_discounts (invoice_id, type, value, description) VALUES ($1, 'percentage', $2, $3)"
        )
        .bind(referee_invoice.id)
        .bind(program.referee_reward_value)
        .bind("Referral welcome discount")
        .execute(&mut *tx).await?;
    }
    
    // 3. Update referral status
    sqlx::query(
        "UPDATE referral_tracking SET status = 'rewarded', referrer_reward_status = 'credited', referee_reward_status = 'applied', rewarded_at = NOW() WHERE id = $1"
    )
    .bind(referral_id)
    .execute(&mut *tx).await?;
    
    // 4. Journal entry
    sqlx::query(
        "INSERT INTO journal_entries (debit_account, credit_account, amount, description, reference_type, reference_id) VALUES ('marketing_expense', 'prepaid_expenses', $1, $2, 'referral', $3)"
    )
    .bind(program.referrer_reward_value)
    .bind("Referral reward - marketing expense")
    .bind(referral_id)
    .execute(&mut *tx).await?;
    
    tx.commit().await?;
    Ok(())
}
```

### Wallet Auto-Application

When an invoice is generated, check for available wallet balance:

```rust
async fn apply_wallet_to_invoice(
    pool: &PgPool,
    customer_id: Uuid,
    invoice_id: Uuid,
    invoice_amount: Decimal,
) -> Result<Decimal> {
    let wallet = sqlx::query_as::<_, CustomerWallet>(
        "SELECT * FROM customer_wallets WHERE customer_id = $1 AND balance > 0"
    )
    .bind(customer_id)
    .fetch_optional(pool).await?;
    
    match wallet {
        Some(w) => {
            let applied = std::cmp::min(w.balance, invoice_amount);
            
            // Debit wallet
            sqlx::query(
                "UPDATE customer_wallets SET balance = balance - $1, total_used = total_used + $1 WHERE id = $2"
            )
            .bind(applied)
            .bind(w.id)
            .execute(pool).await?;
            
            // Create transaction
            sqlx::query(
                "INSERT INTO wallet_transactions (wallet_id, type, amount, reference_type, reference_id, description) VALUES ($1, 'debit', $2, 'invoice', $3, $4)"
            )
            .bind(w.id)
            .bind(applied)
            .bind(invoice_id)
            .bind("Auto-applied to invoice")
            .execute(pool).await?;
            
            // Apply as discount on invoice
            sqlx::query(
                "INSERT INTO invoice_discounts (invoice_id, type, value, description) VALUES ($1, 'fixed', $2, 'Wallet balance applied')"
            )
            .bind(invoice_id)
            .bind(applied)
            .execute(pool).await?;
            
            Ok(applied)
        }
        None => Ok(Decimal::ZERO),
    }
}
```

---

## Events

| Event | Payload | Trigger |
|-------|---------|---------|
| `referral.created` | `{ referral_id, referrer_id, referee_phone, program_id }` | Code shared |
| `referral.registered` | `{ referral_id, referee_id, referrer_id }` | Friend registers |
| `referral.activated` | `{ referral_id, referee_id, referrer_id, subscription_id }` | Friend subscribes |
| `referral.rewarded` | `{ referral_id, referrer_reward, referee_reward }` | Rewards applied |
| `wallet.credited` | `{ wallet_id, amount, reference_id }` | Wallet credited |
| `wallet.debited` | `{ wallet_id, amount, reference_id }` | Wallet debited |

---

## RBAC Permissions

| Permission | Roles |
|------------|-------|
| `referral.view` | customer (own), finance_manager, super_admin |
| `referral.share` | customer (own) |
| `referral.program.view` | finance_manager, super_admin |
| `referral.program.create` | finance_manager, super_admin |
| `referral.program.update` | finance_manager, super_admin |
| `referral.program.delete` | super_admin |
| `referral.export` | finance_manager, super_admin |
| `wallet.view` | customer (own), finance_manager, super_admin |
| `wallet.credit` | system (automated), finance_manager, super_admin |
| `wallet.debit` | system (automated) |
| `wallet.adjust` | finance_manager, super_admin |

---

## Database Indexes

```sql
-- Referral tracking
CREATE INDEX idx_referral_tracking_referrer ON referral_tracking(referrer_id);
CREATE INDEX idx_referral_tracking_referee ON referral_tracking(referee_id);
CREATE INDEX idx_referral_tracking_code ON referral_tracking(referral_code);
CREATE INDEX idx_referral_tracking_status ON referral_tracking(status);
CREATE INDEX idx_referral_tracking_program ON referral_tracking(program_id);

-- Wallets
CREATE INDEX idx_customer_wallets_customer ON customer_wallets(customer_id);
CREATE INDEX idx_wallet_transactions_wallet ON wallet_transactions(wallet_id);

-- Programs
CREATE INDEX idx_referral_programs_status ON referral_programs(status);
```
