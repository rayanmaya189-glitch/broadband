use sqlx::PgPool;

/// Check if a notification already exists for this recipient + type today.
/// Returns true if a duplicate already exists.
pub async fn notification_exists_today(
    pool: &PgPool,
    recipient_id: i64,
    notification_type: &str,
) -> Result<bool, sqlx::Error> {
    let row: (i64,) = sqlx::query_as(
        "SELECT COUNT(*)::bigint FROM notifications
         WHERE recipient_id = $1
           AND body LIKE '%' || $2 || '%'
           AND created_at >= CURRENT_DATE",
    )
    .bind(recipient_id)
    .bind(notification_type)
    .fetch_one(pool)
    .await?;

    Ok(row.0 > 0)
}
