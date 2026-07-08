use utoipa::ToSchema;
use serde::Deserialize;
use validator::Validate;

use crate::common::utils::helpers::PaginationParams;

#[derive(Debug, Deserialize, Validate)]
#[derive(ToSchema)]
pub struct CreatePermissionRequest {
    #[validate(length(min = 3, max = 100, message = "Name must be 3-100 characters"))]
    pub name: String,
    pub description: Option<String>,
    #[validate(length(min = 1, max = 50, message = "Module must be 1-50 characters"))]
    pub module: String,
}

#[derive(Debug, Deserialize)]
#[derive(ToSchema)]
pub struct ListPermissionsQuery {
    #[serde(flatten)]
    pub pagination: PaginationParams,
    pub module: Option<String>,
}
