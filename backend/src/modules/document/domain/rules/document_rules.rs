/// Document business rules and invariants
pub struct DocumentRules;

impl DocumentRules {
    /// Maximum file size per type (in bytes)
    pub const MAX_FILE_SIZES: &[(&str, i64)] = &[
        ("image/jpeg", 10 * 1024 * 1024),
        ("image/png", 10 * 1024 * 1024),
        ("application/pdf", 5 * 1024 * 1024),
    ];

    /// KYC document retention (7 years)
    pub const KYC_RETENTION_DAYS: i64 = 2555;

    /// General document retention (1 year)
    pub const GENERAL_RETENTION_DAYS: i64 = 365;

    /// Allowed MIME types for KYC
    pub const KYC_ALLOWED_TYPES: &[&str] = &["image/jpeg", "image/png", "application/pdf"];

    /// Check if MIME type is allowed for KYC
    pub fn is_kyc_type_allowed(mime_type: &str) -> bool {
        Self::KYC_ALLOWED_TYPES.contains(&mime_type)
    }

    /// Get max file size for MIME type
    pub fn max_file_size(mime_type: &str) -> i64 {
        Self::MAX_FILE_SIZES.iter()
            .find(|(t, _)| *t == mime_type)
            .map(|(_, s)| *s)
            .unwrap_or(5 * 1024 * 1024)
    }

    /// Generate storage key
    pub fn generate_storage_key(entity_type: &str, entity_id: i64, filename: &str) -> String {
        let ext = filename.rsplit('.').next().unwrap_or("bin");
        format!("{}/{}/{}/{}", entity_type, entity_id, chrono::Utc::now().format("%Y/%m/%d"), uuid::Uuid::new_v4().to_string() + "." + ext)
    }
}
