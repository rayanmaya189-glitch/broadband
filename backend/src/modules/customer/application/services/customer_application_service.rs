//! Customer application service.
//!
//! Orchestrates domain operations, repository access, and event publishing.
//! This service coordinates between the domain layer and infrastructure.

use crate::common::errors::app_error::AppError;
use crate::common::shared::events::EventEnvelope;
use crate::modules::customer::application::commands::{
    create_customer::{CreateCustomerCommand, CreateCustomerHandler},
    submit_kyc::{SubmitKycCommand, SubmitKycHandler},
    transition_customer_status::{
        TransitionCustomerStatusCommand, TransitionCustomerStatusHandler,
    },
    update_customer::{UpdateCustomerCommand, UpdateCustomerHandler},
    verify_kyc::{VerifyKycCommand, VerifyKycHandler},
};
use crate::modules::customer::domain::aggregates::customer::customer::Customer;
use crate::modules::customer::infrastructure::messaging::customer_event_publisher::CustomerEventPublisher;
use crate::modules::customer::infrastructure::repository::CustomerRepositoryTrait;

/// Application service for customer operations.
pub struct CustomerApplicationService<R: CustomerRepositoryTrait> {
    repository: R,
    event_publisher: CustomerEventPublisher,
}

impl<R: CustomerRepositoryTrait> CustomerApplicationService<R> {
    /// Create a new customer application service.
    pub fn new(repository: R, event_publisher: CustomerEventPublisher) -> Self {
        Self {
            repository,
            event_publisher,
        }
    }

    /// Create a new customer.
    pub async fn create_customer(
        &self,
        mut command: CreateCustomerCommand,
    ) -> Result<Customer, AppError> {
        // Check phone uniqueness
        if self.repository.phone_exists(&command.phone, None).await? {
            return Err(AppError::Conflict("Phone number already registered".to_string()));
        }

        // Generate customer code
        command.customer_code = self.repository.generate_customer_code("GEN").await?;

        // Execute command
        let result = CreateCustomerHandler::handle(command)?;

        // Persist (repository will set the ID)
        let mut customer = result.customer;
        self.repository.save(&mut customer).await?;

        // Publish event
        let _ = self.event_publisher.publish(result.event).await;

        Ok(customer)
    }

    /// Get a customer by ID.
    pub async fn get_customer(&self, id: i64) -> Result<Customer, AppError> {
        self.repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound("Customer not found".to_string()))
    }

    /// Update a customer.
    pub async fn update_customer(
        &self,
        command: UpdateCustomerCommand,
    ) -> Result<Customer, AppError> {
        let customer = self.repository.find_by_id(command.customer_id).await?
            .ok_or_else(|| AppError::NotFound("Customer not found".to_string()))?;

        let result = UpdateCustomerHandler::handle(customer, command)?;

        // Persist
        self.repository.update(&result.customer).await?;

        Ok(result.customer)
    }

    /// Transition customer status.
    pub async fn transition_status(
        &self,
        command: TransitionCustomerStatusCommand,
    ) -> Result<Customer, AppError> {
        let customer = self.repository.find_by_id(command.customer_id).await?
            .ok_or_else(|| AppError::NotFound("Customer not found".to_string()))?;

        // Check for active subscriptions (simplified)
        let has_active_subscriptions = false;

        let result = TransitionCustomerStatusHandler::handle(
            customer,
            command,
            has_active_subscriptions,
        )?;

        // Persist
        self.repository.update(&result.customer).await?;

        // Publish event
        let event = EventEnvelope::new(
            "customer.status_changed.v1".to_string(),
            1,
            "customer-service".to_string(),
            result.event,
        );
        let _ = self.event_publisher.publish(event).await;

        Ok(result.customer)
    }

    /// Submit KYC.
    pub async fn submit_kyc(
        &self,
        command: SubmitKycCommand,
    ) -> Result<(), AppError> {
        let customer = self.repository.find_by_id(command.customer_id).await?
            .ok_or_else(|| AppError::NotFound("Customer not found".to_string()))?;

        let event = SubmitKycHandler::handle(customer, command)?;

        // Publish event
        let envelope = EventEnvelope::new(
            "customer.kyc_submitted.v1".to_string(),
            1,
            "customer-service".to_string(),
            event,
        );
        let _ = self.event_publisher.publish(envelope).await;

        Ok(())
    }

    /// Verify KYC.
    pub async fn verify_kyc(
        &self,
        command: VerifyKycCommand,
    ) -> Result<(), AppError> {
        let customer = self.repository.find_by_id(command.customer_id).await?
            .ok_or_else(|| AppError::NotFound("Customer not found".to_string()))?;

        let event = VerifyKycHandler::handle(customer, command)?;

        // Publish event
        let envelope = EventEnvelope::new(
            "customer.kyc_verified.v1".to_string(),
            1,
            "customer-service".to_string(),
            event,
        );
        let _ = self.event_publisher.publish(envelope).await;

        Ok(())
    }

    /// List customers with pagination.
    pub async fn list_customers(
        &self,
        offset: u32,
        limit: u32,
        status: Option<&str>,
        branch_id: Option<i64>,
        search: Option<&str>,
    ) -> Result<(Vec<Customer>, i64), AppError> {
        let customers = self.repository.list(offset, limit, status, branch_id, search).await?;
        let total = self.repository.count(status, branch_id, search).await?;
        Ok((customers, total))
    }
}
