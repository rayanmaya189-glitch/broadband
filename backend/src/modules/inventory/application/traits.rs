use async_trait::async_trait;
use sea_orm::{DatabaseConnection};
use crate::shared::errors::AppError;

pub type InventoryItemModel = crate::modules::inventory::domain::entities::inventory_item::Model;
pub type InventoryMovementModel = crate::modules::inventory::domain::entities::inventory_movement::Model;

#[async_trait]
pub trait InventoryServiceTrait: Send + Sync {
    async fn list_items(
        &self,
        db: &DatabaseConnection,
        branch_id: Option<i64>,
    ) -> Result<Vec<InventoryItemModel>, AppError>;

    async fn create_item(
        &self,
        db: &DatabaseConnection,
        branch_id: i64,
        item_type: String,
        serial_number: Option<String>,
    ) -> Result<InventoryItemModel, AppError>;

    async fn assign_item(
        &self,
        db: &DatabaseConnection,
        item_id: i64,
        assigned_to: i64,
        assignment_type: String,
    ) -> Result<InventoryMovementModel, AppError>;

    async fn list_movements(
        &self,
        db: &DatabaseConnection,
        item_id: Option<i64>,
    ) -> Result<Vec<InventoryMovementModel>, AppError>;
}
