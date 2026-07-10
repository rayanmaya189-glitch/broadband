//! SeaORM-based repository for the Inventory domain.

use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel,
    PaginatorTrait, QueryFilter, QueryOrder, Set,
};

use crate::common::errors::app_error::AppError;
use crate::modules::inventory::model::inventory_item_entity::{self, Model as InventoryItemModel};
use crate::modules::inventory::model::inventory_movement_entity::{self, Model as InventoryMovementModel};

pub struct InventoryRepositorySeaorm<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> InventoryRepositorySeaorm<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self { Self { db } }

    pub async fn list(&self, branch_id: Option<i64>, status: Option<&str>, item_type: Option<&str>, page: i64, per_page: i64) -> Result<(Vec<InventoryItemModel>, i64), AppError> {
        let page_size = per_page as u64;
        let page_num = if per_page > 0 { ((page - 1).max(0) as u64 * page_size) / page_size } else { 0 };
        let mut select = inventory_item_entity::Entity::find();
        if let Some(bid) = branch_id { select = select.filter(inventory_item_entity::Column::BranchId.eq(bid)); }
        if let Some(s) = status { select = select.filter(inventory_item_entity::Column::Status.eq(s)); }
        if let Some(it) = item_type { select = select.filter(inventory_item_entity::Column::ItemType.eq(it)); }
        let total = select.clone().count(self.db).await?;
        let items = select.order_by_desc(inventory_item_entity::Column::CreatedAt).paginate(self.db, page_size).fetch_page(page_num).await?;
        Ok((items, total as i64))
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<InventoryItemModel>, AppError> {
        Ok(inventory_item_entity::Entity::find_by_id(id).one(self.db).await?)
    }

    pub async fn create(&self, branch_id: i64, item_type: &str, serial: Option<&str>, barcode: Option<&str>, price: Option<rust_decimal::Decimal>, supplier: Option<&str>, notes: Option<&str>) -> Result<InventoryItemModel, AppError> {
        let now = chrono::Utc::now();
        let active = inventory_item_entity::ActiveModel {
            branch_id: Set(branch_id),
            item_type: Set(item_type.to_owned()),
            serial_number: Set(serial.map(|s| s.to_owned())),
            barcode: Set(barcode.map(|s| s.to_owned())),
            purchase_price: Set(price),
            supplier: Set(supplier.map(|s| s.to_owned())),
            notes: Set(notes.map(|s| s.to_owned())),
            status: Set("in_stock".to_owned()),
            created_at: Set(now.into()),
            updated_at: Set(now.into()),
            ..Default::default()
        };
        Ok(active.insert(self.db).await?)
    }

    pub async fn update_status(&self, id: i64, status: &str) -> Result<InventoryItemModel, AppError> {
        let existing = inventory_item_entity::Entity::find_by_id(id).one(self.db).await?
            .ok_or_else(|| AppError::NotFound("Item not found".into()))?;
        let mut active = existing.into_active_model();
        active.status = Set(status.to_owned());
        active.updated_at = Set(chrono::Utc::now().into());
        Ok(active.update(self.db).await?)
    }

    pub async fn assign(&self, id: i64, user_id: i64) -> Result<InventoryItemModel, AppError> {
        let existing = inventory_item_entity::Entity::find_by_id(id).one(self.db).await?
            .ok_or_else(|| AppError::NotFound("Item not found".into()))?;
        let mut active = existing.into_active_model();
        active.status = Set("assigned".to_owned());
        active.assigned_to = Set(Some(user_id));
        active.updated_at = Set(chrono::Utc::now().into());
        Ok(active.update(self.db).await?)
    }

    pub async fn install(&self, id: i64) -> Result<InventoryItemModel, AppError> {
        let existing = inventory_item_entity::Entity::find_by_id(id).one(self.db).await?
            .ok_or_else(|| AppError::NotFound("Item not found".into()))?;
        let mut active = existing.into_active_model();
        active.status = Set("installed".to_owned());
        active.updated_at = Set(chrono::Utc::now().into());
        Ok(active.update(self.db).await?)
    }

    pub async fn return_item(&self, id: i64) -> Result<InventoryItemModel, AppError> {
        let existing = inventory_item_entity::Entity::find_by_id(id).one(self.db).await?
            .ok_or_else(|| AppError::NotFound("Item not found".into()))?;
        let mut active = existing.into_active_model();
        active.status = Set("in_stock".to_owned());
        active.assigned_to = Set(None);
        active.updated_at = Set(chrono::Utc::now().into());
        Ok(active.update(self.db).await?)
    }

    pub async fn transfer(&self, id: i64, to_branch_id: i64) -> Result<InventoryItemModel, AppError> {
        let existing = inventory_item_entity::Entity::find_by_id(id).one(self.db).await?
            .ok_or_else(|| AppError::NotFound("Item not found".into()))?;
        let mut active = existing.into_active_model();
        active.branch_id = Set(to_branch_id);
        active.status = Set("in_stock".to_owned());
        active.assigned_to = Set(None);
        active.updated_at = Set(chrono::Utc::now().into());
        Ok(active.update(self.db).await?)
    }

    pub async fn scrap(&self, id: i64) -> Result<InventoryItemModel, AppError> {
        let existing = inventory_item_entity::Entity::find_by_id(id).one(self.db).await?
            .ok_or_else(|| AppError::NotFound("Item not found".into()))?;
        let mut active = existing.into_active_model();
        active.status = Set("scrapped".to_owned());
        active.updated_at = Set(chrono::Utc::now().into());
        Ok(active.update(self.db).await?)
    }

    pub async fn record_movement(&self, item_id: i64, movement_type: &str, from_branch_id: Option<i64>, to_branch_id: Option<i64>, reference_type: Option<&str>, reference_id: Option<i64>, performed_by: i64, notes: Option<&str>) -> Result<InventoryMovementModel, AppError> {
        let now = chrono::Utc::now();
        let active = inventory_movement_entity::ActiveModel {
            item_id: Set(item_id),
            movement_type: Set(movement_type.to_owned()),
            from_branch_id: Set(from_branch_id),
            to_branch_id: Set(to_branch_id),
            reference_type: Set(reference_type.map(|s| s.to_owned())),
            reference_id: Set(reference_id),
            performed_by: Set(performed_by),
            notes: Set(notes.map(|s| s.to_owned())),
            created_at: Set(now.into()),
            ..Default::default()
        };
        Ok(active.insert(self.db).await?)
    }

    pub async fn list_movements(&self, item_id: i64) -> Result<Vec<InventoryMovementModel>, AppError> {
        let movements = inventory_movement_entity::Entity::find()
            .filter(inventory_movement_entity::Column::ItemId.eq(item_id))
            .order_by_desc(inventory_movement_entity::Column::CreatedAt)
            .all(self.db).await?;
        Ok(movements)
    }
}
