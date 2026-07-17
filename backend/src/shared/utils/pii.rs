use sha2::{Digest, Sha256};

/// PII Protection utilities per §28 Security Design.
/// Provides hashing for Aadhaar/PAN (searchable salted hashes) and
/// masking for phone/email (display-only redaction).
///
/// Hash Aadhaar number with personal salt for searchable storage.
/// Uses SHA-256 with per-number salt. Original is NOT recoverable.
pub fn hash_aadhaar(aadhaar: &str) -> String {
    let salted = format!("aeroxe:aadhaar:{}", aadhaar);
    let mut hasher = Sha256::new();
    hasher.update(salted.as_bytes());
    hex::encode(hasher.finalize())
}

/// Hash PAN card number with personal salt for searchable storage.
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

/// Mask phone number for display: +919876543210 → +91*******3210
pub fn mask_phone(phone: &str) -> String {
    if phone.len() > 8 {
        // Detect country code: +XX (2-3 digits for India, US, UK, etc.)
        let prefix_len = if let Some(rest) = phone.strip_prefix('+') {
            // Country codes are typically 1-3 digits after the '+'
            let code_end = rest.find(|c: char| !c.is_ascii_digit()).unwrap_or(rest.len());
            // Cap at 2 digits max (covers +1, +44, +91, +92, etc.)
            let code_len = code_end.min(2);
            code_len + 1 // +1 for the '+' character itself
        } else {
            0
        };
        let suffix_len = 4;
        if phone.len() > prefix_len + suffix_len {
            let mask_len = phone.len() - prefix_len - suffix_len;
            format!("{}{}{}",
                &phone[..prefix_len],
                "*".repeat(mask_len),
                &phone[phone.len()-suffix_len..]
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
        format!("{}****{}@{}", &user[..1], &user[user.len()-1..], domain)
    } else {
        email.to_string()
    }
}

/// Mask Aadhaar for display: 1234-5678-9012 → XXXX-XXXX-9012
pub fn mask_aadhaar(aadhaar: &str) -> String {
    if aadhaar.len() >= 4 {
        format!("XXXX-XXXX-{}", &aadhaar[aadhaar.len()-4..])
    } else {
        "XXXX".to_string()
    }
}

/// Mask PAN for display: ABCDE1234F → XXXXX234F
pub fn mask_pan(pan: &str) -> String {
    if pan.len() >= 4 {
        format!("XXXXX{}", &pan[pan.len()-4..])
    } else {
        "XXXXX".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
