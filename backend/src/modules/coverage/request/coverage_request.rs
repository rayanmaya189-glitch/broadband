use utoipa::ToSchema;
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateCoverageAreaRequest {
    pub branch_id: i64,
    pub name: String,
    pub description: Option<String>,
    pub area_type: String,
    pub pincodes: Option<Vec<String>>,
    pub fiber_available: Option<bool>,
    pub estimated_installation_days: Option<i32>,
    pub max_customers: Option<i32>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateCoverageAreaRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub area_type: Option<String>,
    pub fiber_available: Option<bool>,
    pub estimated_installation_days: Option<i32>,
    pub max_customers: Option<i32>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CheckAvailabilityRequest {
    pub pincode: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct AddPincodeRequest {
    #[validate(length(min = 1, max = 10))]
    pub pincode: String,
    pub city: String,
    pub district: Option<String>,
    pub state: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CoverageQuery {
    pub branch_id: Option<i64>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}
