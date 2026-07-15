use crate::modules::device::domain::entities::NetworkDevice;
use crate::shared::errors::AppError;
use sea_orm::{DatabaseConnection, EntityTrait};

pub struct DeviceRepository<'a> {
    db: &'a DatabaseConnection,
}
impl<'a> DeviceRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }
    pub async fn find_by_id(
        &self,
        id: i64,
    ) -> Result<Option<crate::modules::device::domain::entities::network_device::Model>, AppError>
    {
        Ok(NetworkDevice::find_by_id(id).one(self.db).await?)
    }
}
