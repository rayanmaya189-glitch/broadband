use crate::shared::errors::AppError;
use async_trait::async_trait;
use sea_orm::DatabaseConnection;

pub type NetworkDeviceModel = crate::modules::device::domain::entities::network_device::Model;
pub type DevicePortModel = crate::modules::device::domain::entities::device_port::Model;

#[async_trait]
pub trait DeviceServiceTrait: Send + Sync {
    async fn list_devices(
        &self,
        db: &DatabaseConnection,
        branch_id: Option<i64>,
    ) -> Result<Vec<NetworkDeviceModel>, AppError>;

    async fn get_device(
        &self,
        db: &DatabaseConnection,
        id: i64,
    ) -> Result<NetworkDeviceModel, AppError>;

    async fn register_device(
        &self,
        db: &DatabaseConnection,
        branch_id: i64,
        name: String,
        device_model_id: i64,
        serial_number: String,
        management_ip: String,
    ) -> Result<NetworkDeviceModel, AppError>;

    async fn update_device_status(
        &self,
        db: &DatabaseConnection,
        id: i64,
        status: &str,
    ) -> Result<NetworkDeviceModel, AppError>;

    async fn list_ports(
        &self,
        db: &DatabaseConnection,
        device_id: i64,
    ) -> Result<Vec<DevicePortModel>, AppError>;
}
