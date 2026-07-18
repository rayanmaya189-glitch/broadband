use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use rsa::pkcs8::{EncodePrivateKey, EncodePublicKey};
use rsa::RsaPrivateKey;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn};

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

/// JWT key rotation manager with automatic key refresh.
pub struct JwtKeyRotationManager {
    current_keys: Arc<RwLock<JwtKeyPair>>,
    max_age_days: i64,
}

impl JwtKeyRotationManager {
    /// Create a new key rotation manager.
    pub fn new(keys: JwtKeyPair, max_age_days: i64) -> Self {
        Self {
            current_keys: Arc::new(RwLock::new(keys)),
            max_age_days,
        }
    }

    /// Get the current key pair (read lock).
    pub async fn current_keys(&self) -> tokio::sync::RwLockReadGuard<'_, JwtKeyPair> {
        self.current_keys.read().await
    }

    /// Check if rotation is needed and perform it.
    pub async fn check_and_rotate(&self) -> Result<bool> {
        let old_created_at = {
            let keys = self.current_keys.read().await;
            if !keys.needs_rotation(self.max_age_days) {
                return Ok(false);
            }
            keys.created_at()
        };

        // Generate new keys
        let new_keys = JwtKeyPair::generate()?;
        info!(old_created_at = %old_created_at, "Rotating JWT key pair");

        // Update keys
        let mut keys = self.current_keys.write().await;
        *keys = new_keys;

        warn!("JWT key pair rotated — active tokens signed with old key remain valid until expiry");
        Ok(true)
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
pub fn init_jwt_keys(
    private_pem: &Option<String>,
    public_pem: &Option<String>,
) -> Result<JwtKeyPair> {
    match (private_pem, public_pem) {
        (Some(priv_pem), Some(pub_pem)) => {
            info!("Loading JWT RS256 keys from environment variables");
            JwtKeyPair::from_pems(priv_pem, pub_pem)
        }
        (None, None) => {
            info!("No JWT keys in env — generating ephemeral RSA-2048 key pair (development mode)");
            JwtKeyPair::generate()
        }
        _ => {
            anyhow::bail!("Both JWT_PRIVATE_KEY and JWT_PUBLIC_KEY must be set, or neither")
        }
    }
}
