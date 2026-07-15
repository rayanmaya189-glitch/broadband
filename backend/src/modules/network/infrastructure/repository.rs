use sea_orm::{DatabaseConnection, EntityTrait};
use crate::shared::errors::AppError;
use crate::modules::network::domain::entities::{Vlan, IpPool};

pub struct NetworkRepository<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> NetworkRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self { Self { db } }

    pub async fn find_vlan_by_id(&self, id: i64) -> Result<Option<crate::modules::network::domain::entities::vlan::Model>, AppError> {
        Ok(Vlan::find_by_id(id).one(self.db).await?)
    }

    pub async fn find_ip_pool_by_id(&self, id: i64) -> Result<Option<crate::modules::network::domain::entities::ip_pool::Model>, AppError> {
        Ok(IpPool::find_by_id(id).one(self.db).await?)
    }
}
