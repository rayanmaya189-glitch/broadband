use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use rsa::pkcs8::{EncodePrivateKey, EncodePublicKey};
use rsa::RsaPrivateKey;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::sync::RwLock as StdRwLock;
use tracing::info;

/// RS256 JWT key pair — holds both private and public keys in memory.
#[derive(Clone)]
pub struct JwtKeyPair {
    private_key_pem: String,
    public_key_pem: String,
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandardClaims {
    pub sub: String,
    pub email: String,
    pub name: String,
    pub role: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_id: Option<i64>,
    pub is_company_wide: bool,
    pub iat: i64,
    pub exp: i64,
}

impl JwtKeyPair {
    /// Create from PEM strings (production: loaded from env vars).
    pub fn from_pems(private_pem: &str, public_pem: &str) -> Result<Self> {
        let encoding_key = EncodingKey::from_rsa_pem(private_pem.as_bytes())
            .context("Invalid RSA private key PEM")?;
        let decoding_key = DecodingKey::from_rsa_pem(public_pem.as_bytes())
            .context("Invalid RSA public key PEM")?;
        info!("JWT RS256 key pair loaded from PEM");
        Ok(Self {
            private_key_pem: private_pem.to_string(),
            public_key_pem: public_pem.to_string(),
            encoding_key,
            decoding_key,
            created_at: Utc::now(),
        })
    }

    /// Generate a fresh RSA-2048 key pair (for development / first boot).
    pub fn generate() -> Result<Self> {
        let mut rng = rand::thread_rng();
        let private_key =
            RsaPrivateKey::new(&mut rng, 2048).context("RSA key generation failed")?;
        let public_key = rsa::RsaPublicKey::from(&private_key);

        let private_pem = private_key
            .to_pkcs8_pem(rsa::pkcs8::LineEnding::LF)
            .context("Private key PEM encoding failed")?
            .to_string();
        let public_pem = public_key
            .to_public_key_pem(rsa::pkcs8::LineEnding::LF)
            .context("Public key PEM encoding failed")?
            .to_string();

        let encoding_key = EncodingKey::from_rsa_pem(private_pem.as_bytes())
            .context("EncodingKey from generated PEM")?;
        let decoding_key = DecodingKey::from_rsa_pem(public_pem.as_bytes())
            .context("DecodingKey from generated PEM")?;

        info!("JWT RS256 key pair generated (RSA-2048)");
        Ok(Self {
            private_key_pem: private_pem,
            public_key_pem: public_pem,
            encoding_key,
            decoding_key,
            created_at: Utc::now(),
        })
    }

    /// Sign a JWT with RS256.
    pub fn sign(&self, claims: &StandardClaims) -> Result<String> {
        let header = Header::new(Algorithm::RS256);
        encode(&header, claims, &self.encoding_key).context("JWT signing failed")
    }

    /// Verify and decode a JWT with RS256.
    pub fn verify(&self, token: &str) -> Result<StandardClaims> {
        let mut validation = Validation::new(Algorithm::RS256);
        validation.validate_exp = true;
        validation.validate_aud = false;
        let data = decode::<StandardClaims>(token, &self.decoding_key, &validation)
            .context("JWT verification failed")?;
        Ok(data.claims)
    }

    /// Access the raw public key PEM (for embedding in JWKS endpoint).
    pub fn public_key_pem(&self) -> &str {
        &self.public_key_pem
    }

    /// Access the raw private key PEM (for key rotation / export).
    pub fn private_key_pem(&self) -> &str {
        &self.private_key_pem
    }

    /// Get the key creation time.
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    /// Check if the key needs rotation (older than max_age_days).
    pub fn needs_rotation(&self, max_age_days: i64) -> bool {
        let age = Utc::now() - self.created_at;
        age.num_days() > max_age_days
    }
}

/// Thread-safe JWT key store used for signing and verification.
///
/// Holds the active key pair plus a read-only copy of the immediately-previous
/// key's decoding key. When a rotation happens, tokens that were issued with
/// the old key remain verifiable for their remaining lifetime instead of being
/// rejected en masse — which would otherwise force a logout of every user on
/// each 90-day rotation.
#[derive(Clone)]
pub struct JwtKeys {
    inner: Arc<StdRwLock<JwtKeyStoreInner>>,
}

struct JwtKeyStoreInner {
    current: JwtKeyPair,
    previous: Option<PreviousKey>,
}

/// Rotated-out key pair kept only for verification (no private key).
struct PreviousKey {
    decoding_key: DecodingKey,
}

impl JwtKeys {
    /// Create a store seeded with the initial key pair.
    pub fn new(keys: JwtKeyPair) -> Self {
        Self {
            inner: Arc::new(StdRwLock::new(JwtKeyStoreInner {
                current: keys,
                previous: None,
            })),
        }
    }

    /// Sign a JWT with the current key pair.
    pub fn sign(&self, claims: &StandardClaims) -> Result<String> {
        let inner = self
            .inner
            .read()
            .map_err(|_| anyhow::anyhow!("JWT key store poisoned"))?;
        inner.current.sign(claims)
    }

    /// Verify a JWT against the current key pair, then the previous pair as a
    /// fallback so tokens issued shortly before a rotation still validate.
    pub fn verify(&self, token: &str) -> Result<StandardClaims> {
        let inner = self
            .inner
            .read()
            .map_err(|_| anyhow::anyhow!("JWT key store poisoned"))?;
        match inner.current.verify(token) {
            Ok(claims) => Ok(claims),
            Err(_) => match &inner.previous {
                Some(prev) => {
                    let mut validation = Validation::new(Algorithm::RS256);
                    validation.validate_exp = true;
                    validation.validate_aud = false;
                    decode::<StandardClaims>(token, &prev.decoding_key, &validation)
                        .map(|d| d.claims)
                        .context("JWT verification failed")
                }
                None => Err(anyhow::anyhow!("JWT verification failed")),
            },
        }
    }

    /// Whether the current key is older than `max_age_days`.
    fn needs_rotation(&self, max_age_days: i64) -> bool {
        let inner = self
            .inner
            .read()
            .unwrap_or_else(|_| panic!("JWT key store poisoned"));
        inner.current.needs_rotation(max_age_days)
    }

    /// Generate a fresh key pair, promote the current one to `previous` and
    /// drop the private key of the old pair (kept for verification only).
    fn rotate(&self) -> Result<bool> {
        let new_keys = JwtKeyPair::generate()?;
        let mut inner = self
            .inner
            .write()
            .map_err(|_| anyhow::anyhow!("JWT key store poisoned"))?;
        let old = std::mem::replace(&mut inner.current, new_keys);
        inner.previous = Some(PreviousKey {
            decoding_key: old.decoding_key,
        });
        info!(old_created_at = %old.created_at, "JWT key pair rotated; previous key kept for token grace period");
        Ok(true)
    }
}

/// JWT key rotation manager with automatic key refresh.
pub struct JwtKeyRotationManager {
    keys: Arc<JwtKeys>,
    max_age_days: i64,
}

impl JwtKeyRotationManager {
    /// Create a new key rotation manager wrapping a fresh key store.
    pub fn new(keys: JwtKeyPair, max_age_days: i64) -> Self {
        Self {
            keys: Arc::new(JwtKeys::new(keys)),
            max_age_days,
        }
    }

    /// Create a manager over an existing key store (same store shared with the
    /// auth path via [`Self::keys`]).
    pub fn from_arc(keys: Arc<JwtKeys>, max_age_days: i64) -> Self {
        Self { keys, max_age_days }
    }

    /// The shared key store used by signing/verification.
    pub fn keys(&self) -> Arc<JwtKeys> {
        self.keys.clone()
    }

    /// Check if rotation is needed and perform it.
    pub async fn check_and_rotate(&self) -> Result<bool> {
        if !self.keys.needs_rotation(self.max_age_days) {
            return Ok(false);
        }
        self.keys.rotate()
    }

    /// Start background rotation checker (runs daily).
    pub fn start_background_rotation(self: Arc<Self>) {
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(std::time::Duration::from_secs(86400));
            loop {
                interval.tick().await;
                match self.check_and_rotate().await {
                    Ok(rotated) if rotated => {
                        tracing::info!("JWT keys rotated successfully");
                    }
                    Ok(_) => {
                        tracing::debug!("JWT keys still valid, no rotation needed");
                    }
                    Err(e) => {
                        tracing::error!(error = %e, "JWT key rotation check failed");
                    }
                }
            }
        });
    }
}

/// Initialize JwtKeyPair from settings. Uses PEM env vars if set, otherwise generates ephemeral keys.
/// SECURITY: In production, panics if RSA keys are not set (ephemeral keys are insecure).
pub fn init_jwt_keys(
    private_pem: &Option<String>,
    public_pem: &Option<String>,
) -> Result<JwtKeyPair> {
    let is_production =
        std::env::var("APP_ENV").unwrap_or_else(|_| "development".to_string()) == "production";

    match (private_pem, public_pem) {
        (Some(priv_pem), Some(pub_pem)) => {
            info!("Loading JWT RS256 keys from environment variables");
            JwtKeyPair::from_pems(priv_pem, pub_pem)
        }
        (None, None) => {
            if is_production {
                anyhow::bail!(
                    "FATAL: JWT_PRIVATE_KEY and JWT_PUBLIC_KEY must be set in production. \
                     Ephemeral keys are insecure — tokens signed with dev keys are rejected."
                );
            }
            info!("No JWT keys in env — generating ephemeral RSA-2048 key pair (development mode)");
            JwtKeyPair::generate()
        }
        _ => {
            anyhow::bail!("Both JWT_PRIVATE_KEY and JWT_PUBLIC_KEY must be set, or neither")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn claims(sub: &str) -> StandardClaims {
        StandardClaims {
            sub: sub.to_string(),
            email: format!("{sub}@aeroxe.test"),
            name: "Test User".to_string(),
            role: "user".to_string(),
            branch_id: None,
            is_company_wide: false,
            iat: Utc::now().timestamp(),
            exp: (Utc::now() + chrono::Duration::seconds(300)).timestamp(),
        }
    }

    #[test]
    fn token_signed_before_rotation_remains_verifiable() {
        let store = JwtKeys::new(JwtKeyPair::generate().unwrap());
        let token = store.sign(&claims("42")).unwrap();

        // Tokens verify before rotation.
        assert_eq!(store.verify(&token).unwrap().sub, "42");

        // After rotation the same token must still verify via the previous key.
        store.rotate().unwrap();
        assert_eq!(store.verify(&token).unwrap().sub, "42");
    }

    #[test]
    fn tokens_signed_after_rotation_verify_and_oldest_key_is_dropped() {
        let store = JwtKeys::new(JwtKeyPair::generate().unwrap());
        let first = store.sign(&claims("1")).unwrap();

        store.rotate().unwrap();
        let second = store.sign(&claims("2")).unwrap();
        assert_eq!(store.verify(&second).unwrap().sub, "2");
        assert_eq!(store.verify(&first).unwrap().sub, "1");

        // Only one previous key is retained: after a second rotation the token
        // signed with the oldest key is no longer accepted.
        store.rotate().unwrap();
        let third = store.sign(&claims("3")).unwrap();
        assert_eq!(store.verify(&third).unwrap().sub, "3");
        assert_eq!(store.verify(&second).unwrap().sub, "2");
        assert!(store.verify(&first).is_err());
    }

    #[test]
    fn rotation_manager_reports_when_no_rotation_is_due() {
        let keys = JwtKeyPair::generate().unwrap();
        let manager = JwtKeyRotationManager::new(keys, 90);
        let runtime = tokio::runtime::Runtime::new().unwrap();
        assert!(!runtime.block_on(manager.check_and_rotate()).unwrap());
    }
}
