//! Customer repository trait.
//!
//! This trait defines the contract for customer data access.
//! The domain layer depends on this trait, not on the implementation.

use async_trait::async_trait;

use crate::common::errors::app_error::AppError;
use crate::modules::customer::domain::aggregates::customer::customer::Customer;

/// Repository trait for the Customer aggregate.
///
/// This trait defines the contract for persisting and retrieving customers.
/// The domain layer depends on this trait, not on the implementation.
/// Implementations can use SeaORM, in-memory stores, or any other data source.
#[async_trait]
pub trait CustomerRepositoryTrait: Send + Sync {
    /// Find a customer by ID.
    async fn find_by_id(&self, id: i64) -> Result<Option<Customer>, AppError>;

    /// Find a customer by code.
    async fn find_by_code(&self, code: &str) -> Result<Option<Customer>, AppError>;

    /// Save a new customer.
    ///
    /// The repository is responsible for:
    /// 1. Generating the ID (auto-increment)
    /// 2. Setting the ID on the customer via `customer.set_id(id)`
    async fn save(&self, customer: &mut Customer) -> Result<(), AppError>;

    /// Update an existing customer.
    async fn update(&self, customer: &Customer) -> Result<(), AppError>;

    /// Check if a phone number exists.
    async fn phone_exists(&self, phone: &str, exclude_id: Option<i64>) -> Result<bool, AppError>;

    /// Generate a unique customer code.
    async fn generate_customer_code(&self, branch_code: &str) -> Result<String, AppError>;

    /// List customers with pagination.
    async fn list(
        &self,
        offset: u32,
        limit: u32,
        status: Option<&str>,
        branch_id: Option<i64>,
        search: Option<&str>,
    ) -> Result<Vec<Customer>, AppError>;

    /// Count customers with filters.
    async fn count(
        &self,
        status: Option<&str>,
        branch_id: Option<i64>,
        search: Option<&str>,
    ) -> Result<i64, AppError>;
}
