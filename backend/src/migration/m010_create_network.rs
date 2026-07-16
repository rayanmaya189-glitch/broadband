use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = include_str!("../../migrations/sql/010_create_network.sql");
        crate::migration::exec_sql_file(manager, sql).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        crate::migration::drop_tables(manager, vec![
            "customer_sessions_2026_07",
            "customer_sessions",
            "mac_bindings",
            "dhcp_leases",
            "pppoe_sessions_history",
            "pppoe_sessions",
            "ip_addresses",
            "ip_pools_history",
            "ip_pools",
            "vlans_history",
            "vlans",
        ]).await
    }
}
