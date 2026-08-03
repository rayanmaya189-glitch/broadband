//! OTP Login Service
//!
//! Generates, stores, delivers and verifies one-time passwords for login
//! via SMS (MSG91), WhatsApp, Telegram bot and FCM push (Firebase).
//!
//! Security properties:
//! - OTP codes are stored hashed (SHA-256), never in plaintext.
//! - Codes expire after `AuthRules::OTP_EXPIRY_SECONDS` (5 minutes).
//! - Per-phone rate limit of `AuthRules::OTP_RATE_LIMIT_PER_HOUR` requests/hour.
//! - One-time use: verified codes are marked and cannot be reused.

use chrono::{Duration, Utc};
use rand::Rng;
use redis::AsyncCommands;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, Set,
};
use sha2::{Digest, Sha256};
use tracing::{debug, info, warn};

use crate::infrastructure::cache::CacheKeys;
use crate::modules::identity::domain::entities::{otp_code, user};
use crate::modules::identity::domain::rules::auth_rules::AuthRules;
use crate::modules::integrations::factory::DeviceAdapterFactory;
use crate::modules::integrations::push::fcm::FcmAdapter;
use crate::modules::integrations::telegram::TelegramBotAdapter;
use crate::modules::integrations::whatsapp::WhatsAppAdapter;
use crate::shared::errors::AppError;

/// OTP delivery channel
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OtpChannel {
    Sms,
    WhatsApp,
    Telegram,
    Firebase,
}

impl OtpChannel {
    pub fn from_str(value: &str) -> Result<Self, AppError> {
        match value.to_lowercase().as_str() {
            "sms" | "msg91" => Ok(Self::Sms),
            "whatsapp" | "wa" => Ok(Self::WhatsApp),
            "telegram" | "tg" => Ok(Self::Telegram),
            "firebase" | "fcm" => Ok(Self::Firebase),
            other => Err(AppError::BadRequest(format!(
                "Unsupported OTP channel '{}'. Supported: sms, whatsapp, telegram, firebase",
                other
            ))),
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Sms => "sms",
            Self::WhatsApp => "whatsapp",
            Self::Telegram => "telegram",
            Self::Firebase => "firebase",
        }
    }
}

/// Result of a successful OTP send
#[derive(Debug, Clone, serde::Serialize)]
pub struct OtpSent {
    pub expires_in_secs: u64,
    pub channel: String,
    pub provider_request_id: Option<String>,
}

/// Normalize a phone number to a canonical 10-digit Indian number.
pub fn normalize_phone(phone: &str) -> String {
    let digits: String = phone.chars().filter(|c| c.is_ascii_digit()).collect();
    if digits.len() == 12 && digits.starts_with("91") {
        digits[2..].to_string()
    } else if digits.len() == 11 && digits.starts_with('0') {
        digits[1..].to_string()
    } else {
        digits
    }
}

fn generate_otp() -> String {
    let mut rng = rand::thread_rng();
    format!("{:06}", rng.gen_range(100000..999999))
}

fn hash_otp(code: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(code.as_bytes());
    hex::encode(hasher.finalize())
}

fn otp_message(code: &str, app_name: &str) -> String {
    format!(
        "Your {} verification code is {}. It is valid for {} minutes. Do not share it with anyone.",
        app_name,
        code,
        AuthRules::OTP_EXPIRY_SECONDS / 60
    )
}

/// Send an OTP to the given phone via the selected channel.
pub async fn send_otp(
    db: &DatabaseConnection,
    redis: &mut redis::aio::ConnectionManager,
    app_name: &str,
    phone: &str,
    channel: &OtpChannel,
    fcm_token: Option<&str>,
) -> Result<OtpSent, AppError> {
    let phone = normalize_phone(phone);
    if !AuthRules::is_valid_indian_phone(&phone) {
        return Err(AppError::BadRequest(format!(
            "Invalid phone number '{}'",
            phone
        )));
    }

    // Per-phone rate limit (sliding hourly window via Redis INCR).
    let rl_key = format!("{}:rl", CacheKeys::otp(&phone));
    let count: i64 = redis
        .incr(&rl_key, 1)
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Redis rate limit error: {}", e)))?;
    if count == 1 {
        let _: () = redis
            .expire(&rl_key, 3600)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Redis expire error: {}", e)))?;
    }
    if count as u32 > AuthRules::OTP_RATE_LIMIT_PER_HOUR {
        warn!(phone = %phone, attempts = count, "OTP rate limit exceeded");
        return Err(AppError::RateLimited);
    }

    let code = generate_otp();
    let expires_at = Utc::now() + Duration::seconds(AuthRules::OTP_EXPIRY_SECONDS as i64);

    // Persist hashed code for verification.
    let row = otp_code::ActiveModel {
        phone: Set(phone.clone()),
        channel: Set(channel.as_str().to_string()),
        code_hash: Set(hash_otp(&code)),
        expires_at: Set(expires_at),
        verified_at: Set(None),
        created_at: Set(Utc::now()),
        ..Default::default()
    };
    let saved = row.insert(db).await?;
    debug!(id = saved.id, phone = %phone, channel = %channel.as_str(), "Stored OTP");

    let message = otp_message(&code, app_name);

    let provider_request_id = match channel {
        OtpChannel::Sms => {
            let provider = DeviceAdapterFactory::create_sms_provider("msg91");
            let request_id = provider.send_sms(&phone, &message, None).await?;
            Some(request_id)
        }
        OtpChannel::WhatsApp => {
            let adapter = WhatsAppAdapter::from_env();
            if !adapter.is_configured() {
                return Err(AppError::External(
                    "WhatsApp adapter is not configured (WHATSAPP_ACCESS_TOKEN / WHATSAPP_PHONE_NUMBER_ID)".to_string(),
                ));
            }
            let status = adapter.send_text_message(&phone, &message).await?;
            Some(status.message_id)
        }
        OtpChannel::Telegram => {
            let adapter = TelegramBotAdapter::from_env();
            if !adapter.is_configured() {
                return Err(AppError::External(
                    "Telegram bot is not configured (TELEGRAM_BOT_TOKEN)".to_string(),
                ));
            }
            let chat_id = lookup_telegram_chat(redis, &phone).await?;
            let sent = adapter.send_message(chat_id, &message).await?;
            Some(sent.message_id.to_string())
        }
        OtpChannel::Firebase => {
            let mut adapter = FcmAdapter::from_env();
            let token = match fcm_token {
                Some(t) if !t.is_empty() => {
                    bind_fcm_token(redis, &phone, t).await?;
                    t.to_string()
                }
                _ => lookup_fcm_token(redis, &phone).await?,
            };
            let status = adapter
                .send_push(
                    &token,
                    &format!("{} OTP", app_name),
                    &format!("Your {} code is {}.", app_name, code),
                    None,
                )
                .await?;
            Some(status.message_id)
        }
    };

    info!(phone = %phone, channel = %channel.as_str(), "OTP delivered");

    Ok(OtpSent {
        expires_in_secs: AuthRules::OTP_EXPIRY_SECONDS,
        channel: channel.as_str().to_string(),
        provider_request_id,
    })
}

/// Verify an OTP code. Marks the code as used on success.
pub async fn verify_otp(
    db: &DatabaseConnection,
    redis: &mut redis::aio::ConnectionManager,
    phone: &str,
    channel: &OtpChannel,
    code: &str,
) -> Result<(), AppError> {
    let phone = normalize_phone(phone);
    let now = Utc::now();

    let row = otp_code::Entity::find()
        .filter(otp_code::Column::Phone.eq(&phone))
        .filter(otp_code::Column::Channel.eq(channel.as_str()))
        .filter(otp_code::Column::VerifiedAt.is_null())
        .filter(otp_code::Column::ExpiresAt.gt(now))
        .order_by_desc(otp_code::Column::Id)
        .one(db)
        .await?
        .ok_or_else(|| AppError::BadRequest("Invalid or expired OTP".to_string()))?;

    if row.code_hash != hash_otp(code) {
        return Err(AppError::BadRequest("Invalid or expired OTP".to_string()));
    }

    // Mark as verified (one-time use).
    let mut active: otp_code::ActiveModel = row.into();
    active.verified_at = Set(Some(Utc::now()));
    active.update(db).await?;

    // Reset the hourly rate-limit counter so a fresh login session can begin.
    let rl_key = format!("{}:rl", CacheKeys::otp(&phone));
    let _: () = redis
        .del(&rl_key)
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Redis del error: {}", e)))?;

    Ok(())
}

/// Bind a Telegram chat_id to a phone for OTP delivery.
pub async fn bind_telegram_chat(
    redis: &mut redis::aio::ConnectionManager,
    phone: &str,
    chat_id: i64,
) -> Result<(), AppError> {
    let phone = normalize_phone(phone);
    if !AuthRules::is_valid_indian_phone(&phone) {
        return Err(AppError::BadRequest(format!(
            "Invalid phone number '{}'",
            phone
        )));
    }
    let key = format!("{}:telegram", CacheKeys::otp(&phone));
    let _: () = redis
        .set_ex(&key, chat_id, 30 * 24 * 3600)
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Redis set error: {}", e)))?;
    info!(phone = %phone, chat_id = chat_id, "Bound Telegram chat to phone");
    Ok(())
}

async fn lookup_telegram_chat(
    redis: &mut redis::aio::ConnectionManager,
    phone: &str,
) -> Result<i64, AppError> {
    let key = format!("{}:telegram", CacheKeys::otp(phone));
    let chat_id: Option<i64> = redis
        .get(&key)
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Redis get error: {}", e)))?;
    match chat_id {
        Some(id) => Ok(id),
        None => Err(AppError::BadRequest(
            "No Telegram chat linked to this phone. Message the bot with: /login <phone>"
                .to_string(),
        )),
    }
}

/// Store an FCM device token for a phone.
pub async fn bind_fcm_token(
    redis: &mut redis::aio::ConnectionManager,
    phone: &str,
    token: &str,
) -> Result<(), AppError> {
    let key = format!("{}:fcm", CacheKeys::otp(phone));
    let _: () = redis
        .set_ex(&key, token, 90 * 24 * 3600)
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Redis set error: {}", e)))?;
    Ok(())
}

async fn lookup_fcm_token(
    redis: &mut redis::aio::ConnectionManager,
    phone: &str,
) -> Result<String, AppError> {
    let key = format!("{}:fcm", CacheKeys::otp(phone));
    let token: Option<String> = redis
        .get(&key)
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Redis get error: {}", e)))?;
    token.ok_or_else(|| {
        AppError::BadRequest(
            "No FCM device token registered for this phone. Provide fcm_token in the request."
                .to_string(),
        )
    })
}

/// Find a user by phone number.
pub async fn find_user_by_phone(
    db: &DatabaseConnection,
    phone: &str,
) -> Result<Option<user::Model>, AppError> {
    let phone = normalize_phone(phone);
    Ok(user::Entity::find()
        .filter(user::Column::Phone.eq(&phone))
        .one(db)
        .await?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_phone() {
        assert_eq!(normalize_phone("+919876543210"), "9876543210");
        assert_eq!(normalize_phone("9876543210"), "9876543210");
        assert_eq!(normalize_phone("09876543210"), "9876543210");
        assert_eq!(normalize_phone("+91 98765 43210"), "9876543210");
    }

    #[test]
    fn test_channel_parsing() {
        assert_eq!(OtpChannel::from_str("sms").unwrap(), OtpChannel::Sms);
        assert_eq!(
            OtpChannel::from_str("whatsapp").unwrap(),
            OtpChannel::WhatsApp
        );
        assert_eq!(
            OtpChannel::from_str("telegram").unwrap(),
            OtpChannel::Telegram
        );
        assert_eq!(
            OtpChannel::from_str("firebase").unwrap(),
            OtpChannel::Firebase
        );
        assert!(OtpChannel::from_str("email").is_err());
    }

    #[test]
    fn test_hash_otp_is_stable() {
        let a = hash_otp("123456");
        let b = hash_otp("123456");
        let c = hash_otp("654321");
        assert_eq!(a, b);
        assert_ne!(a, c);
    }
}
