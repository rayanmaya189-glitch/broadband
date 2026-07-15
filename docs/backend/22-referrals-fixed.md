# AeroXe Backend — Referrals Module

## Overview

The Referral Program module manages customer referral incentives, tracking them through a funnel from sharing to reward application. Customers share unique referral codes, and both referrer and referee receive rewards when the referred customer subscribes. The program is fully managed by admins.

---

## Business Logic

### Reward Processing (SeaORM)

```rust
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait,
    QueryFilter, Set, TransactionTrait,
};
use chrono::Utc;
use uuid::Uuid;

/// Process referral reward after referee subscribes.
/// Fetches referral record before transaction to avoid closure capture issues.
async fn process_referral_reward(
    db: &DatabaseConnection,
    referral_id: Uuid,
    program: &ReferralProgram,
    referee_invoice_id: Option<Uuid>,
) -> Result<()> {
    // 1. Fetch the referral record BEFORE starting the transaction
    let referral = referral_tracking::Entity::find_by_id(referral_id)
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound("Referral not found".into()))?;

    let referrer_id = referral.referrer_id;
    let referee_name = referral.referee_name.clone();
    let referral_model: referral_tracking::ActiveModel = referral.into();

    db.transaction::<_, _, DbErr>(|txn| {
        Box::pin(async move {
            // 2. Credit referrer wallet
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

            // 3. Apply discount to referee's first invoice
            if program.referee_reward_type == "discount" {
                if let Some(invoice_id) = referee_invoice_id {
                    let discount = invoice_discount::ActiveModel {
                        invoice_id: Set(invoice_id),
                        r#type: Set("percentage".to_owned()),
                        value: Set(program.referee_reward_value),
                        description: Set("Referral welcome discount".to_owned()),
                        ..Default::default()
                    };
                    discount.insert(txn).await?;
                }
            }

            // 4. Update referral status
            let mut active = referral_model;
            active.status = Set("rewarded".to_owned());
            active.referrer_reward_status = Set("credited".to_owned());
            active.referee_reward_status = Set("applied".to_owned());
            active.rewarded_at = Set(Some(Utc::now()));
            active.update(txn).await?;

            // 5. Journal entry
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

### Wallet Auto-Application (SeaORM)

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
