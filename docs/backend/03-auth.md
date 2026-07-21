# AeroXe Backend — Authentication Module

> **Req Ref:** §19.1–19.3 Authentication, JWT, 2FA

---

## 1. Overview

Handles user authentication via email/phone + password, OTP-based login, JWT token management, and two-factor authentication (TOTP).

## 2. Database Tables

### users (shared with §6-users.md)
```sql
CREATE TABLE users (
    id BIGSERIAL PRIMARY KEY,
    email VARCHAR(255) NOT NULL UNIQUE,
    phone VARCHAR(20) NOT NULL UNIQUE,
    password_hash VARCHAR(255),
    name VARCHAR(255) NOT NULL,
    avatar_url TEXT,
    branch_id BIGINT REFERENCES branches(id),
    status VARCHAR(20) DEFAULT 'active'
        CHECK (status IN ('active', 'inactive', 'suspended', 'locked')),
    last_login_at TIMESTAMPTZ,
    failed_login_attempts INTEGER DEFAULT 0,
    locked_until TIMESTAMPTZ,
    two_factor_enabled BOOLEAN DEFAULT FALSE,
    two_factor_secret VARCHAR(255),
    phone_verified BOOLEAN DEFAULT FALSE,
    email_verified BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    deleted_at TIMESTAMPTZ
);
```

### user_sessions
```sql
CREATE TABLE user_sessions (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    refresh_token_hash VARCHAR(255) NOT NULL,
    ip_address INET,
    user_agent TEXT,
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);
```

## 3. API Endpoints

> **API Convention:** Protobuf-first. No GET, no PUT, no path variables, no query strings. See `API-CONVENTIONS.md`.

| Method | Path | Auth | Description |
|--------|------|------|-------------|
| POST | `/api/v1/auth/login` | No | Email/password login |
| POST | `/api/v1/auth/login/otp/send` | No | Send OTP to phone |
| POST | `/api/v1/auth/login/otp/verify` | No | Verify OTP & login |
| POST | `/api/v1/auth/register` | No | Self-service registration |
| POST | `/api/v1/auth/refresh` | Refresh Token | Refresh access token |
| POST | `/api/v1/auth/logout` | Yes | Invalidate session |
| POST | `/api/v1/auth/logout/all` | Yes | Invalidate all sessions |
| POST | `/api/v1/auth/password/change` | Yes | Change password |
| POST | `/api/v1/auth/password/reset/request` | No | Request password reset |
| POST | `/api/v1/auth/password/reset/confirm` | No | Confirm password reset |
| POST | `/api/v1/auth/2fa/enable` | Yes | Enable TOTP 2FA |
| POST | `/api/v1/auth/2fa/verify` | Yes | Verify 2FA setup |
| POST | `/api/v1/auth/2fa/disable` | Yes | Disable 2FA |
| POST | `/api/v1/auth/2fa/backup-codes` | Yes | Generate backup codes |
| POST | `/api/v1/auth/sessions/list` | Yes | List active sessions |
| DELETE | `/api/v1/auth/sessions/delete` | Yes | Revoke specific session |

## 4. JWT Token Structure

### Access Token (RS256, 24h expiry)
```json
{
  "sub": "user-uuid",
  "email": "admin@aeroxe.com",
  "role": "network_admin",
  "branch_id": 1,
  "is_company_wide": false,
  "permissions": ["device.*.view", "network.*.view"],
  "iat": 1783500000,
  "exp": 1783586400,
  "jti": "unique-token-id"
}
```

### Refresh Token (opaque, 7d expiry)
- 256-bit random token
- Stored as SHA-256 hash in `user_sessions`
- One refresh token per session (device)

## 5. Login Flow

### Email/Password
```
1. Client POST /auth/login { email, password }
2. Lookup user by email
3. Check: user status != 'locked'
4. Verify password (argon2id)
5. Check: 2FA enabled?
   ├── YES → Return { requires_2fa: true, temp_token }
   │         Client POST /auth/2fa/verify { temp_token, totp_code }
   └── NO  → Generate JWT + refresh token
6. Create user_session record
7. Store session in Redis
8. Update last_login_at, reset failed_login_attempts
9. Return { access_token, refresh_token, user }
```

### OTP Login
```
1. Client POST /auth/login/otp/send { phone }
2. Check rate limit (5 OTP/hour per phone)
3. Generate 6-digit OTP
4. Store in Redis: SET otp:login:{phone} {otp} EX 300
5. Send OTP via SMS provider
6. Client POST /auth/login/otp/verify { phone, otp }
7. Verify OTP from Redis
8. Lookup or create user by phone
9. Generate JWT + refresh token
10. Return tokens
```

## 6. 2FA (TOTP) Flow

```
Enable:
1. Client POST /auth/2fa/enable
2. Generate TOTP secret (RFC 6238)
3. Store secret temporarily in Redis (pending verification)
4. Return { secret, otpauth_url, qr_code_base64 }

Verify:
1. Client POST /auth/2fa/verify { code }
2. Verify TOTP code against stored secret
3. Generate 10 backup codes (single-use)
4. Persist secret + backup codes to database
5. Set two_factor_enabled = true

Login with 2FA:
1. After password/OTP verification
2. Return { requires_2fa: true, temp_token }
3. Client POST /auth/2fa/verify { temp_token, code }
4. Verify code (TOTP or backup code)
5. Complete login, issue tokens
```

## 7. Password Hashing

- **Algorithm:** argon2id
- **Memory:** 65536 KB (64 MB)
- **Iterations:** 3
- **Parallelism:** 4
- **Output:** 32 bytes

```rust
use argon2::{Argon2, PasswordHasher, PasswordHash, PasswordVerifier};

pub fn hash_password(password: &str) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default()
        .hash_password(password.as_bytes(), &salt)?
        .to_string();
    Ok(hash)
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool> {
    let parsed = PasswordHash::new(hash)?;
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed)
        .is_ok())
}
```

## 8. Account Lockout

```
Failed login attempt:
1. Increment failed_login_attempts
2. If failed_login_attempts >= 5:
   - Set locked_until = NOW() + 30 minutes
   - Set status = 'locked'
   - Send notification to user
3. If locked_until > NOW():
   - Reject login with "Account locked" message
```

## 9. Session Management

- Max active sessions per user: 5
- On new login when at limit: revoke oldest session
- Session tracking in Redis for fast lookup
- Logout: delete Redis key + mark session inactive

## 10. RBAC Permissions

```
auth.login
auth.logout
auth.register
auth.password.change
auth.password.reset
auth.2fa.enable
auth.2fa.disable
auth.sessions.view
auth.sessions.revoke
```

## Known Issues & Gap Reference (v3.0)

> **Full details:** `GAP-finance-compliance.md`, `GAP-architecture-patterns.md`, `DESIGN-GAPS-DEEP-ANALYSIS.md` §11
