use sqlx::{Executor, Postgres};

/// Check if a notification already exists for this recipient + type today.
/// Returns true if a duplicate already exists.
/// Works with both PgPool and PgConnection (via the Executor trait).
pub async fn notification_exists_today<'e, E>(
    executor: E,
    recipient_id: i64,
    notification_type: &str,
) -> Result<bool, sqlx::Error>
where
    E: Executor<'e, Database = Postgres>,
{
    let row: (i64,) = sqlx::query_as(
        "SELECT COUNT(*)::bigint FROM notifications
         WHERE recipient_id = $1
           AND body LIKE '%' || $2 || '%'
           AND created_at >= CURRENT_DATE",
    )
    .bind(recipient_id)
    .bind(notification_type)
    .fetch_one(executor)
    .await?;

    Ok(row.0 > 0)
}
