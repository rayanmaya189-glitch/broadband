use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = include_str!("../../migrations/sql/009_create_devices.sql");
        crate::migration::exec_sql_file(manager, sql).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        crate::migration::drop_tables(manager, vec![
            "inventory_movements",
            "inventory_items",
            "subnet_location_map",
            "discovery_scan_history",
            "discovery_results",
            "discovery_scans",
            "device_metrics_2026_07",
            "device_metrics",
            "device_logs_2026_07",
            "device_logs",
            "device_ports",
            "network_devices_history",
            "network_devices",
            "device_models",
        ]).await
    }
}
