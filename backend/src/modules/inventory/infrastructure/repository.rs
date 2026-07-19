use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, QuerySelect, Set,
};

use crate::modules::inventory::domain::entities::{inventory_item, inventory_movement};
use crate::shared::errors::AppError;

pub struct InventoryRepository<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> InventoryRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn find_by_id(&self, id: i64) -> Result<Option<inventory_item::Model>, AppError> {
        Ok(inventory_item::Entity::find_by_id(id).one(self.db).await?)
    }

    pub async fn list_items(
        &self,
        branch_id: Option<i64>,
        status: Option<&str>,
        item_type: Option<&str>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<inventory_item::Model>, AppError> {
        let mut query = inventory_item::Entity::find();
        if let Some(bid) = branch_id {
            query = query.filter(inventory_item::Column::BranchId.eq(bid));
        }
        if let Some(s) = status {
            query = query.filter(inventory_item::Column::Status.eq(s));
        }
        if let Some(t) = item_type {
            query = query.filter(inventory_item::Column::ItemType.eq(t));
        }
        Ok(query
            .order_by_desc(inventory_item::Column::CreatedAt)
            .limit(limit as u64)
            .offset(offset as u64)
            .all(self.db)
            .await?)
    }

    pub async fn count_items(&self, branch_id: Option<i64>) -> Result<i64, AppError> {
        let mut query = inventory_item::Entity::find();
        if let Some(bid) = branch_id {
            query = query.filter(inventory_item::Column::BranchId.eq(bid));
        }
        Ok(query.count(self.db).await? as i64)
    }

    pub async fn create_item(
        &self,
        branch_id: i64,
        item_type: String,
        device_model_id: Option<i64>,
        serial_number: Option<String>,
        barcode: Option<String>,
        purchase_date: Option<chrono::NaiveDate>,
        purchase_price: Option<sea_orm::prelude::Decimal>,
        warranty_expiry: Option<chrono::NaiveDate>,
        supplier: Option<String>,
        notes: Option<String>,
    ) -> Result<inventory_item::Model, AppError> {
        let now = chrono::Utc::now();
        let model = inventory_item::ActiveModel {
            branch_id: Set(branch_id),
            item_type: Set(item_type),
            device_model_id: Set(device_model_id),
            serial_number: Set(serial_number),
            barcode: Set(barcode),
            purchase_date: Set(purchase_date),
            purchase_price: Set(purchase_price),
            warranty_expiry: Set(warranty_expiry),
            supplier: Set(supplier),
            status: Set("in_stock".to_string()),
            notes: Set(notes),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };
        Ok(model.insert(self.db).await?)
    }

    pub async fn assign_item(
        &self,
        model: inventory_item::Model,
        assigned_to: i64,
        assigned_to_branch_id: Option<i64>,
    ) -> Result<inventory_item::Model, AppError> {
        let mut active: inventory_item::ActiveModel = model.into();
        active.status = Set("assigned".to_string());
        active.assigned_to = Set(Some(assigned_to));
        active.assigned_to_branch_id = Set(assigned_to_branch_id);
        active.updated_at = Set(chrono::Utc::now());
        Ok(active.update(self.db).await?)
    }

    pub async fn update_status(
        &self,
        model: inventory_item::Model,
        status: String,
    ) -> Result<inventory_item::Model, AppError> {
        let mut active: inventory_item::ActiveModel = model.into();
        active.status = Set(status);
        active.updated_at = Set(chrono::Utc::now());
        Ok(active.update(self.db).await?)
    }

    // ── Movements ─────────────────────────────────────────────────────

    pub async fn record_movement(
        &self,
        item_id: i64,
        movement_type: String,
        from_branch_id: Option<i64>,
        to_branch_id: Option<i64>,
        reference_type: Option<String>,
        reference_id: Option<i64>,
        performed_by: i64,
        notes: Option<String>,
    ) -> Result<inventory_movement::Model, AppError> {
        let model = inventory_movement::ActiveModel {
            item_id: Set(item_id),
            movement_type: Set(movement_type),
            from_branch_id: Set(from_branch_id),
            to_branch_id: Set(to_branch_id),
            reference_type: Set(reference_type),
            reference_id: Set(reference_id),
            performed_by: Set(performed_by),
            notes: Set(notes),
            created_at: Set(chrono::Utc::now()),
            ..Default::default()
        };
        Ok(model.insert(self.db).await?)
    }

    pub async fn list_movements(
        &self,
        item_id: i64,
    ) -> Result<Vec<inventory_movement::Model>, AppError> {
        Ok(inventory_movement::Entity::find()
            .filter(inventory_movement::Column::ItemId.eq(item_id))
            .order_by_desc(inventory_movement::Column::CreatedAt)
            .all(self.db)
            .await?)
    }
}
