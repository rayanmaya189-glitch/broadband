//! JWT and authentication security tests per OWASP ASVS v4.0 §V2

use aeroxe_backend::shared::utils::jwt_keys::{init_jwt_keys, JwtKeyPair};

/// Test JWT RS256 key pair generation and verification
#[test]
fn test_jwt_rs256_key_generation() {
    let key_pair = JwtKeyPair::generate().expect("Should generate RSA key pair");

    // Verify keys are different (asymmetric)
    assert_ne!(key_pair.private_key_pem(), key_pair.public_key_pem());

    // Verify signing and verification work
    let test_payload = serde_json::json!({
        "sub": "user-123",
        "email": "test@example.com",
        "role": "admin",
        "branch_id": 1,
        "exp": chrono::Utc::now().timestamp() + 3600,
    });

    let token = key_pair.sign(&test_payload).expect("Should sign payload");
    let verified = key_pair.verify(&token).expect("Should verify token");

    assert_eq!(verified["sub"], "user-123");
    assert_eq!(verified["role"], "admin");
}

/// Test that expired tokens are rejected
#[test]
fn test_expired_token_rejection() {
    let key_pair = JwtKeyPair::generate().expect("Should generate key pair");

    let expired_payload = serde_json::json!({
        "sub": "user-123",
        "exp": 1000000000, // Far in the past
    });

    let token = key_pair.sign(&expired_payload).expect("Should sign");
    let result = key_pair.verify(&token);

    assert!(result.is_err(), "Expired token should be rejected");
}

/// Test that tampered tokens are rejected
#[test]
fn test_tampered_token_rejection() {
    let key_pair = JwtKeyPair::generate().expect("Should generate key pair");

    let payload = serde_json::json!({
        "sub": "user-123",
        "role": "customer",
    });

    let mut token = key_pair.sign(&payload).expect("Should sign");

    // Tamper with the token by modifying the payload section
    // JWT format: header.payload.signature
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() == 3 {
        // Corrupt the signature by appending a character
        token = format!("{}.{}{}X", parts[0], parts[1], parts[2].chars().next().unwrap());
    }

    let result = key_pair.verify(&token);
    assert!(result.is_err(), "Tampered token should be rejected");
}

/// Test password hashing with argon2id
#[test]
fn test_argon2id_password_hashing() {
    use argon2::{Argon2, PasswordHash, PasswordVerifier};
    use password_hash::{rand_core::OsRng, PasswordHasher, SaltString};

    let password = "SecureP@ssw0rd123!";

    // Hash the password
    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .expect("Should hash password")
        .to_string();

    // Verify the password
    let parsed = PasswordHash::new(&hash).expect("Should parse hash");
    let result = Argon2::default()
        .verify_password(password.as_bytes(), &parsed);

    assert!(result.is_ok(), "Password verification should succeed");

    // Verify wrong password fails
    let wrong_result = Argon2::default()
        .verify_password("WrongPassword".as_bytes(), &parsed);

    assert!(wrong_result.is_err(), "Wrong password should fail");
}

/// Test that password hash includes salt (unique hashes for same password)
#[test]
fn test_password_hash_uniqueness() {
    use argon2::Argon2;
    use password_hash::{rand_core::OsRng, PasswordHasher, SaltString};

    let password = "SamePassword123!";

    let salt1 = SaltString::generate(&mut OsRng);
    let hash1 = Argon2::default()
        .hash_password(password.as_bytes(), &salt1)
        .unwrap()
        .to_string();

    let salt2 = SaltString::generate(&mut OsRng);
    let hash2 = Argon2::default()
        .hash_password(password.as_bytes(), &salt2)
        .unwrap()
        .to_string();

    // Same password should produce different hashes due to random salt
    assert_ne!(hash1, hash2, "Same password should produce different hashes");
}
