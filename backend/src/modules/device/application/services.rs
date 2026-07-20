use crate::modules::device::domain::entities::{
    DeviceLog, DeviceMetric, DevicePort, DevicePortActiveModel, DevicePortColumn, NetworkDevice,
    NetworkDeviceActiveModel, NetworkDeviceColumn,
};
use crate::shared::errors::AppError;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, Set,
};

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

    pub async fn restart_device(
        db: &DatabaseConnection,
        id: i64,
    ) -> Result<crate::modules::device::domain::entities::network_device::Model, AppError> {
        let device = Self::get_device(db, id).await?;
        let mut active = <crate::modules::device::domain::entities::network_device::Entity as sea_orm::EntityTrait>::ActiveModel::from(device);
        active.status = Set("maintenance".to_string());
        active.updated_at = Set(chrono::Utc::now());
        Ok(active.update(db).await?)
    }

    pub async fn list_ports(
        db: &DatabaseConnection,
        device_id: i64,
    ) -> Result<Vec<crate::modules::device::domain::entities::device_port::Model>, AppError> {
        let ports = DevicePort::find()
            .filter(DevicePortColumn::DeviceId.eq(device_id))
            .all(db)
            .await?;
        Ok(ports)
    }

    pub async fn update_port_status(
        db: &DatabaseConnection,
        port_id: i64,
        status: &str,
    ) -> Result<crate::modules::device::domain::entities::device_port::Model, AppError> {
        let port = DevicePort::find_by_id(port_id)
            .one(db)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Port {} not found", port_id)))?;
        let mut active: DevicePortActiveModel = port.into();
        active.status = Set(status.to_string());
        Ok(active.update(db).await?)
    }

    pub async fn list_logs(
        db: &DatabaseConnection,
        device_id: i64,
        page: u64,
        limit: u64,
    ) -> Result<
        (
            Vec<crate::modules::device::domain::entities::device_log::Model>,
            u64,
        ),
        AppError,
    > {
        let query = DeviceLog::find()
            .filter(crate::modules::device::domain::entities::device_log::Column::DeviceId.eq(device_id))
            .order_by_desc(crate::modules::device::domain::entities::device_log::Column::CreatedAt);
        let total = query.clone().count(db).await?;
        let items = query.paginate(db, limit).fetch_page(page).await?;
        Ok((items, total))
    }

    pub async fn list_metrics(
        db: &DatabaseConnection,
        device_id: i64,
        page: u64,
        limit: u64,
    ) -> Result<
        (
            Vec<crate::modules::device::domain::entities::device_metric::Model>,
            u64,
        ),
        AppError,
    > {
        let query = DeviceMetric::find()
            .filter(crate::modules::device::domain::entities::device_metric::Column::DeviceId.eq(device_id))
            .order_by_desc(crate::modules::device::domain::entities::device_metric::Column::RecordedAt);
        let total = query.clone().count(db).await?;
        let items = query.paginate(db, limit).fetch_page(page).await?;
        Ok((items, total))
    }
}
