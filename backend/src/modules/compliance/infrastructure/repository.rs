use crate::modules::compliance::domain::entities::{Consent, DataRetentionPolicy, KycVerification};
use crate::shared::errors::AppError;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

/// Compliance repository for database queries.
pub struct ComplianceRepository;

impl ComplianceRepository {
    /// Find all KYC verifications for a customer.
    pub async fn find_kyc_by_customer(
        db: &DatabaseConnection,
        customer_id: i64,
    ) -> Result<Vec<kyc_verification::Model>, AppError> {
        Ok(KycVerification::find()
            .filter(kyc_verification::Column::CustomerId.eq(customer_id))
            .all(db)
            .await?)
    }

    /// Find pending KYC verifications.
    pub async fn find_pending_kyc(
        db: &DatabaseConnection,
    ) -> Result<Vec<kyc_verification::Model>, AppError> {
        Ok(KycVerification::find()
            .filter(kyc_verification::Column::Status.eq("pending"))
            .all(db)
            .await?)
    }

    /// Find active consents for a customer.
    pub async fn find_active_consents(
        db: &DatabaseConnection,
        customer_id: i64,
    ) -> Result<Vec<consent::Model>, AppError> {
        Ok(Consent::find()
            .filter(consent::Column::CustomerId.eq(customer_id))
            .filter(consent::Column::Granted.eq(true))
            .all(db)
            .await?)
    }

    /// Find active retention policies.
    pub async fn find_active_policies(
        db: &DatabaseConnection,
    ) -> Result<Vec<data_retention_policy::Model>, AppError> {
        Ok(DataRetentionPolicy::find()
            .filter(data_retention_policy::Column::IsActive.eq(true))
            .all(db)
            .await?)
    }

    /// Find retention policy by entity type.
    pub async fn find_policy_by_entity_type(
        db: &DatabaseConnection,
        entity_type: &str,
    ) -> Result<Option<data_retention_policy::Model>, AppError> {
        Ok(DataRetentionPolicy::find()
            .filter(data_retention_policy::Column::EntityType.eq(entity_type))
            .filter(data_retention_policy::Column::IsActive.eq(true))
            .one(db)
            .await?)
    }
}

use crate::modules::compliance::domain::entities::{
    consent, data_retention_policy, kyc_verification,
};
