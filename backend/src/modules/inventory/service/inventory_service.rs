use sqlx::PgPool;
use crate::common::errors::app_error::AppError;
use crate::modules::inventory::repository::inventory_repository::InventoryRepository;
use crate::modules::inventory::request::inventory_request::*;
use crate::modules::inventory::response::inventory_response::*;

pub struct InventoryService<'a> { repo: InventoryRepository<'a> }
impl<'a> InventoryService<'a> {
    pub fn new(pool: &'a PgPool) -> Self { Self { repo: InventoryRepository::new(pool) } }
    pub async fn list(&self, q: InventoryQuery) -> Result<InventoryListResponse, AppError> {
        let page = q.page.unwrap_or(1); let per_page = q.per_page.unwrap_or(20);
        let (items, total) = self.repo.list(q.branch_id, q.status.as_deref(), page, per_page).await?;
        Ok(InventoryListResponse { items: items.iter().map(|i| InventoryItemResponse { id: i.id, branch_id: i.branch_id, item_type: i.item_type.clone(), serial_number: i.serial_number.clone(), status: i.status.clone(), assigned_to: i.assigned_to, created_at: i.created_at }).collect(), total })
    }
    pub async fn get(&self, id: i64) -> Result<InventoryItemResponse, AppError> {
        let i = self.repo.get_by_id(id).await?.ok_or_else(|| AppError::NotFound("Not found".into()))?;
        Ok(InventoryItemResponse { id: i.id, branch_id: i.branch_id, item_type: i.item_type, serial_number: i.serial_number, status: i.status, assigned_to: i.assigned_to, created_at: i.created_at })
    }
    pub async fn create(&self, req: CreateInventoryItemRequest) -> Result<InventoryItemResponse, AppError> {
        let i = self.repo.create(req.branch_id, &req.item_type, req.serial_number.as_deref(), req.barcode.as_deref(), req.purchase_price, req.supplier.as_deref()).await?;
        Ok(InventoryItemResponse { id: i.id, branch_id: i.branch_id, item_type: i.item_type, serial_number: i.serial_number, status: i.status, assigned_to: i.assigned_to, created_at: i.created_at })
    }
    pub async fn delete(&self, id: i64) -> Result<MessageResponse, AppError> {
        if !self.repo.delete(id).await? { return Err(AppError::NotFound("Not found".into())); }
        Ok(MessageResponse { message: "Deleted".into() })
    }
}
