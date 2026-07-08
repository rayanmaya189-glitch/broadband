use utoipa::ToSchema;
use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, Serialize, FromRow)]
#[derive(ToSchema)]
pub struct CustomerResponse {
    pub id: i64,
    pub customer_code: String,
    pub first_name: String,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub phone: String,
    pub alternate_phone: Option<String>,
    pub status: String,
    pub branch_id: i64,
    pub kyc_status: String,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub type CustomerDetailResponse = CustomerResponse;

#[derive(Debug, Serialize)]
#[derive(ToSchema)]
pub struct MessageResponse {
    pub message: String,
}
