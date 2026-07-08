use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct InventoryItem {
    pub id: i64,
    pub branch_id: i64,
    pub item_type: String,
    pub device_model_id: Option<i64>,
    pub serial_number: Option<String>,
    pub barcode: Option<String>,
    pub purchase_date: Option<NaiveDate>,
    pub purchase_price: Option<Decimal>,
    pub warranty_expiry: Option<NaiveDate>,
    pub supplier: Option<String>,
    pub status: String,
    pub assigned_to: Option<i64>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
