//! KYC provider adapter.
//!
//! Adapter for integrating with external KYC verification providers.

use serde::{Deserialize, Serialize};

/// KYC verification result from external provider.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KycVerificationResult {
    pub verified: bool,
    pub provider: String,
    pub reference_id: Option<String>,
    pub confidence_score: Option<f64>,
    pub verification_date: Option<chrono::DateTime<chrono::Utc>>,
    pub raw_response: Option<String>,
}

/// KYC provider adapter trait.
#[async_trait::async_trait]
pub trait KycProviderAdapter: Send + Sync {
    /// Verify identity with external provider.
    async fn verify_identity(
        &self,
        id_proof_type: &str,
        id_proof_number: &str,
        name: &str,
    ) -> Result<KycVerificationResult, KycProviderError>;

    /// Check verification status.
    async fn check_status(
        &self,
        reference_id: &str,
    ) -> Result<KycVerificationResult, KycProviderError>;
}

/// KYC provider errors.
#[derive(Debug, Clone)]
pub enum KycProviderError {
    ProviderUnavailable(String),
    InvalidInput(String),
    VerificationFailed(String),
    NetworkError(String),
}

impl std::fmt::Display for KycProviderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KycProviderError::ProviderUnavailable(msg) => write!(f, "KYC provider unavailable: {}", msg),
            KycProviderError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            KycProviderError::VerificationFailed(msg) => write!(f, "Verification failed: {}", msg),
            KycProviderError::NetworkError(msg) => write!(f, "Network error: {}", msg),
        }
    }
}

impl std::error::Error for KycProviderError {}

/// Demo KYC provider adapter for testing.
pub struct DemoKycProviderAdapter;

#[async_trait::async_trait]
impl KycProviderAdapter for DemoKycProviderAdapter {
    async fn verify_identity(
        &self,
        id_proof_type: &str,
        id_proof_number: &str,
        _name: &str,
    ) -> Result<KycVerificationResult, KycProviderError> {
        // Demo implementation - always succeeds
        Ok(KycVerificationResult {
            verified: true,
            provider: "demo".to_string(),
            reference_id: Some(format!("REF-{}-{}", id_proof_type, id_proof_number)),
            confidence_score: Some(0.95),
            verification_date: Some(chrono::Utc::now()),
            raw_response: None,
        })
    }

    async fn check_status(
        &self,
        reference_id: &str,
    ) -> Result<KycVerificationResult, KycProviderError> {
        // Demo implementation - always verified
        Ok(KycVerificationResult {
            verified: true,
            provider: "demo".to_string(),
            reference_id: Some(reference_id.to_string()),
            confidence_score: Some(0.95),
            verification_date: Some(chrono::Utc::now()),
            raw_response: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_demo_kyc_provider() {
        let adapter = DemoKycProviderAdapter;
        let result = adapter
            .verify_identity("aadhaar", "1234-5678-9012", "John Doe")
            .await
            .unwrap();

        assert!(result.verified);
        assert_eq!(result.provider, "demo");
    }
}
