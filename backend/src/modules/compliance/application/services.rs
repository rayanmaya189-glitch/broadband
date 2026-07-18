use crate::modules::compliance::domain::entities::{
    consent, data_retention_policy, kyc_verification, Consent, ConsentActiveModel,
    DataRetentionPolicy, DataRetentionPolicyActiveModel, KycVerification,
    KycVerificationActiveModel,
};
use crate::shared::errors::AppError;
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    Set,
};

/// Compliance service for KYC verification, GDPR consent, and data retention.
pub struct ComplianceService;

impl ComplianceService {
    // ── KYC Verifications ──

    pub async fn list_kyc_verifications(
        db: &DatabaseConnection,
    ) -> Result<Vec<kyc_verification::Model>, AppError> {
        Ok(KycVerification::find().all(db).await?)
    }

    pub async fn create_kyc_verification(
        db: &DatabaseConnection,
        customer_id: i64,
        document_type: String,
        document_number_hash: String,
        provider: Option<String>,
    ) -> Result<kyc_verification::Model, AppError> {
        let now = Utc::now();
        let kyc = KycVerificationActiveModel {
            customer_id: Set(customer_id),
            document_type: Set(document_type),
            document_number_hash: Set(document_number_hash),
            status: Set("pending".to_string()),
            provider: Set(provider),
            provider_reference: Set(None),
            rejection_reason: Set(None),
            verified_at: Set(None),
            expires_at: Set(None),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };
        Ok(kyc.insert(db).await?)
    }

    pub async fn update_kyc_status(
        db: &DatabaseConnection,
        id: i64,
        status: String,
        rejection_reason: Option<String>,
        provider_reference: Option<String>,
    ) -> Result<kyc_verification::Model, AppError> {
        let kyc = KycVerification::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("KYC verification {} not found", id)))?;
        let mut active: kyc_verification::ActiveModel = kyc.into();
        active.status = Set(status.clone());
        active.rejection_reason = Set(rejection_reason);
        active.provider_reference = Set(provider_reference);
        active.updated_at = Set(Utc::now());
        if status == "verified" {
            active.verified_at = Set(Some(Utc::now()));
            // KYC valid for 1 year
            active.expires_at = Set(Some(Utc::now() + chrono::Duration::days(365)));
        }
        Ok(active.update(db).await?)
    }

    pub async fn get_kyc_by_customer(
        db: &DatabaseConnection,
        customer_id: i64,
    ) -> Result<Vec<kyc_verification::Model>, AppError> {
        Ok(KycVerification::find()
            .filter(kyc_verification::Column::CustomerId.eq(customer_id))
            .all(db)
            .await?)
    }

    pub async fn is_customer_kyc_verified(
        db: &DatabaseConnection,
        customer_id: i64,
    ) -> Result<bool, AppError> {
        let count = KycVerification::find()
            .filter(kyc_verification::Column::CustomerId.eq(customer_id))
            .filter(kyc_verification::Column::Status.eq("verified"))
            .filter(kyc_verification::Column::VerifiedAt.is_not_null())
            .count(db)
            .await? as i64;
        Ok(count > 0)
    }

    // ── Consent Management ──

    pub async fn list_consents(
        db: &DatabaseConnection,
        customer_id: i64,
    ) -> Result<Vec<consent::Model>, AppError> {
        Ok(Consent::find()
            .filter(consent::Column::CustomerId.eq(customer_id))
            .all(db)
            .await?)
    }

    pub async fn grant_consent(
        db: &DatabaseConnection,
        customer_id: i64,
        consent_type: String,
        collection_channel: String,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<consent::Model, AppError> {
        let now = Utc::now();
        let c = ConsentActiveModel {
            customer_id: Set(customer_id),
            consent_type: Set(consent_type),
            granted: Set(true),
            collection_channel: Set(collection_channel),
            ip_address: Set(ip_address),
            user_agent: Set(user_agent),
            expires_at: Set(None),
            revoked_at: Set(None),
            created_at: Set(now),
            ..Default::default()
        };
        Ok(c.insert(db).await?)
    }

    pub async fn revoke_consent(
        db: &DatabaseConnection,
        customer_id: i64,
        consent_type: String,
    ) -> Result<(), AppError> {
        let consent = Consent::find()
            .filter(consent::Column::CustomerId.eq(customer_id))
            .filter(consent::Column::ConsentType.eq(consent_type))
            .filter(consent::Column::Granted.eq(true))
            .one(db)
            .await?;

        if let Some(c) = consent {
            let mut active: consent::ActiveModel = c.into();
            active.granted = Set(false);
            active.revoked_at = Set(Some(Utc::now()));
            active.update(db).await?;
        }
        Ok(())
    }

    pub async fn has_consent(
        db: &DatabaseConnection,
        customer_id: i64,
        consent_type: &str,
    ) -> Result<bool, AppError> {
        let consent = Consent::find()
            .filter(consent::Column::CustomerId.eq(customer_id))
            .filter(consent::Column::ConsentType.eq(consent_type))
            .filter(consent::Column::Granted.eq(true))
            .one(db)
            .await?;
        Ok(consent.is_some())
    }

    // ── Data Retention Policies ──

    pub async fn list_retention_policies(
        db: &DatabaseConnection,
    ) -> Result<Vec<data_retention_policy::Model>, AppError> {
        Ok(DataRetentionPolicy::find().all(db).await?)
    }

    pub async fn create_retention_policy(
        db: &DatabaseConnection,
        entity_type: String,
        retention_days: i32,
        action: String,
        description: Option<String>,
        legal_basis: Option<String>,
    ) -> Result<data_retention_policy::Model, AppError> {
        let now = Utc::now();
        let policy = DataRetentionPolicyActiveModel {
            entity_type: Set(entity_type),
            retention_days: Set(retention_days),
            action: Set(action),
            is_active: Set(true),
            description: Set(description),
            legal_basis: Set(legal_basis),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };
        Ok(policy.insert(db).await?)
    }

    pub async fn update_retention_policy(
        db: &DatabaseConnection,
        id: i64,
        retention_days: Option<i32>,
        action: Option<String>,
        is_active: Option<bool>,
    ) -> Result<data_retention_policy::Model, AppError> {
        let policy = DataRetentionPolicy::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Retention policy {} not found", id)))?;
        let mut active: data_retention_policy::ActiveModel = policy.into();
        if let Some(days) = retention_days {
            active.retention_days = Set(days);
        }
        if let Some(act) = action {
            active.action = Set(act);
        }
        if let Some(a) = is_active {
            active.is_active = Set(a);
        }
        active.updated_at = Set(Utc::now());
        Ok(active.update(db).await?)
    }
}
