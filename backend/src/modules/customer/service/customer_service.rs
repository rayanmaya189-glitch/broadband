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
}
