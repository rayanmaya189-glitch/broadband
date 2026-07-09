use sqlx::PgPool;
use crate::modules::inventory::model::inventory::*;

pub struct InventoryRepository<'a> { pool: &'a PgPool }
impl<'a> InventoryRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self { Self { pool } }

    pub async fn list(&self, branch_id: Option<i64>, status: Option<&str>, item_type: Option<&str>, page: i64, per_page: i64) -> Result<(Vec<InventoryItem>, i64), sqlx::Error> {
        let offset = (page - 1) * per_page;
        let count_row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM inventory_items WHERE ($1::bigint IS NULL OR branch_id = $1) AND ($2::text IS NULL OR status = $2) AND ($3::text IS NULL OR item_type = $3)")
            .bind(branch_id).bind(status).bind(item_type).fetch_one(self.pool).await?;
        let items: Vec<InventoryItem> = sqlx::query_as("SELECT id, branch_id, item_type, device_model_id, serial_number, barcode, purchase_date, purchase_price, warranty_expiry, supplier, status, assigned_to, assigned_to_branch_id, notes, created_at, updated_at FROM inventory_items WHERE ($1::bigint IS NULL OR branch_id = $1) AND ($2::text IS NULL OR status = $2) AND ($3::text IS NULL OR item_type = $3) ORDER BY created_at DESC LIMIT $4 OFFSET $5")
            .bind(branch_id).bind(status).bind(item_type).bind(per_page).bind(offset).fetch_all(self.pool).await?;
        Ok((items, count_row.0))
    }

    pub async fn create(&self, branch_id: i64, item_type: &str, serial: Option<&str>, barcode: Option<&str>, price: Option<rust_decimal::Decimal>, supplier: Option<&str>, notes: Option<&str>) -> Result<InventoryItem, sqlx::Error> {
        sqlx::query_as::<_, InventoryItem>("INSERT INTO inventory_items (branch_id, item_type, serial_number, barcode, purchase_price, supplier, notes) VALUES ($1,$2,$3,$4,$5,$6,$7) RETURNING id, branch_id, item_type, device_model_id, serial_number, barcode, purchase_date, purchase_price, warranty_expiry, supplier, status, assigned_to, assigned_to_branch_id, notes, created_at, updated_at")
            .bind(branch_id).bind(item_type).bind(serial).bind(barcode).bind(price).bind(supplier).bind(notes).fetch_one(self.pool).await
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<InventoryItem>, sqlx::Error> {
        sqlx::query_as::<_, InventoryItem>("SELECT id, branch_id, item_type, device_model_id, serial_number, barcode, purchase_date, purchase_price, warranty_expiry, supplier, status, assigned_to, assigned_to_branch_id, notes, created_at, updated_at FROM inventory_items WHERE id = $1").bind(id).fetch_optional(self.pool).await
    }

    pub async fn update_status(&self, id: i64, status: &str) -> Result<InventoryItem, sqlx::Error> {
        sqlx::query_as::<_, InventoryItem>("UPDATE inventory_items SET status = $2, updated_at = NOW() WHERE id = $1 RETURNING id, branch_id, item_type, device_model_id, serial_number, barcode, purchase_date, purchase_price, warranty_expiry, supplier, status, assigned_to, assigned_to_branch_id, notes, created_at, updated_at")
            .bind(id).bind(status).fetch_one(self.pool).await
    }

    pub async fn assign(&self, id: i64, user_id: i64) -> Result<InventoryItem, sqlx::Error> {
        sqlx::query_as::<_, InventoryItem>("UPDATE inventory_items SET status = 'assigned', assigned_to = $2, updated_at = NOW() WHERE id = $1 RETURNING id, branch_id, item_type, device_model_id, serial_number, barcode, purchase_date, purchase_price, warranty_expiry, supplier, status, assigned_to, assigned_to_branch_id, notes, created_at, updated_at")
            .bind(id).bind(user_id).fetch_one(self.pool).await
    }

    pub async fn install(&self, id: i64) -> Result<InventoryItem, sqlx::Error> {
        sqlx::query_as::<_, InventoryItem>("UPDATE inventory_items SET status = 'installed', updated_at = NOW() WHERE id = $1 RETURNING id, branch_id, item_type, device_model_id, serial_number, barcode, purchase_date, purchase_price, warranty_expiry, supplier, status, assigned_to, assigned_to_branch_id, notes, created_at, updated_at")
            .bind(id).fetch_one(self.pool).await
    }

    pub async fn return_item(&self, id: i64) -> Result<InventoryItem, sqlx::Error> {
        sqlx::query_as::<_, InventoryItem>("UPDATE inventory_items SET status = 'in_stock', assigned_to = NULL, updated_at = NOW() WHERE id = $1 RETURNING id, branch_id, item_type, device_model_id, serial_number, barcode, purchase_date, purchase_price, warranty_expiry, supplier, status, assigned_to, assigned_to_branch_id, notes, created_at, updated_at")
            .bind(id).fetch_one(self.pool).await
    }

    pub async fn transfer(&self, id: i64, to_branch_id: i64) -> Result<InventoryItem, sqlx::Error> {
        sqlx::query_as::<_, InventoryItem>("UPDATE inventory_items SET branch_id = $2, status = 'in_stock', assigned_to = NULL, updated_at = NOW() WHERE id = $1 RETURNING id, branch_id, item_type, device_model_id, serial_number, barcode, purchase_date, purchase_price, warranty_expiry, supplier, status, assigned_to, assigned_to_branch_id, notes, created_at, updated_at")
            .bind(id).bind(to_branch_id).fetch_one(self.pool).await
    }

    pub async fn scrap(&self, id: i64) -> Result<InventoryItem, sqlx::Error> {
        sqlx::query_as::<_, InventoryItem>("UPDATE inventory_items SET status = 'scrapped', updated_at = NOW() WHERE id = $1 RETURNING id, branch_id, item_type, device_model_id, serial_number, barcode, purchase_date, purchase_price, warranty_expiry, supplier, status, assigned_to, assigned_to_branch_id, notes, created_at, updated_at")
            .bind(id).fetch_one(self.pool).await
    }

    // ── Movement History ──────────────────────────────────

    pub async fn record_movement(&self, item_id: i64, movement_type: &str, from_branch_id: Option<i64>, to_branch_id: Option<i64>, reference_type: Option<&str>, reference_id: Option<i64>, performed_by: i64, notes: Option<&str>) -> Result<InventoryMovement, sqlx::Error> {
        sqlx::query_as::<_, InventoryMovement>("INSERT INTO inventory_movements (item_id, movement_type, from_branch_id, to_branch_id, reference_type, reference_id, performed_by, notes) VALUES ($1,$2,$3,$4,$5,$6,$7,$8) RETURNING id, item_id, movement_type, from_branch_id, to_branch_id, reference_type, reference_id, performed_by, notes, created_at")
            .bind(item_id).bind(movement_type).bind(from_branch_id).bind(to_branch_id).bind(reference_type).bind(reference_id).bind(performed_by).bind(notes).fetch_one(self.pool).await
    }

    pub async fn list_movements(&self, item_id: i64) -> Result<Vec<InventoryMovement>, sqlx::Error> {
        sqlx::query_as::<_, InventoryMovement>("SELECT id, item_id, movement_type, from_branch_id, to_branch_id, reference_type, reference_id, performed_by, notes, created_at FROM inventory_movements WHERE item_id = $1 ORDER BY created_at DESC").bind(item_id).fetch_all(self.pool).await
    }

    // ── Reports ───────────────────────────────────────────

    pub async fn get_report(&self, branch_id: Option<i64>) -> Result<InventoryReport, sqlx::Error> {
        let row = sqlx::query_as::<_, (i64, i64, i64, i64, i64, i64, Option<rust_decimal::Decimal>)>(
            "SELECT
                (SELECT COUNT(*) FROM inventory_items WHERE ($1::bigint IS NULL OR branch_id = $1)) as total,
                (SELECT COUNT(*) FROM inventory_items WHERE status = 'in_stock' AND ($1::bigint IS NULL OR branch_id = $1)) as in_stock,
                (SELECT COUNT(*) FROM inventory_items WHERE status = 'assigned' AND ($1::bigint IS NULL OR branch_id = $1)) as assigned,
                (SELECT COUNT(*) FROM inventory_items WHERE status = 'installed' AND ($1::bigint IS NULL OR branch_id = $1)) as installed,
                (SELECT COUNT(*) FROM inventory_items WHERE status = 'damaged' AND ($1::bigint IS NULL OR branch_id = $1)) as damaged,
                (SELECT COUNT(*) FROM inventory_items WHERE status = 'scrapped' AND ($1::bigint IS NULL OR branch_id = $1)) as scrapped,
                (SELECT COALESCE(SUM(purchase_price), 0) FROM inventory_items WHERE status != 'scrapped' AND ($1::bigint IS NULL OR branch_id = $1)) as total_value"
        ).bind(branch_id).fetch_one(self.pool).await?;
        Ok(InventoryReport { total_items: row.0, in_stock: row.1, assigned: row.2, installed: row.3, damaged: row.4, scrapped: row.5, total_value: row.6 })
    }

    pub async fn get_warranty_alerts(&self, branch_id: Option<i64>, days_threshold: i64) -> Result<Vec<WarrantyAlert>, sqlx::Error> {
        sqlx::query_as::<_, WarrantyAlert>(
            "SELECT id, item_type, serial_number, warranty_expiry, branch_id, EXTRACT(DAY FROM warranty_expiry - NOW())::bigint as days_until_expiry FROM inventory_items WHERE warranty_expiry IS NOT NULL AND warranty_expiry <= NOW() + make_interval(days => $2) AND status != 'scrapped' AND ($1::bigint IS NULL OR branch_id = $1) ORDER BY warranty_expiry ASC"
        ).bind(branch_id).bind(days_threshold).fetch_all(self.pool).await
    }
}
