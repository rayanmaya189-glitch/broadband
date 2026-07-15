use crate::modules::device::domain::entities::{
    DevicePort, NetworkDevice, NetworkDeviceActiveModel, NetworkDeviceColumn,
};
use crate::shared::errors::AppError;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};

pub struct DeviceService;

impl DeviceService {
    pub async fn list_devices(
        db: &DatabaseConnection,
        branch_id: Option<i64>,
    ) -> Result<Vec<crate::modules::device::domain::entities::network_device::Model>, AppError>
    {
        let mut query = NetworkDevice::find();
        if let Some(bid) = branch_id {
            query = query.filter(NetworkDeviceColumn::BranchId.eq(bid));
        }
        Ok(query.all(db).await?)
    }

    pub async fn get_device(
        db: &DatabaseConnection,
        id: i64,
    ) -> Result<crate::modules::device::domain::entities::network_device::Model, AppError> {
        NetworkDevice::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Device {} not found", id)))
    }

    pub async fn register_device(
        db: &DatabaseConnection,
        branch_id: i64,
        name: String,
        device_model_id: i64,
        serial_number: String,
        management_ip: String,
    ) -> Result<crate::modules::device::domain::entities::network_device::Model, AppError> {
        let now = chrono::Utc::now();
        let device = NetworkDeviceActiveModel {
            branch_id: Set(branch_id),
            name: Set(name),
            device_model_id: Set(device_model_id),
            serial_number: Set(serial_number),
            management_ip: Set(management_ip),
            status: Set("offline".to_string()),
            health_score: Set(Some(0)),
            review_status: Set(Some("pending".to_string())),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };
        Ok(device.insert(db).await?)
    }

    pub async fn update_device_status(
        db: &DatabaseConnection,
        id: i64,
        status: &str,
    ) -> Result<crate::modules::device::domain::entities::network_device::Model, AppError> {
        let device = Self::get_device(db, id).await?;
        let mut active = <crate::modules::device::domain::entities::network_device::Entity as sea_orm::EntityTrait>::ActiveModel::from(device);
        active.status = Set(status.to_string());
        active.updated_at = Set(chrono::Utc::now());
        Ok(active.update(db).await?)
    }

    pub async fn list_ports(
        db: &DatabaseConnection,
        device_id: i64,
    ) -> Result<Vec<crate::modules::device::domain::entities::device_port::Model>, AppError> {
        let ports = DevicePort::find()
            .filter(
                crate::modules::device::domain::entities::device_port::Column::DeviceId
                    .eq(device_id),
            )
            .all(db)
            .await?;
        Ok(ports)
    }
}
