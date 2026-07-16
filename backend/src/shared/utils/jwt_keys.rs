use anyhow::{Context, Result};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use rsa::pkcs8::{EncodePrivateKey, EncodePublicKey};
use rsa::RsaPrivateKey;
use serde::{Deserialize, Serialize};
use tracing::info;

/// RS256 JWT key pair — holds both private and public keys in memory.
#[derive(Clone)]
pub struct JwtKeyPair {
    private_key_pem: String,
    public_key_pem: String,
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
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
        let encoding_key =
            EncodingKey::from_rsa_pem(private_pem.as_bytes()).context("Invalid RSA private key PEM")?;
        let decoding_key =
            DecodingKey::from_rsa_pem(public_pem.as_bytes()).context("Invalid RSA public key PEM")?;
        info!("JWT RS256 key pair loaded from PEM");
        Ok(Self {
            private_key_pem: private_pem.to_string(),
            public_key_pem: public_pem.to_string(),
            encoding_key,
            decoding_key,
        })
    }

    /// Generate a fresh RSA-2048 key pair (for development / first boot).
    pub fn generate() -> Result<Self> {
        let mut rng = rand::thread_rng();
        let private_key = RsaPrivateKey::new(&mut rng, 2048).context("RSA key generation failed")?;
        let public_key = rsa::RsaPublicKey::from(&private_key);

        let private_pem = private_key
            .to_pkcs8_pem(rsa::pkcs8::LineEnding::LF)
            .context("Private key PEM encoding failed")?
            .to_string();
        let public_pem = public_key
            .to_public_key_pem(rsa::pkcs8::LineEnding::LF)
            .context("Public key PEM encoding failed")?
            .to_string();

        let encoding_key =
            EncodingKey::from_rsa_pem(private_pem.as_bytes()).context("EncodingKey from generated PEM")?;
        let decoding_key =
            DecodingKey::from_rsa_pem(public_pem.as_bytes()).context("DecodingKey from generated PEM")?;

        info!("JWT RS256 key pair generated (RSA-2048)");
        Ok(Self {
            private_key_pem: private_pem,
            public_key_pem: public_pem,
            encoding_key,
            decoding_key,
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
