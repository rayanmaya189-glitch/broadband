use sqlx::PgPool;

use crate::common::errors::app_error::AppError;
use crate::common::utils::helpers::{total_pages, PaginatedResponse};
use crate::modules::customer::model::customer::{Customer, CustomerProfile, KycDocument};
use crate::modules::customer::response::customer_response::{CustomerResponse, AddressResponse};

pub struct CustomerRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> CustomerRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self { Self { pool } }

    pub fn pool(&self) -> &'a PgPool {
        self.pool
    }

    pub async fn find_by_id(&self, id: i64) -> Result<Option<Customer>, AppError> {
        let r = sqlx::query_as::<_, Customer>(
            "SELECT id, customer_code, first_name, last_name, email, phone, alternate_phone, status, branch_id, lead_id, referred_by, created_by, kyc_status, notes, created_at, updated_at FROM customers WHERE id = $1 AND deleted_at IS NULL",
        ).bind(id).fetch_optional(self.pool).await?;
        Ok(r)
    }

    pub async fn find_by_code(&self, code: &str) -> Result<Option<Customer>, AppError> {
        let r = sqlx::query_as::<_, Customer>(
            "SELECT id, customer_code, first_name, last_name, email, phone, alternate_phone, status, branch_id, lead_id, referred_by, created_by, kyc_status, notes, created_at, updated_at FROM customers WHERE customer_code = $1 AND deleted_at IS NULL",
        ).bind(code).fetch_optional(self.pool).await?;
        Ok(r)
    }

    pub async fn create(
        &self, customer_code: &str, first_name: &str, last_name: Option<&str>,
        email: Option<&str>, phone: &str, alternate_phone: Option<&str>,
        branch_id: i64, lead_id: Option<i64>, referred_by: Option<i64>,
        created_by: Option<i64>, notes: Option<&str>,
    ) -> Result<Customer, AppError> {
        let r = sqlx::query_as::<_, Customer>(
            "INSERT INTO customers (customer_code, first_name, last_name, email, phone, alternate_phone, branch_id, lead_id, referred_by, created_by, notes) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11) RETURNING id, customer_code, first_name, last_name, email, phone, alternate_phone, status, branch_id, lead_id, referred_by, created_by, kyc_status, notes, created_at, updated_at",
        ).bind(customer_code).bind(first_name).bind(last_name).bind(email).bind(phone).bind(alternate_phone).bind(branch_id).bind(lead_id).bind(referred_by).bind(created_by).bind(notes).fetch_one(self.pool).await?;
        Ok(r)
    }

    pub async fn update(
        &self, id: i64, first_name: Option<&str>, last_name: Option<&str>,
        email: Option<&str>, phone: Option<&str>, alternate_phone: Option<&str>,
        notes: Option<&str>,
    ) -> Result<Customer, AppError> {
        let r = sqlx::query_as::<_, Customer>(
            "UPDATE customers SET first_name = COALESCE($2, first_name), last_name = COALESCE($3, last_name), email = COALESCE($4, email), phone = COALESCE($5, phone), alternate_phone = COALESCE($6, alternate_phone), notes = COALESCE($7, notes), updated_at = NOW() WHERE id = $1 AND deleted_at IS NULL RETURNING id, customer_code, first_name, last_name, email, phone, alternate_phone, status, branch_id, lead_id, referred_by, created_by, kyc_status, notes, created_at, updated_at",
        ).bind(id).bind(first_name).bind(last_name).bind(email).bind(phone).bind(alternate_phone).bind(notes).fetch_one(self.pool).await?;
        Ok(r)
    }

    pub async fn update_status(&self, id: i64, status: &str) -> Result<Customer, AppError> {
        let r = sqlx::query_as::<_, Customer>(
            "UPDATE customers SET status = $2, updated_at = NOW() WHERE id = $1 AND deleted_at IS NULL RETURNING id, customer_code, first_name, last_name, email, phone, alternate_phone, status, branch_id, lead_id, referred_by, created_by, kyc_status, notes, created_at, updated_at",
        ).bind(id).bind(status).fetch_one(self.pool).await?;
        Ok(r)
    }

    pub async fn soft_delete(&self, id: i64) -> Result<(), AppError> {
        sqlx::query("UPDATE customers SET deleted_at = NOW(), updated_at = NOW() WHERE id = $1").bind(id).execute(self.pool).await?;
        Ok(())
    }

    pub async fn list(
        &self, offset: u32, limit: u32, status: Option<&str>, branch_id: Option<i64>,
        search: Option<&str>,
    ) -> Result<PaginatedResponse<CustomerResponse>, AppError> {
        let limit_i64 = limit.min(100) as i64;
        let offset_i64 = offset as i64;
        let mut conditions = vec!["deleted_at IS NULL".to_string()];
        let mut idx = 1;
        if status.is_some() { conditions.push(format!("status = ${idx}")); idx += 1; }
        if branch_id.is_some() { conditions.push(format!("branch_id = ${idx}")); idx += 1; }
        if search.is_some() {
            conditions.push(format!("(first_name ILIKE ${idx} OR last_name ILIKE ${idx} OR phone ILIKE ${idx} OR email ILIKE ${idx} OR customer_code ILIKE ${idx})"));
            idx += 1;
        }
        let wc = format!("WHERE {}", conditions.join(" AND "));

        let count_sql = format!("SELECT COUNT(*) FROM customers {wc}");
        let mut cq = sqlx::query_scalar::<_, i64>(&count_sql);
        if let Some(v) = status { cq = cq.bind(v); }
        if let Some(v) = branch_id { cq = cq.bind(v); }
        if let Some(v) = search { cq = cq.bind(format!("%{v}%")); }
        let total = cq.fetch_one(self.pool).await?;

        let lp = idx;
        let op = idx + 1;
        let data_sql = format!(
            "SELECT id, customer_code, first_name, last_name, email, phone, alternate_phone, status, branch_id, kyc_status, notes, created_at, updated_at FROM customers {wc} ORDER BY created_at DESC LIMIT ${lp} OFFSET ${op}"
        );
        let mut dq = sqlx::query_as::<_, CustomerResponse>(&data_sql);
        if let Some(v) = status { dq = dq.bind(v); }
        if let Some(v) = branch_id { dq = dq.bind(v); }
        if let Some(v) = search { dq = dq.bind(format!("%{v}%")); }
        dq = dq.bind(limit_i64).bind(offset_i64);
        let customers = dq.fetch_all(self.pool).await?;
        let tp = total_pages(total, limit);
        Ok(PaginatedResponse { data: customers, total, page: (offset / limit) + 1, limit, total_pages: tp })
    }

    pub async fn phone_exists(&self, phone: &str, exclude: Option<i64>) -> Result<bool, AppError> {
        let r = if let Some(id) = exclude {
            sqlx::query_scalar::<_, bool>("SELECT EXISTS(SELECT 1 FROM customers WHERE phone = $1 AND id != $2 AND deleted_at IS NULL)").bind(phone).bind(id).fetch_one(self.pool).await?
        } else {
            sqlx::query_scalar::<_, bool>("SELECT EXISTS(SELECT 1 FROM customers WHERE phone = $1 AND deleted_at IS NULL)").bind(phone).fetch_one(self.pool).await?
        };
        Ok(r)
    }

    pub async fn generate_customer_code(&self, branch_code: &str) -> Result<String, AppError> {
        let month = chrono::Utc::now().format("%Y%m").to_string();
        let pattern = format!("AX-{branch_code}-{month}-%");
        let count = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM customers WHERE customer_code LIKE $1",
        ).bind(&pattern).fetch_one(self.pool).await?;
        Ok(format!("AX-{branch_code}-{month}-{:04}", count + 1))
    }

    // ── Customer Profile (KYC) ──────────────────────────────

    pub async fn get_profile(&self, customer_id: i64) -> Result<Option<CustomerProfile>, AppError> {
        let r = sqlx::query_as::<_, CustomerProfile>(
            "SELECT id, customer_id, date_of_birth, gender, nationality, id_proof_type, id_proof_number, id_proof_expiry, pan_number, aadhaar_number, gstin, company_name, designation, occupation, annual_income_range, preferred_language, communication_opt_in, notes, created_at, updated_at FROM customer_profiles WHERE customer_id = $1",
        ).bind(customer_id).fetch_optional(self.pool).await?;
        Ok(r)
    }

    pub async fn upsert_profile(
        &self, customer_id: i64,
        date_of_birth: Option<chrono::NaiveDate>, gender: Option<&str>, nationality: Option<&str>,
        id_proof_type: Option<&str>, id_proof_number: Option<&str>,
        pan_number: Option<&str>, aadhaar_number: Option<&str>, gstin: Option<&str>,
        company_name: Option<&str>, designation: Option<&str>, occupation: Option<&str>,
        annual_income_range: Option<&str>, preferred_language: Option<&str>,
        communication_opt_in: Option<bool>, notes: Option<&str>,
    ) -> Result<CustomerProfile, AppError> {
        let r = sqlx::query_as::<_, CustomerProfile>(
            "INSERT INTO customer_profiles (customer_id, date_of_birth, gender, nationality, id_proof_type, id_proof_number, pan_number, aadhaar_number, gstin, company_name, designation, occupation, annual_income_range, preferred_language, communication_opt_in, notes) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15,$16) ON CONFLICT (customer_id) DO UPDATE SET date_of_birth = COALESCE($2, customer_profiles.date_of_birth), gender = COALESCE($3, customer_profiles.gender), nationality = COALESCE($4, customer_profiles.nationality), id_proof_type = COALESCE($5, customer_profiles.id_proof_type), id_proof_number = COALESCE($6, customer_profiles.id_proof_number), pan_number = COALESCE($7, customer_profiles.pan_number), aadhaar_number = COALESCE($8, customer_profiles.aadhaar_number), gstin = COALESCE($9, customer_profiles.gstin), company_name = COALESCE($10, customer_profiles.company_name), designation = COALESCE($11, customer_profiles.designation), occupation = COALESCE($12, customer_profiles.occupation), annual_income_range = COALESCE($13, customer_profiles.annual_income_range), preferred_language = COALESCE($14, customer_profiles.preferred_language), communication_opt_in = COALESCE($15, customer_profiles.communication_opt_in), notes = COALESCE($16, customer_profiles.notes), updated_at = NOW() RETURNING id, customer_id, date_of_birth, gender, nationality, id_proof_type, id_proof_number, id_proof_expiry, pan_number, aadhaar_number, gstin, company_name, designation, occupation, annual_income_range, preferred_language, communication_opt_in, notes, created_at, updated_at",
        ).bind(customer_id).bind(date_of_birth).bind(gender).bind(nationality).bind(id_proof_type).bind(id_proof_number).bind(pan_number).bind(aadhaar_number).bind(gstin).bind(company_name).bind(designation).bind(occupation).bind(annual_income_range).bind(preferred_language).bind(communication_opt_in).bind(notes).fetch_one(self.pool).await?;
        Ok(r)
    }

    pub async fn submit_kyc(&self, customer_id: i64, id_proof_type: &str, id_proof_number: &str) -> Result<(), AppError> {
        sqlx::query("UPDATE customers SET kyc_status = 'submitted', updated_at = NOW() WHERE id = $1")
            .bind(customer_id).execute(self.pool).await?;
        sqlx::query("UPDATE customer_profiles SET id_proof_type = $2, id_proof_number = $3, updated_at = NOW() WHERE customer_id = $1")
            .bind(customer_id).bind(id_proof_type).bind(id_proof_number).execute(self.pool).await?;
        Ok(())
    }

    pub async fn verify_kyc(&self, customer_id: i64, status: &str, rejection_reason: Option<&str>, verified_by: Option<i64>) -> Result<(), AppError> {
        sqlx::query("UPDATE customers SET kyc_status = $2, updated_at = NOW() WHERE id = $1")
            .bind(customer_id).bind(status).execute(self.pool).await?;
        sqlx::query("UPDATE kyc_documents SET verification_status = $2, rejection_reason = $3, verified_by = $4, verified_at = NOW(), updated_at = NOW() WHERE customer_id = $1 AND verification_status = 'pending'")
            .bind(customer_id).bind(status).bind(rejection_reason).bind(verified_by).execute(self.pool).await?;
        Ok(())
    }

    // ── KYC Documents ───────────────────────────────────────

    pub async fn list_kyc_documents(&self, customer_id: i64) -> Result<Vec<KycDocument>, AppError> {
        let r = sqlx::query_as::<_, KycDocument>(
            "SELECT id, customer_id, document_type, document_url, file_name, file_size_bytes, mime_type, verification_status, rejection_reason, verified_by, verified_at, uploaded_at, created_at, updated_at FROM kyc_documents WHERE customer_id = $1 ORDER BY uploaded_at DESC",
        ).bind(customer_id).fetch_all(self.pool).await?;
        Ok(r)
    }

    pub async fn create_kyc_document(&self, customer_id: i64, document_type: &str, document_url: &str, file_name: Option<&str>, file_size: Option<i64>, mime_type: Option<&str>) -> Result<KycDocument, AppError> {
        let r = sqlx::query_as::<_, KycDocument>(
            "INSERT INTO kyc_documents (customer_id, document_type, document_url, file_name, file_size_bytes, mime_type) VALUES ($1,$2,$3,$4,$5,$6) RETURNING id, customer_id, document_type, document_url, file_name, file_size_bytes, mime_type, verification_status, rejection_reason, verified_by, verified_at, uploaded_at, created_at, updated_at",
        ).bind(customer_id).bind(document_type).bind(document_url).bind(file_name).bind(file_size).bind(mime_type).fetch_one(self.pool).await?;
        Ok(r)
    }

    pub async fn delete_kyc_document(&self, id: i64, customer_id: i64) -> Result<(), AppError> {
        sqlx::query("DELETE FROM kyc_documents WHERE id = $1 AND customer_id = $2")
            .bind(id).bind(customer_id).execute(self.pool).await?;
        Ok(())
    }

    // ── Customer Addresses ──────────────────────────────────

    pub async fn list_addresses(&self, customer_id: i64) -> Result<Vec<AddressResponse>, AppError> {
        let r = sqlx::query_as::<_, AddressResponse>(
            "SELECT id, customer_id, type as address_type, address_line1, address_line2, city, state, pincode, landmark, is_primary, created_at, updated_at FROM customer_addresses WHERE customer_id = $1 ORDER BY is_primary DESC, created_at DESC",
        ).bind(customer_id).fetch_all(self.pool).await?;
        Ok(r)
    }

    pub async fn create_address(&self, customer_id: i64, address_type: &str, address_line1: &str, address_line2: Option<&str>, city: &str, state: &str, pincode: &str, landmark: Option<&str>, is_primary: bool) -> Result<AddressResponse, AppError> {
        if is_primary {
            sqlx::query("UPDATE customer_addresses SET is_primary = false WHERE customer_id = $1")
                .bind(customer_id).execute(self.pool).await?;
        }
        let r = sqlx::query_as::<_, AddressResponse>(
            "INSERT INTO customer_addresses (customer_id, type, address_line1, address_line2, city, state, pincode, landmark, is_primary) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9) RETURNING id, customer_id, type as address_type, address_line1, address_line2, city, state, pincode, landmark, is_primary, created_at, updated_at",
        ).bind(customer_id).bind(address_type).bind(address_line1).bind(address_line2).bind(city).bind(state).bind(pincode).bind(landmark).bind(is_primary).fetch_one(self.pool).await?;
        Ok(r)
    }

    pub async fn update_address(&self, id: i64, customer_id: i64, address_type: Option<&str>, address_line1: Option<&str>, address_line2: Option<&str>, city: Option<&str>, state: Option<&str>, pincode: Option<&str>, landmark: Option<&str>, is_primary: Option<bool>) -> Result<AddressResponse, AppError> {
        if is_primary == Some(true) {
            sqlx::query("UPDATE customer_addresses SET is_primary = false WHERE customer_id = $1")
                .bind(customer_id).execute(self.pool).await?;
        }
        let r = sqlx::query_as::<_, AddressResponse>(
            "UPDATE customer_addresses SET type = COALESCE($3, type), address_line1 = COALESCE($4, address_line1), address_line2 = COALESCE($5, address_line2), city = COALESCE($6, city), state = COALESCE($7, state), pincode = COALESCE($8, pincode), landmark = COALESCE($9, landmark), is_primary = COALESCE($10, is_primary), updated_at = NOW() WHERE id = $1 AND customer_id = $2 RETURNING id, customer_id, type as address_type, address_line1, address_line2, city, state, pincode, landmark, is_primary, created_at, updated_at",
        ).bind(id).bind(customer_id).bind(address_type).bind(address_line1).bind(address_line2).bind(city).bind(state).bind(pincode).bind(landmark).bind(is_primary).fetch_one(self.pool).await?;
        Ok(r)
    }

    pub async fn delete_address(&self, id: i64, customer_id: i64) -> Result<(), AppError> {
        sqlx::query("DELETE FROM customer_addresses WHERE id = $1 AND customer_id = $2")
            .bind(id).bind(customer_id).execute(self.pool).await?;
        Ok(())
    }
}
