use chrono::{DateTime, NaiveDate, Utc};
use sqlx::FromRow;

/// Customer status lifecycle.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, sqlx::Type)]
#[sqlx(type_name = "VARCHAR", rename_all = "snake_case")]
pub enum CustomerStatus {
    Lead,
    Prospect,
    Active,
    Suspended,
    Deactivated,
    Blacklist,
}

impl std::fmt::Display for CustomerStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Lead => write!(f, "lead"),
            Self::Prospect => write!(f, "prospect"),
            Self::Active => write!(f, "active"),
            Self::Suspended => write!(f, "suspended"),
            Self::Deactivated => write!(f, "deactivated"),
            Self::Blacklist => write!(f, "blacklist"),
        }
    }
}

/// Row type mapping to the `customers` table.
#[derive(Debug, Clone, FromRow)]
pub struct Customer {
    pub id: i64,
    pub customer_code: String,
    pub first_name: String,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub phone: String,
    pub alternate_phone: Option<String>,
    pub status: String,
    pub branch_id: i64,
    pub lead_id: Option<i64>,
    pub referred_by: Option<i64>,
    pub created_by: Option<i64>,
    pub kyc_status: String,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Customer profile (KYC & personal details).
#[derive(Debug, Clone, FromRow)]
pub struct CustomerProfile {
    pub id: i64,
    pub customer_id: i64,
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
    pub communication_opt_in: bool,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// KYC document uploaded by customer.
#[derive(Debug, Clone, FromRow)]
pub struct KycDocument {
    pub id: i64,
    pub customer_id: i64,
    pub document_type: String,
    pub document_url: String,
    pub file_name: Option<String>,
    pub file_size_bytes: Option<i64>,
    pub mime_type: Option<String>,
    pub verification_status: String,
    pub rejection_reason: Option<String>,
    pub verified_by: Option<i64>,
    pub verified_at: Option<DateTime<Utc>>,
    pub uploaded_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Customer address.
#[derive(Debug, Clone, FromRow)]
pub struct CustomerAddress {
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
