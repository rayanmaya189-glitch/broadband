# AeroXe Backend — Threat Modeling Document

> **Req Ref:** §28 Security Design, OWASP ASVS v4.0 Compliance  
> **Version:** 1.0

---

## 1. Overview

This document provides a comprehensive threat model for the AeroXe Broadband backend system, identifying potential security threats, their impact, and mitigation strategies. It follows the STRIDE threat modeling framework and maps to OWASP Top 10 (2021) and OWASP ASVS v4.0.

---

## 2. System Boundaries

### Trust Boundaries

| Boundary | Description | Components |
|----------|-------------|------------|
| **TB1** | External Internet → CDN/Load Balancer | Public API, Customer Portal |
| **TB2** | CDN/Load Balancer → Backend API | Axum HTTP Handlers |
| **TB3** | Backend API → Database | SeaORM → PostgreSQL |
| **TB4** | Backend API → Cache | Redis (sessions, rate limiting) |
| **TB5** | Backend API → Message Bus | NATS JetStream |
| **TB6** | Backend API → Object Storage | MinIO/S3 |
| **TB7** | Backend → External Services | SMS (MSG91), Email (SMTP), Payment Gateway (Razorpay) |
| **TB8** | Backend → Network Devices | SSH/SNMP to MikroTik, Huawei, ZTE |

### Data Classification

| Classification | Examples | Protection Level |
|----------------|----------|------------------|
| **Critical** | Passwords, JWT keys, API secrets | Encryption at rest + transit, access logging |
| **Sensitive** | Customer PII (Aadhaar, PAN, phone), Financial data | Encryption, RBAC, audit trail |
| **Internal** | Network configs, device credentials | RBAC, network isolation |
| **Public** | Plan details, pricing, public endpoints | Integrity verification |

---

## 3. STRIDE Threat Analysis

### 3.1 Spoofing (Authentication Bypass)

| Threat | Risk | Mitigation | OWASP |
|--------|------|------------|-------|
| Brute force login | High | Account lockout (5 attempts/30 min), rate limiting (5 req/min auth endpoints) | A07 |
| JWT token theft | Critical | RS256 asymmetric keys, short-lived access tokens (24h), refresh token rotation | A02 |
| Session fixation | High | Server-side session creation, secure cookie flags | A07 |
| OTP interception | High | 5-min expiry, rate limiting, phone verification required | A07 |
| TOTP replay attacks | Medium | Time-based window, backup codes, 2FA enforcement for admin roles | A07 |

**Implementation:**
```rust
// Rate limiting for auth endpoints
pub const AUTH_RATE_LIMIT: u64 = 5;       // 5 requests per minute
pub const API_RATE_LIMIT: u64 = 100;     // 100 requests per minute
pub const LOCKOUT_THRESHOLD: u32 = 5;    // 5 failed attempts
pub const LOCKOUT_DURATION: i64 = 1800;  // 30 minutes
```

### 3.2 Tampering (Data Integrity)

| Threat | Risk | Mitigation | OWASP |
|--------|------|------------|-------|
| SQL injection | Critical | SeaORM parameterized queries, no raw SQL in application code | A03 |
| Request body tampering | High | Input validation (serde + validator), request signing | A03 |
| Event payload tampering | High | Event envelope with event_id, version, producer signature | A08 |
| File upload malicious content | High | File type whitelist, content-type validation, size limits | A04 |
| Device configuration tampering | Critical | Approval workflow for critical changes, checksum verification | A04 |

**Implementation:**
```rust
// Request validation
pub struct CreateCustomerRequest {
    #[validate(length(min = 2, max = 255))]
    pub name: String,
    #[validate(email)]
    pub email: Option<String>,
    #[validate(regex = "phone_regex")]
    pub phone: String,
}
```

### 3.3 Repudiation (Audit Trail)

| Threat | Risk | Mitigation | OWASP |
|--------|------|------------|-------|
| Denying actions | Medium | Immutable audit logs, middleware-based logging | A09 |
| Log tampering | High | Write-only audit table, append-only, partitioned monthly | A09 |
| Missing accountability | Medium | User context in all API calls, IP tracking | A09 |

**Implementation:**
```rust
// Audit middleware captures every request
pub struct AuditMiddleware;

impl<S> Middleware<S> for AuditMiddleware {
    async fn call(&self, req: Request, next: Next<S>) -> Response {
        let user = req.extensions().get::<UserContext>().cloned();
        let ip = extract_ip(&req);
        let action = format!("{} {}", req.method(), req.uri());
        
        let response = next.run(req).await;
        
        // Async audit log insert
        tokio::spawn(async move {
            insert_audit_log(InsertAuditLog {
                user_id: user.as_ref().map(|u| u.id),
                action,
                ip_address: ip,
                result: determine_result(&response),
                // ...
            }).await.ok();
        });
        
        response
    }
}
```

### 3.4 Information Disclosure

| Threat | Risk | Mitigation | OWASP |
|--------|------|------------|-------|
| Stack traces in responses | Medium | Generic error messages, logging internals separately | A05 |
| Database error leakage | High | `AppError::Internal` converts DB errors to generic message | A05 |
| PII in logs | High | PII masking, structured logging with field filters | A02 |
| Debug statements in production | High | Clippy lints (`clippy::print_stdout`, `clippy::print_stderr`), CI security audit | A05 |
| Verbose error messages | Medium | Production mode hides details, development mode shows details | A05 |

**Implementation:**
```rust
// AppError never exposes internal details to client
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            AppError::Internal(err) => {
                tracing::error!("Internal error: {:?}", err); // Log full error
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string())
            }
            // ...
        };
    }
}
```

### 3.5 Denial of Service

| Threat | Risk | Mitigation | OWASP |
|--------|------|------------|-------|
| API flooding | High | Sliding window rate limiting (Redis), per-IP and per-user | A04 |
| Large payload attacks | Medium | Request body size limits (10MB default, 50MB uploads) | A04 |
| Database connection exhaustion | High | Connection pooling (SeaORM max_connections=20), PgBouncer | A04 |
| Event storm amplification | Medium | Outbox pattern with batching, subscriber backpressure | A04 |
| Resource exhaustion via queries | Medium | Query pagination limits, index optimization | A04 |

**Implementation:**
```rust
// Rate limiting middleware
pub async fn rate_limit_middleware(
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let key = extract_rate_limit_key(&req);
    let store = req.extensions().get::<Arc<RateLimitStore>>().unwrap();
    
    if !store.check_rate_limit(&key, max_requests, window_seconds).await {
        return Err(StatusCode::TOO_MANY_REQUESTS);
    }
    
    Ok(next.run(req).await)
}
```

### 3.6 Elevation of Privilege

| Threat | Risk | Mitigation | OWASP |
|--------|------|------------|-------|
| Horizontal privilege escalation | Critical | Branch scoping middleware, resource ownership checks | A01 |
| Vertical privilege escalation | Critical | RBAC + ABAC enforcement, permission checks in handlers | A01 |
| JWT claim manipulation | Critical | RS256 signing (tamper-proof), server-side validation | A02 |
| Missing authorization checks | High | `RequirePermission` middleware, explicit checks in handlers | A01 |
| Role inheritance bypass | High | Hierarchical permission resolution with caching | A01 |

**Implementation:**
```rust
// Branch scope middleware ensures data isolation
pub async fn branch_scope_middleware(
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let user = extract_user_context(&req)?;
    
    if user.is_company_wide {
        return Ok(next.run(req).await);
    }
    
    req.extensions_mut().insert(BranchFilter {
        branch_ids: user.branch_ids,
        is_company_wide: false,
    });
    
    Ok(next.run(req).await)
}
```

---

## 4. OWASP Top 10 (2021) Mapping

| # | Threat | AeroXe Mitigation | Status |
|---|--------|-------------------|--------|
| A01 | Broken Access Control | RBAC + ABAC engine, branch scoping, resource ownership | ✅ Implemented |
| A02 | Cryptographic Failures | argon2id, RS256 JWT, AES-256-GCM secrets, TLS 1.3 | ✅ Implemented |
| A03 | Injection | SeaORM parameterized queries, input validation | ✅ Implemented |
| A04 | Insecure Design | DDD bounded contexts, threat modeling, security-first architecture | ✅ Documented |
| A05 | Security Misconfiguration | CORS lockdown, security headers, no debug in production | ⚠️ In Progress |
| A06 | Vulnerable Components | cargo audit CI, dependency scanning | ⚠️ CI Script Created |
| A07 | Auth Failures | Account lockout, 2FA, rate limiting, OTP expiry | ✅ Implemented |
| A08 | Data Integrity Failures | Outbox pattern, event versioning, idempotency keys | ✅ Implemented |
| A09 | Logging & Monitoring | Audit middleware, structured tracing, Prometheus metrics | ✅ Implemented |
| A10 | SSRF | No user-controlled URLs in server requests | ✅ N/A |

---

## 5. Security Controls Matrix

### 5.1 Authentication Controls

| Control | Implementation | Test Coverage |
|---------|---------------|---------------|
| Password hashing (argon2id) | `shared/utils/helpers.rs` | Unit tests ✅ |
| JWT RS256 signing | `shared/utils/jwt_keys.rs` | Unit tests ✅ |
| TOTP 2FA | `modules/identity/` | Integration tests ⚠️ |
| OTP generation/verification | Redis with TTL | Unit tests ✅ |
| Account lockout | Failed attempts counter | Unit tests ✅ |
| Session management | Redis + DB dual storage | Integration tests ⚠️ |

### 5.2 Authorization Controls

| Control | Implementation | Test Coverage |
|---------|---------------|---------------|
| RBAC permission check | `RequirePermission` middleware | Integration tests ⚠️ |
| ABAC policy engine | `security/domain/rules/abac_engine.rs` | Unit tests ✅ |
| Branch scoping | `BranchScope` middleware | Integration tests ⚠️ |
| Permission resolution | Redis-cached hierarchical lookup | Unit tests ✅ |
| Temporary permissions | `user_roles.expires_at` | Unit tests ✅ |

### 5.3 Data Protection Controls

| Control | Implementation | Test Coverage |
|---------|---------------|---------------|
| PII hashing (Aadhaar, PAN) | SHA-256 with personal salt | Unit tests ✅ |
| PII masking in logs | `PiiProtection` utility | Unit tests ✅ |
| Encryption at rest | PostgreSQL TDE, MinIO SSE | Infrastructure ⚠️ |
| Encryption in transit | TLS 1.3 for all connections | Infrastructure ⚠️ |
| Data retention enforcement | Partition worker + cleanup jobs | Integration tests ⚠️ |

### 5.4 Infrastructure Controls

| Control | Implementation | Test Coverage |
|---------|---------------|---------------|
| Rate limiting | Redis sliding window | Integration tests ✅ |
| Request size limits | Tower middleware (10MB) | Unit tests ✅ |
| CORS configuration | Environment-based, production lockdown | Configuration tests ⚠️ |
| Security headers | Custom middleware | Unit tests ✅ |
| Audit logging | Async middleware + partitioned table | Integration tests ⚠️ |

---

## 6. Security Testing Strategy

### 6.1 Unit Tests

- Domain logic invariants (customer lifecycle, invoice calculations)
- Permission resolution and wildcard matching
- ABAC policy evaluation
- PII masking and hashing
- JWT signing and verification

### 6.2 Integration Tests

- Repository CRUD with real PostgreSQL (testcontainers)
- Rate limiting behavior
- Authentication flow (login, 2FA, refresh)
- Branch scoping enforcement

### 6.3 Security Audit Script

```bash
# Run comprehensive security audit
./scripts/security-audit.sh

# Checks performed:
# 1. cargo audit (known vulnerabilities)
# 2. clippy with security lints
# 3. Hardcoded secrets detection
# 4. Debug statement detection
# 5. Format verification
# 6. License compliance (optional)
```

### 6.4 Penetration Testing Checklist

| Test | Priority | Frequency |
|------|----------|-----------|
| SQL injection attempts | Critical | Every release |
| XSS payload injection | Critical | Every release |
| JWT manipulation | Critical | Every release |
| Privilege escalation | Critical | Every release |
| Rate limiting bypass | High | Every release |
| CSRF attacks | High | Quarterly |
| SSRF attempts | Medium | Quarterly |
| Buffer overflow | Medium | Quarterly |

---

## 7. Incident Response

### 7.1 Security Event Levels

| Level | Description | Response Time | Examples |
|-------|-------------|---------------|----------|
| **P0 Critical** | Active breach, data exfiltration | Immediate | JWT key compromise, database breach |
| **P1 High** | Attempted breach, vulnerability exploited | 1 hour | Brute force attack, SQL injection attempt |
| **P2 Medium** | Suspicious activity, policy violation | 4 hours | Multiple failed logins, unusual API patterns |
| **P3 Low** | Informational, minor policy deviation | 24 hours | Outdated user session, minor config issue |

### 7.2 Response Procedures

1. **Detection**: Automated alerts from audit logs, rate limiting, error monitoring
2. **Containment**: Automatic account lockout, IP blocking, session invalidation
3. **Eradication**: Key rotation, credential reset, patch deployment
4. **Recovery**: Service restoration, data integrity verification
5. **Lessons Learned**: Post-incident review, policy updates, documentation

---

## 8. Compliance Checklist

| Requirement | Status | Evidence |
|-------------|--------|----------|
| OWASP ASVS v4.0 Level 2 | ⚠️ Partial | Threat model, security controls matrix |
| GDPR (data retention, consent) | ⚠️ Partial | Compliance module, data retention policies |
| PCI DSS (payment data) | ⚠️ Partial | Razorpay integration (PCI-compliant gateway) |
| IT Act 2000 (India) | ⚠️ Partial | Audit logging, data protection |
| ISO 27001 (information security) | 🔲 Planned | Security controls documentation |

---

## 9. Next Steps

1. **Complete ABAC policy implementation** for all modules
2. **Add penetration testing** to CI/CD pipeline
3. **Implement automated security scanning** (SAST/DAST)
4. **Complete GDPR compliance module** with data export/deletion
5. **Add security headers testing** (CSP, HSTS, X-Frame-Options)
6. **Implement webhook signature verification** for payment gateway
7. **Add device communication encryption** (SNMPv3, SSH key rotation)
