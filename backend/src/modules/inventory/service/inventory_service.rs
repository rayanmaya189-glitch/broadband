//! SeaORM-based service for the Inventory domain.

use sea_orm::DatabaseConnection;

use crate::common::errors::app_error::AppError;
use crate::modules::inventory::repository::inventory_repository::InventoryRepository;
use crate::modules::inventory::request::inventory_request::*;
use crate::modules::inventory::response::inventory_response::*;

pub struct InventoryService<'a> {
    repo: InventoryRepository<'a>,
}

impl<'a> InventoryService<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { repo: InventoryRepository::new(db) }
    }

    fn to_response(i: crate::modules::inventory::model::inventory_item_entity::Model) -> InventoryItemResponse {
        InventoryItemResponse {
            id: i.id, branch_id: i.branch_id, item_type: i.item_type, device_model_id: i.device_model_id,
            serial_number: i.serial_number, barcode: i.barcode, purchase_date: i.purchase_date,
            purchase_price: i.purchase_price, warranty_expiry: i.warranty_expiry, supplier: i.supplier,
            status: i.status, assigned_to: i.assigned_to, notes: i.notes,
            created_at: i.created_at.into(), updated_at: i.updated_at.into(),
        }
    }

    pub async fn list(&self, branch_id: Option<i64>, status: Option<&str>, item_type: Option<&str>, page: i64, per_page: i64) -> Result<(Vec<InventoryItemResponse>, i64), AppError> {
        let (items, total) = self.repo.list(branch_id, status, item_type, page, per_page).await?;
        Ok((items.into_iter().map(Self::to_response).collect(), total))
    }

    pub async fn get_by_id(&self, id: i64) -> Result<InventoryItemResponse, AppError> {
        let i = self.repo.get_by_id(id).await?.ok_or_else(|| AppError::NotFound("Inventory item not found".into()))?;
        Ok(Self::to_response(i))
    }

    pub async fn create(&self, req: CreateInventoryItemRequest) -> Result<InventoryItemResponse, AppError> {
        let i = self.repo.create(req.branch_id, &req.item_type, req.serial_number.as_deref(), req.barcode.as_deref(), req.purchase_price, req.supplier.as_deref(), req.notes.as_deref()).await?;
        Ok(Self::to_response(i))
    }

    pub async fn update_status(&self, id: i64, status: &str) -> Result<InventoryItemResponse, AppError> {
        let i = self.repo.update_status(id, status).await?;
        Ok(Self::to_response(i))
    }

    pub async fn assign(&self, id: i64, user_id: i64) -> Result<InventoryItemResponse, AppError> {
        let i = self.repo.assign(id, user_id).await?;
        Ok(Self::to_response(i))
    }

    pub async fn install(&self, id: i64) -> Result<InventoryItemResponse, AppError> {
        let i = self.repo.install(id).await?;
        Ok(Self::to_response(i))
    }

    pub async fn return_item(&self, id: i64) -> Result<InventoryItemResponse, AppError> {
        let i = self.repo.return_item(id).await?;
        Ok(Self::to_response(i))
    }

    pub async fn transfer(&self, id: i64, to_branch_id: i64) -> Result<InventoryItemResponse, AppError> {
        let i = self.repo.transfer(id, to_branch_id).await?;
        Ok(Self::to_response(i))
    }

    pub async fn scrap(&self, id: i64) -> Result<InventoryItemResponse, AppError> {
        let i = self.repo.scrap(id).await?;
        Ok(Self::to_response(i))
    }

    pub async fn delete(&self, id: i64) -> Result<MessageResponse, AppError> {
        self.repo.update_status(id, "scrapped").await?;
        Ok(MessageResponse { message: "Inventory item deleted".into() })
    }

    pub async fn list_movements(&self, item_id: i64) -> Result<Vec<InventoryMovementResponse>, AppError> {
        let movements = self.repo.list_movements(item_id).await?;
        Ok(movements.into_iter().map(|m| InventoryMovementResponse {
            id: m.id, item_id: m.item_id, movement_type: m.movement_type,
            from_branch_id: m.from_branch_id, to_branch_id: m.to_branch_id,
            reference_type: m.reference_type, reference_id: m.reference_id,
            performed_by: m.performed_by, notes: m.notes,
            created_at: m.created_at.into(),
        }).collect())
    }

    pub async fn get_report(&self) -> Result<InventoryReportResponse, AppError> {
        let (items, total) = self.repo.list(None, None, None, 1, 10000).await?;
        let in_stock = items.iter().filter(|i| i.status == "in_stock").count() as i64;
        let assigned = items.iter().filter(|i| i.status == "assigned").count() as i64;
        let installed = items.iter().filter(|i| i.status == "installed").count() as i64;
        let damaged = items.iter().filter(|i| i.status == "damaged").count() as i64;
        let scrapped = items.iter().filter(|i| i.status == "scrapped").count() as i64;
        let total_value = items.iter().filter_map(|i| i.purchase_price).reduce(|a, b| a + b);
        Ok(InventoryReportResponse { total_items: total, in_stock, assigned, installed, damaged, scrapped, total_value })
    }

    pub async fn get_warranty_alerts(&self) -> Result<Vec<WarrantyAlertResponse>, AppError> {
        let (items, _) = self.repo.list(None, None, None, 1, 10000).await?;
        let now = chrono::Utc::now().date_naive();
        Ok(items.into_iter().filter(|i| {
            i.warranty_expiry.map(|exp| exp >= now && (exp - now).num_days() <= 30).unwrap_or(false)
        }).map(|i| WarrantyAlertResponse {
            id: i.id, item_type: i.item_type, serial_number: i.serial_number,
            warranty_expiry: i.warranty_expiry.unwrap(),
            branch_id: i.branch_id,
            days_until_expiry: (i.warranty_expiry.unwrap() - now).num_days(),
        }).collect())
    }
}
