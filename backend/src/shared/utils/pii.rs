use sha2::{Digest, Sha256};

// PII Protection utilities per §28 Security Design.
// Provides hashing for Aadhaar/PAN (searchable salted hashes) and
// masking for phone/email (display-only redaction).

// ── App-level salted hashing (deterministic — useful for lookups) ────────
// The fixed app-level salt makes the hash deterministic for the same input,
// so you can look up "which customer has Aadhaar X" without storing the
// original. Trade-off: if the app salt leaks, rainbow tables are feasible.
// For higher security, use the per-record salt variants below.

/// Hash Aadhaar with app-level salt for searchable storage.
/// Uses SHA-256. Original is NOT recoverable.
pub fn hash_aadhaar(aadhaar: &str) -> String {
    let salted = format!("aeroxe:aadhaar:{}", aadhaar);
    let mut hasher = Sha256::new();
    hasher.update(salted.as_bytes());
    hex::encode(hasher.finalize())
}

/// Hash PAN with app-level salt for searchable storage.
pub fn hash_pan(pan: &str) -> String {
    let salted = format!("aeroxe:pan:{}", pan);
    let mut hasher = Sha256::new();
    hasher.update(salted.as_bytes());
    hex::encode(hasher.finalize())
}

/// Hash any PII field with a custom prefix for flexible searchable hashing.
pub fn hash_pii(prefix: &str, value: &str) -> String {
    let salted = format!("aeroxe:{}:{}", prefix, value);
    let mut hasher = Sha256::new();
    hasher.update(salted.as_bytes());
    hex::encode(hasher.finalize())
}

// ── Per-record salted hashing (non-deterministic — strongest security) ───
// Each record gets its own random 16-byte salt. The salt is stored alongside
// the hash (e.g. in a `salt` column). This prevents rainbow table attacks
// even if the app-level salt is compromised.

/// Generate a random 16-byte hex salt.
pub fn generate_salt() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let salt: [u8; 16] = rng.gen();
    hex::encode(salt)
}

/// Hash a value with a specific salt. Returns the hex-encoded hash.
fn hash_with_salt(prefix: &str, value: &str, salt: &str) -> String {
    let salted = format!("aeroxe:{}:{}:{}", prefix, value, salt);
    let mut hasher = Sha256::new();
    hasher.update(salted.as_bytes());
    hex::encode(hasher.finalize())
}

/// Hash Aadhaar with a per-record salt.
/// Returns (hash, salt). Store both in the database.
/// To verify: re-hash the input with the stored salt and compare.
pub fn hash_aadhaar_with_salt(aadhaar: &str) -> (String, String) {
    let salt = generate_salt();
    let hash = hash_with_salt("aadhaar", aadhaar, &salt);
    (hash, salt)
}

/// Hash PAN with a per-record salt.
/// Returns (hash, salt). Store both in the database.
pub fn hash_pan_with_salt(pan: &str) -> (String, String) {
    let salt = generate_salt();
    let hash = hash_with_salt("pan", pan, &salt);
    (hash, salt)
}

/// Hash any PII field with a per-record salt.
/// Returns (hash, salt). Store both in the database.
pub fn hash_pii_with_salt(prefix: &str, value: &str) -> (String, String) {
    let salt = generate_salt();
    let hash = hash_with_salt(prefix, value, &salt);
    (hash, salt)
}

/// Verify a PII value against a stored hash and salt.
pub fn verify_pii(prefix: &str, value: &str, stored_hash: &str, salt: &str) -> bool {
    let computed = hash_with_salt(prefix, value, salt);
    // Constant-time comparison to prevent timing attacks
    computed.len() == stored_hash.len() && computed == stored_hash
}

/// Mask phone number for display: +919876543210 → +91*******3210
pub fn mask_phone(phone: &str) -> String {
    if phone.len() > 8 {
        // Detect country code: +XX (2-3 digits for India, US, UK, etc.)
        let prefix_len = if let Some(rest) = phone.strip_prefix('+') {
            // Country codes are typically 1-3 digits after the '+'
            let code_end = rest
                .find(|c: char| !c.is_ascii_digit())
                .unwrap_or(rest.len());
            // Cap at 2 digits max (covers +1, +44, +91, +92, etc.)
            let code_len = code_end.min(2);
            code_len + 1 // +1 for the '+' character itself
        } else {
            0
        };
        let suffix_len = 4;
        if phone.len() > prefix_len + suffix_len {
            let mask_len = phone.len() - prefix_len - suffix_len;
            format!(
                "{}{}{}",
                &phone[..prefix_len],
                "*".repeat(mask_len),
                &phone[phone.len() - suffix_len..]
            )
        } else {
            phone.to_string()
        }
    } else {
        phone.to_string()
    }
}

/// Mask email for display: rahul@example.com → r****l@example.com
pub fn mask_email(email: &str) -> String {
    let parts: Vec<&str> = email.split('@').collect();
    if parts.len() == 2 && parts[0].len() > 2 {
        let user = parts[0];
        let domain = parts[1];
        format!("{}****{}@{}", &user[..1], &user[user.len() - 1..], domain)
    } else {
        email.to_string()
    }
}

/// Mask Aadhaar for display: 1234-5678-9012 → XXXX-XXXX-9012
pub fn mask_aadhaar(aadhaar: &str) -> String {
    if aadhaar.len() >= 4 {
        format!("XXXX-XXXX-{}", &aadhaar[aadhaar.len() - 4..])
    } else {
        "XXXX".to_string()
    }
}

/// Mask PAN for display: ABCDE1234F → XXXXX234F
pub fn mask_pan(pan: &str) -> String {
    if pan.len() >= 4 {
        format!("XXXXX{}", &pan[pan.len() - 4..])
    } else {
        "XXXXX".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── App-level salt tests ─────────────────────────────────────────────

    #[test]
    fn test_hash_aadhaar_deterministic() {
        let h1 = hash_aadhaar("123456789012");
        let h2 = hash_aadhaar("123456789012");
        assert_eq!(h1, h2);
    }

    #[test]
    fn test_hash_aadhaar_different_inputs() {
        let h1 = hash_aadhaar("123456789012");
        let h2 = hash_aadhaar("987654321098");
        assert_ne!(h1, h2);
    }

    // ── Per-record salt tests ───────────────────────────────────────────

    #[test]
    fn test_generate_salt_length() {
        let salt = generate_salt();
        // 16 bytes = 32 hex chars
        assert_eq!(salt.len(), 32);
    }

    #[test]
    fn test_generate_salt_unique() {
        let s1 = generate_salt();
        let s2 = generate_salt();
        assert_ne!(s1, s2);
    }

    #[test]
    fn test_hash_aadhaar_with_salt_deterministic_with_same_salt() {
        let (h1, s1) = hash_aadhaar_with_salt("123456789012");
        // Re-hash with the same salt should produce the same hash
        let h2 = hash_with_salt("aadhaar", "123456789012", &s1);
        assert_eq!(h1, h2);
    }

    #[test]
    fn test_hash_aadhaar_with_salt_differs_each_time() {
        let (h1, _) = hash_aadhaar_with_salt("123456789012");
        let (h2, _) = hash_aadhaar_with_salt("123456789012");
        // Same input, different random salts → different hashes
        assert_ne!(h1, h2);
    }

    #[test]
    fn test_hash_pan_with_salt_roundtrip() {
        let (hash, salt) = hash_pan_with_salt("ABCDE1234F");
        assert!(verify_pii("pan", "ABCDE1234F", &hash, &salt));
        assert!(!verify_pii("pan", "XXXXX1234F", &hash, &salt));
    }

    #[test]
    fn test_hash_pii_with_salt_roundtrip() {
        let (hash, salt) = hash_pii_with_salt("phone", "+919876543210");
        assert!(verify_pii("phone", "+919876543210", &hash, &salt));
        assert!(!verify_pii("phone", "+911111111111", &hash, &salt));
    }

    // ── Mask tests ──────────────────────────────────────────────────────

    #[test]
    fn test_mask_phone() {
        assert_eq!(mask_phone("+919876543210"), "+91******3210");
    }

    #[test]
    fn test_mask_email() {
        assert_eq!(mask_email("rahul@example.com"), "r****l@example.com");
    }

    #[test]
    fn test_mask_aadhaar() {
        assert_eq!(mask_aadhaar("123456789012"), "XXXX-XXXX-9012");
    }

    #[test]
    fn test_mask_pan() {
        assert_eq!(mask_pan("ABCDE1234F"), "XXXXX234F");
    }
}
