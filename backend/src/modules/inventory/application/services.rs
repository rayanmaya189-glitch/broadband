use crate::modules::inventory::domain::entities::{
    InventoryItem, InventoryItemActiveModel, InventoryItemColumn, InventoryMovement,
};
use crate::shared::errors::AppError;
use sea_orm::{PaginatorTrait, ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};

pub struct InventoryService;

impl InventoryService {
    pub async fn list_items(
        db: &DatabaseConnection,
        branch_id: Option<i64>,
        _page: u64,
        _limit: u64,
    ) -> Result<(Vec<crate::modules::inventory::domain::entities::inventory_item::Model>, u64), AppError>
    {
        let mut query = InventoryItem::find();
        if let Some(bid) = branch_id {
            query = query.filter(InventoryItemColumn::BranchId.eq(bid));
        }
        let t = query.clone().count(db).await?;
        Ok((query.all(db).await?, t))
    }

    pub async fn create_item(
        db: &DatabaseConnection,
        branch_id: i64,
        item_type: String,
        serial_number: Option<String>,
        barcode: Option<String>,
    ) -> Result<crate::modules::inventory::domain::entities::inventory_item::Model, AppError> {
        let now = chrono::Utc::now();
        let item = InventoryItemActiveModel {
            branch_id: Set(branch_id),
            item_type: Set(item_type),
            serial_number: Set(serial_number),
            barcode: Set(barcode),
            status: Set("in_stock".to_string()),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };
        Ok(item.insert(db).await?)
    }

    pub async fn assign_item(
        db: &DatabaseConnection,
        id: i64,
        assigned_to: i64,
    ) -> Result<crate::modules::inventory::domain::entities::inventory_item::Model, AppError> {
        let item = InventoryItem::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Item {} not found", id)))?;
        let mut active = <crate::modules::inventory::domain::entities::inventory_item::Entity as sea_orm::EntityTrait>::ActiveModel::from(item);
        active.status = Set("assigned".to_string());
        active.assigned_to = Set(Some(assigned_to));
        active.updated_at = Set(chrono::Utc::now());
        Ok(active.update(db).await?)
    }

    pub async fn list_movements(
        db: &DatabaseConnection,
        item_id: i64,
    ) -> Result<Vec<crate::modules::inventory::domain::entities::inventory_movement::Model>, AppError>
    {
        Ok(InventoryMovement::find()
            .filter(
                crate::modules::inventory::domain::entities::inventory_movement::Column::ItemId
                    .eq(item_id),
            )
            .all(db)
            .await?)
    }
}

