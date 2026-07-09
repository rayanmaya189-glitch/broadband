use utoipa::ToSchema;
use chrono::NaiveDate;
use serde::Deserialize;
use validator::Validate;

use crate::common::utils::helpers::PaginationParams;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateCustomerRequest {
    #[validate(length(min = 1, max = 255, message = "First name is required"))]
    pub first_name: String,
    pub last_name: Option<String>,
    #[validate(email(message = "Invalid email format"))]
    pub email: Option<String>,
    #[validate(length(min = 10, max = 20, message = "Invalid phone number"))]
    pub phone: String,
    pub alternate_phone: Option<String>,
    pub branch_id: i64,
    pub lead_id: Option<i64>,
    pub referred_by: Option<i64>,
    pub created_by: Option<i64>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateCustomerRequest {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    #[validate(email(message = "Invalid email format"))]
    pub email: Option<String>,
    pub phone: Option<String>,
    pub alternate_phone: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct ListCustomersQuery {
    #[serde(flatten)]
    pub pagination: PaginationParams,
    pub status: Option<String>,
    pub branch_id: Option<i64>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CustomerStatusTransition {
    pub status: String,
    pub reason: Option<String>,
}

// ── Customer Profile (KYC) ──────────────────────────────────

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateCustomerProfileRequest {
    pub date_of_birth: Option<NaiveDate>,
    pub gender: Option<String>,
    pub nationality: Option<String>,
    pub id_proof_type: Option<String>,
    pub id_proof_number: Option<String>,
    pub id_proof_expiry: Option<NaiveDate>,
    pub pan_number: Option<String>,
    pub aadhaar_number: Option<String>,
    pub gstin: Option<String>,
    pub company_name: Option<String>,
    pub designation: Option<String>,
    pub occupation: Option<String>,
    pub annual_income_range: Option<String>,
    pub preferred_language: Option<String>,
    pub communication_opt_in: Option<bool>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct SubmitKycRequest {
    pub id_proof_type: String,
    pub id_proof_number: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct VerifyKycRequest {
    pub status: String, // verified or rejected
    pub rejection_reason: Option<String>,
}

// ── KYC Documents ───────────────────────────────────────────

#[derive(Debug, Deserialize, ToSchema)]
pub struct ListKycDocumentsQuery {
    pub status: Option<String>,
}

// ── Customer Addresses ──────────────────────────────────────

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateAddressRequest {
    #[validate(length(min = 1))]
    pub address_type: String, // installation, billing, correspondence
    #[validate(length(min = 1))]
    pub address_line1: String,
    pub address_line2: Option<String>,
    #[validate(length(min = 1))]
    pub city: String,
    #[validate(length(min = 1))]
    pub state: String,
    #[validate(length(min = 4, max = 10))]
    pub pincode: String,
    pub landmark: Option<String>,
    pub is_primary: Option<bool>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateAddressRequest {
    pub address_type: Option<String>,
    pub address_line1: Option<String>,
    pub address_line2: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub pincode: Option<String>,
    pub landmark: Option<String>,
    pub is_primary: Option<bool>,
}
