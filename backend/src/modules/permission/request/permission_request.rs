use serde::Deserialize;
use validator::Validate;

use crate::common::utils::helpers::PaginationParams;

#[derive(Debug, Deserialize, Validate)]
pub struct CreatePermissionRequest {
    #[validate(length(min = 3, max = 100, message = "Name must be 3-100 characters"))]
    pub name: String,
    pub description: Option<String>,
    #[validate(length(min = 1, max = 50, message = "Module must be 1-50 characters"))]
    pub module: String,
}

#[derive(Debug, Deserialize)]
pub struct ListPermissionsQuery {
    #[serde(flatten)]
    pub pagination: PaginationParams,
    pub module: Option<String>,
}
