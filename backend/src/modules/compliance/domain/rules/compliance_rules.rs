/// Compliance business rules and invariants
pub struct ComplianceRules;

impl ComplianceRules {
    /// Valid KYC document types
    pub const VALID_DOCUMENT_TYPES: &[&str] =
        &["aadhaar", "pan", "voter_id", "driving_license", "passport"];

    /// KYC document retention period (7 years)
    pub const KYC_RETENTION_YEARS: i32 = 7;

    /// GDPR consent validity (1 year)
    pub const CONSENT_VALIDITY_YEARS: i32 = 1;

    /// Check if document type is valid
    pub fn is_valid_document_type(doc_type: &str) -> bool {
        Self::VALID_DOCUMENT_TYPES.contains(&doc_type)
    }

    /// Check if KYC is required for activation
    pub fn is_kyc_required_for_activation() -> bool {
        true
    }
}
