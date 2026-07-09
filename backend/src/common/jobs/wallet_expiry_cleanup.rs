use std::time::Duration;

use sqlx::{FromRow, PgPool};
use tracing::{info, warn, error};

use crate::app::SharedState;

/// Default check interval: every 6 hours.
const DEFAULT_INTERVAL_SECS: u64 = 21600;

#[derive(Debug, FromRow)]
struct ExpiringWalletCredit {
    id: i64,
    wallet_id: i64,
    amount: rust_decimal::Decimal,
    expires_at: chrono::DateTime<chrono::Utc>,
    customer_id: i64,
}

/// Find wallet transactions that have expired but haven't been processed yet.
async fn find_expired_credits(pool: &PgPool) -> Result<Vec<ExpiringWalletCredit>, sqlx::Error> {
    sqlx::query_as::<_, ExpiringWalletCredit>(
        "SELECT wt.id, wt.wallet_id, wt.amount, wt.expires_at, cw.customer_id
         FROM wallet_transactions wt
         JOIN customer_wallets cw ON cw.id = wt.wallet_id
         WHERE wt.expires_at IS NOT NULL
           AND wt.expires_at < NOW()
           AND wt.transaction_type = 'credit'
           AND wt.reference_type != 'expiry'
           AND NOT EXISTS (
               SELECT 1 FROM wallet_transactions wt2
               WHERE wt2.wallet_id = wt.wallet_id
                 AND wt2.reference_type = 'expiry'
                 AND wt2.reference_id = wt.id
           )
         ORDER BY wt.expires_at ASC
         LIMIT 200",
    )
    .fetch_all(pool)
    .await
}

/// Process a single expired credit: deduct the amount and record an expiry transaction.
async fn process_expiry(
    pool: &PgPool,
    credit: &ExpiringWalletCredit,
) -> Result<(), sqlx::Error> {
    // Deduct the expired amount from the wallet (atomic with balance guard)
    let result = sqlx::query(
        "UPDATE customer_wallets
         SET balance = balance - $2,
             updated_at = NOW()
         WHERE id = $1 AND balance >= $2",
    )
    .bind(credit.wallet_id)
    .bind(credit.amount)
    .execute(pool)
    .await?;

    if result.rows_affected() == 0 {
        warn!(
            wallet_id = credit.wallet_id,
            amount = %credit.amount,
            "Insufficient balance to expire credit — wallet may already be depleted"
        );
        // Still record the expiry transaction with zero deduction for audit trail
    }

    // Fetch updated wallet for balance_after
    let wallet: (rust_decimal::Decimal,) = sqlx::query_as(
        "SELECT balance FROM customer_wallets WHERE id = $1",
    )
    .bind(credit.wallet_id)
    .fetch_one(pool)
    .await?;

    // Record the expiry transaction
    sqlx::query(
        "INSERT INTO wallet_transactions
             (wallet_id, transaction_type, amount, balance_after,
              reference_type, reference_id, description)
         VALUES ($1, 'expiry', $2, $3, 'expiry', $4, $5)",
    )
    .bind(credit.wallet_id)
    .bind(-credit.amount) // Negative to show deduction
    .bind(wallet.0)
    .bind(credit.id)
    .bind(format!(
        "Credit expired (originally expires_at: {})",
        credit.expires_at.format("%Y-%m-%d %H:%M:%S UTC")
    ))
    .execute(pool)
    .await?;

    info!(
        wallet_id = credit.wallet_id,
        customer_id = credit.customer_id,
        expired_amount = %credit.amount,
        new_balance = %wallet.0,
        "Wallet credit expired"
    );

    Ok(())
}

/// Main wallet expiry cleanup loop.
pub async fn run_wallet_expiry_cleanup(state: SharedState) {
    let interval_secs = std::env::var("WALLET_EXPIRY_INTERVAL_SECS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(DEFAULT_INTERVAL_SECS);

    let mut interval = tokio::time::interval(Duration::from_secs(interval_secs));

    info!(
        interval_secs = interval_secs,
        "Wallet expiry cleanup job started"
    );

    loop {
        interval.tick().await;

        match find_expired_credits(&state.db).await {
            Ok(credits) if credits.is_empty() => {
                // No expired credits
            }
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
                            warn!(
                                wallet_id = credit.wallet_id,
                                transaction_id = credit.id,
                                error = %e,
                                "Failed to process wallet credit expiry"
                            );
                        }
                    }
                }

                info!(
                    total = count,
                    processed = processed,
                    failed = failed,
                    "Wallet expiry cleanup batch complete"
                );
            }
            Err(e) => {
                error!(error = %e, "Failed to query expired wallet credits");
            }
        }
    }
}
