use serde::Deserialize;
use validator::Validate;

use crate::common::utils::helpers::PaginationParams;

#[derive(Debug, Deserialize, Validate)]
pub struct CreatePlanRequest {
    #[validate(length(min = 2, max = 255, message = "Name must be 2-255 characters"))]
    pub name: String,
    #[validate(length(min = 2, max = 50, message = "Code must be 2-50 characters"))]
    pub code: String,
    pub description: Option<String>,
    pub speed_down_mbps: i32,
    pub speed_up_mbps: i32,
    pub data_cap_gb: Option<i32>,
    pub price_monthly: rust_decimal::Decimal,
    pub price_quarterly: Option<rust_decimal::Decimal>,
    pub price_half_yearly: Option<rust_decimal::Decimal>,
    pub price_yearly: Option<rust_decimal::Decimal>,
    pub gst_percent: Option<rust_decimal::Decimal>,
    pub is_promotional: Option<bool>,
    pub category: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdatePlanRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub speed_down_mbps: Option<i32>,
    pub speed_up_mbps: Option<i32>,
    pub data_cap_gb: Option<i32>,
    pub price_monthly: Option<rust_decimal::Decimal>,
    pub price_quarterly: Option<rust_decimal::Decimal>,
    pub price_half_yearly: Option<rust_decimal::Decimal>,
    pub price_yearly: Option<rust_decimal::Decimal>,
    pub gst_percent: Option<rust_decimal::Decimal>,
    pub is_active: Option<bool>,
    pub is_promotional: Option<bool>,
    pub category: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ListPlansQuery {
    #[serde(flatten)]
    pub pagination: PaginationParams,
    pub is_active: Option<bool>,
    pub category: Option<String>,
}
