use sqlx::PgPool;

use crate::common::errors::app_error::AppError;
use crate::common::utils::helpers::{total_pages, PaginatedResponse};
use crate::modules::customer::model::customer::Customer;
use crate::modules::customer::response::customer_response::CustomerResponse;

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
}
