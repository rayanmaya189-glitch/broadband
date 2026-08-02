use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = include_str!("../../migrations/sql/020_create_missing_tables.sql");
        crate::migration::exec_sql_file(manager, sql).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let tables = vec![
            "notification.notification_delivery_history",
            "bandwidth.bandwidth_policies",
            "workflow.workflow_steps",
            "workflow.workflow_instances",
            "scheduler.job_executions",
            "scheduler.job_definitions",
            "referral.wallet_transactions",
            "referral.referral_tracking",
            "referral.referral_programs",
            "referral.customer_wallets",
            "payment.payment_links",
            "payment.webhook_logs",
            "payment.gateway_configs",
            "monitoring.metric_records",
            "monitoring.monitoring_alerts",
            "monitoring.alert_rules",
            "installation.installation_photos",
            "installation.installation_equipment",
            "gateway.request_logs",
            "gateway.rate_limit_rules",
            "gateway.api_keys",
            "compliance.consents",
            "compliance.data_retention_policies",
            "compliance.kyc_verifications",
        ];
        crate::migration::drop_tables(manager, tables).await
    }
}
