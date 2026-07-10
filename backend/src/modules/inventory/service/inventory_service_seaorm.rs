//! SeaORM-based service for the Inventory domain.

use sea_orm::DatabaseConnection;

use crate::common::errors::app_error::AppError;
use crate::modules::inventory::repository::inventory_repository_seaorm::InventoryRepositorySeaorm;
use crate::modules::inventory::request::inventory_request::*;
use crate::modules::inventory::response::inventory_response::*;

pub struct InventoryServiceSeaorm<'a> {
    repo: InventoryRepositorySeaorm<'a>,
}

impl<'a> InventoryServiceSeaorm<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { repo: InventoryRepositorySeaorm::new(db) }
    }

    pub async fn list(&self, branch_id: Option<i64>, status: Option<&str>, item_type: Option<&str>, page: i64, per_page: i64) -> Result<(Vec<InventoryItemResponse>, i64), AppError> {
        let (items, total) = self.repo.list(branch_id, status, item_type, page, per_page).await?;
        let responses = items.into_iter().map(|i| InventoryItemResponse {
            id: i.id, branch_id: i.branch_id, item_type: i.item_type, device_model_id: i.device_model_id,
            serial_number: i.serial_number, barcode: i.barcode, purchase_date: i.purchase_date,
            purchase_price: i.purchase_price, warranty_expiry: i.warranty_expiry, supplier: i.supplier,
            status: i.status, assigned_to: i.assigned_to, notes: i.notes,
            created_at: i.created_at.into(), updated_at: i.updated_at.into(),
        }).collect();
        Ok((responses, total))
    }

    pub async fn create(&self, req: CreateInventoryItemRequest) -> Result<InventoryItemResponse, AppError> {
        let i = self.repo.create(req.branch_id, &req.item_type, req.serial_number.as_deref(), req.barcode.as_deref(), req.purchase_price, req.supplier.as_deref(), req.notes.as_deref()).await?;
        Ok(InventoryItemResponse {
            id: i.id, branch_id: i.branch_id, item_type: i.item_type, device_model_id: i.device_model_id,
            serial_number: i.serial_number, barcode: i.barcode, purchase_date: i.purchase_date,
            purchase_price: i.purchase_price, warranty_expiry: i.warranty_expiry, supplier: i.supplier,
            status: i.status, assigned_to: i.assigned_to, notes: i.notes,
            created_at: i.created_at.into(), updated_at: i.updated_at.into(),
        })
    }

    pub async fn update_status(&self, id: i64, status: &str) -> Result<InventoryItemResponse, AppError> {
        let i = self.repo.update_status(id, status).await?;
        Ok(InventoryItemResponse {
            id: i.id, branch_id: i.branch_id, item_type: i.item_type, device_model_id: i.device_model_id,
            serial_number: i.serial_number, barcode: i.barcode, purchase_date: i.purchase_date,
            purchase_price: i.purchase_price, warranty_expiry: i.warranty_expiry, supplier: i.supplier,
            status: i.status, assigned_to: i.assigned_to, notes: i.notes,
            created_at: i.created_at.into(), updated_at: i.updated_at.into(),
        })
    }

    pub async fn assign(&self, id: i64, user_id: i64) -> Result<InventoryItemResponse, AppError> {
        let i = self.repo.assign(id, user_id).await?;
        Ok(InventoryItemResponse {
            id: i.id, branch_id: i.branch_id, item_type: i.item_type, device_model_id: i.device_model_id,
            serial_number: i.serial_number, barcode: i.barcode, purchase_date: i.purchase_date,
            purchase_price: i.purchase_price, warranty_expiry: i.warranty_expiry, supplier: i.supplier,
            status: i.status, assigned_to: i.assigned_to, notes: i.notes,
            created_at: i.created_at.into(), updated_at: i.updated_at.into(),
        })
    }

    pub async fn install(&self, id: i64) -> Result<InventoryItemResponse, AppError> {
        let i = self.repo.install(id).await?;
        Ok(InventoryItemResponse {
            id: i.id, branch_id: i.branch_id, item_type: i.item_type, device_model_id: i.device_model_id,
            serial_number: i.serial_number, barcode: i.barcode, purchase_date: i.purchase_date,
            purchase_price: i.purchase_price, warranty_expiry: i.warranty_expiry, supplier: i.supplier,
            status: i.status, assigned_to: i.assigned_to, notes: i.notes,
            created_at: i.created_at.into(), updated_at: i.updated_at.into(),
        })
    }

    pub async fn return_item(&self, id: i64) -> Result<InventoryItemResponse, AppError> {
        let i = self.repo.return_item(id).await?;
        Ok(InventoryItemResponse {
            id: i.id, branch_id: i.branch_id, item_type: i.item_type, device_model_id: i.device_model_id,
            serial_number: i.serial_number, barcode: i.barcode, purchase_date: i.purchase_date,
            purchase_price: i.purchase_price, warranty_expiry: i.warranty_expiry, supplier: i.supplier,
            status: i.status, assigned_to: i.assigned_to, notes: i.notes,
            created_at: i.created_at.into(), updated_at: i.updated_at.into(),
        })
    }

    pub async fn transfer(&self, id: i64, to_branch_id: i64) -> Result<InventoryItemResponse, AppError> {
        let i = self.repo.transfer(id, to_branch_id).await?;
        Ok(InventoryItemResponse {
            id: i.id, branch_id: i.branch_id, item_type: i.item_type, device_model_id: i.device_model_id,
            serial_number: i.serial_number, barcode: i.barcode, purchase_date: i.purchase_date,
            purchase_price: i.purchase_price, warranty_expiry: i.warranty_expiry, supplier: i.supplier,
            status: i.status, assigned_to: i.assigned_to, notes: i.notes,
            created_at: i.created_at.into(), updated_at: i.updated_at.into(),
        })
    }

    pub async fn scrap(&self, id: i64) -> Result<InventoryItemResponse, AppError> {
        let i = self.repo.scrap(id).await?;
        Ok(InventoryItemResponse {
            id: i.id, branch_id: i.branch_id, item_type: i.item_type, device_model_id: i.device_model_id,
            serial_number: i.serial_number, barcode: i.barcode, purchase_date: i.purchase_date,
            purchase_price: i.purchase_price, warranty_expiry: i.warranty_expiry, supplier: i.supplier,
            status: i.status, assigned_to: i.assigned_to, notes: i.notes,
            created_at: i.created_at.into(), updated_at: i.updated_at.into(),
        })
    }
}
