use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;

#[derive(Debug, Clone)]
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
    pub assigned_to_branch_id: Option<i64>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct InventoryMovement {
    pub id: i64,
    pub item_id: i64,
    pub movement_type: String,
    pub from_branch_id: Option<i64>,
    pub to_branch_id: Option<i64>,
    pub reference_type: Option<String>,
    pub reference_id: Option<i64>,
    pub performed_by: i64,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct InventoryReport {
    pub total_items: i64,
    pub in_stock: i64,
    pub assigned: i64,
    pub installed: i64,
    pub damaged: i64,
    pub scrapped: i64,
    pub total_value: Option<Decimal>,
}

#[derive(Debug, Clone)]
pub struct WarrantyAlert {
    pub id: i64,
    pub item_type: String,
    pub serial_number: Option<String>,
    pub warranty_expiry: NaiveDate,
    pub branch_id: i64,
    pub days_until_expiry: i64,
}
