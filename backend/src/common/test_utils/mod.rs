//! Shared test utilities for integration tests.
//!
//! Provides reusable helpers for setting up test database schemas
//! and inserting test data, so test files don't duplicate boilerplate.

use sea_orm::{ActiveModelTrait, ConnectionTrait, DatabaseConnection, Set, Statement};

// ── Notification helpers ────────────────────────────────────

/// SQL to create the `notifications` table for integration tests.
pub const NOTIFICATIONS_TABLE_SQL: &str = "CREATE TABLE IF NOT EXISTS notifications (
    id BIGSERIAL PRIMARY KEY,
    customer_id BIGINT,
    branch_id BIGINT,
    type VARCHAR(30) NOT NULL,
    channel VARCHAR(30) NOT NULL,
    title VARCHAR(255),
    body TEXT,
    metadata JSONB,
    status VARCHAR(20) NOT NULL DEFAULT 'queued',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
)";

/// Create the notifications table in a test database.
pub async fn setup_notifications_schema(db: &DatabaseConnection) {
    db.execute(Statement::from_string(
        sea_orm::DatabaseBackend::Postgres,
        NOTIFICATIONS_TABLE_SQL.to_string(),
    ))
    .await
    .unwrap();
}

/// Insert a test notification and return its id.
pub async fn insert_test_notification(
    db: &DatabaseConnection,
    status: &str,
    channel: &str,
) -> i64 {
    use crate::modules::notification::model::notification_entity;

    let active = notification_entity::ActiveModel {
        r#type: Set("direct".to_owned()),
        channel: Set(channel.to_owned()),
        title: Set(Some("Test Title".to_owned())),
        body: Set(Some("Test body content".to_owned())),
        status: Set(status.to_owned()),
        created_at: Set(chrono::Utc::now().into()),
        ..Default::default()
    };
    let model = active.insert(db).await.unwrap();
    model.id
}
