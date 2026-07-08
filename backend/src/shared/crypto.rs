use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use sha2::{Digest, Sha256};

use crate::error::AppError;

/// Hash a password with argon2id.
pub fn hash_password(password: &str) -> Result<String, AppError> {
    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Password hash failed: {e}")))?
        .to_string();
    Ok(hash)
}

/// Verify a password against an argon2id hash.
pub fn verify_password(password: &str, hash: &str) -> Result<bool, AppError> {
    let parsed =
        PasswordHash::new(hash).map_err(|e| AppError::Internal(anyhow::anyhow!("Invalid hash format: {e}")))?;
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed)
        .is_ok())
}

/// SHA-256 hash (used for refresh token storage).
pub fn sha256(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}

/// Generate a random 256-bit token as a hex string.
pub fn generate_token() -> String {
    use rand::Rng;
    let mut bytes = [0u8; 32];
    rand::thread_rng().fill(&mut bytes);
    hex::encode(bytes)
}

/// AES-256-GCM encrypt plaintext (for sensitive fields like 2FA secrets).
pub fn aes_encrypt(plaintext: &[u8], key: &[u8; 32]) -> Result<Vec<u8>, AppError> {
    use aes_gcm::{
        aead::{Aead, KeyInit},
        Aes256Gcm, Nonce,
    };
    use rand::RngCore;

    let cipher = Aes256Gcm::new_from_slice(key)
        .map_err(|e| AppError::Internal(anyhow::anyhow!("AES init failed: {e}")))?;

    let mut nonce_bytes = [0u8; 12];
    rand::thread_rng().fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let mut ciphertext = cipher
        .encrypt(nonce, plaintext)
        .map_err(|e| AppError::Internal(anyhow::anyhow!("AES encrypt failed: {e}")))?;

    // Prepend nonce to ciphertext
    let mut result = nonce_bytes.to_vec();
    result.append(&mut ciphertext);
    Ok(result)
}

/// AES-256-GCM decrypt ciphertext.
pub fn aes_decrypt(ciphertext: &[u8], key: &[u8; 32]) -> Result<Vec<u8>, AppError> {
    use aes_gcm::{
        aead::{Aead, KeyInit},
        Aes256Gcm, Nonce,
    };

    if ciphertext.len() < 12 {
        return Err(AppError::Internal(anyhow::anyhow!(
            "Ciphertext too short"
        )));
    }

    let (nonce_bytes, encrypted) = ciphertext.split_at(12);
    let cipher = Aes256Gcm::new_from_slice(key)
        .map_err(|e| AppError::Internal(anyhow::anyhow!("AES init failed: {e}")))?;
    let nonce = Nonce::from_slice(nonce_bytes);

    cipher
        .decrypt(nonce, encrypted)
        .map_err(|e| AppError::Internal(anyhow::anyhow!("AES decrypt failed: {e}")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_hash_and_verify() {
        let password = "secure_password_123";
        let hash = hash_password(password).unwrap();
        assert!(verify_password(password, &hash).unwrap());
        assert!(!verify_password("wrong_password", &hash).unwrap());
    }

    #[test]
    fn test_sha256() {
        let hash = sha256(b"hello");
        assert_eq!(
            hash,
            "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
        );
    }

    #[test]
    fn test_aes_encrypt_decrypt() {
        let key = [0u8; 32];
        let plaintext = b"hello, world!";
        let ciphertext = aes_encrypt(plaintext, &key).unwrap();
        let decrypted = aes_decrypt(&ciphertext, &key).unwrap();
        assert_eq!(plaintext.as_slice(), decrypted.as_slice());
    }
}
