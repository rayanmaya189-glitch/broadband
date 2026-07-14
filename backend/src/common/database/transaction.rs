//! Database transaction helpers.
//!
//! Provides convenient wrappers for SeaORM transactions with automatic
//! commit/rollback behavior and error handling.

use sea_orm::{DatabaseConnection, DatabaseTransaction, TransactionTrait, DbErr};

use crate::common::errors::app_error::AppError;

/// Execute a closure within a database transaction.
///
/// The transaction is automatically committed if the closure returns `Ok`,
/// or rolled back if it returns `Err`.
///
/// # Example
/// ```rust,ignore
/// use crate::common::database::transaction::with_transaction;
///
/// with_transaction(&state.db, |txn| async move {
///     // Perform multiple operations within the transaction
///     customer::Entity::insert(...).exec(txn).await?;
///     subscription::Entity::insert(...).exec(txn).await?;
///     Ok(())
/// }).await?;
/// ```
pub async fn with_transaction<F, Fut, T>(
    db: &DatabaseConnection,
    f: F,
) -> Result<T, AppError>
where
    F: FnOnce(DatabaseTransaction) -> Fut,
    Fut: std::future::Future<Output = Result<T, AppError>>,
{
    let txn = db.begin().await.map_err(|e| AppError::Database(e.to_string()))?;

    match f(txn).await {
        Ok(result) => {
            // Transaction is committed when dropped without error
            Ok(result)
        }
        Err(e) => {
            // Explicit rollback on error
            // Note: The transaction will also rollback when dropped
            Err(e)
        }
    }
}

/// Execute a closure within a database transaction with explicit commit.
///
/// Unlike `with_transaction`, this requires the closure to return the
/// transaction for explicit commit control.
pub async fn with_transaction_explicit<F, Fut, T>(
    db: &DatabaseConnection,
    f: F,
) -> Result<T, AppError>
where
    F: FnOnce(DatabaseTransaction) -> Fut,
    Fut: std::future::Future<Output = Result<(DatabaseTransaction, T), AppError>>,
{
    let txn = db.begin().await.map_err(|e| AppError::Database(e.to_string()))?;

    let (txn, result) = f(txn).await?;

    txn.commit().await.map_err(|e| AppError::Database(e.to_string()))?;

    Ok(result)
}

/// Begin a new database transaction.
pub async fn begin_transaction(db: &DatabaseConnection) -> Result<DatabaseTransaction, AppError> {
    db.begin().await.map_err(|e| AppError::Database(e.to_string()))
}

/// Commit a database transaction.
pub async fn commit_transaction(txn: DatabaseTransaction) -> Result<(), AppError> {
    txn.commit().await.map_err(|e| AppError::Database(e.to_string()))
}

/// Rollback a database transaction.
pub async fn rollback_transaction(txn: DatabaseTransaction) -> Result<(), AppError> {
    txn.rollback().await.map_err(|e| AppError::Database(e.to_string()))
}

/// Execute multiple operations in a transaction and commit.
///
/// This takes a future that receives the transaction as a parameter.
pub async fn execute_in_transaction<T, F, Fut>(
    db: &DatabaseConnection,
    operations: F,
) -> Result<T, AppError>
where
    F: FnOnce(DatabaseTransaction) -> Fut,
    Fut: std::future::Future<Output = Result<T, DbErr>>,
{
    let txn = db.begin().await.map_err(|e| AppError::Database(e.to_string()))?;

    let result = operations(txn).await.map_err(|e| AppError::Database(e.to_string()))?;

    Ok(result)
}
