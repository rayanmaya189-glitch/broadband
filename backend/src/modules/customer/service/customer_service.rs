use sqlx::PgPool;

use crate::common::errors::app_error::AppError;
use crate::common::utils::helpers::PaginatedResponse;
use crate::modules::customer::mapper::customer_mapper::customer_to_response;
use crate::modules::customer::repository::customer_repository::CustomerRepository;
use crate::modules::customer::request::customer_request::*;
use crate::modules::customer::response::customer_response::*;

pub struct CustomerService<'a> {
    repo: CustomerRepository<'a>,
}

impl<'a> CustomerService<'a> {
    pub fn new(pool: &'a PgPool) -> Self { Self { repo: CustomerRepository::new(pool) } }

    pub async fn list_customers(&self, query: &ListCustomersQuery) -> Result<PaginatedResponse<CustomerResponse>, AppError> {
        let offset = query.pagination.offset();
        let limit = query.pagination.limit_i64() as u32;
        self.repo.list(offset, limit, query.status.as_deref(), query.branch_id, query.pagination.search.as_deref()).await
    }

    pub async fn get_customer(&self, id: i64) -> Result<CustomerDetailResponse, AppError> {
        let c = self.repo.find_by_id(id).await?.ok_or_else(|| AppError::NotFound("Customer not found".into()))?;
        Ok(customer_to_response(&c))
    }

    pub async fn create_customer(&self, req: &CreateCustomerRequest) -> Result<CustomerDetailResponse, AppError> {
        if self.repo.phone_exists(&req.phone, None).await? {
            return Err(AppError::Conflict("Phone number already registered".into()));
        }
        // Fetch branch code for customer code generation
        let branch_code = sqlx::query_scalar::<_, String>("SELECT code FROM branches WHERE id = $1")
            .bind(req.branch_id).fetch_optional(self.repo.pool()).await?
            .unwrap_or_else(|| "GEN".into());
        let customer_code = self.repo.generate_customer_code(&branch_code).await?;
        let c = self.repo.create(&customer_code, &req.first_name, req.last_name.as_deref(), req.email.as_deref(), &req.phone, req.alternate_phone.as_deref(), req.branch_id, req.lead_id, req.referred_by, req.created_by, req.notes.as_deref()).await?;
        Ok(customer_to_response(&c))
    }

    pub async fn update_customer(&self, id: i64, req: &UpdateCustomerRequest) -> Result<CustomerDetailResponse, AppError> {
        if self.repo.find_by_id(id).await?.is_none() {
            return Err(AppError::NotFound("Customer not found".into()));
        }
        if let Some(ref phone) = req.phone {
            if self.repo.phone_exists(phone, Some(id)).await? {
                return Err(AppError::Conflict("Phone number already registered".into()));
            }
        }
        let c = self.repo.update(id, req.first_name.as_deref(), req.last_name.as_deref(), req.email.as_deref(), req.phone.as_deref(), req.alternate_phone.as_deref(), req.notes.as_deref()).await?;
        Ok(customer_to_response(&c))
    }

    pub async fn update_status(&self, id: i64, req: &CustomerStatusTransition) -> Result<CustomerDetailResponse, AppError> {
        let valid = matches!(req.status.as_str(), "lead" | "prospect" | "active" | "suspended" | "deactivated" | "blacklist");
        if !valid { return Err(AppError::Validation("Invalid customer status".into())); }
        let c = self.repo.update_status(id, &req.status).await?;
        Ok(customer_to_response(&c))
    }

    pub async fn delete_customer(&self, id: i64) -> Result<MessageResponse, AppError> {
        if self.repo.find_by_id(id).await?.is_none() {
            return Err(AppError::NotFound("Customer not found".into()));
        }
        self.repo.soft_delete(id).await?;
        Ok(MessageResponse { message: "Customer deleted successfully".into() })
    }

    // ── Customer Profile (KYC) ──────────────────────────────

    pub async fn get_profile(&self, customer_id: i64) -> Result<CustomerProfileResponse, AppError> {
        self.repo.find_by_id(customer_id).await?.ok_or_else(|| AppError::NotFound("Customer not found".into()))?;
        let p = self.repo.get_profile(customer_id).await?;
        Ok(match p {
            Some(p) => CustomerProfileResponse {
                id: p.id, customer_id: p.customer_id, date_of_birth: p.date_of_birth, gender: p.gender,
                nationality: p.nationality, id_proof_type: p.id_proof_type, id_proof_number: p.id_proof_number,
                pan_number: p.pan_number, gstin: p.gstin, company_name: p.company_name,
                occupation: p.occupation, preferred_language: p.preferred_language,
                communication_opt_in: p.communication_opt_in, notes: p.notes,
                created_at: p.created_at, updated_at: p.updated_at,
            },
            None => return Err(AppError::NotFound("No profile found for this customer".into())),
        })
    }

    pub async fn update_profile(&self, customer_id: i64, req: &UpdateCustomerProfileRequest) -> Result<CustomerProfileResponse, AppError> {
        self.repo.find_by_id(customer_id).await?.ok_or_else(|| AppError::NotFound("Customer not found".into()))?;
        let p = self.repo.upsert_profile(
            customer_id, req.date_of_birth, req.gender.as_deref(), req.nationality.as_deref(),
            req.id_proof_type.as_deref(), req.id_proof_number.as_deref(),
            req.pan_number.as_deref(), req.aadhaar_number.as_deref(), req.gstin.as_deref(),
            req.company_name.as_deref(), req.designation.as_deref(), req.occupation.as_deref(),
            req.annual_income_range.as_deref(), req.preferred_language.as_deref(),
            req.communication_opt_in, req.notes.as_deref(),
        ).await?;
        Ok(CustomerProfileResponse {
            id: p.id, customer_id: p.customer_id, date_of_birth: p.date_of_birth, gender: p.gender,
            nationality: p.nationality, id_proof_type: p.id_proof_type, id_proof_number: p.id_proof_number,
            pan_number: p.pan_number, gstin: p.gstin, company_name: p.company_name,
            occupation: p.occupation, preferred_language: p.preferred_language,
            communication_opt_in: p.communication_opt_in, notes: p.notes,
            created_at: p.created_at, updated_at: p.updated_at,
        })
    }

    pub async fn submit_kyc(&self, customer_id: i64, req: &SubmitKycRequest) -> Result<MessageResponse, AppError> {
        self.repo.find_by_id(customer_id).await?.ok_or_else(|| AppError::NotFound("Customer not found".into()))?;
        self.repo.submit_kyc(customer_id, &req.id_proof_type, &req.id_proof_number).await?;
        Ok(MessageResponse { message: "KYC submitted for verification".into() })
    }

    pub async fn verify_kyc(&self, customer_id: i64, req: &VerifyKycRequest) -> Result<MessageResponse, AppError> {
        self.repo.find_by_id(customer_id).await?.ok_or_else(|| AppError::NotFound("Customer not found".into()))?;
        if !matches!(req.status.as_str(), "verified" | "rejected") {
            return Err(AppError::Validation("Status must be 'verified' or 'rejected'".into()));
        }
        self.repo.verify_kyc(customer_id, &req.status, req.rejection_reason.as_deref(), None).await?;
        Ok(MessageResponse { message: format!("KYC {}", req.status) })
    }

    // ── KYC Documents ───────────────────────────────────────

    pub async fn list_kyc_documents(&self, customer_id: i64) -> Result<Vec<KycDocumentResponse>, AppError> {
        self.repo.find_by_id(customer_id).await?.ok_or_else(|| AppError::NotFound("Customer not found".into()))?;
        let docs = self.repo.list_kyc_documents(customer_id).await?;
        Ok(docs.into_iter().map(|d| KycDocumentResponse {
            id: d.id, customer_id: d.customer_id, document_type: d.document_type,
            document_url: d.document_url, file_name: d.file_name,
            verification_status: d.verification_status, rejection_reason: d.rejection_reason,
            uploaded_at: d.uploaded_at, created_at: d.created_at,
        }).collect())
    }

    pub async fn upload_kyc_document(&self, customer_id: i64, document_type: &str, document_url: &str, file_name: Option<&str>, file_size: Option<i64>, mime_type: Option<&str>) -> Result<KycDocumentResponse, AppError> {
        self.repo.find_by_id(customer_id).await?.ok_or_else(|| AppError::NotFound("Customer not found".into()))?;
        let d = self.repo.create_kyc_document(customer_id, document_type, document_url, file_name, file_size, mime_type).await?;
        Ok(KycDocumentResponse {
            id: d.id, customer_id: d.customer_id, document_type: d.document_type,
            document_url: d.document_url, file_name: d.file_name,
            verification_status: d.verification_status, rejection_reason: d.rejection_reason,
            uploaded_at: d.uploaded_at, created_at: d.created_at,
        })
    }

    pub async fn delete_kyc_document(&self, customer_id: i64, doc_id: i64) -> Result<MessageResponse, AppError> {
        self.repo.find_by_id(customer_id).await?.ok_or_else(|| AppError::NotFound("Customer not found".into()))?;
        self.repo.delete_kyc_document(doc_id, customer_id).await?;
        Ok(MessageResponse { message: "Document deleted".into() })
    }

    // ── Customer Addresses ──────────────────────────────────

    pub async fn list_addresses(&self, customer_id: i64) -> Result<Vec<AddressResponse>, AppError> {
        self.repo.find_by_id(customer_id).await?.ok_or_else(|| AppError::NotFound("Customer not found".into()))?;
        self.repo.list_addresses(customer_id).await
    }

    pub async fn create_address(&self, customer_id: i64, req: &CreateAddressRequest) -> Result<AddressResponse, AppError> {
        self.repo.find_by_id(customer_id).await?.ok_or_else(|| AppError::NotFound("Customer not found".into()))?;
        self.repo.create_address(customer_id, &req.address_type, &req.address_line1, req.address_line2.as_deref(), &req.city, &req.state, &req.pincode, req.landmark.as_deref(), req.is_primary.unwrap_or(true)).await
    }

    pub async fn update_address(&self, customer_id: i64, address_id: i64, req: &UpdateAddressRequest) -> Result<AddressResponse, AppError> {
        self.repo.find_by_id(customer_id).await?.ok_or_else(|| AppError::NotFound("Customer not found".into()))?;
        self.repo.update_address(address_id, customer_id, req.address_type.as_deref(), req.address_line1.as_deref(), req.address_line2.as_deref(), req.city.as_deref(), req.state.as_deref(), req.pincode.as_deref(), req.landmark.as_deref(), req.is_primary).await
    }

    pub async fn delete_address(&self, customer_id: i64, address_id: i64) -> Result<MessageResponse, AppError> {
        self.repo.find_by_id(customer_id).await?.ok_or_else(|| AppError::NotFound("Customer not found".into()))?;
        self.repo.delete_address(address_id, customer_id).await?;
        Ok(MessageResponse { message: "Address deleted".into() })
    }
}
