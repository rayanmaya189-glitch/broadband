use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;


#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct InventoryItemResponse {
    pub id: i64,
    pub branch_id: i64,
    pub item_type: String,
    pub serial_number: Option<String>,
    pub barcode: Option<String>,
    pub purchase_price: Option<Decimal>,
    pub warranty_expiry: Option<NaiveDate>,
    pub supplier: Option<String>,
    pub status: String,
    pub assigned_to: Option<i64>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct InventoryMovementResponse {
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

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct InventoryListResponse {
    pub items: Vec<InventoryItemResponse>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct InventoryReportResponse {
    pub total_items: i64,
    pub in_stock: i64,
    pub assigned: i64,
    pub installed: i64,
    pub damaged: i64,
    pub scrapped: i64,
    pub total_value: Option<Decimal>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct WarrantyAlertResponse {
    pub id: i64,
    pub item_type: String,
    pub serial_number: Option<String>,
    pub warranty_expiry: NaiveDate,
    pub branch_id: i64,
    pub days_until_expiry: i64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct MessageResponse {
    pub message: String,
}
