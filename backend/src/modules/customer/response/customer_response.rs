use utoipa::ToSchema;
use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, Serialize, FromRow, ToSchema)]
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

#[derive(Debug, Serialize, ToSchema)]
pub struct MessageResponse {
    pub message: String,
}

// ── Customer Profile ─────────────────────────────────────────

#[derive(Debug, Serialize, ToSchema)]
pub struct CustomerProfileResponse {
    pub id: i64,
    pub customer_id: i64,
    pub date_of_birth: Option<NaiveDate>,
    pub gender: Option<String>,
    pub nationality: Option<String>,
    pub id_proof_type: Option<String>,
    pub id_proof_number: Option<String>,
    pub pan_number: Option<String>,
    pub gstin: Option<String>,
    pub company_name: Option<String>,
    pub occupation: Option<String>,
    pub preferred_language: Option<String>,
    pub communication_opt_in: bool,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ── KYC Document ────────────────────────────────────────────

#[derive(Debug, Serialize, ToSchema)]
pub struct KycDocumentResponse {
    pub id: i64,
    pub customer_id: i64,
    pub document_type: String,
    pub document_url: String,
    pub file_name: Option<String>,
    pub verification_status: String,
    pub rejection_reason: Option<String>,
    pub uploaded_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

// ── Customer Address ────────────────────────────────────────

#[derive(Debug, Serialize, FromRow, ToSchema)]
pub struct AddressResponse {
    pub id: i64,
    pub customer_id: i64,
    pub address_type: String,
    pub address_line1: String,
    pub address_line2: Option<String>,
    pub city: String,
    pub state: String,
    pub pincode: String,
    pub landmark: Option<String>,
    pub is_primary: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ── Customer Detail (with profile + addresses) ──────────────

#[derive(Debug, Serialize, ToSchema)]
pub struct CustomerFullResponse {
    pub customer: CustomerResponse,
    pub profile: Option<CustomerProfileResponse>,
    pub addresses: Vec<AddressResponse>,
    pub kyc_documents: Vec<KycDocumentResponse>,
}

// ── Pro-Rata Adjustment ────────────────────────────────────

#[derive(Debug, Serialize, ToSchema)]
pub struct ProRataAdjustment {
    pub old_plan_credit: Decimal,
    pub new_plan_charge: Decimal,
    pub adjustment: Decimal,
    pub remaining_days: i32,
    pub billing_period_days: i32,
}

// ── Plan Speed Profile ─────────────────────────────────────

#[derive(Debug, Serialize, ToSchema)]
pub struct SpeedProfileResponse {
    pub id: i64,
    pub plan_id: i64,
    pub name: String,
    pub download_limit_kbps: i32,
    pub upload_limit_kbps: i32,
    pub burst_download_kbps: Option<i32>,
    pub burst_upload_kbps: Option<i32>,
    pub burst_duration_seconds: i32,
    pub priority_queue: i32,
    pub qos_marking: Option<String>,
    pub fq_codel_enabled: bool,
    pub device_type: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
