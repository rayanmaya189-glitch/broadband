#[cfg(test)]
mod tests {
    use sea_orm::{DatabaseConnection, EntityTrait, Set, ActiveModelTrait};
    use crate::modules::notification::model::notification_entity;

    // Note: These tests require a running PostgreSQL database.
    // Run with: cargo test -- --nocapture

    async fn setup_test_db() -> DatabaseConnection {
        let database_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://postgres:postgres@localhost/aeroxe_test".to_string());
        
        sea_orm::Database::connect(&database_url)
            .await
            .expect("Failed to connect to test database")
    }

    #[tokio::test]
    async fn test_send_creates_notification_with_correct_fields() {
        let db = setup_test_db().await;
        
        let new_notification = notification_entity::ActiveModel {
            customer_id: Set(1),
            channel: Set("email".to_string()),
            subject: Set("Test Subject".to_string()),
            body: Set("Test body content".to_string()),
            status: Set("pending".to_string()),
            ..Default::default()
        };

        let result = new_notification.insert(&db).await;
        assert!(result.is_ok(), "Failed to create notification: {:?}", result.err());
        
        let notification = result.unwrap();
        assert_eq!(notification.customer_id, 1);
        assert_eq!(notification.channel, "email");
        assert_eq!(notification.subject, "Test Subject");
    }

    #[tokio::test]
    async fn test_send_without_subject() {
        let db = setup_test_db().await;
        
        let new_notification = notification_entity::ActiveModel {
            customer_id: Set(1),
            channel: Set("sms".to_string()),
            subject: Set(None),
            body: Set("SMS content".to_string()),
            status: Set("pending".to_string()),
            ..Default::default()
        };

        let result = new_notification.insert(&db).await;
        assert!(result.is_ok(), "Failed to create notification without subject: {:?}", result.err());
    }

    #[tokio::test]
    async fn test_send_different_channels() {
        let db = setup_test_db().await;
        
        let channels = vec!["email", "sms", "whatsapp", "telegram"];
        
        for channel in channels {
            let new_notification = notification_entity::ActiveModel {
                customer_id: Set(1),
                channel: Set(channel.to_string()),
                subject: Set(Some(format!("Test {}", channel))),
                body: Set(format!("{} content", channel)),
                status: Set("pending".to_string()),
                ..Default::default()
            };

            let result = new_notification.insert(&db).await;
            assert!(result.is_ok(), "Failed to create notification for channel {}: {:?}", channel, result.err());
        }
    }

    #[tokio::test]
    async fn test_send_persists_in_database() {
        let db = setup_test_db().await;
        
        let new_notification = notification_entity::ActiveModel {
            customer_id: Set(1),
            channel: Set("email".to_string()),
            subject: Set(Some("Persistence Test".to_string())),
            body: Set("This should persist".to_string()),
            status: Set("pending".to_string()),
            ..Default::default()
        };

        let created = new_notification.insert(&db).await.unwrap();
        
        // Verify it was persisted
        let found = notification_entity::Entity::find_by_id(created.id)
            .one(&db)
            .await
            .unwrap();
        
        assert!(found.is_some(), "Notification not found in database");
        let found = found.unwrap();
        assert_eq!(found.subject, Some("Persistence Test".to_string()));
    }

    #[tokio::test]
    async fn test_send_long_body() {
        let db = setup_test_db().await;
        
        let long_body = "x".repeat(10000);
        
        let new_notification = notification_entity::ActiveModel {
            customer_id: Set(1),
            channel: Set("email".to_string()),
            subject: Set(Some("Long Body Test".to_string())),
            body: Set(long_body),
            status: Set("pending".to_string()),
            ..Default::default()
        };

        let result = new_notification.insert(&db).await;
        assert!(result.is_ok(), "Failed to create notification with long body: {:?}", result.err());
    }

    #[tokio::test]
    async fn test_send_multiple_notifications() {
        let db = setup_test_db().await;
        
        let mut ids = Vec::new();
        
        for i in 0..5 {
            let new_notification = notification_entity::ActiveModel {
                customer_id: Set(1),
                channel: Set("email".to_string()),
                subject: Set(Some(format!("Notification {}", i))),
                body: Set(format!("Body {}", i)),
                status: Set("pending".to_string()),
                ..Default::default()
            };

            let created = new_notification.insert(&db).await.unwrap();
            ids.push(created.id);
        }
        
        assert_eq!(ids.len(), 5, "Should have created 5 notifications");
        
        // Verify all exist
        for id in &ids {
            let found = notification_entity::Entity::find_by_id(*id)
                .one(&db)
                .await
                .unwrap();
            assert!(found.is_some(), "Notification {} not found", id);
        }
    }

    #[tokio::test]
    async fn test_retry_notification_success() {
        let db = setup_test_db().await;
        
        // Create a failed notification
        let new_notification = notification_entity::ActiveModel {
            customer_id: Set(1),
            channel: Set("email".to_string()),
            subject: Set(Some("Retry Test".to_string())),
            body: Set("This will be retried".to_string()),
            status: Set("failed".to_string()),
            ..Default::default()
        };

        let created = new_notification.insert(&db).await.unwrap();
        
        // Update status to pending (simulate retry)
        let mut active_model: notification_entity::ActiveModel = created.into();
        active_model.status = Set("pending".to_string());
        let updated = active_model.update(&db).await.unwrap();
        
        assert_eq!(updated.status, "pending");
    }

    #[tokio::test]
    async fn test_notification_with_template() {
        let db = setup_test_db().await;
        
        let new_notification = notification_entity::ActiveModel {
            customer_id: Set(1),
            channel: Set("email".to_string()),
            subject: Set(Some("Template Test".to_string())),
            body: Set("Template content".to_string()),
            template_id: Set(Some(1)),
            status: Set("pending".to_string()),
            ..Default::default()
        };

        let result = new_notification.insert(&db).await;
        assert!(result.is_ok(), "Failed to create notification with template: {:?}", result.err());
        
        let notification = result.unwrap();
        assert_eq!(notification.template_id, Some(1));
    }
}
