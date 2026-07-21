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

> **API Convention:** Protobuf-first. No GET, no PUT, no path variables, no query strings. See `API-CONVENTIONS.md`.

### Customer-Facing APIs

| Method | Path | Required Role | Description |
|--------|------|---------------|-------------|
| `POST` | `/api/v1/customer/referrals/my-code` | customer | Get my referral code and stats |
| `POST` | `/api/v1/customer/referrals/my-referrals/list` | customer | List my referrals |
| `POST` | `/api/v1/customer/referrals/stats` | customer | Get referral stats and rewards |
| `POST` | `/api/v1/customer/referrals/program` | customer | Get active program details |
| `POST` | `/api/v1/customer/referrals/share` | customer | Record a referral share |
| `POST` | `/api/v1/customer/wallet/list` | customer | Get wallet balance and transactions |

### Admin APIs

| Method | Path | Required Role | Description |
|--------|------|---------------|-------------|
| `POST` | `/api/v1/admin/referral-programs/list` | finance_manager+ | List all programs |
| `POST` | `/api/v1/admin/referral-programs/create` | finance_manager+ | Create a new program |
| `PATCH` | `/api/v1/admin/referral-programs/update` | finance_manager+ | Update program |
| `DELETE` | `/api/v1/admin/referral-programs/delete` | super_admin | Delete program |
| `POST` | `/api/v1/admin/referrals/list` | finance_manager+ | List all referrals |
| `POST` | `/api/v1/admin/referrals/analytics` | finance_manager+ | Referral analytics |
| `POST` | `/api/v1/admin/referrals/export` | finance_manager+ | Export referral data |
| `POST` | `/api/v1/admin/wallets/list` | finance_manager+ | List all wallets |
| `POST` | `/api/v1/admin/wallets/adjust` | finance_manager+ | Manual wallet adjustment |

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
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set, TransactionTrait};
use chrono::Utc;

async fn process_referral_reward(
    db: &DatabaseConnection,
    referral_id: Uuid,
    program: &ReferralProgram,
) -> Result<()> {
    db.transaction::<_, _, DbErr>(|txn| {
        Box::pin(async move {
            // 1. Credit referrer wallet
            if program.referrer_reward_type == "credit" {
                let wallet = customer_wallet::Entity::find()
                    .filter(customer_wallet::Column::CustomerId.eq(referrer_id))
                    .one(txn)
                    .await?;

                if let Some(w) = wallet {
                    let mut active: customer_wallet::ActiveModel = w.into();
                    let new_balance = active.balance.clone().unwrap() + program.referrer_reward_value;
                    let new_earned = active.total_earned.clone().unwrap() + program.referrer_reward_value;
                    active.balance = Set(new_balance);
                    active.total_earned = Set(new_earned);
                    active.updated_at = Set(Utc::now());
                    active.update(txn).await?;

                    // Create wallet transaction
                    let txn_record = wallet_transaction::ActiveModel {
                        wallet_id: Set(w.id),
                        r#type: Set("credit".to_owned()),
                        amount: Set(program.referrer_reward_value),
                        reference_type: Set("referral".to_owned()),
                        reference_id: Set(Some(referral_id.to_string())),
                        description: Set(format!("Referral reward for {}", referee_name)),
                        ..Default::default()
                    };
                    txn_record.insert(txn).await?;
                }
            }

            // 2. Apply discount to referee's first invoice
            if program.referee_reward_type == "discount" {
                if let Some(invoice_id) = referee_invoice_id {
                    let discount = invoice_discount::ActiveModel {
                        invoice_id: Set(invoice_id),
                        r#type: Set("percentage".to_owned()),
                        value: Set(program.referrer_reward_value),
                        description: Set("Referral welcome discount".to_owned()),
                        ..Default::default()
                    };
                    discount.insert(txn).await?;
                }
            }

            // 3. Update referral status
            let mut active = referral_model;
            active.status = Set("rewarded".to_owned());
            active.referrer_reward_status = Set("credited".to_owned());
            active.referee_reward_status = Set("applied".to_owned());
            active.rewarded_at = Set(Some(Utc::now()));
            active.update(txn).await?;

            // 4. Journal entry
            let journal = journal_entry::ActiveModel {
                debit_account: Set("marketing_expense".to_owned()),
                credit_account: Set("prepaid_expenses".to_owned()),
                amount: Set(program.referrer_reward_value),
                description: Set("Referral reward - marketing expense".to_owned()),
                reference_type: Set(Some("referral".to_owned())),
                reference_id: Set(Some(referral_id.to_string())),
                ..Default::default()
            };
            journal.insert(txn).await?;

            Ok(())
        })
    })
    .await?;
    Ok(())
}
```

### Wallet Auto-Application

When an invoice is generated, check for available wallet balance:

```rust
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set, TransactionTrait};
use rust_decimal::Decimal;
use std::cmp::min;

async fn apply_wallet_to_invoice(
    db: &DatabaseConnection,
    customer_id: Uuid,
    invoice_id: Uuid,
    invoice_amount: Decimal,
) -> Result<Decimal> {
    let wallet = customer_wallet::Entity::find()
        .filter(customer_wallet::Column::CustomerId.eq(customer_id))
        .filter(customer_wallet::Column::Balance.gt(Decimal::ZERO))
        .one(db)
        .await?;

    match wallet {
        Some(w) => {
            let applied = min(w.balance, invoice_amount);

            db.transaction::<_, _, DbErr>(|txn| {
                Box::pin(async move {
                    // Debit wallet
                    let mut active: customer_wallet::ActiveModel = w.clone().into();
                    active.balance = Set(w.balance - applied);
                    active.total_used = Set(w.total_used + applied);
                    active.updated_at = Set(Utc::now());
                    active.update(txn).await?;

                    // Create transaction record
                    let txn_record = wallet_transaction::ActiveModel {
                        wallet_id: Set(w.id),
                        r#type: Set("debit".to_owned()),
                        amount: Set(applied),
                        reference_type: Set("invoice".to_owned()),
                        reference_id: Set(Some(invoice_id.to_string())),
                        description: Set("Auto-applied to invoice".to_owned()),
                        ..Default::default()
                    };
                    txn_record.insert(txn).await?;

                    // Apply as discount on invoice
                    let discount = invoice_discount::ActiveModel {
                        invoice_id: Set(invoice_id),
                        r#type: Set("fixed".to_owned()),
                        value: Set(applied),
                        description: Set("Wallet balance applied".to_owned()),
                        ..Default::default()
                    };
                    discount.insert(txn).await?;

                    Ok(())
                })
            })
            .await?;

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
