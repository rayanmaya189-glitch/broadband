use sqlx::{PgPool, Postgres, Transaction};

/// Begin a transaction with RLS bypass enabled.
///
/// Sets `app.is_company_wide = true` via `SET LOCAL` which persists
/// for the entire transaction. All queries within the transaction
/// will bypass RLS policies (return TRUE for all rows).
///
/// The transaction must be committed or rolled back to release the
/// setting. This also provides atomicity for multi-query operations.
///
/// # Safety
/// This should ONLY be used by background jobs that legitimately
/// need cross-branch access. Never call this from HTTP request handlers.
pub async fn begin_bypass_transaction(pool: &PgPool) -> Result<Transaction<'static, Postgres>, sqlx::Error> {
    let mut tx = pool.begin().await?;
    sqlx::query("SELECT set_config('app.is_company_wide', 'true', true)")
        .execute(&mut *tx)
        .await?;
    Ok(tx)
}
