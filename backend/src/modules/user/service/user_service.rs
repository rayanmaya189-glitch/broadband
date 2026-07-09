use sqlx::PgPool;

use crate::common::cache::redis::RedisService;
use crate::common::errors::app_error::AppError;
use crate::common::security::crypto::{aes_decrypt, aes_encrypt, hash_password, sha256, verify_password};
use crate::common::security::jwt::{create_access_token, create_refresh_token_pair, refresh_token_expiry, MAX_SESSIONS};
use crate::common::security::totp;
use crate::common::utils::helpers::PaginatedResponse;
use crate::common::middleware::auth_middleware::Claims;
use crate::modules::user::mapper::user_mapper::{user_to_auth_response, user_to_response};
use crate::modules::user::repository::user_repository::UserRepository;
use crate::modules::user::request::user_request::*;
use crate::modules::user::response::user_response::*;

/// Get the AES-256-GCM key for encrypting 2FA secrets.
/// Falls back to deriving from JWT secret if TWO_FA_ENCRYPTION_KEY is not set.
fn get_2fa_encryption_key() -> [u8; 32] {
    if let Ok(env_key) = std::env::var("TWO_FA_ENCRYPTION_KEY") {
        if !env_key.is_empty() {
            let hash = sha256(env_key.as_bytes());
            let mut key = [0u8; 32];
            let bytes = hex::decode(&hash).unwrap_or_else(|_| vec![0u8; 32]);
            key[..bytes.len().min(32)].copy_from_slice(&bytes[..bytes.len().min(32)]);
            return key;
        }
    }
    let config = crate::common::config::config::Config::get();
    let key_material = format!("aeroxe-2fa-{}", config.jwt_secret);
    let hash = sha256(key_material.as_bytes());
    let mut key = [0u8; 32];
    let bytes = hex::decode(&hash).unwrap_or_else(|_| vec![0u8; 32]);
    key[..bytes.len().min(32)].copy_from_slice(&bytes[..bytes.len().min(32)]);
    key
}

/// Generate a 6-digit OTP.
fn generate_otp() -> String {
    use rand::Rng;
    format!("{:06}", rand::thread_rng().gen_range(100_000..999_999))
}

/// Generate TOTP secret (RFC 6238 compatible).
fn generate_totp_secret() -> String {
    let bytes: Vec<u8> = (0..20).map(|_| rand::random::<u8>()).collect();
    base32_encode(&bytes)
}

/// Base32 encoding for TOTP secrets.
fn base32_encode(data: &[u8]) -> String {
    const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";
    let mut result = String::new();
    let mut bits = 0u32;
    let mut value = 0i32;
    for &byte in data {
        value = (value << 8) | (byte as i32);
        bits += 8;
        while bits >= 5 {
            bits -= 5;
            result.push(ALPHABET[((value >> bits) & 31) as usize] as char);
        }
    }
    if bits > 0 {
        result.push(ALPHABET[((value << (5 - bits)) & 31) as usize] as char);
    }
    result
}

/// Generate a random temp token for 2FA flow.
fn generate_temp_token() -> String {
    use rand::Rng;
    let mut bytes = [0u8; 32];
    rand::thread_rng().fill(&mut bytes);
    hex::encode(bytes)
}

pub struct UserService<'a> {
    repo: UserRepository<'a>,
    redis: &'a RedisService,
}

impl<'a> UserService<'a> {
    pub fn new(pool: &'a PgPool, redis: &'a RedisService) -> Self {
        Self { repo: UserRepository::new(pool), redis }
    }

    pub async fn login(&self, req: &LoginRequest, ip: Option<&str>, device: Option<&str>) -> Result<LoginResponse, AppError> {
        let user = self.repo.find_by_email(&req.email).await?
            .ok_or_else(|| AppError::Validation("Invalid email or password".into()))?;

        if user.is_locked {
            if let Some(locked) = user.locked_until {
                if locked > chrono::Utc::now() {
                    return Err(AppError::Forbidden("Account is locked".into()));
                }
            }
        }
        if !user.is_active { return Err(AppError::Forbidden("Account is inactive".into())); }

        let valid = verify_password(&req.password, &user.password_hash)?;
        if !valid {
            self.repo.increment_failed_attempts(user.id).await?;
            return Err(AppError::Validation("Invalid credentials".into()));
        }

        // If 2FA is enabled, return temp token instead of full tokens
        if user.two_factor_enabled {
            let temp_token = generate_temp_token();
            let redis_key = format!("temp2fa:{}", temp_token);
            self.redis.set(&redis_key, &user.id.to_string(), Some(600)).await?;
            return Ok(LoginResponse {
                access_token: String::new(),
                refresh_token: temp_token,
                token_type: "temp_2fa",
                expires_in: 600,
                user: user_to_auth_response(&user, "user"),
            });
        }

        let role_name = self.repo.get_role_name(user.role_id).await?.unwrap_or_else(|| "user".into());
        let permissions = self.repo.resolve_permissions(user.role_id).await?;
        let (access_token, _) = create_access_token(user.id, &user.email, &role_name, user.role_id, user.branch_id, user.is_company_wide, &permissions)?;
        let (raw_refresh, hash_refresh) = create_refresh_token_pair();
        let expiry = refresh_token_expiry();

        let count = self.repo.count_active_tokens(user.id).await?;
        if count >= MAX_SESSIONS { self.repo.revoke_oldest_token(user.id).await?; }
        self.repo.create_refresh_token(user.id, &hash_refresh, device, ip, expiry).await?;
        self.repo.update_last_login(user.id).await?;

        let config = crate::common::config::config::Config::get();
        let user_resp = user_to_auth_response(&user, &role_name);
        Ok(LoginResponse { access_token, refresh_token: raw_refresh, token_type: "Bearer", expires_in: config.jwt_access_expiry_hours * 3600, user: user_resp })
    }

    pub async fn register(&self, req: &RegisterRequest) -> Result<RegisterResponse, AppError> {
        if self.repo.find_by_email(&req.email).await?.is_some() {
            return Err(AppError::Conflict("Email already registered".into()));
        }
        if self.repo.find_by_phone(&req.phone).await?.is_some() {
            return Err(AppError::Conflict("Phone already registered".into()));
        }

        let password_hash = hash_password(&req.password)?;
        let user = self.repo.create(&req.email, &password_hash, &req.name, &req.phone, 10, req.branch_id, false).await?;

        let role_name = self.repo.get_role_name(user.role_id).await?.unwrap_or_else(|| "customer".into());
        let permissions = self.repo.resolve_permissions(user.role_id).await?;
        let (access_token, _) = create_access_token(user.id, &user.email, &role_name, user.role_id, user.branch_id, user.is_company_wide, &permissions)?;
        let (raw_refresh, hash_refresh) = create_refresh_token_pair();
        let expiry = refresh_token_expiry();
        self.repo.create_refresh_token(user.id, &hash_refresh, None, None, expiry).await?;
        self.repo.update_last_login(user.id).await?;

        let config = crate::common::config::config::Config::get();
        let user_resp = user_to_auth_response(&user, &role_name);
        Ok(RegisterResponse { user: user_resp, access_token, refresh_token: raw_refresh, token_type: "Bearer", expires_in: config.jwt_access_expiry_hours * 3600 })
    }

    pub async fn refresh_token(&self, req: &RefreshTokenRequest) -> Result<TokenRefreshResponse, AppError> {
        let hash = sha256(req.refresh_token.as_bytes());
        let token_row = self.repo.find_valid_refresh_token(&hash).await?
            .ok_or_else(|| AppError::Unauthorized)?;
        let user = self.repo.find_by_id(token_row.user_id).await?
            .ok_or_else(|| AppError::Unauthorized)?;
        if !user.is_active { return Err(AppError::Forbidden("Account is inactive".into())); }

        self.repo.revoke_refresh_token(&hash).await?;
        let role_name = self.repo.get_role_name(user.role_id).await?.unwrap_or_else(|| "user".into());
        let permissions = self.repo.resolve_permissions(user.role_id).await?;
        let (access_token, _) = create_access_token(user.id, &user.email, &role_name, user.role_id, user.branch_id, user.is_company_wide, &permissions)?;
        let (raw_refresh, hash_refresh) = create_refresh_token_pair();
        let expiry = refresh_token_expiry();
        self.repo.create_refresh_token(user.id, &hash_refresh, token_row.device_info.as_deref(), token_row.ip_address.as_deref(), expiry).await?;

        let config = crate::common::config::config::Config::get();
        Ok(TokenRefreshResponse { access_token, refresh_token: raw_refresh, token_type: "Bearer", expires_in: config.jwt_access_expiry_hours * 3600 })
    }

    pub async fn logout(&self, req: &LogoutRequest) -> Result<MessageResponse, AppError> {
        if let Some(ref token) = req.refresh_token {
            let hash = sha256(token.as_bytes());
            self.repo.revoke_refresh_token(&hash).await?;
        }
        Ok(MessageResponse { message: "Logged out successfully".into() })
    }

    pub async fn logout_all(&self, user_id: i64) -> Result<MessageResponse, AppError> {
        let count = self.repo.revoke_all_user_tokens(user_id).await?;
        Ok(MessageResponse { message: format!("Revoked {count} active sessions") })
    }

    pub async fn change_password(&self, user_id: i64, req: &ChangePasswordRequest) -> Result<MessageResponse, AppError> {
        let user = self.repo.find_by_id(user_id).await?.ok_or_else(|| AppError::NotFound("User not found".into()))?;
        if !verify_password(&req.current_password, &user.password_hash)? {
            return Err(AppError::Validation("Invalid current password".into()));
        }
        let new_hash = hash_password(&req.new_password)?;
        self.repo.update_password(user_id, &new_hash).await?;
        self.repo.revoke_all_user_tokens(user_id).await?;
        Ok(MessageResponse { message: "Password changed successfully".into() })
    }

    pub async fn get_current_user(&self, claims: &Claims) -> Result<AuthUserResponse, AppError> {
        let user = self.repo.find_by_id(claims.sub).await?.ok_or_else(|| AppError::NotFound("User not found".into()))?;
        Ok(user_to_auth_response(&user, &claims.role))
    }

    pub async fn list_sessions(&self, user_id: i64, current_jti: &str) -> Result<Vec<SessionResponse>, AppError> {
        let sessions = self.repo.list_active_sessions(user_id).await?;
        Ok(sessions.into_iter().map(|s| SessionResponse { id: s.id, device_info: s.device_info, ip_address: s.ip_address, created_at: s.created_at, expires_at: s.expires_at, is_current: s.token_hash == current_jti }).collect())
    }

    pub async fn list_users(&self, query: &ListUsersQuery) -> Result<PaginatedResponse<UserResponse>, AppError> {
        let offset = query.pagination.offset();
        let limit = query.pagination.limit_i64() as u32;
        self.repo.list(offset, limit, query.role_id, query.branch_id, query.is_active, query.pagination.search.as_deref()).await
    }

    pub async fn get_user(&self, user_id: i64) -> Result<UserDetailResponse, AppError> {
        let user = self.repo.find_by_id(user_id).await?.ok_or_else(|| AppError::NotFound("User not found".into()))?;
        let role_name = self.repo.get_role_name(user.role_id).await?;
        Ok(user_to_response(&user, role_name.as_deref()))
    }

    pub async fn create_user(&self, req: &CreateUserRequest) -> Result<UserDetailResponse, AppError> {
        if self.repo.email_exists(&req.email, None).await? { return Err(AppError::Conflict("Email already registered".into())); }
        if self.repo.phone_exists(&req.phone, None).await? { return Err(AppError::Conflict("Phone already registered".into())); }
        if !self.repo.role_exists(req.role_id).await? { return Err(AppError::NotFound("Role not found".into())); }

        let password_hash = hash_password(&req.password)?;
        let user = self.repo.create(&req.email, &password_hash, &req.name, &req.phone, req.role_id, req.branch_id, req.is_company_wide.unwrap_or(false)).await?;
        let role_name = self.repo.get_role_name(user.role_id).await?;
        Ok(user_to_response(&user, role_name.as_deref()))
    }

    pub async fn update_user(&self, user_id: i64, req: &UpdateUserRequest) -> Result<UserDetailResponse, AppError> {
        if self.repo.find_by_id(user_id).await?.is_none() { return Err(AppError::NotFound("User not found".into())); }
        if let Some(ref phone) = req.phone {
            if self.repo.phone_exists(phone, Some(user_id)).await? { return Err(AppError::Conflict("Phone already registered".into())); }
        }
        let user = self.repo.update(user_id, req.name.as_deref(), req.phone.as_deref(), req.branch_id, req.avatar_url.as_deref()).await?;
        let role_name = self.repo.get_role_name(user.role_id).await?;
        Ok(user_to_response(&user, role_name.as_deref()))
    }

    pub async fn delete_user(&self, user_id: i64, current_id: i64) -> Result<MessageResponse, AppError> {
        if user_id == current_id { return Err(AppError::Validation("Cannot delete your own account".into())); }
        let user = self.repo.find_by_id(user_id).await?.ok_or_else(|| AppError::NotFound("User not found".into()))?;
        let role_name = self.repo.get_role_name(user.role_id).await?;
        if role_name.as_deref() == Some("super_admin") { return Err(AppError::Forbidden("Cannot deactivate a super admin".into())); }
        self.repo.soft_delete(user_id).await?;
        Ok(MessageResponse { message: "User deleted successfully".into() })
    }

    pub async fn update_status(&self, user_id: i64, req: &UpdateUserStatusRequest, current_id: i64) -> Result<UserDetailResponse, AppError> {
        if user_id == current_id { return Err(AppError::Validation("Cannot change your own status".into())); }
        let is_active = match req.status.as_str() {
            "active" => true,
            "inactive" => false,
            _ => return Err(AppError::Validation("Invalid status".into())),
        };
        let existing = self.repo.find_by_id(user_id).await?.ok_or_else(|| AppError::NotFound("User not found".into()))?;
        let role_name = self.repo.get_role_name(existing.role_id).await?;
        if role_name.as_deref() == Some("super_admin") && !is_active { return Err(AppError::Forbidden("Cannot deactivate a super admin".into())); }
        let user = self.repo.update_status(user_id, is_active).await?;
        Ok(user_to_response(&user, role_name.as_deref()))
    }

    // ── OTP Login (Redis-backed + rate limited) ─────────────

    pub async fn send_otp(&self, req: &SendOtpRequest) -> Result<OtpSentResponse, AppError> {
        let rate_key = format!("ratelimit:otp:{}", req.phone);
        let count = self.redis.incr(&rate_key, 3600).await?;
        if count > 5 {
            return Err(AppError::RateLimited);
        }

        let otp = generate_otp();
        let redis_key = format!("otp:login:{}", req.phone);
        self.redis.del(&[&redis_key]).await.ok();
        self.redis.set(&redis_key, &otp, Some(300)).await?;

        tracing::info!(phone = %req.phone, "OTP generated (dev mode: {})", otp);
        Ok(OtpSentResponse { message: "OTP sent successfully".into(), expires_in: 300 })
    }

    pub async fn verify_otp(&self, req: &VerifyOtpRequest, ip: Option<&str>, device: Option<&str>) -> Result<LoginResponse, AppError> {
        let rate_key = format!("ratelimit:otp:verify:{}", req.phone);
        let count = self.redis.incr(&rate_key, 600).await?;
        if count > 10 {
            return Err(AppError::RateLimited);
        }

        let redis_key = format!("otp:login:{}", req.phone);
        let stored_otp = self.redis.get(&redis_key).await?
            .ok_or_else(|| AppError::Validation("OTP expired or not found".into()))?;

        if !totp::constant_time_eq(&stored_otp, &req.otp) {
            return Err(AppError::Validation("Invalid OTP".into()));
        }

        self.redis.del(&[&redis_key]).await.ok();
        self.redis.del(&[&rate_key]).await.ok();

        let user = if let Some(u) = self.repo.find_by_phone(&req.phone).await? {
            u
        } else {
            let password_hash = hash_password(&uuid::Uuid::new_v4().to_string())?;
            let suffix = if req.phone.len() > 4 { &req.phone[req.phone.len()-4..] } else { &req.phone };
            self.repo.create(&format!("user_{}@aeroxe.local", suffix), &password_hash, &format!("User {}", suffix), &req.phone, 10, None, false).await?
        };

        let role_name = self.repo.get_role_name(user.role_id).await?.unwrap_or_else(|| "user".into());
        let permissions = self.repo.resolve_permissions(user.role_id).await?;
        let (access_token, _) = create_access_token(user.id, &user.email, &role_name, user.role_id, user.branch_id, user.is_company_wide, &permissions)?;
        let (raw_refresh, hash_refresh) = create_refresh_token_pair();
        let expiry = refresh_token_expiry();
        let count = self.repo.count_active_tokens(user.id).await?;
        if count >= MAX_SESSIONS { self.repo.revoke_oldest_token(user.id).await?; }
        self.repo.create_refresh_token(user.id, &hash_refresh, device, ip, expiry).await?;
        self.repo.update_last_login(user.id).await?;

        let config = crate::common::config::config::Config::get();
        let user_resp = user_to_auth_response(&user, &role_name);
        Ok(LoginResponse { access_token, refresh_token: raw_refresh, token_type: "Bearer", expires_in: config.jwt_access_expiry_hours * 3600, user: user_resp })
    }

    // ── Password Reset (Redis-backed + rate limited) ────────

    pub async fn request_password_reset(&self, req: &PasswordResetRequest) -> Result<PasswordResetResponse, AppError> {
        let rate_key = format!("ratelimit:pwreset:{}", req.email);
        let count = self.redis.incr(&rate_key, 3600).await?;
        if count > 3 {
            return Err(AppError::RateLimited);
        }

        let user = self.repo.find_by_email(&req.email).await?;
        if user.is_some() {
            let token = crate::common::security::crypto::generate_token();
            let hash = sha256(token.as_bytes());
            let redis_key = format!("pwreset:{}", hash);
            self.redis.set(&redis_key, &req.email, Some(3600)).await?;
            tracing::info!(email = %req.email, "Password reset token generated (dev)");
        }
        Ok(PasswordResetResponse { message: "If the email exists, a reset link has been sent".into() })
    }

    pub async fn confirm_password_reset(&self, req: &PasswordResetConfirmRequest) -> Result<MessageResponse, AppError> {
        let hash = sha256(req.token.as_bytes());
        let redis_key = format!("pwreset:{}", hash);
        let email = self.redis.get(&redis_key).await?
            .ok_or_else(|| AppError::Validation("Invalid or expired reset token".into()))?;

        let user = self.repo.find_by_email(&email).await?
            .ok_or_else(|| AppError::NotFound("User not found".into()))?;
        let new_hash = hash_password(&req.new_password)?;
        self.repo.update_password(user.id, &new_hash).await?;
        self.redis.del(&[&redis_key]).await.ok();
        self.repo.revoke_all_user_tokens(user.id).await?;
        Ok(MessageResponse { message: "Password reset successfully".into() })
    }

    // ── 2FA (TOTP) — Encrypted + real verification ──────────

    pub async fn enable_2fa(&self, user_id: i64) -> Result<TwoFaSetupResponse, AppError> {
        let user = self.repo.find_by_id(user_id).await?.ok_or_else(|| AppError::NotFound("User not found".into()))?;
        if user.two_factor_enabled {
            return Err(AppError::Validation("2FA is already enabled".into()));
        }
        let secret = generate_totp_secret();
        let key = get_2fa_encryption_key();
        let encrypted = aes_encrypt(secret.as_bytes(), &key)?;
        let encrypted_hex = hex::encode(&encrypted);
        self.repo.store_2fa_secret_pending(user_id, &encrypted_hex).await?;
        let otpauth_url = format!("otpauth://totp/AeroXe:{}?secret={}&issuer=AeroXe", user.email, secret);
        Ok(TwoFaSetupResponse { secret, otpauth_url })
    }

    pub async fn confirm_2fa(&self, user_id: i64, req: &Confirm2FaRequest) -> Result<TwoFaEnabledResponse, AppError> {
        let encrypted_hex = self.repo.get_pending_2fa_secret(user_id).await?
            .ok_or_else(|| AppError::Validation("No pending 2FA setup".into()))?;

        let key = get_2fa_encryption_key();
        let encrypted_bytes = hex::decode(&encrypted_hex)
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Invalid 2FA secret format: {e}")))?;
        let secret_bytes = aes_decrypt(&encrypted_bytes, &key)?;
        let secret = String::from_utf8(secret_bytes)
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Invalid 2FA secret encoding: {e}")))?;

        if !totp::verify_totp(&secret, &req.code)
            .map_err(|e| AppError::Internal(anyhow::anyhow!("TOTP verification failed: {e}")))?
        {
            return Err(AppError::Validation("Invalid TOTP code".into()));
        }

        self.repo.enable_2fa(user_id, &encrypted_hex).await?;

        let backup_codes: Vec<String> = (0..10).map(|_| {
            use rand::Rng;
            format!("{:08}", rand::thread_rng().gen_range(0u32..99_999_999u32))
        }).collect();
        let hashed_codes: Vec<String> = backup_codes.iter().map(|c| sha256(c.as_bytes())).collect();
        let codes_json = serde_json::to_string(&hashed_codes).unwrap_or_else(|_| "[]".into());
        self.repo.store_backup_codes(user_id, &codes_json).await?;

        Ok(TwoFaEnabledResponse { message: "2FA enabled successfully".into(), backup_codes })
    }

    pub async fn disable_2fa(&self, user_id: i64) -> Result<MessageResponse, AppError> {
        let user = self.repo.find_by_id(user_id).await?.ok_or_else(|| AppError::NotFound("User not found".into()))?;
        if !user.two_factor_enabled {
            return Err(AppError::Validation("2FA is not enabled".into()));
        }
        self.repo.disable_2fa(user_id).await?;
        Ok(MessageResponse { message: "2FA disabled successfully".into() })
    }

    pub async fn verify_2fa_login(&self, req: &Verify2FaRequest) -> Result<LoginResponse, AppError> {
        let rate_key = format!("ratelimit:2fa:{}", req.temp_token);
        let count = self.redis.incr(&rate_key, 600).await?;
        if count > 5 {
            return Err(AppError::RateLimited);
        }

        let user_id_str = self.redis.get(&format!("temp2fa:{}", req.temp_token)).await?
            .ok_or_else(|| AppError::Validation("Invalid or expired 2FA session".into()))?;
        let user_id: i64 = user_id_str.parse()
            .map_err(|_| AppError::Validation("Invalid 2FA session".into()))?;

        let user = self.repo.find_by_id(user_id).await?
            .ok_or_else(|| AppError::NotFound("User not found".into()))?;

        // Verify 2FA is actually enabled for this user
        if !user.two_factor_enabled {
            return Err(AppError::Forbidden("2FA is not enabled for this account".into()));
        }

        let is_backup_code = req.code.len() == 8 && req.code.chars().all(|c| c.is_ascii_digit());

        if is_backup_code {
            let verified = self.repo.consume_backup_code(user_id, &req.code).await?;
            if !verified {
                return Err(AppError::Validation("Invalid backup code".into()));
            }
        } else {
            let encrypted_hex = self.repo.get_encrypted_2fa_secret(user_id).await?
                .ok_or_else(|| AppError::Validation("2FA not configured".into()))?;
            let key = get_2fa_encryption_key();
            let encrypted_bytes = hex::decode(&encrypted_hex)
                .map_err(|e| AppError::Internal(anyhow::anyhow!("Invalid 2FA secret format: {e}")))?;
            let secret_bytes = aes_decrypt(&encrypted_bytes, &key)?;
            let secret = String::from_utf8(secret_bytes)
                .map_err(|e| AppError::Internal(anyhow::anyhow!("Invalid 2FA secret encoding: {e}")))?;

            if !totp::verify_totp(&secret, &req.code)
                .map_err(|e| AppError::Internal(anyhow::anyhow!("TOTP verification failed: {e}")))?
            {
                return Err(AppError::Validation("Invalid TOTP code".into()));
            }
        }

        self.redis.del(&[&format!("temp2fa:{}", req.temp_token)]).await.ok();
        self.redis.del(&[&rate_key]).await.ok();

        let role_name = self.repo.get_role_name(user.role_id).await?.unwrap_or_else(|| "user".into());
        let permissions = self.repo.resolve_permissions(user.role_id).await?;
        let (access_token, _) = create_access_token(user.id, &user.email, &role_name, user.role_id, user.branch_id, user.is_company_wide, &permissions)?;
        let (raw_refresh, hash_refresh) = create_refresh_token_pair();
        let expiry = refresh_token_expiry();
        let count = self.repo.count_active_tokens(user.id).await?;
        if count >= MAX_SESSIONS { self.repo.revoke_oldest_token(user.id).await?; }
        self.repo.create_refresh_token(user.id, &hash_refresh, None, None, expiry).await?;
        self.repo.update_last_login(user.id).await?;

        let config = crate::common::config::config::Config::get();
        let user_resp = user_to_auth_response(&user, &role_name);
        Ok(LoginResponse { access_token, refresh_token: raw_refresh, token_type: "Bearer", expires_in: config.jwt_access_expiry_hours * 3600, user: user_resp })
    }
}
