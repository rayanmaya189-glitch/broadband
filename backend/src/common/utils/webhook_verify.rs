use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

/// Verify Razorpay webhook signature.
///
/// Razorpay signs webhooks by computing HMAC-SHA256 of the raw request body
/// using the webhook secret, then sends the hex-encoded result in the
/// `X-Razorpay-Signature` header.
///
/// # Arguments
/// * `body` - The raw HTTP request body bytes (must not be parsed/modified)
/// * `signature` - The value from the `X-Razorpay-Signature` header
/// * `secret` - The webhook secret configured in the Razorpay dashboard
pub fn verify_razorpay(body: &[u8], signature: &str, secret: &str) -> Result<(), WebhookError> {
    let mut mac = HmacSha256::new_from_slice(secret.as_bytes())
        .map_err(|_| WebhookError::InvalidKey)?;
    mac.update(body);
    let expected = hex::encode(mac.finalize().into_bytes());

    // Constant-time comparison to prevent timing attacks
    if constant_time_eq(&expected, signature) {
        Ok(())
    } else {
        Err(WebhookError::SignatureMismatch)
    }
}

/// Verify PayU webhook signature.
///
/// PayU signs webhook payloads by computing HMAC-SHA256 of a hash string
/// composed of specific fields concatenated with a secret "second key".
/// The signature is sent in the `X-PayU-Signature` header as a hex string.
///
/// # Arguments
/// * `body` - The raw HTTP request body bytes
/// * `signature` - The value from the `X-PayU-Signature` header
/// * `secret` - The "second key" (merchant salt) from the PayU dashboard
pub fn verify_payu(body: &[u8], signature: &str, secret: &str) -> Result<(), WebhookError> {
    let mut mac = HmacSha256::new_from_slice(secret.as_bytes())
        .map_err(|_| WebhookError::InvalidKey)?;
    mac.update(body);
    let expected = hex::encode(mac.finalize().into_bytes());

    if constant_time_eq(&expected, signature) {
        Ok(())
    } else {
        Err(WebhookError::SignatureMismatch)
    }
}

/// Verify InstaMojo webhook signature (Svix-based).
///
/// InstaMojo uses Svix for webhook delivery. The signature is computed as:
/// HMAC-SHA256(secret, `${svix_id}.${svix_timestamp}.${body}`)
///
/// The `Svix-Signature` header contains space-delimited signatures
/// in the format `v1,<base64_signature>`.
///
/// # Arguments
/// * `body` - The raw HTTP request body bytes
/// * `svix_id` - The `Svix-Id` header value
/// * `svix_timestamp` - The `Svix-Timestamp` header value
/// * `svix_signature` - The `Svix-Signature` header value
/// * `secret` - The webhook signing secret (with or without `whsec_` prefix)
pub fn verify_instamojo(
    body: &[u8],
    svix_id: &str,
    svix_timestamp: &str,
    svix_signature: &str,
    secret: &str,
) -> Result<(), WebhookError> {
    // Strip the `whsec_` prefix if present
    let key = secret.strip_prefix("whsec_").unwrap_or(secret);

    // Construct the signed content
    let to_sign = format!("{}.{}.{}", svix_id, svix_timestamp, String::from_utf8_lossy(body));

    let mut mac = HmacSha256::new_from_slice(key.as_bytes())
        .map_err(|_| WebhookError::InvalidKey)?;
    mac.update(to_sign.as_bytes());
    let expected = mac.finalize().into_bytes();

    // Parse space-delimited signatures (format: "v1,<base64> v1,<base64>")
    let signatures: Vec<&str> = svix_signature.split_whitespace().collect();
    for sig_part in &signatures {
        if let Some(sig_value) = sig_part.strip_prefix("v1,") {
            if let Ok(decoded) = base64::Engine::decode(
                &base64::engine::general_purpose::STANDARD,
                sig_value,
            ) {
                if decoded.len() == expected.len()
                    && constant_time_eq_bytes(&decoded, &expected.as_slice())
                {
                    return Ok(());
                }
            }
        }
    }

    Err(WebhookError::SignatureMismatch)
}

/// Constant-time string comparison to prevent timing attacks.
fn constant_time_eq(a: &str, b: &str) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let a_bytes = a.as_bytes();
    let b_bytes = b.as_bytes();
    let mut result = 0u8;
    for i in 0..a.len() {
        result |= a_bytes[i] ^ b_bytes[i];
    }
    result == 0
}

/// Constant-time byte slice comparison to prevent timing attacks.
fn constant_time_eq_bytes(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut result = 0u8;
    for i in 0..a.len() {
        result |= a[i] ^ b[i];
    }
    result == 0
}

/// Errors that can occur during webhook signature verification.
#[derive(Debug, Clone)]
pub enum WebhookError {
    /// The HMAC key could not be created.
    InvalidKey,
    /// The computed signature does not match the provided signature.
    SignatureMismatch,
    /// A required header or field is missing from the request.
    MissingField(String),
}

impl std::fmt::Display for WebhookError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WebhookError::InvalidKey => write!(f, "Invalid webhook signing key"),
            WebhookError::SignatureMismatch => write!(f, "Webhook signature verification failed"),
            WebhookError::MissingField(field) => write!(f, "Missing required field: {}", field),
        }
    }
}

impl std::error::Error for WebhookError {}
