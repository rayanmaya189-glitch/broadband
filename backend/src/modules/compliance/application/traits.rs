use async_trait::async_trait;
use sea_orm::{DatabaseConnection};
use crate::shared::errors::AppError;

pub type KycVerificationModel = crate::modules::compliance::domain::entities::kyc_verification::Model;
pub type ConsentModel = crate::modules::compliance::domain::entities::consent::Model;
pub type DataRetentionPolicyModel = crate::modules::compliance::domain::entities::data_retention_policy::Model;

#[async_trait]
pub trait ComplianceServiceTrait: Send + Sync {
    async fn list_kyc_verifications(
        &self,
        db: &DatabaseConnection,
    ) -> Result<Vec<KycVerificationModel>, AppError>;

    async fn create_kyc_verification(
        &self,
        db: &DatabaseConnection,
        customer_id: i64,
        document_type: String,
        document_number: String,
    ) -> Result<KycVerificationModel, AppError>;

    async fn update_kyc_status(
        &self,
        db: &DatabaseConnection,
        id: i64,
        status: &str,
    ) -> Result<KycVerificationModel, AppError>;

    async fn grant_consent(
        &self,
        db: &DatabaseConnection,
        customer_id: i64,
        consent_type: String,
    ) -> Result<ConsentModel, AppError>;

    async fn revoke_consent(
        &self,
        db: &DatabaseConnection,
        customer_id: i64,
        consent_type: &str,
    ) -> Result<(), AppError>;

    async fn has_consent(
        &self,
        db: &DatabaseConnection,
        customer_id: i64,
        consent_type: &str,
    ) -> Result<bool, AppError>;
}
