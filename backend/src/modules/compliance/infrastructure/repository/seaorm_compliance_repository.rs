//! SeaORM implementation of the compliance repository.

use async_trait::async_trait;
use sea_orm::DatabaseConnection;

use crate::common::errors::app_error::AppError;
use crate::modules::compliance::domain::aggregates::consent::consent::Consent;
use crate::modules::compliance::domain::aggregates::kyc_verification::kyc_verification::KycVerification;
use crate::modules::compliance::infrastructure::repository::compliance_repository_trait::ComplianceRepositoryTrait;

/// SeaORM implementation of the compliance repository.
///
/// NOTE: All methods are currently stubs. The `db` field will be used once
/// SeaORM entities are created for the compliance schema.
#[allow(dead_code)]
pub struct SeaOrmComplianceRepository {
    db: DatabaseConnection,
}

impl SeaOrmComplianceRepository {
    /// Create a new SeaORM compliance repository.
    pub fn new(db: &DatabaseConnection) -> Self {
        Self { db: db.clone() }
    }
}

#[async_trait]
impl ComplianceRepositoryTrait for SeaOrmComplianceRepository {
    async fn find_kyc_by_id(&self, _id: i64) -> Result<Option<KycVerification>, AppError> {
        // TODO: Implement with actual SeaORM entity
        Ok(None)
    }

    async fn find_kyc_by_customer_id(&self, _customer_id: i64) -> Result<Option<KycVerification>, AppError> {
        // TODO: Implement with actual SeaORM entity
        Ok(None)
    }

    async fn save_kyc(&self, _kyc: &mut KycVerification) -> Result<(), AppError> {
        // TODO: Implement with actual SeaORM entity
        Ok(())
    }

    async fn update_kyc(&self, _kyc: &KycVerification) -> Result<(), AppError> {
        // TODO: Implement with actual SeaORM entity
        Ok(())
    }

    async fn find_consent_by_id(&self, _id: i64) -> Result<Option<Consent>, AppError> {
        // TODO: Implement with actual SeaORM entity
        Ok(None)
    }

    async fn find_consents_by_customer_id(&self, _customer_id: i64) -> Result<Vec<Consent>, AppError> {
        // TODO: Implement with actual SeaORM entity
        Ok(vec![])
    }

    async fn save_consent(&self, _consent: &mut Consent) -> Result<(), AppError> {
        // TODO: Implement with actual SeaORM entity
        Ok(())
    }

    async fn update_consent(&self, _consent: &Consent) -> Result<(), AppError> {
        // TODO: Implement with actual SeaORM entity
        Ok(())
    }
}
