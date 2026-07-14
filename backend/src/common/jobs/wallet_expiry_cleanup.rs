//! Wallet expiry cleanup — expires wallet credits that have passed their expiry date.
//!
//! Pure SeaORM implementation — zero raw SQL queries.

use std::time::Duration;

use sea_orm::*;
use tokio_util::sync::CancellationToken;
use tracing::{info, warn, error};

use crate::app::SharedState;
use crate::modules::referral::model::wallet_transaction_entity::{self, Entity as WalletTxEntity};
use crate::modules::referral::model::customer_wallet_entity::{self, Entity as WalletEntity};

const DEFAULT_INTERVAL_SECS: u64 = 21600; // 6 hours

/// Find expired wallet credits using pure SeaORM queries.
/// Batch approach: first get all expiry reference IDs, then exclude them.
async fn find_expired_credits(
    db: &DatabaseConnection,
) -> Result<Vec<wallet_transaction_entity::Model>, DbErr> {
    let now = chrono::Utc::now();

    // Step 1: Get all reference_ids that already have expiry records (batch query)
    let expiry_records = WalletTxEntity::find()
        .filter(wallet_transaction_entity::Column::TransactionType.eq("expiry"))
        .filter(wallet_transaction_entity::Column::ReferenceType.eq("expiry"))
        .filter(wallet_transaction_entity::Column::ReferenceId.is_not_null())
        .all(db)
        .await?;

    let already_expired_ids: Vec<i64> = expiry_records
        .into_iter()
        .filter_map(|r| r.reference_id)
        .collect();

    // Step 2: Find expired credits excluding those already processed (single batch query)
    let mut query = WalletTxEntity::find()
        .filter(wallet_transaction_entity::Column::ExpiresAt.is_not_null())
        .filter(wallet_transaction_entity::Column::ExpiresAt.lt(now))
        .filter(wallet_transaction_entity::Column::TransactionType.eq("credit"))
        .filter(wallet_transaction_entity::Column::ReferenceType.ne("expiry"));

    if !already_expired_ids.is_empty() {
        query = query.filter(wallet_transaction_entity::Column::Id.is_not_in(already_expired_ids));
    }

    let credits = query
        .order_by_asc(wallet_transaction_entity::Column::ExpiresAt)
        .limit(200)
        .all(db)
        .await?;

    Ok(credits)
}

/// Process expiry for a single wallet credit.
async fn process_expiry(
    db: &DatabaseConnection,
    credit: &wallet_transaction_entity::Model,
) -> Result<(), DbErr> {
    let wallet = WalletEntity::find_by_id(credit.wallet_id)
        .one(db)
        .await?
        .ok_or_else(|| DbErr::Custom("Wallet not found".to_string()))?;

    if wallet.balance < credit.amount {
        warn!(
            wallet_id = credit.wallet_id,
            amount = %credit.amount,
            "Insufficient balance to expire credit"
        );
        return Ok(());
    }

    let new_balance = wallet.balance - credit.amount;
    let mut wallet_active: customer_wallet_entity::ActiveModel = wallet.into();
    wallet_active.balance = Set(new_balance);
    wallet_active.updated_at = Set(chrono::Utc::now().into());
    wallet_active.update(db).await?;

    let expiry_active = wallet_transaction_entity::ActiveModel {
        wallet_id: Set(credit.wallet_id),
        transaction_type: Set("expiry".to_string()),
        amount: Set(-credit.amount),
        balance_after: Set(new_balance),
        reference_type: Set(Some("expiry".to_string())),
        reference_id: Set(Some(credit.id)),
        description: Set(Some(format!(
            "Credit expired (expires_at: {})",
            credit.expires_at
                .map(|t| t.format("%Y-%m-%d %H:%M:%S UTC").to_string())
                .unwrap_or_default()
        ))),
        ..Default::default()
    };
    expiry_active.insert(db).await?;

    info!(
        wallet_id = credit.wallet_id,
        expired_amount = %credit.amount,
        "Wallet credit expired"
    );
    Ok(())
}

pub async fn run_wallet_expiry_cleanup(state: SharedState, token: CancellationToken) {
    let interval_secs = std::env::var("WALLET_EXPIRY_INTERVAL_SECS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(DEFAULT_INTERVAL_SECS);
    let mut interval = tokio::time::interval(Duration::from_secs(interval_secs));
    info!(interval_secs = interval_secs, "Wallet expiry cleanup job started");

    loop {
        tokio::select! {
            _ = interval.tick() => {
                if let Err(e) = super::set_rls_bypass(&state.db).await {
                    warn!(error = %e, "Failed to set RLS bypass context");
                    continue;
                }

                match find_expired_credits(&state.db).await {
                    Ok(credits) if credits.is_empty() => {}
                    Ok(credits) => {
                        let count = credits.len();
                        info!(count = count, "Found expired wallet credits, processing");
                        let mut processed = 0u64;
                        let mut failed = 0u64;
                        for credit in &credits {
                            match process_expiry(&state.db, credit).await {
                                Ok(()) => processed += 1,
                                Err(e) => {
                                    failed += 1;
                                    warn!(wallet_id = credit.wallet_id, error = %e, "Failed to process wallet credit expiry");
                                }
                            }
                        }
                        info!(total = count, processed = processed, failed = failed, "Wallet expiry cleanup batch complete");
                    }
                    Err(e) => error!(error = %e, "Failed to query expired wallet credits"),
                }
            }
            _ = token.cancelled() => {
                info!("Wallet expiry cleanup shutting down gracefully");
                break;
            }
        }
    }
}
