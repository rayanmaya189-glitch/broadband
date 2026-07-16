/// Domain-level unit tests for 2FA/TOTP implementation.
/// Tests ensure correctness of TOTP generation, verification, and backup codes.

#[cfg(test)]
mod totp_tests {
    use crate::modules::identity::application::two_factor::*;

    // ──────────────────────────────────────────────
    // Base32 Tests
    // ──────────────────────────────────────────────

    #[test]
    fn test_base32_roundtrip() {
        let original = b"Hello, World!";
        let encoded = base32_encode(original);
        let decoded = base32_decode(&encoded).unwrap();
        assert_eq!(original.to_vec(), decoded);
    }

    #[test]
    fn test_base32_empty() {
        let encoded = base32_encode(b"");
        let decoded = base32_decode(&encoded).unwrap();
        assert!(decoded.is_empty());
    }

    #[test]
    fn test_base32_invalid() {
        let result = base32_decode("INVALID1!@#");
        assert!(result.is_err());
    }

    // ──────────────────────────────────────────────
    // TOTP Code Generation Tests
    // ──────────────────────────────────────────────

    #[test]
    fn test_generate_totp_code_deterministic() {
        let secret = b"JBSWY3DPEHPK3PXP";
        let step1 = generate_totp_code(secret, 1234567890 / TOTP_PERIOD as i64);
        let step2 = generate_totp_code(secret, 1234567890 / TOTP_PERIOD as i64);
        assert_eq!(step1, step2, "Same step should produce same code");
    }

    #[test]
    fn test_generate_totp_code_different_steps() {
        let secret = b"JBSWY3DPEHPK3PXP";
        let code1 = generate_totp_code(secret, 1000);
        let code2 = generate_totp_code(secret, 1001);
        // Very unlikely to be equal, but possible
        // This test mainly ensures no panic
        assert!(code1.len() == TOTP_DIGITS);
        assert!(code2.len() == TOTP_DIGITS);
    }

    // ──────────────────────────────────────────────
    // TOTP Verification Tests
    // ──────────────────────────────────────────────

    #[test]
    fn test_verify_totp_current_step() {
        let secret = "JBSWY3DPEHPK3PXP";
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let current_step = now / TOTP_PERIOD as u64;
        let code = generate_totp_code(secret.as_bytes(), current_step as i64);
        assert!(verify_totp(secret, &code), "Should verify current step code");
    }

    #[test]
    fn test_verify_totp_invalid_code() {
        let secret = "JBSWY3DPEHPK3PXP";
        assert!(!verify_totp(secret, "000000"), "Should reject invalid code");
    }

    #[test]
    fn test_verify_totp_wrong_length() {
        let secret = "JBSWY3DPEHPK3PXP";
        assert!(!verify_totp(secret, "12345"), "Should reject code with wrong length");
        assert!(!verify_totp(secret, "1234567"), "Should reject code with wrong length");
    }

    // ──────────────────────────────────────────────
    // Backup Code Tests
    // ──────────────────────────────────────────────

    #[test]
    fn test_hash_backup_code_deterministic() {
        let code = "abc123def456";
        let hash1 = hash_backup_code(code);
        let hash2 = hash_backup_code(code);
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_verify_backup_code_valid() {
        let code = "test-backup-code-123";
        let hash = hash_backup_code(code);
        let hashes = vec![hash];
        let (valid, remaining) = verify_backup_code(code, &hashes).unwrap();
        assert!(valid);
        assert!(remaining.is_empty(), "Used code should be removed");
    }

    #[test]
    fn test_verify_backup_code_invalid() {
        let hash = hash_backup_code("correct-code");
        let hashes = vec![hash];
        let (valid, remaining) = verify_backup_code("wrong-code", &hashes).unwrap();
        assert!(!valid);
        assert_eq!(remaining.len(), 1, "Invalid attempt should not remove hash");
    }

    // ──────────────────────────────────────────────
    // Setup Tests
    // ──────────────────────────────────────────────

    #[test]
    fn test_setup_two_factor() {
        let result = setup_two_factor("test@example.com");
        assert!(result.is_ok());

        let setup = result.unwrap();
        assert!(!setup.secret_base32.is_empty());
        assert!(!setup.otpauth_uri.is_empty());
        assert_eq!(setup.backup_codes.len(), 10);
        assert_eq!(setup.backup_code_hashes.len(), 10);

        // Verify otpauth URI format
        assert!(setup.otpauth_uri.starts_with("otpauth://totp/"));
        assert!(setup.otpauth_uri.contains("AeroXe"));
        assert!(setup.otpauth_uri.contains("test@example.com"));
    }

    #[test]
    fn test_otpauth_uri_format() {
        let result = setup_two_factor("user@aeroxe.com");
        let setup = result.unwrap();
        assert!(setup.otpauth_uri.contains("secret="));
        assert!(setup.otpauth_uri.contains("issuer=AeroXe"));
        assert!(setup.otpauth_uri.contains("digits=6"));
        assert!(setup.otpauth_uri.contains("period=30"));
    }
}

#[cfg(test)]
mod template_tests {
    use crate::shared::utils::templates::TemplateEngine;
    use serde_json::json;

    #[test]
    fn test_render_invoice_email() {
        let engine = TemplateEngine::new();
        let data = json!({
            "invoice_number": "INV-202607-0001",
            "customer_name": "Rahul Sharma",
            "total_amount": "708.00",
            "due_date": "2026-07-25",
            "payment_url": "https://pay.aeroxe.com/inv123"
        });
        let rendered = engine.render("invoice_email", &data).unwrap();
        assert!(rendered.contains("INV-202607-0001"));
        assert!(rendered.contains("Rahul Sharma"));
        assert!(rendered.contains("708.00"));
    }

    #[test]
    fn test_render_otp_sms() {
        let engine = TemplateEngine::new();
        let data = json!({ "otp": "482916" });
        let rendered = engine.render("otp_sms", &data).unwrap();
        assert!(rendered.contains("482916"));
        assert!(rendered.len() < 160, "SMS should be concise");
    }

    #[test]
    fn test_render_all_templates() {
        let engine = TemplateEngine::new();
        let templates = vec![
            "invoice_email",
            "payment_reminder",
            "welcome_email",
            "otp_sms",
            "installation_notify",
            "ticket_confirm",
            "referral_reward",
        ];

        for template_name in templates {
            let data = json!({
                "customer_name": "Test",
                "invoice_number": "INV-001",
                "total_amount": "708",
                "due_date": "2026-07-25",
                "payment_url": "https://pay.aeroxe.com",
                "days_overdue": "5",
                "plan_name": "100Mbps",
                "pppoe_username": "user123",
                "scheduled_date": "2026-07-20",
                "scheduled_time_slot": "10:00-12:00",
                "technician_name": "Tech A",
                "ticket_number": "TK-001",
                "subject": "Test",
                "category": "Network",
                "referrer_name": "Referrer",
                "reward_amount": "100",
                "reward_type": "cash",
                "otp": "123456",
            });
            let result = engine.render(template_name, &data);
            assert!(result.is_ok(), "Template '{}' should render", template_name);
        }
    }
}
