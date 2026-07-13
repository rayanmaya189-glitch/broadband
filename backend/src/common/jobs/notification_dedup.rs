//! Notification deduplication — checks if a notification already exists today.
//!
//! Pure SeaORM implementation — zero raw SQL queries.

use sea_orm::*;

use crate::modules::notification::model::notification_entity::{self, Entity as NotificationEntity};

/// Check if a notification already exists for this recipient + type today.
/// Returns true if a duplicate already exists.
pub async fn notification_exists_today(
    db: &DatabaseConnection,
    recipient_id: i64,
    notification_type: &str,
) -> Result<bool, DbErr> {
    let today = chrono::Utc::now().date_naive();

    let count = NotificationEntity::find()
        .filter(notification_entity::Column::CustomerId.eq(recipient_id))
        .filter(notification_entity::Column::Body.contains(notification_type))
        .filter(notification_entity::Column::CreatedAt.gte(today))
        .count(db)
        .await?;

    Ok(count > 0)
}
