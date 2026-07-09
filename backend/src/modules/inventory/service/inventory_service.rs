use sqlx::PgPool;
use crate::common::errors::app_error::AppError;
use crate::modules::inventory::repository::inventory_repository::InventoryRepository;
use crate::modules::inventory::request::inventory_request::*;
use crate::modules::inventory::response::inventory_response::*;

pub struct InventoryService<'a> { repo: InventoryRepository<'a> }
impl<'a> InventoryService<'a> {
    pub fn new(pool: &'a PgPool) -> Self { Self { repo: InventoryRepository::new(pool) } }

    pub async fn list(&self, q: InventoryQuery) -> Result<InventoryListResponse, AppError> {
        let page = q.page.unwrap_or(1);
        let per_page = q.per_page.unwrap_or(20);
        let (items, total) = self.repo.list(q.branch_id, q.status.as_deref(), q.item_type.as_deref(), page, per_page).await?;
        Ok(InventoryListResponse {
            items: items.into_iter().map(|i| InventoryItemResponse { id: i.id, branch_id: i.branch_id, item_type: i.item_type, serial_number: i.serial_number, barcode: i.barcode, purchase_price: i.purchase_price, warranty_expiry: i.warranty_expiry, supplier: i.supplier, status: i.status, assigned_to: i.assigned_to, notes: i.notes, created_at: i.created_at, updated_at: i.updated_at }).collect(),
            total, page, per_page,
        })
    }

    pub async fn get(&self, id: i64) -> Result<InventoryItemResponse, AppError> {
        let i = self.repo.get_by_id(id).await?.ok_or_else(|| AppError::NotFound("Item not found".into()))?;
        Ok(InventoryItemResponse { id: i.id, branch_id: i.branch_id, item_type: i.item_type, serial_number: i.serial_number, barcode: i.barcode, purchase_price: i.purchase_price, warranty_expiry: i.warranty_expiry, supplier: i.supplier, status: i.status, assigned_to: i.assigned_to, notes: i.notes, created_at: i.created_at, updated_at: i.updated_at })
    }

    pub async fn create(&self, req: CreateInventoryItemRequest, user_id: i64) -> Result<InventoryItemResponse, AppError> {
        let i = self.repo.create(req.branch_id, &req.item_type, req.serial_number.as_deref(), req.barcode.as_deref(), req.purchase_price, req.supplier.as_deref(), req.notes.as_deref()).await?;
        self.repo.record_movement(i.id, "received", None, Some(i.branch_id), None, None, user_id, Some("Item received into inventory")).await.ok();
        Ok(InventoryItemResponse { id: i.id, branch_id: i.branch_id, item_type: i.item_type, serial_number: i.serial_number, barcode: i.barcode, purchase_price: i.purchase_price, warranty_expiry: i.warranty_expiry, supplier: i.supplier, status: i.status, assigned_to: i.assigned_to, notes: i.notes, created_at: i.created_at, updated_at: i.updated_at })
    }

    // ── Lifecycle Transitions ─────────────────────────────

    pub async fn assign(&self, id: i64, req: AssignInventoryRequest, performed_by: i64) -> Result<InventoryItemResponse, AppError> {
        let old = self.repo.get_by_id(id).await?.ok_or_else(|| AppError::NotFound("Item not found".into()))?;
        if old.status != "in_stock" { return Err(AppError::BadRequest("Item is not in stock".into())); }
        let i = self.repo.assign(id, req.user_id).await?;
        self.repo.record_movement(id, "assigned", Some(old.branch_id), None, Some("user"), Some(req.user_id), performed_by, req.notes.as_deref()).await.ok();
        Ok(InventoryItemResponse { id: i.id, branch_id: i.branch_id, item_type: i.item_type, serial_number: i.serial_number, barcode: i.barcode, purchase_price: i.purchase_price, warranty_expiry: i.warranty_expiry, supplier: i.supplier, status: i.status, assigned_to: i.assigned_to, notes: i.notes, created_at: i.created_at, updated_at: i.updated_at })
    }

    pub async fn install(&self, id: i64, user_id: i64) -> Result<InventoryItemResponse, AppError> {
        let old = self.repo.get_by_id(id).await?.ok_or_else(|| AppError::NotFound("Item not found".into()))?;
        if old.status != "assigned" { return Err(AppError::BadRequest("Item must be assigned before installation".into())); }
        let i = self.repo.install(id).await?;
        self.repo.record_movement(id, "installed", None, None, None, None, user_id, Some("Item installed")).await.ok();
        Ok(InventoryItemResponse { id: i.id, branch_id: i.branch_id, item_type: i.item_type, serial_number: i.serial_number, barcode: i.barcode, purchase_price: i.purchase_price, warranty_expiry: i.warranty_expiry, supplier: i.supplier, status: i.status, assigned_to: i.assigned_to, notes: i.notes, created_at: i.created_at, updated_at: i.updated_at })
    }

    pub async fn return_item(&self, id: i64, user_id: i64) -> Result<InventoryItemResponse, AppError> {
        let old = self.repo.get_by_id(id).await?.ok_or_else(|| AppError::NotFound("Item not found".into()))?;
        if old.status != "assigned" && old.status != "installed" { return Err(AppError::BadRequest("Item must be assigned or installed to return".into())); }
        let i = self.repo.return_item(id).await?;
        self.repo.record_movement(id, "returned", None, Some(old.branch_id), None, None, user_id, Some("Item returned to stock")).await.ok();
        Ok(InventoryItemResponse { id: i.id, branch_id: i.branch_id, item_type: i.item_type, serial_number: i.serial_number, barcode: i.barcode, purchase_price: i.purchase_price, warranty_expiry: i.warranty_expiry, supplier: i.supplier, status: i.status, assigned_to: i.assigned_to, notes: i.notes, created_at: i.created_at, updated_at: i.updated_at })
    }

    pub async fn transfer(&self, id: i64, req: TransferInventoryRequest, user_id: i64) -> Result<InventoryItemResponse, AppError> {
        let old = self.repo.get_by_id(id).await?.ok_or_else(|| AppError::NotFound("Item not found".into()))?;
        let i = self.repo.transfer(id, req.to_branch_id).await?;
        self.repo.record_movement(id, "transferred", Some(old.branch_id), Some(req.to_branch_id), None, None, user_id, req.notes.as_deref()).await.ok();
        Ok(InventoryItemResponse { id: i.id, branch_id: i.branch_id, item_type: i.item_type, serial_number: i.serial_number, barcode: i.barcode, purchase_price: i.purchase_price, warranty_expiry: i.warranty_expiry, supplier: i.supplier, status: i.status, assigned_to: i.assigned_to, notes: i.notes, created_at: i.created_at, updated_at: i.updated_at })
    }

    pub async fn scrap(&self, id: i64, user_id: i64) -> Result<InventoryItemResponse, AppError> {
        let old = self.repo.get_by_id(id).await?.ok_or_else(|| AppError::NotFound("Item not found".into()))?;
        if old.status == "scrapped" { return Err(AppError::BadRequest("Item is already scrapped".into())); }
        let i = self.repo.scrap(id).await?;
        self.repo.record_movement(id, "scrapped", Some(old.branch_id), None, None, None, user_id, Some("Item scrapped")).await.ok();
        Ok(InventoryItemResponse { id: i.id, branch_id: i.branch_id, item_type: i.item_type, serial_number: i.serial_number, barcode: i.barcode, purchase_price: i.purchase_price, warranty_expiry: i.warranty_expiry, supplier: i.supplier, status: i.status, assigned_to: i.assigned_to, notes: i.notes, created_at: i.created_at, updated_at: i.updated_at })
    }

    pub async fn delete(&self, id: i64, user_id: i64) -> Result<MessageResponse, AppError> {
        let old = self.repo.get_by_id(id).await?.ok_or_else(|| AppError::NotFound("Item not found".into()))?;
        self.repo.scrap(id).await?;
        self.repo.record_movement(id, "scrapped", Some(old.branch_id), None, None, None, user_id, Some("Item deleted")).await.ok();
        Ok(MessageResponse { message: "Item scrapped".into() })
    }

    // ── Movement History ──────────────────────────────────

    pub async fn list_movements(&self, item_id: i64) -> Result<Vec<InventoryMovementResponse>, AppError> {
        self.repo.get_by_id(item_id).await?.ok_or_else(|| AppError::NotFound("Item not found".into()))?;
        let movements = self.repo.list_movements(item_id).await?;
        Ok(movements.into_iter().map(|m| InventoryMovementResponse { id: m.id, item_id: m.item_id, movement_type: m.movement_type, from_branch_id: m.from_branch_id, to_branch_id: m.to_branch_id, reference_type: m.reference_type, reference_id: m.reference_id, performed_by: m.performed_by, notes: m.notes, created_at: m.created_at }).collect())
    }

    // ── Reports ───────────────────────────────────────────

    pub async fn get_report(&self, branch_id: Option<i64>) -> Result<InventoryReportResponse, AppError> {
        let r = self.repo.get_report(branch_id).await?;
        Ok(InventoryReportResponse { total_items: r.total_items, in_stock: r.in_stock, assigned: r.assigned, installed: r.installed, damaged: r.damaged, scrapped: r.scrapped, total_value: r.total_value })
    }

    pub async fn get_warranty_alerts(&self, branch_id: Option<i64>, days: Option<i64>) -> Result<Vec<WarrantyAlertResponse>, AppError> {
        let alerts = self.repo.get_warranty_alerts(branch_id, days.unwrap_or(30)).await?;
        Ok(alerts.into_iter().map(|a| WarrantyAlertResponse { id: a.id, item_type: a.item_type, serial_number: a.serial_number, warranty_expiry: a.warranty_expiry, branch_id: a.branch_id, days_until_expiry: a.days_until_expiry }).collect())
    }
}
