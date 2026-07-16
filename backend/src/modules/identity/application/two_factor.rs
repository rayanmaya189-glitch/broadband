use rand::Rng;
use sha2::{Digest, Sha256};

use crate::shared::errors::AppError;

/// TOTP configuration per security doc §28
const TOTP_DIGITS: usize = 6;
const TOTP_PERIOD: u64 = 30;
const TOTP_SKEW: u8 = 1;
const BACKUP_CODE_COUNT: usize = 10;

/// Result of setting up 2FA for a user.
pub struct TwoFactorSetup {
    /// The TOTP secret encoded in Base32 — show this as QR code data.
    pub secret_base32: String,
    /// The otpauth:// URI for QR code generation.
    pub otpauth_uri: String,
    /// One-time backup codes — show these to the user ONCE.
    pub backup_codes: Vec<String>,
    /// SHA-256 hashes of the backup codes (store in DB).
    pub backup_code_hashes: Vec<String>,
}

/// Generate a new TOTP secret and backup codes for a user.
pub fn setup_two_factor(user_email: &str) -> Result<TwoFactorSetup, AppError> {
    // Generate random TOTP secret (20 bytes = 160 bits)
    let secret_bytes: Vec<u8> = (0..20).map(|_| rand::thread_rng().gen()).collect();
    // Encode to Base32
    let secret_base32 = base32_encode(&secret_bytes);

    // Build otpauth URI manually (compatible with all authenticator apps)
    let otpauth_uri = format!(
        "otpauth://totp/{}:{}?secret={}&issuer={}&digits={}&period={}",
        url_encode("AeroXeBroadband"),
        url_encode(user_email),
        secret_base32,
        url_encode("AeroXeBroadband"),
        TOTP_DIGITS,
        TOTP_PERIOD
    );

    // Generate backup codes
    let mut backup_codes = Vec::with_capacity(BACKUP_CODE_COUNT);
    let mut backup_code_hashes = Vec::with_capacity(BACKUP_CODE_COUNT);
    for _ in 0..BACKUP_CODE_COUNT {
        let code = generate_backup_code();
        let hash = hash_backup_code(&code);
        backup_codes.push(code);
        backup_code_hashes.push(hash);
    }

    Ok(TwoFactorSetup {
        secret_base32,
        otpauth_uri,
        backup_codes,
        backup_code_hashes,
    })
}

/// Verify a TOTP code against the stored base32 secret.
/// Uses RFC 6238 compatible time-based verification.
pub fn verify_totp(secret_base32: &str, code: &str) -> bool {
    let secret_bytes = match base32_decode(secret_base32) {
        Ok(b) => b,
        Err(_) => return false,
    };

    // Get current time step
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let current_step = now / TOTP_PERIOD;

    // Check current step and ±1 skew window
    for offset in 0..=TOTP_SKEW as u64 {
        // Check current + offset and current - offset
        for &step in &[current_step.wrapping_add(offset), current_step.wrapping_sub(offset)] {
            let expected = generate_totp_code(&secret_bytes, step);
            if expected == code {
                return true;
            }
        }
    }
    false
}

/// Generate a TOTP code for a given time step (RFC 6238 compatible).
fn generate_totp_code(secret: &[u8], step: u64) -> String {
    use hmac::{Hmac, Mac};
    type HmacSha1 = Hmac<sha1::Sha1>;

    let mut step_bytes = [0u8; 8];
    step_bytes[0..8].copy_from_slice(&step.to_be_bytes());

    let mut mac = HmacSha1::new_from_slice(secret).expect("HMAC can take key of any size");
    mac.update(&step_bytes);
    let result = mac.finalize().into_bytes();

    // Dynamic truncation
    let offset = (result[19] & 0x0F) as usize;
    let code = ((result[offset] as u32 & 0x7F) << 24)
        | ((result[offset + 1] as u32) << 16)
        | ((result[offset + 2] as u32) << 8)
        | (result[offset + 3] as u32);

    format!("{:0>width$}", code % 10u32.pow(TOTP_DIGITS as u32), width = TOTP_DIGITS)
}

/// Verify a backup code against its stored hash.
/// Returns true if valid, and the remaining hashes (code removed — one-time use).
pub fn verify_backup_code(
    code: &str,
    stored_hashes: &[String],
) -> Result<(bool, Vec<String>), AppError> {
    let code_hash = hash_backup_code(code);
    let mut remaining = stored_hashes.to_vec();
    if let Some(pos) = remaining.iter().position(|h| *h == code_hash) {
        remaining.remove(pos);
        Ok((true, remaining))
    } else {
        Ok((false, remaining))
    }
}

/// Hash a backup code for secure storage.
fn hash_backup_code(code: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(format!("aeroxe-backup-{}", code).as_bytes());
    hex::encode(hasher.finalize())
}

/// Generate a single backup code (e.g., "AB3K-7M9P").
fn generate_backup_code() -> String {
    let mut rng = rand::thread_rng();
    let chars: Vec<char> = "ABCDEFGHJKLMNPQRSTUVWXYZ23456789".chars().collect();
    let part1: String = (0..4)
        .map(|_| chars[rng.gen_range(0..chars.len())])
        .collect();
    let part2: String = (0..4)
        .map(|_| chars[rng.gen_range(0..chars.len())])
        .collect();
    format!("{}-{}", part1, part2)
}

// ── Base32 helpers (RFC 4648) ──

const BASE32_ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";

fn base32_encode(data: &[u8]) -> String {
    let mut result = String::new();
    let mut buf: u32 = 0;
    let mut bits: u32 = 0;

    for &byte in data {
        buf = (buf << 8) | (byte as u32);
        bits += 8;
        while bits >= 5 {
            bits -= 5;
            result.push(BASE32_ALPHABET[((buf >> bits) & 0x1F) as usize] as char);
        }
    }
    if bits > 0 {
        result.push(BASE32_ALPHABET[((buf << (5 - bits)) & 0x1F) as usize] as char);
    }
    result
}

fn base32_decode(input: &str) -> Result<Vec<u8>, ()> {
    let mut buf: u32 = 0;
    let mut bits: u32 = 0;
    let mut result = Vec::new();

    for ch in input.chars() {
        let val = match ch {
            'A'..='Z' => ch as u8 - b'A',
            '2'..='7' => ch as u8 - b'2' + 26,
            _ => return Err(()),
        };
        buf = (buf << 5) | (val as u32);
        bits += 5;
        if bits >= 8 {
            bits -= 8;
            result.push((buf >> bits) as u8);
        }
    }
    Ok(result)
}

fn url_encode(input: &str) -> String {
    input
        .bytes()
        .map(|b| match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                String::from(b as char)
            }
            _ => format!("%{:02X}", b),
        })
        .collect()
}
