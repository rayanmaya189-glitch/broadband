use std::time::Duration;

use sqlx::FromRow;
use tokio_util::sync::CancellationToken;
use tracing::{info, warn, error};

use crate::app::SharedState;

const DEFAULT_INTERVAL_SECS: u64 = 21600;

#[derive(Debug, FromRow)]
struct ExpiringWalletCredit {
    id: i64, wallet_id: i64, amount: rust_decimal::Decimal,
    expires_at: chrono::DateTime<chrono::Utc>, _customer_id: i64,
}

async fn find_expired_credits(conn: &mut sqlx::PgConnection) -> Result<Vec<ExpiringWalletCredit>, sqlx::Error> {
    sqlx::query_as::<_, ExpiringWalletCredit>(
        "SELECT wt.id, wt.wallet_id, wt.amount, wt.expires_at, cw.customer_id
         FROM wallet_transactions wt JOIN customer_wallets cw ON cw.id = wt.wallet_id
         WHERE wt.expires_at IS NOT NULL AND wt.expires_at < NOW()
           AND wt.transaction_type = 'credit' AND wt.reference_type != 'expiry'
           AND NOT EXISTS (SELECT 1 FROM wallet_transactions wt2 WHERE wt2.wallet_id = wt.wallet_id AND wt2.reference_type = 'expiry' AND wt2.reference_id = wt.id)
         ORDER BY wt.expires_at ASC LIMIT 200",
    ).fetch_all(&mut *conn).await
}

async fn process_expiry(conn: &mut sqlx::PgConnection, credit: &ExpiringWalletCredit) -> Result<(), sqlx::Error> {
    let result = sqlx::query("UPDATE customer_wallets SET balance = balance - $2, updated_at = NOW() WHERE id = $1 AND balance >= $2")
        .bind(credit.wallet_id).bind(credit.amount).execute(&mut *conn).await?;
    if result.rows_affected() == 0 { warn!(wallet_id = credit.wallet_id, amount = %credit.amount, "Insufficient balance to expire credit"); }
    let wallet: (rust_decimal::Decimal,) = sqlx::query_as("SELECT balance FROM customer_wallets WHERE id = $1").bind(credit.wallet_id).fetch_one(&mut *conn).await?;
    sqlx::query("INSERT INTO wallet_transactions (wallet_id, transaction_type, amount, balance_after, reference_type, reference_id, description) VALUES ($1, 'expiry', $2, $3, 'expiry', $4, $5)")
        .bind(credit.wallet_id).bind(-credit.amount).bind(wallet.0).bind(credit.id)
        .bind(format!("Credit expired (expires_at: {})", credit.expires_at.format("%Y-%m-%d %H:%M:%S UTC")))
        .execute(&mut *conn).await?;
    info!(wallet_id = credit.wallet_id, expired_amount = %credit.amount, "Wallet credit expired");
    Ok(())
}

pub async fn run_wallet_expiry_cleanup(state: SharedState, token: CancellationToken) {
    let interval_secs = std::env::var("WALLET_EXPIRY_INTERVAL_SECS").ok().and_then(|v| v.parse().ok()).unwrap_or(DEFAULT_INTERVAL_SECS);
    let mut interval = tokio::time::interval(Duration::from_secs(interval_secs));
    info!(interval_secs = interval_secs, "Wallet expiry cleanup job started");

    loop {
        tokio::select! {
            _ = interval.tick() => {
                let mut tx = match super::rls_bypass::begin_bypass_transaction(&state.db).await {
                    Ok(t) => t, Err(e) => { warn!(error = %e, "Failed to begin RLS bypass transaction"); continue; }
                };
                match find_expired_credits(&mut tx).await {
                    Ok(credits) if credits.is_empty() => {}
                    Ok(credits) => {
                        let count = credits.len(); info!(count = count, "Found expired wallet credits, processing");
                        let mut processed = 0u64; let mut failed = 0u64;
                        for credit in &credits {
                            match process_expiry(&mut tx, credit).await {
                                Ok(()) => processed += 1,
                                Err(e) => { failed += 1; warn!(wallet_id = credit.wallet_id, error = %e, "Failed to process wallet credit expiry"); }
                            }
                        }
                        info!(total = count, processed = processed, failed = failed, "Wallet expiry cleanup batch complete");
                    }
                    Err(e) => error!(error = %e, "Failed to query expired wallet credits"),
                }
                if let Err(e) = tx.commit().await { error!(error = %e, "Failed to commit RLS bypass transaction"); }
            }
            _ = token.cancelled() => { info!("Wallet expiry cleanup shutting down gracefully"); break; }
        }
    }
}
