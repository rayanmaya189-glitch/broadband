# AeroXe Backend — Security Design

> **Req Ref:** §19 Security Design

---

## 1. Overview

Comprehensive security framework covering authentication, encryption, API security, input validation, and audit logging. Designed to protect customer PII, financial data, and network infrastructure.

## 2. Authentication & Session Security

| Control | Implementation |
|---------|---------------|
| Password hashing | argon2id (64MB, 3 iterations, 4 parallelism) |
| JWT algorithm | RS256 (asymmetric keys) |
| Access token TTL | 24 hours |
| Refresh token TTL | 7 days |
| Max sessions per user | 5 |
| Account lockout | 5 failed attempts → 30 min lock |
| OTP expiry | 5 minutes |
| 2FA | TOTP (RFC 6238) + backup codes |

## 3. Encryption

### At Rest
| Data Type | Method | Details |
|-----------|--------|---------|
| Passwords | argon2id | Salted, one-way hash |
| PII (Aadhaar, PAN) | SHA-256 | Salted hash, searchable |
| SNMP communities | AES-256-GCM | Encrypted in database |
| SSH keys | Encrypted storage | MinIO with SSE |
| JWT secret | Environment variable | Never in code |
| API keys (gateway) | AES-256-GCM | Encrypted in config |
| 2FA secrets | AES-256-GCM | Encrypted in database |

### In Transit
| Connection | Protocol |
|-----------|----------|
| API (public) | TLS 1.3 |
| Database | TLS 1.2+ (PgBouncer) |
| Redis | TLS 1.2+ |
| NATS | TLS 1.3 |
| MinIO | TLS 1.3 |
| WebSocket | WSS (TLS) |
| Device management | SSH v2 / SNMPv3 |

## 4. API Security

### Rate Limiting
```
Unauthenticated:  30 req/min
Authenticated:   100 req/min
Write operations:  30 req/min
File uploads:      10 req/5min
Auth endpoints:     5 req/min
```

### Input Validation
- All inputs validated against schemas (serde + validator)
- SQL injection prevented by parameterized queries (SeaORM)
- XSS prevention via HTML escaping in templates
- Request body size limits: 10 MB default, 50 MB for uploads
- File type validation: whitelist per document type

### CORS Configuration
```toml
[security.cors]
origins = ["https://aeroxebroadband.com", "https://admin.aeroxe.com"]
methods = ["GET", "POST", "PUT", "DELETE"]
headers = ["Authorization", "Content-Type", "X-Idempotency-Key"]
max_age = 3600
```

### Security Headers
```
Strict-Transport-Security: max-age=31536000; includeSubDomains
X-Content-Type-Options: nosniff
X-Frame-Options: DENY
X-XSS-Protection: 1; mode=block
Content-Security-Policy: default-src 'self'
Referrer-Policy: strict-origin-when-cross-origin
Permissions-Policy: camera=(), microphone=(), geolocation=()
```

## 5. Network Security

### Management VLAN Isolation
- All device management traffic on dedicated VLAN (100-199)
- Management interfaces not exposed to customer network
- SNMP community strings rotated quarterly
- SSH key-based authentication (no passwords)

### Device Access Control
| Protocol | Minimum Version | Authentication |
|----------|----------------|---------------|
| SNMP | v3 | USM (authPriv) |
| SSH | v2 | Public key |
| API (MikroTik) | TLS | Token-based |
| TR-069 | TLS | Certificate |

## 6. Data Protection

### PII Handling
```rust
pub struct PiiProtection;

impl PiiProtection {
    pub fn hash_aadhaar(aadhaar: &str) -> String {
        // SHA-256 with personal salt
        let salt = format!("aeroxe:{}", aadhaar);
        sha256(salt.as_bytes())
    }

    pub fn hash_pan(pan: &str) -> String {
        let salt = format!("aeroxe:{}", pan);
        sha256(salt.as_bytes())
    }

    pub fn mask_phone(phone: &str) -> String {
        // +919876543210 → +91******3210
        if phone.len() > 6 {
            format!("{}{}{}", &phone[..4], "*".repeat(phone.len() - 8), &phone[phone.len()-4..])
        } else {
            phone.to_string()
        }
    }

    pub fn mask_email(email: &str) -> String {
        // rahul@example.com → r****l@example.com
        let parts: Vec<&str> = email.split('@').collect();
        if parts.len() == 2 && parts[0].len() > 2 {
            format!("{}****{}@{}", &parts[0][..1], &parts[0][parts[0].len()-1..], parts[1])
        } else {
            email.to_string()
        }
    }
}
```

### Data Retention
| Data Type | Retention | Deletion |
|-----------|-----------|----------|
| Customer PII | Account lifetime + 7 years | Anonymized |
| KYC documents | 7 years from verification | Secure delete |
| Audit logs | 7 years | Archived, then deleted |
| Financial records | 7 years | Archived |
| Device logs | 90 days | Auto-purged |
| Session data | 30 days | Auto-purged |
| OTP codes | 5 minutes | Auto-expired |
| Temporary files | 24 hours | Auto-purged |

## 7. Secrets Management

```yaml
# Environment variables (never in code or git)
DATABASE_URL: postgresql://...
REDIS_URL: redis://...
NATS_URL: nats://...
JWT_PRIVATE_KEY: |
  -----BEGIN RSA PRIVATE KEY-----
  ...
JWT_PUBLIC_KEY: |
  -----BEGIN PUBLIC KEY-----
  ...
MINIO_ACCESS_KEY: ...
MINIO_SECRET_KEY: ...
SMTP_PASSWORD: ...
SMS_API_KEY: ...
RAZORPAY_KEY_SECRET: ...
```

## 8. Security Monitoring

| Event | Alert |
|-------|-------|
| 5+ failed logins in 5 min | Alert NOC + lock account |
| Login from new IP/location | Email notification to user |
| Unauthorized API access (403) | Log + alert if > 10/min |
| Database connection spike | Alert NOC |
| Configuration change | Audit log + notification |
| Firmware update attempt | Approval workflow + notification |

## 9. RBAC Permissions

```
security.config.view
security.config.update
security.keys.rotate
security.audit.view
security.audit.export
security.secrets.view
security.secrets.rotate
```

---

## Security Vulnerability Reference (v2.0)

> **Cross-reference:** `GAP-security.md`, `DESIGN-GAPS-DEEP-ANALYSIS.md` §9.0

The following Tier 0 security vulnerabilities were identified in code-level audit:

| SEC ID | Severity | Vulnerability | Location | Fix Effort |
|--------|----------|---------------|----------|------------|
| SEC-001 | CRITICAL | Aadhaar hash uses static salt — rainbow table attack | `shared/utils/pii.rs` | 1 day |
| SEC-002 | CRITICAL | MikroTik `execute_command` allows arbitrary RouterOS commands | `integrations/mikrotik/adapter.rs:501` | 0.5 day |
| SEC-003 | CRITICAL | MikroTik `danger_accept_invalid_certs(true)` — MITM possible | `integrations/mikrotik/adapter.rs:167` | 0.5 day |
| SEC-004 | CRITICAL | WebSocket `/ws` exposed without authentication | `routes/mod.rs:12` | 1 day |
| SEC-005 | CRITICAL | Swagger UI publicly accessible in production | `routes/mod.rs:13-16` | 0.5 day |
| SEC-006 | HIGH | No distributed rate limiting — per-server limits bypassed | `28-security.md:51-57` | 1 day |
| SEC-007 | CRITICAL | RADIUS password encoding broken for >16 bytes | `integrations/radius/adapter.rs:231` | 1 day |
| SEC-008 | HIGH | RADIUS response authenticator not validated | `integrations/radius/adapter.rs:355` | 1 day |
| SEC-009 | HIGH | No JWT refresh token rotation | `28-security.md:15-22` | 1 day |
| SEC-010 | MEDIUM | Account lockout is DoS vector | `28-security.md:15-22` | 1 day |
| SEC-011 | MEDIUM | No DPDP Act 2023 compliance | `28-security.md:154` | 3 days |
| SEC-012 | MEDIUM | No IT Act §43A / CERT-In compliance | `28-security.md:153` | 2 days |
| SEC-013 | MEDIUM | No Aadhaar Act UIDAI authorization | `28-security.md:155` | 1 day |

**P0 Sprint (Days 1-3):** Fix SEC-001 through SEC-005, SEC-007 before any deployment.
**Full details:** `GAP-security.md`
