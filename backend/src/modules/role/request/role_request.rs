use utoipa::ToSchema;
use serde::Deserialize;
use validator::Validate;

use crate::common::utils::helpers::PaginationParams;

#[derive(Debug, Deserialize, Validate)]
#[derive(ToSchema)]
pub struct CreateRoleRequest {
    #[validate(length(min = 2, max = 50, message = "Name must be 2-50 characters"))]
    pub name: String,
    #[validate(length(min = 2, max = 100, message = "Display name must be 2-100 characters"))]
    pub display_name: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
#[derive(ToSchema)]
pub struct UpdateRoleRequest {
    #[validate(length(min = 2, max = 50, message = "Name must be 2-50 characters"))]
    pub name: Option<String>,
    #[validate(length(min = 2, max = 100, message = "Display name must be 2-100 characters"))]
    pub display_name: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
#[derive(ToSchema)]
pub struct ListRolesQuery {
    #[serde(flatten)]
    pub pagination: PaginationParams,
    pub is_active: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[derive(ToSchema)]
pub struct AssignPermissionsRequest {
    pub permission_ids: Vec<i64>,
}
