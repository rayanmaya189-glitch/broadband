use utoipa::ToSchema;
use serde::Deserialize;
use validator::Validate;

use crate::common::utils::helpers::PaginationParams;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateBranchRequest {
    #[validate(length(min = 2, max = 255, message = "Name must be 2-255 characters"))]
    pub name: String,
    #[validate(length(min = 2, max = 50, message = "Code must be 2-50 characters"))]
    pub code: String,
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub pincode: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub timezone: Option<String>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateBranchRequest {
    #[validate(length(min = 2, max = 255, message = "Name must be 2-255 characters"))]
    pub name: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub pincode: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub timezone: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct ListBranchesQuery {
    #[serde(flatten)]
    pub pagination: PaginationParams,
    pub is_active: Option<bool>,
    pub city: Option<String>,
}

// ── Working Hours ──────────────────────────────────────────

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct WorkingHourEntry {
    pub day_of_week: i32,
    pub open_time: String,
    pub close_time: String,
    pub is_closed: bool,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateWorkingHoursRequest {
    pub hours: Vec<WorkingHourEntry>,
}

// ── User-Branch Assignment ─────────────────────────────────

#[derive(Debug, Deserialize, ToSchema)]
pub struct AssignUserToBranchRequest {
    pub user_id: i64,
    pub is_primary: Option<bool>,
}
