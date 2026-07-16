/// Domain-level unit tests for PII protection utilities.
/// Tests ensure correctness of hashing and masking functions.

#[cfg(test)]
mod pii_tests {
    use crate::shared::utils::pii::*;

    // ──────────────────────────────────────────────
    // Hash Tests
    // ──────────────────────────────────────────────

    #[test]
    fn test_hash_aadhaar_deterministic() {
        let h1 = hash_aadhaar("123456789012");
        let h2 = hash_aadhaar("123456789012");
        assert_eq!(h1, h2);
        assert_eq!(h1.len(), 64, "SHA-256 hex output should be 64 chars");
    }

    #[test]
    fn test_hash_aadhaar_different_inputs() {
        let h1 = hash_aadhaar("123456789012");
        let h2 = hash_aadhaar("987654321098");
        assert_ne!(h1, h2);
    }

    #[test]
    fn test_hash_pan_deterministic() {
        let h1 = hash_pan("ABCDE1234F");
        let h2 = hash_pan("ABCDE1234F");
        assert_eq!(h1, h2);
    }

    #[test]
    fn test_hash_pan_different_inputs() {
        let h1 = hash_pan("ABCDE1234F");
        let h2 = hash_pan("FGHIJ5678K");
        assert_ne!(h1, h2);
    }

    #[test]
    fn test_hash_pii_flexible() {
        let h1 = hash_pii("custom", "value1");
        let h2 = hash_pii("custom", "value2");
        assert_ne!(h1, h2);
        assert!(h1.starts_with("") == false); // Just ensure it doesn't panic
    }

    // ──────────────────────────────────────────────
    // Mask Tests
    // ──────────────────────────────────────────────

    #[test]
    fn test_mask_phone_indian() {
        let masked = mask_phone("+919876543210");
        assert!(masked.starts_with("+91"), "Should preserve country code prefix");
        assert!(masked.ends_with("3210"), "Should preserve last 4 digits");
        assert!(masked.contains('*'), "Should contain masking characters");
        assert_eq!(masked.len(), 13, "Should maintain length");
    }

    #[test]
    fn test_mask_phone_short() {
        let short = mask_phone("12345");
        assert_eq!(short, "12345", "Short phone should not be masked");
    }

    #[test]
    fn test_mask_email() {
        let masked = mask_email("rahul@example.com");
        assert_eq!(masked, "r****l@example.com");
    }

    #[test]
    fn test_mask_email_short_user() {
        let masked = mask_email("ab@example.com");
        assert_eq!(masked, "ab@example.com", "Short username should not be masked");
    }

    #[test]
    fn test_mask_aadhaar() {
        let masked = mask_aadhaar("123456789012");
        assert_eq!(masked, "XXXX-XXXX-9012");
    }

    #[test]
    fn test_mask_pan() {
        let masked = mask_pan("ABCDE1234F");
        assert_eq!(masked, "XXXXX234F");
    }

    #[test]
    fn test_mask_edge_cases() {
        assert_eq!(mask_aadhaar(""), "XXXX");
        assert_eq!(mask_pan(""), "XXXXX");
        assert_eq!(mask_email("invalid"), "invalid");
    }
}
