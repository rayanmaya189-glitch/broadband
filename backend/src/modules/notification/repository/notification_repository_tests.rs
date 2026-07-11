#[cfg(test)]
mod tests {
    use sea_orm::{DatabaseConnection, SqlxPostgresConnector, EntityTrait, Set, ActiveModelTrait, QueryFilter, ColumnTrait};
    use crate::modules::notification::model::notification_entity::{self, Entity as NotificationEntity};
    use crate::modules::notification::repository::notification_repository::NotificationRepository;

    /// Helper: create the notifications table in a fresh test database.
    async fn setup_schema(db: &DatabaseConnection) {
        db.execute(sea_orm::Statement::from_string(
            sea_orm::DatabaseBackend::Postgres,
            "CREATE TABLE IF NOT EXISTS notifications (
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
            )".to_string(),
        )).await.unwrap();
    }

    /// Helper: insert a notification with a given status and return its id.
    async fn insert_notification(db: &DatabaseConnection, status: &str, channel: &str) -> i64 {
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

    // ── send notification tests ────────────────────────────────

    #[sqlx::test(migrations = false)]
    async fn test_send_creates_notification_with_correct_fields(pool: sqlx::PgPool) {
        let db = SqlxPostgresConnector::from_sqlx_postgres_pool(pool);
        setup_schema(&db).await;

        let repo = NotificationRepository::new(&db);

        let model = repo.send("sms", 42, "+919876543210", Some("Hello"), "Your OTP is 123456").await.unwrap();

        assert!(model.id > 0, "should return a valid id");
        assert_eq!(model.r#type, "direct", "type should be 'direct'");
        assert_eq!(model.channel, "sms", "channel should be 'sms'");
        assert_eq!(model.status, "queued", "status should be 'queued'");
        assert_eq!(model.title.as_deref(), Some("Hello"), "title should match input");
        assert_eq!(model.body.as_deref(), Some("Your OTP is 123456"), "body should match input");
    }

    #[sqlx::test(migrations = false)]
    async fn test_send_without_subject(pool: sqlx::PgPool) {
        let db = SqlxPostgresConnector::from_sqlx_postgres_pool(pool);
        setup_schema(&db).await;

        let repo = NotificationRepository::new(&db);

        let model = repo.send("push", 10, "token-abc", None, "You have a new message").await.unwrap();

        assert!(model.id > 0);
        assert_eq!(model.channel, "push");
        assert_eq!(model.status, "queued");
        assert!(model.title.is_none(), "title should be None when not provided");
        assert_eq!(model.body.as_deref(), Some("You have a new message"));
    }

    #[sqlx::test(migrations = false)]
    async fn test_send_different_channels(pool: sqlx::PgPool) {
        let db = SqlxPostgresConnector::from_sqlx_postgres_pool(pool);
        setup_schema(&db).await;

        let repo = NotificationRepository::new(&db);

        let channels = ["sms", "email", "push", "whatsapp"];
        for channel in &channels {
            let model = repo.send(channel, 1, "addr", Some("Test"), "Body").await.unwrap();
            assert_eq!(model.channel, *channel, "channel should match '{}'", channel);
            assert_eq!(model.status, "queued", "all new notifications should be queued");
            assert_eq!(model.r#type, "direct", "type should always be 'direct'");
        }

        // Verify all 4 notifications exist
        let (notifications, total) = repo.list_notifications(None, None, 1, 100).await.unwrap();
        assert_eq!(total, 4, "should have 4 notifications");
        let channels_in_db: Vec<&str> = notifications.iter().map(|n| n.channel.as_str()).collect();
        for ch in &channels {
            assert!(channels_in_db.contains(ch), "notification for '{}' should exist in DB", ch);
        }
    }

    #[sqlx::test(migrations = false)]
    async fn test_send_persists_in_database(pool: sqlx::PgPool) {
        let db = SqlxPostgresConnector::from_sqlx_postgres_pool(pool);
        setup_schema(&db).await;

        let repo = NotificationRepository::new(&db);

        let model = repo.send("email", 7, "user@example.com", Some("Invoice Ready"), "Your invoice is ready for review.").await.unwrap();

        // Verify it was persisted by querying it back
        let fetched = NotificationEntity::find_by_id(model.id).one(&db).await.unwrap();
        assert!(fetched.is_some(), "notification should exist in DB after send");
        let fetched = fetched.unwrap();
        assert_eq!(fetched.channel, "email");
        assert_eq!(fetched.status, "queued");
        assert_eq!(fetched.title.as_deref(), Some("Invoice Ready"));
        assert_eq!(fetched.body.as_deref(), Some("Your invoice is ready for review."));
    }

    #[sqlx::test(migrations = false)]
    async fn test_send_long_body(pool: sqlx::PgPool) {
        let db = SqlxPostgresConnector::from_sqlx_postgres_pool(pool);
        setup_schema(&db).await;

        let repo = NotificationRepository::new(&db);

        let long_body = "A".repeat(10000);
        let model = repo.send("sms", 1, "+919876543210", None, &long_body).await.unwrap();

        assert!(model.id > 0);
        assert_eq!(model.body.as_deref(), Some(long_body.as_str()), "long body should be stored correctly");
    }

    #[sqlx::test(migrations = false)]
    async fn test_send_multiple_notifications(pool: sqlx::PgPool) {
        let db = SqlxPostgresConnector::from_sqlx_postgres_pool(pool);
        setup_schema(&db).await;

        let repo = NotificationRepository::new(&db);

        // Send multiple notifications to the same channel
        let model1 = repo.send("email", 1, "a@test.com", Some("Subject 1"), "Body 1").await.unwrap();
        let model2 = repo.send("email", 2, "b@test.com", Some("Subject 2"), "Body 2").await.unwrap();

        assert_ne!(model1.id, model2.id, "each notification should get a unique id");

        // Both should be queued
        let (queued, total) = repo.list_notifications(None, Some("queued"), 1, 100).await.unwrap();
        assert_eq!(total, 2);
        assert!(queued.iter().all(|n| n.status == "queued"));
    }

    // ── retry_notification tests ───────────────────────────────

    #[sqlx::test(migrations = false)]
    async fn test_retry_notification_success(pool: sqlx::PgPool) {
        let db = SqlxPostgresConnector::from_sqlx_postgres_pool(pool);
        setup_schema(&db).await;

        let repo = NotificationRepository::new(&db);
        let id = insert_notification(&db, "failed", "sms").await;

        let result = repo.retry_notification(id).await.unwrap();
        assert!(result, "retry_notification should return true for a failed notification");

        let updated = NotificationEntity::find_by_id(id).one(&db).await.unwrap().unwrap();
        assert_eq!(updated.status, "queued", "status should be 'queued' after retry");
    }

    #[sqlx::test(migrations = false)]
    async fn test_retry_notification_not_failed(pool: sqlx::PgPool) {
        let db = SqlxPostgresConnector::from_sqlx_postgres_pool(pool);
        setup_schema(&db).await;

        let repo = NotificationRepository::new(&db);

        let id_queued = insert_notification(&db, "queued", "email").await;
        let result = repo.retry_notification(id_queued).await.unwrap();
        assert!(!result, "retry_notification should return false for a queued notification");

        let id_sent = insert_notification(&db, "sent", "sms").await;
        let result = repo.retry_notification(id_sent).await.unwrap();
        assert!(!result, "retry_notification should return false for a sent notification");

        let id_delivered = insert_notification(&db, "delivered", "push").await;
        let result = repo.retry_notification(id_delivered).await.unwrap();
        assert!(!result, "retry_notification should return false for a delivered notification");
    }

    #[sqlx::test(migrations = false)]
    async fn test_retry_notification_not_found(pool: sqlx::PgPool) {
        let db = SqlxPostgresConnector::from_sqlx_postgres_pool(pool);
        setup_schema(&db).await;

        let repo = NotificationRepository::new(&db);

        let result = repo.retry_notification(999999).await.unwrap();
        assert!(!result, "retry_notification should return false for a non-existent notification");
    }

    // ── list_history tests ─────────────────────────────────────

    #[sqlx::test(migrations = false)]
    async fn test_list_history_all_events(pool: sqlx::PgPool) {
        let db = SqlxPostgresConnector::from_sqlx_postgres_pool(pool);
        setup_schema(&db).await;

        let repo = NotificationRepository::new(&db);

        insert_notification(&db, "queued", "sms").await;
        insert_notification(&db, "sent", "email").await;
        insert_notification(&db, "delivered", "push").await;
        insert_notification(&db, "failed", "sms").await;

        let (history, total) = repo.list_history(None, 1, 100).await.unwrap();

        // queued: 1 event, sent: 2, delivered: 2, failed: 2 = 7 total events
        assert_eq!(total, 7, "should have 7 total history events");
        assert!(history.len() <= 7, "should not return more events than total");

        for event in &history {
            assert!(event.notification_id > 0, "notification_id should be positive");
            assert!(!event.event.is_empty(), "event should not be empty");
            assert!(event.recorded_at.timestamp() > 0, "recorded_at should be valid");
        }
    }

    #[sqlx::test(migrations = false)]
    async fn test_list_history_by_notification_id(pool: sqlx::PgPool) {
        let db = SqlxPostgresConnector::from_sqlx_postgres_pool(pool);
        setup_schema(&db).await;

        let repo = NotificationRepository::new(&db);

        let id = insert_notification(&db, "sent", "email").await;

        let (history, total) = repo.list_history(Some(id), 1, 100).await.unwrap();

        assert_eq!(total, 2, "sent notification should have 2 history events (created + status_changed)");
        assert_eq!(history.len(), 2);

        for event in &history {
            assert_eq!(event.notification_id, id);
        }

        let event_types: Vec<&str> = history.iter().map(|e| e.event.as_str()).collect();
        assert!(event_types.contains(&"created"), "should have 'created' event");
        assert!(event_types.contains(&"status_changed"), "should have 'status_changed' event");

        let status_event = history.iter().find(|e| e.event == "status_changed").unwrap();
        let details = status_event.details.as_ref().unwrap();
        assert_eq!(details["status"].as_str().unwrap(), "sent");
    }

    #[sqlx::test(migrations = false)]
    async fn test_list_history_nonexistent_notification(pool: sqlx::PgPool) {
        let db = SqlxPostgresConnector::from_sqlx_postgres_pool(pool);
        setup_schema(&db).await;

        let repo = NotificationRepository::new(&db);

        let (history, total) = repo.list_history(Some(999999), 1, 100).await.unwrap();

        assert_eq!(total, 0, "non-existent notification should have 0 history events");
        assert!(history.is_empty(), "should return empty history for non-existent notification");
    }

    #[sqlx::test(migrations = false)]
    async fn test_list_history_pagination(pool: sqlx::PgPool) {
        let db = SqlxPostgresConnector::from_sqlx_postgres_pool(pool);
        setup_schema(&db).await;

        let repo = NotificationRepository::new(&db);

        for i in 0..10 {
            let status = if i % 2 == 0 { "sent" } else { "queued" };
            insert_notification(&db, status, "sms").await;
        }

        let (page1, total) = repo.list_history(None, 1, 5).await.unwrap();
        assert_eq!(total, 15, "should have 15 total events (5 queued: 5 created, 5 sent: 10 events)");
        assert!(!page1.is_empty(), "page 1 should have at least 1 event");
        assert!(page1.len() <= 5, "page 1 should have at most 5 events");

        let (page2, _) = repo.list_history(None, 2, 5).await.unwrap();

        let page1_ids: Vec<i64> = page1.iter().map(|e| e.id).collect();
        let page2_ids: Vec<i64> = page2.iter().map(|e| e.id).collect();
        for id in &page2_ids {
            assert!(!page1_ids.contains(id), "page 2 should not contain events from page 1");
        }
    }

    #[sqlx::test(migrations = false)]
    async fn test_list_history_failed_notification_details(pool: sqlx::PgPool) {
        let db = SqlxPostgresConnector::from_sqlx_postgres_pool(pool);
        setup_schema(&db).await;

        let repo = NotificationRepository::new(&db);

        let id = insert_notification(&db, "failed", "sms").await;

        let (history, total) = repo.list_history(Some(id), 1, 100).await.unwrap();

        assert_eq!(total, 2, "failed notification should have 2 history events");

        let status_event = history.iter().find(|e| e.event == "status_changed").unwrap();
        let details = status_event.details.as_ref().unwrap();
        assert_eq!(details["status"].as_str().unwrap(), "failed");

        let created_event = history.iter().find(|e| e.event == "created").unwrap();
        assert!(created_event.details.is_none(), "created event should have null details");
    }

    // ── Integration test: retry then check history ──────────────

    #[sqlx::test(migrations = false)]
    async fn test_retry_then_list_history(pool: sqlx::PgPool) {
        let db = SqlxPostgresConnector::from_sqlx_postgres_pool(pool);
        setup_schema(&db).await;

        let repo = NotificationRepository::new(&db);

        let id = insert_notification(&db, "failed", "email").await;

        let (history_before, total_before) = repo.list_history(Some(id), 1, 100).await.unwrap();
        assert_eq!(total_before, 2);
        let status_before = history_before.iter().find(|e| e.event == "status_changed").unwrap();
        assert_eq!(status_before.details.as_ref().unwrap()["status"].as_str().unwrap(), "failed");

        let retry_result = repo.retry_notification(id).await.unwrap();
        assert!(retry_result, "retry should succeed");

        let updated = NotificationEntity::find_by_id(id).one(&db).await.unwrap().unwrap();
        assert_eq!(updated.status, "queued");

        let (history_after, _) = repo.list_history(Some(id), 1, 100).await.unwrap();
        let status_after = history_after.iter().find(|e| e.event == "status_changed").unwrap();
        assert_eq!(status_after.details.as_ref().unwrap()["status"].as_str().unwrap(), "queued",
            "after retry, history should show status_changed to 'queued'");
    }
}
