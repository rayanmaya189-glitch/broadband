use sha1::Sha1;
use hmac::{Hmac, Mac};
use std::time::{SystemTime, UNIX_EPOCH};

type HmacSha1 = Hmac<Sha1>;

/// Base32 decode a string to bytes.
pub fn base32_decode(input: &str) -> Result<Vec<u8>, String> {
    const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";
    let input = input.to_uppercase().replace('=', "");
    let mut bits: u32 = 0;
    let mut value: u32 = 0;
    let mut output = Vec::new();

    for byte in input.bytes() {
        let val = ALPHABET.iter().position(|&b| b == byte)
            .ok_or_else(|| format!("Invalid base32 character: {}", byte as char))?;
        value = (value << 5) | (val as u32);
        bits += 5;
        if bits >= 8 {
            bits -= 8;
            output.push((value >> bits) as u8);
        }
    }
    Ok(output)
}

/// Generate a TOTP code for a given secret and time step.
/// This implements RFC 6238 with SHA-1, 30-second steps, and 6-digit output.
pub fn generate_totp(secret_base32: &str, time_step: u64) -> Result<String, String> {
    let secret = base32_decode(secret_base32)?;
    
    // Time counter (30-second intervals since epoch)
    let time_counter = time_step;
    let counter_bytes = time_counter.to_be_bytes();
    
    // HMAC-SHA1
    let mut mac = HmacSha1::new_from_slice(&secret)
        .map_err(|e| format!("HMAC init failed: {e}"))?;
    mac.update(&counter_bytes);
    let result = mac.finalize().into_bytes();
    
    // Dynamic truncation (RFC 4226)
    let offset = (result[19] & 0x0F) as usize;
    let code = ((result[offset] as u32 & 0x7F) << 24)
        | ((result[offset + 1] as u32) << 16)
        | ((result[offset + 2] as u32) << 8)
        | (result[offset + 3] as u32);
    
    let otp = code % 1_000_000;
    Ok(format!("{:06}", otp))
}

/// Verify a TOTP code against a secret, allowing for a window of time steps.
/// Returns true if the code is valid within the ±1 step window (±30 seconds).
pub fn verify_totp(secret_base32: &str, code: &str) -> Result<bool, String> {
    if code.len() != 6 || !code.chars().all(|c| c.is_ascii_digit()) {
        return Ok(false);
    }
    
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| format!("System time error: {e}"))?
        .as_secs();
    
    let current_step = now / 30;
    
    // Check current step and ±1 step window
    for offset in [current_step.wrapping_sub(1), current_step, current_step + 1] {
        let expected = generate_totp(secret_base32, offset)?;
        if constant_time_eq(&expected, code) {
            return Ok(true);
        }
    }
    
    Ok(false)
}

/// Constant-time string comparison to prevent timing attacks.
pub fn constant_time_eq(a: &str, b: &str) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let a_bytes = a.as_bytes();
    let b_bytes = b.as_bytes();
    let mut result = 0u8;
    for i in 0..a_bytes.len() {
        result |= a_bytes[i] ^ b_bytes[i];
    }
    result == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base32_decode() {
        // "Hello" in base32 is "JBSWY3DP" (5 bytes = 8 base32 chars)
        let decoded = base32_decode("JBSWY3DP").unwrap();
        assert_eq!(decoded, b"Hello");
    }

    #[test]
    fn test_generate_totp() {
        // Known test vector: secret "JBSWY3DPEE" at time 0
        let code = generate_totp("JBSWY3DPEE", 0).unwrap();
        assert_eq!(code.len(), 6);
        assert!(code.chars().all(|c| c.is_ascii_digit()));
    }

    #[test]
    fn test_constant_time_eq() {
        assert!(constant_time_eq("abc", "abc"));
        assert!(!constant_time_eq("abc", "def"));
        assert!(!constant_time_eq("abc", "ab"));
    }
}
