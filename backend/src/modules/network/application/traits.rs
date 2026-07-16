use async_trait::async_trait;
use sea_orm::{DatabaseConnection};
use crate::shared::errors::AppError;

pub type VlanModel = crate::modules::network::domain::entities::vlan::Model;
pub type IpPoolModel = crate::modules::network::domain::entities::ip_pool::Model;
pub type PppoeSessionModel = crate::modules::network::domain::entities::pppoe_session::Model;
pub type MacBindingModel = crate::modules::network::domain::entities::mac_binding::Model;

#[async_trait]
pub trait NetworkServiceTrait: Send + Sync {
    async fn list_vlans(
        &self,
        db: &DatabaseConnection,
    ) -> Result<Vec<VlanModel>, AppError>;

    async fn create_vlan(
        &self,
        db: &DatabaseConnection,
        name: String,
        vlan_id: i32,
        branch_id: i64,
    ) -> Result<VlanModel, AppError>;

    async fn delete_vlan(
        &self,
        db: &DatabaseConnection,
        id: i64,
    ) -> Result<(), AppError>;

    async fn list_ip_pools(
        &self,
        db: &DatabaseConnection,
    ) -> Result<Vec<IpPoolModel>, AppError>;

    async fn create_ip_pool(
        &self,
        db: &DatabaseConnection,
        name: String,
        network_cidr: String,
        branch_id: i64,
    ) -> Result<IpPoolModel, AppError>;

    async fn list_pppoe_sessions(
        &self,
        db: &DatabaseConnection,
    ) -> Result<Vec<PppoeSessionModel>, AppError>;

    async fn create_pppoe_session(
        &self,
        db: &DatabaseConnection,
        username: String,
        nas_ip: String,
        framed_ip: Option<String>,
    ) -> Result<PppoeSessionModel, AppError>;

    async fn terminate_pppoe_session(
        &self,
        db: &DatabaseConnection,
        id: i64,
    ) -> Result<(), AppError>;

    async fn list_mac_bindings(
        &self,
        db: &DatabaseConnection,
    ) -> Result<Vec<MacBindingModel>, AppError>;

    async fn create_mac_binding(
        &self,
        db: &DatabaseConnection,
        mac_address: String,
        ip_address: String,
        port_id: i64,
    ) -> Result<MacBindingModel, AppError>;
}
