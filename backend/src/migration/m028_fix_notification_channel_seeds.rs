use sea_orm_migration::prelude::*;

/// Repair notification channel config seeded by m017 with literal `${ENV}`
/// placeholders. The migration runner never interpolates env vars, so the
/// stored JSON contained placeholder strings. SMS config mirrors the runtime
/// MSG91 adapter defaults (secrets live in env, not in the DB).
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared(
                "UPDATE notification.notification_channels
                    SET config = '{\"host\": \"smtp.gmail.com\", \"port\": 587, \"username\": \"\", \"from_email\": \"\"}'
                  WHERE channel = 'email' AND provider = 'smtp'",
            )
            .await?;
        manager
            .get_connection()
            .execute_unprepared(
                "UPDATE notification.notification_channels
                    SET config = '{\"auth_key\": \"\", \"sender_id\": \"AEROXE\", \"route\": \"4\", \"country_code\": \"91\"}'
                  WHERE channel = 'sms' AND provider = 'msg91'",
            )
            .await?;
        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        // No-op: repaired data is the desired state; nothing to restore.
        Ok(())
    }
}
