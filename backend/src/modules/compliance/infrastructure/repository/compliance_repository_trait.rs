//! Compliance repository trait.
//!
//! Defines the contract for compliance data access.

use async_trait::async_trait;

use crate::common::errors::app_error::AppError;
use crate::modules::compliance::domain::aggregates::consent::consent::Consent;
use crate::modules::compliance::domain::aggregates::kyc_verification::kyc_verification::KycVerification;

/// Repository trait for compliance operations.
#[async_trait]
pub trait ComplianceRepositoryTrait: Send + Sync {
    /// Find KYC verification by ID.
    async fn find_kyc_by_id(&self, id: i64) -> Result<Option<KycVerification>, AppError>;

    /// Find KYC verification by customer ID.
    async fn find_kyc_by_customer_id(&self, customer_id: i64) -> Result<Option<KycVerification>, AppError>;

    /// Save a KYC verification.
    async fn save_kyc(&self, kyc: &mut KycVerification) -> Result<(), AppError>;

    /// Update a KYC verification.
    async fn update_kyc(&self, kyc: &KycVerification) -> Result<(), AppError>;

    /// Find consent by ID.
    async fn find_consent_by_id(&self, id: i64) -> Result<Option<Consent>, AppError>;

    /// Find all consents for a customer.
    async fn find_consents_by_customer_id(&self, customer_id: i64) -> Result<Vec<Consent>, AppError>;

    /// Save a consent record.
    async fn save_consent(&self, consent: &mut Consent) -> Result<(), AppError>;

    /// Update a consent record.
    async fn update_consent(&self, consent: &Consent) -> Result<(), AppError>;
}
