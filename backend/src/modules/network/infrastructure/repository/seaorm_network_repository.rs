//! SeaORM implementation of the network repository.

use async_trait::async_trait;
use sea_orm::DatabaseConnection;

use crate::common::errors::app_error::AppError;
use crate::modules::network::domain::aggregates::vlan::vlan::Vlan;
use crate::modules::network::infrastructure::repository::network_repository_trait::NetworkRepositoryTrait;

/// SeaORM implementation of the network repository.
#[allow(dead_code)]
pub struct SeaOrmNetworkRepository {
    db: DatabaseConnection,
}

impl SeaOrmNetworkRepository {
    pub fn new(db: &DatabaseConnection) -> Self {
        Self { db: db.clone() }
    }
}

#[async_trait]
impl NetworkRepositoryTrait for SeaOrmNetworkRepository {
    async fn find_vlan_by_id(&self, _id: i64) -> Result<Option<Vlan>, AppError> {
        // TODO: Implement with actual SeaORM entity
        Ok(None)
    }

    async fn save_vlan(&self, _vlan: &mut Vlan) -> Result<(), AppError> {
        // TODO: Implement with actual SeaORM entity
        Ok(())
    }

    async fn update_vlan(&self, _vlan: &Vlan) -> Result<(), AppError> {
        // TODO: Implement with actual SeaORM entity
        Ok(())
    }

    async fn list_vlans_by_branch(&self, _branch_id: i64) -> Result<Vec<Vlan>, AppError> {
        // TODO: Implement with actual SeaORM entity
        Ok(vec![])
    }
}
