use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateInventoryItemRequest {
    pub branch_id: i64,
    pub item_type: String,
    pub device_model_id: Option<i64>,
    pub serial_number: Option<String>,
    pub barcode: Option<String>,
    pub purchase_price: Option<rust_decimal::Decimal>,
    pub supplier: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct InventoryQuery {
    pub branch_id: Option<i64>,
    pub status: Option<String>,
    pub item_type: Option<String>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}
