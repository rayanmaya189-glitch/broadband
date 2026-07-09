use sqlx::{Executor, PgPool, Postgres, Transaction};

/// Set the PostgreSQL session variable to mark the current connection
/// as company-wide, bypassing all RLS policies.
///
/// This must be called at the START of each background job tick,
/// before any queries against RLS-protected tables.
///
/// Uses `SET LOCAL` so the setting is scoped to the current transaction
/// and does not leak to other connections in the pool.
///
/// # Safety
/// This should ONLY be used by background jobs (SLA checker, dunning,
/// renewal reminders, partition manager, data cleanup) that legitimately
/// need cross-branch access. Never call this from HTTP request handlers.
pub async fn set_company_wide<'e, E>(executor: E) -> Result<(), sqlx::Error>
where
    E: Executor<'e, Database = Postgres>,
{
    sqlx::query("SELECT set_config('app.is_company_wide', 'true', true)")
        .execute(executor)
        .await?;
    Ok(())
}

/// Acquire a connection from the pool and set it as company-wide
/// for the duration of the returned connection.
///
/// Returns a connection that bypasses RLS for all subsequent queries.
/// The setting is automatically cleared when the connection is returned
/// to the pool (because it uses `SET LOCAL` within a transaction).
pub async fn acquire_bypass_connection(pool: &PgPool) -> Result<sqlx::pool::PoolConnection<Postgres>, sqlx::Error> {
    let mut conn = pool.acquire().await?;
    sqlx::query("SELECT set_config('app.is_company_wide', 'true', true)")
        .execute(&mut *conn)
        .await?;
    Ok(conn)
}

/// Begin a transaction with RLS bypass enabled.
///
/// The `SET LOCAL` is scoped to the transaction, so all queries within
/// the transaction will bypass RLS. When the transaction is committed
/// or rolled back, the setting is automatically cleared.
pub async fn begin_bypass_transaction(pool: &PgPool) -> Result<Transaction<'static, Postgres>, sqlx::Error> {
    let mut tx = pool.begin().await?;
    sqlx::query("SELECT set_config('app.is_company_wide', 'true', true)")
        .execute(&mut *tx)
        .await?;
    Ok(tx)
}
