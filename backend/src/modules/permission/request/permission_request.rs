use utoipa::ToSchema;
use serde::Deserialize;
use validator::Validate;

use crate::common::utils::helpers::PaginationParams;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreatePermissionRequest {
    #[validate(length(min = 3, max = 100, message = "Name must be 3-100 characters"))]
    pub name: String,
    #[validate(length(min = 1, max = 10, message = "Method must be 1-10 characters"))]
    pub method: String,
    #[validate(length(min = 1, max = 500, message = "API URL must be 1-500 characters"))]
    pub api_url: String,
    #[validate(length(min = 1, max = 50, message = "Guard must be 1-50 characters"))]
    pub guard: String,
    #[validate(length(min = 1, max = 50, message = "Module must be 1-50 characters"))]
    pub module: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct ListPermissionsQuery {
    #[serde(flatten)]
    pub pagination: PaginationParams,
    pub module: Option<String>,
}
