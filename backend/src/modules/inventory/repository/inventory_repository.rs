use sqlx::PgPool;
use crate::modules::inventory::model::inventory::InventoryItem;

pub struct InventoryRepository<'a> { pool: &'a PgPool }
impl<'a> InventoryRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self { Self { pool } }
    pub fn pool(&self) -> &'a PgPool { self.pool }

    pub async fn list(&self, branch_id: Option<i64>, status: Option<&str>, page: i64, per_page: i64) -> Result<(Vec<InventoryItem>, i64), sqlx::Error> {
        let offset = (page - 1) * per_page;
        let count_row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM inventory_items WHERE ($1::bigint IS NULL OR branch_id = $1) AND ($2::text IS NULL OR status = $2)").bind(branch_id).bind(status).fetch_one(self.pool).await?;
        let items: Vec<InventoryItem> = sqlx::query_as("SELECT * FROM inventory_items WHERE ($1::bigint IS NULL OR branch_id = $1) AND ($2::text IS NULL OR status = $2) ORDER BY created_at DESC LIMIT $3 OFFSET $4").bind(branch_id).bind(status).bind(per_page).bind(offset).fetch_all(self.pool).await?;
        Ok((items, count_row.0))
    }

    pub async fn create(&self, branch_id: i64, item_type: &str, serial: Option<&str>, barcode: Option<&str>, price: Option<rust_decimal::Decimal>, supplier: Option<&str>) -> Result<InventoryItem, sqlx::Error> {
        sqlx::query_as::<_, InventoryItem>("INSERT INTO inventory_items (branch_id, item_type, serial_number, barcode, purchase_price, supplier) VALUES ($1,$2,$3,$4,$5,$6) RETURNING *").bind(branch_id).bind(item_type).bind(serial).bind(barcode).bind(price).bind(supplier).fetch_one(self.pool).await
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<InventoryItem>, sqlx::Error> {
        sqlx::query_as::<_, InventoryItem>("SELECT * FROM inventory_items WHERE id = $1").bind(id).fetch_optional(self.pool).await
    }

    pub async fn delete(&self, id: i64) -> Result<bool, sqlx::Error> {
        let r = sqlx::query("DELETE FROM inventory_items WHERE id = $1").bind(id).execute(self.pool).await?;
        Ok(r.rows_affected() > 0)
    }
}
