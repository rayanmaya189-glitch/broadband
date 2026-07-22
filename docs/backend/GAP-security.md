# AeroXe Broadband — Security & Compliance Gap Analysis (v2.0)

**Date:** 2026-07-21
**Severity:** TIER 0 — Fix Before Any Deployment
**Cross-reference:** `DESIGN-GAPS-DEEP-ANALYSIS.md` §9, `BACKEND-STATUS-REPORT.md` §12

---

## Executive Summary

Code-level security audit identified **13 vulnerabilities** across authentication, cryptography, network device management, regulatory compliance, and application security. **7 are Critical** (immediate exploit risk), **3 High**, **3 Medium**.

| Severity | Count | Action |
|----------|-------|--------|
| CRITICAL | 7 | Fix before any deployment |
| HIGH | 3 | Fix before production |
| MEDIUM | 3 | Fix before public launch |

---

## SEC-001: Aadhaar Hash Uses Static Salt

**Severity:** CRITICAL
**Files:** `shared/utils/pii.rs`, `28-security.md:111`

### Current Code
```rust
pub fn hash_aadhaar(aadhaar: &str) -> String {
    format!("aeroxe:{}", aadhaar) // Static prefix as "salt"
}
```

### Attack Vector
Same Aadhaar always produces same hash. Attacker dumps DB → builds rainbow table → recovers all Aadhaar numbers. The `aeroxe:` prefix is not a salt — it's a static, globally-known prefix.

### Impact
- **DPDP Act 2023 violation** — Aadhaar is sensitive personal data
- **UIDAI penalty** — up to ₹250 crore fine under Aadhaar Act §37
- **Re-identification** of "anonymized" customer records

### Fix
```rust
use rand::Rng;
use sha2::{Sha256, Digest};

pub fn hash_aadhaar(aadhaar: &str) -> (String, String) {
    let mut rng = rand::thread_rng();
    let salt: [u8; 32] = rng.gen();
    let salt_hex = hex::encode(salt);

    let mut hasher = Sha256::new();
    hasher.update(&salt);
    hasher.update(aadhaar.as_bytes());
    let hash = hex::encode(hasher.finalize());

    (salt_hex, hash) // Store both in DB
}

pub fn verify_aadhaar(aadhaar: &str, stored_salt: &str, stored_hash: &str) -> bool {
    let salt = hex::decode(stored_salt).unwrap();
    let mut hasher = Sha256::new();
    hasher.update(&salt);
    hasher.update(aadhaar.as_bytes());
    hex::encode(hasher.finalize()) == stored_hash
}
```

### Migration Required
```sql
ALTER TABLE customer_profiles
    ADD COLUMN aadhaar_salt VARCHAR(64),
    ADD COLUMN aadhaar_hash VARCHAR(128);

-- One-time migration: re-hash all existing Aadhaar values with per-record salt
-- Requires access to original Aadhaar values (if stored encrypted, decrypt first)
```

### Priority: P0 — Block deployment

---

## SEC-002: MikroTik `execute_command` Allows Arbitrary RouterOS Commands

**Severity:** CRITICAL
**File:** `integrations/mikrotik/adapter.rs:501-508`

### Current Code
```rust
pub async fn execute_command(&self, command: &str) -> Result<MikroTikResponse> {
    let body = serde_json::json!({ "command": command });
    let response = self.rest_post("/run", body).await?;
    Ok(response)
}
```

### Attack Vector
Any code path that calls `execute_command` with user-controlled input can execute **any** RouterOS command:
```
/system shutdown
/interface ethernet set [find name=ether1] disabled=yes
/user set admin password=hacked
/export (leaks all config including secrets)
```

### Impact
- **Full device takeover** — attacker controls network infrastructure
- **Service disruption** — can shut down OLT, BNG, or core routers
- **Data exfiltration** — export reveals RADIUS secrets, VPN keys, user passwords

### Fix
```rust
const ALLOWED_COMMANDS: &[&str] = &[
    "/queue/simple/add",
    "/queue/simple/remove",
    "/queue/simple/set",
    "/queue/simple/get",
    "/ppp/secret/add",
    "/ppp/secret/remove",
    "/ppp/secret/set",
    "/ppp/secret/get",
    "/ip/dhcp-server/lease/get",
    "/interface/ethernet/get",
    "/system/resource/get",
    "/interface/get",
];

pub async fn execute_command(&self, command: &str) -> Result<MikroTikResponse> {
    // Validate command prefix against whitelist
    let normalized = command.trim().to_lowercase();
    let is_allowed = ALLOWED_COMMANDS.iter()
        .any(|&prefix| normalized.starts_with(prefix));

    if !is_allowed {
        return Err(AppError::Forbidden(format!(
            "MikroTik command not in whitelist: {}",
            command.split_whitespace().next().unwrap_or("empty")
        )));
    }

    let body = serde_json::json!({ "command": command });
    let response = self.rest_post("/run", body).await?;
    Ok(response)
}
```

### Priority: P0 — Block deployment

---

## SEC-003: MikroTik TLS Certificate Validation Disabled

**Severity:** CRITICAL
**File:** `integrations/mikrotik/adapter.rs:167`

### Current Code
```rust
let client = reqwest::Client::builder()
    .danger_accept_invalid_certs(true)  // ← CRITICAL
    .build()?;
```

### Attack Vector
Man-in-the-middle attack on MikroTik REST connection:
1. ARP spoof on management VLAN
2. Intercept TLS handshake (self-signed cert accepted)
3. Capture MikroTik admin credentials
4. Full device control

### Impact
- Credential theft → full device takeover
- Configuration manipulation → traffic interception
- Data exfiltration

### Fix
```rust
use std::fs;

let ca_cert = fs::read(&self.config.ca_cert_path)
    .map_err(|e| AppError::Config(format!("CA cert not found: {}", e)))?;
let cert = reqwest::Certificate::from_pem(&ca_cert)?;

let client = reqwest::Client::builder()
    .add_root_certificate(cert)
    .timeout(Duration::from_secs(30))
    .build()?;
```

### New Config Fields
```toml
[mikrotik]
ca_cert_path = "/etc/aeroxe/certs/mikrotik-ca.pem"
```

### Priority: P0 — Block deployment

---

## SEC-004: WebSocket Exposed Without Authentication

**Severity:** CRITICAL
**File:** `routes/mod.rs:12`

### Current Code
```rust
// Under health_routes() — no auth middleware
.route("/ws", get(ws_handler))
```

### Attack Vector
Attacker connects to `ws://host/health/ws` without any token → receives real-time:
- Device status changes
- Customer session events
- Bandwidth alerts
- Network topology updates

### Impact
- Full operational data exposure to unauthenticated users
- Competitive intelligence for attackers
- Real-time monitoring of ISP infrastructure

### Fix
```rust
// In routes/mod.rs: Move /ws to authenticated route group
let protected_routes = Router::new()
    .route("/ws", get(ws_handler))
    .layer(middleware::from_fn(auth_middleware));

// Or add auth check inside ws_handler:
async fn ws_handler(
    ws: WebSocketUpgrade,
    headers: HeaderMap,
) -> Response {
    // Extract and validate JWT from query param or header
    let token = headers.get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .or_else(|| {
            // Also check query param: /ws?token=xxx
            None // implement query parsing
        });

    match token.and_then(|t| validate_jwt(t).ok()) {
        Some(user) => ws.on_upgrade(move |socket| handle_ws(socket, user)),
        None => Response::builder()
            .status(401)
            .body("WebSocket requires authentication".into())
            .unwrap(),
    }
}
```

### Priority: P0 — Block deployment

---

## SEC-005: Swagger UI Publicly Accessible in Production

**Severity:** CRITICAL
**File:** `routes/mod.rs:13-16`

### Current Code
```rust
.route("/api-docs", get(openapi_doc))
.merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json"))
```

### Attack Vector
Attacker visits `http://host/swagger-ui/` → complete API documentation including:
- All endpoint paths and parameters
- Authentication mechanisms
- Request/response schemas
- Internal data models

### Impact
- Attackers get a complete API map for targeted attacks
- Reduces attack surface complexity to zero

### Fix
```rust
#[cfg(debug_assertions)]
let api_routes = api_routes.merge(
    SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json")
);

// Or environment-based:
if std::env::var("ENABLE_SWAGGER").unwrap_or_default() == "true" {
    api_routes = api_routes.merge(
        SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json")
    );
}
```

### Priority: P0 — Block deployment

---

## SEC-006: No Distributed Rate Limiting

**Severity:** HIGH
**File:** `28-security.md:51-57`

### Current Implementation
Per-server in-memory rate limiting (tokio::sync::Semaphore or similar).

### Attack Vector
With 3 backend instances behind load balancer:
- Each server allows 100 req/min → effective limit is 300 req/min
- Attacker generates 150 req/min → no single server triggers limit

### Impact
- Brute force attacks across distributed deployment
- API abuse at 3× intended rate

### Fix
```rust
use redis::AsyncCommands;

async fn check_rate_limit(
    redis: &RedisPool,
    key: &str,
    max_requests: u32,
    window_secs: u64,
) -> Result<bool> {
    let mut conn = redis.get().await?;
    let now = chrono::Utc::now().timestamp();
    let window_start = now - window_secs as i64;

    // Sliding window counter
    let pipe = redis::pipe()
        .cmd("ZREMRANGEBYSCORE").arg(&key).arg(0).arg(window_start)
        .ignore()
        .cmd("ZADD").arg(&key).arg(now).arg(format!("{}:{}", now, rand::random::<u64>()))
        .ignore()
        .cmd("ZCARD").arg(&key)
        .ignore()
        .cmd("EXPIRE").arg(&key).arg(window_secs)
        .ignore();

    let (_, _, count, _): ((), (), u32, ()) = pipe.query_async(&mut conn).await?;
    Ok(count <= max_requests)
}
```

### Priority: P1 — Fix before production

---

## SEC-007: RADIUS Password Encoding Broken for >16 Bytes

**Severity:** CRITICAL
**File:** `integrations/radius/adapter.rs:231-252`

### Current Code
```rust
fn encode_password(password: &str, secret: &[u8]) -> Vec<u8> {
    let hash = md5::compute(secret);
    password.bytes().enumerate().map(|(i, b)| {
        let h = if i < 16 { hash[i] } else { hash[i % 16] };
        b ^ h
    }).collect()
}
```

### Attack Vector
RFC 2865 requires password chaining: each 16-byte block XORs with MD5(previous_ciphertext + shared_secret). The current code uses `hash[i % 16]` which breaks the chaining for passwords >16 bytes.

### Impact
- Passwords >16 bytes won't decrypt correctly on RADIUS server
- PPPoE authentication silently fails for long passwords
- Users can't connect to broadband service

### Fix
```rust
fn encode_password(password: &str, secret: &[u8]) -> Vec<u8> {
    let mut result = Vec::new();
    let mut prev_cipher: Vec<u8> = vec![0u8; 16]; // First block uses 16 zeros

    for chunk in password.as_bytes().chunks(16) {
        let mut md5_input = Vec::with_capacity(16 + secret.len());
        md5_input.extend_from_slice(&prev_cipher);
        md5_input.extend_from_slice(secret);

        let hash = md5::compute(&md5_input);

        let mut cipher = Vec::with_capacity(chunk.len());
        for (i, &b) in chunk.iter().enumerate() {
            cipher.push(b ^ hash[i]);
        }

        // Pad short blocks to 16 bytes for chaining
        prev_cipher = vec![0u8; 16];
        prev_cipher[..cipher.len()].copy_from_slice(&cipher);

        result.extend(cipher);
    }

    result
}
```

### Priority: P0 — Block deployment

---

## SEC-008: RADIUS Response Authenticator Not Validated

**Severity:** HIGH
**File:** `integrations/radius/adapter.rs:355-424`

### Current Code
```rust
// Receives response, only checks identifier match
async fn receive_response(&self, expected_id: u8) -> Result<RadiusPacket> {
    // ... receive UDP packet ...
    if packet.identifier == expected_id {
        Ok(packet) // No authenticator validation!
    }
}
```

### Attack Vector
Attacker on same network segment spoofs RADIUS responses:
1. Sniff RADIUS request → learn identifier
2. Send spoofed Accept response with matching identifier
3. System accepts fake response → creates fake PPPoE session

### Impact
- Bypass authentication → free internet access
- Create phantom sessions → inflate billing metrics
- Man-in-the-middle on accounting → hide data theft

### Fix
```rust
fn validate_response(
    response: &RadiusPacket,
    request_auth: &[u8; 16],
    shared_secret: &[u8],
) -> bool {
    // RFC 2865 §3: Response Authenticator = MD5(Code + ID + Length + RequestAuth + Attributes + Secret)
    let mut data = Vec::new();
    data.push(response.code);
    data.push(response.identifier);
    data.extend_from_slice(&(response.length as u16).to_be_bytes());
    data.extend_from_slice(request_auth);
    data.extend_from_slice(&response.attributes);
    data.extend_from_slice(shared_secret);

    let expected = md5::compute(&data);
    response.authenticator == expected
}
```

### Priority: P1 — Fix before production

---

## SEC-009: No JWT Refresh Token Rotation

**Severity:** HIGH
**File:** `28-security.md:15-22`

### Current Implementation
Refresh tokens are valid for 7 days with no rotation.

### Attack Vector
1. Steal refresh token (XSS, logs, network sniff)
2. Use for 7 days without detection
3. Original user's token still works → no revocation signal

### Impact
- Extended undetected access
- Account takeover persistence

### Fix
```rust
// On each refresh:
// 1. Validate current refresh token
// 2. Generate NEW refresh token (single-use)
// 3. Invalidate old refresh token
// 4. Return new access + refresh token pair

async fn refresh_token(
    db: &DbPool,
    old_refresh: &str,
) -> Result<AuthTokens> {
    let session = find_session_by_refresh(db, old_refresh).await?;

    // Check if token was already used (replay attack)
    if session.refresh_used {
        // Invalidate ALL sessions for this user
        invalidate_all_user_sessions(db, &session.user_id).await?;
        return Err(AppError::Unauthorized("Refresh token reuse detected".into()));
    }

    // Mark current token as used
    mark_refresh_used(db, &session.id).await?;

    // Issue new pair
    let new_access = generate_access_token(&session.user_id)?;
    let new_refresh = generate_refresh_token(&session.user_id)?;

    // Store new refresh token
    create_session(db, &session.user_id, &new_refresh).await?;

    Ok(AuthTokens { access: new_access, refresh: new_refresh })
}
```

### Priority: P1 — Fix before production

---

## SEC-010: Account Lockout is a DoS Vector

**Severity:** MEDIUM
**File:** `28-security.md:15-22`

### Attack Vector
Attacker locks out any user account with 5 failed requests:
```bash
for i in {1..5}; do
  curl -X POST /api/v1/auth/login -d '{"email":"victim@example.com","password":"wrong"}'
done
# Account locked for 30 minutes
```

### Impact
- Denial of service for legitimate users
- Targeted account lockout during critical operations

### Fix
```rust
// Progressive approach:
// Attempt 1-3: Standard processing
// Attempt 4: Add CAPTCHA requirement
// Attempt 5: Progressive delay (1s, 2s, 4s, 8s...)
// Never hard lock — instead require CAPTCHA + progressive delay

async fn handle_login_attempt(
    db: &DbPool,
    email: &str,
) -> LoginAttemptPolicy {
    let recent_failures = count_recent_failures(db, email, Duration::from_secs(900)).await;

    match recent_failures {
        0..=2 => LoginAttemptPolicy::Standard,
        3 => LoginAttemptPolicy::RequireCaptcha,
        4 => LoginAttemptPolicy::RequireCaptchaAndDelay(Duration::from_secs(2)),
        n => LoginAttemptPolicy::RequireCaptchaAndDelay(
            Duration::from_secs(2u64.pow((n - 3) as u32).min(120))
        ),
    }
}
```

### Priority: P2 — Fix before public launch

---

## SEC-011: No DPDP Act 2023 Compliance

**Severity:** MEDIUM
**File:** `28-security.md:154`

### Requirements (Digital Personal Data Protection Act, 2023)

| Requirement | Status | Gap |
|-------------|--------|-----|
| Consent management (§6) | Missing | No consent table, no consent API |
| Purpose limitation (§5) | Missing | Data collected without purpose tagging |
| Data principal rights (§11-12) | Missing | No data access/erasure APIs |
| Breach notification (§8(6)) | Missing | No 72-hour notification workflow |
| Data retention limits (§8(7)) | Partial | Appendix C exists, no enforcement worker |
| Cross-border transfer (§16) | N/A | India-only operation initially |

### Implementation Plan

```sql
-- New tables
CREATE TABLE consent_records (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    customer_id UUID NOT NULL REFERENCES customers(id),
    purpose VARCHAR(255) NOT NULL,
    consent_text TEXT NOT NULL,
    granted_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    withdrawn_at TIMESTAMPTZ,
    ip_address INET,
    user_agent TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE data_access_requests (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    customer_id UUID NOT NULL REFERENCES customers(id),
    request_type VARCHAR(20) NOT NULL, -- 'access', 'erasure', 'correction'
    status VARCHAR(20) NOT NULL DEFAULT 'pending',
    requested_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMPTZ,
    response_data JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE breach_notifications (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    incident_id UUID NOT NULL,
    detected_at TIMESTAMPTZ NOT NULL,
    notified_at TIMESTAMPTZ,
    authority_notified BOOLEAN DEFAULT FALSE,
    affected_customers INTEGER DEFAULT 0,
    description TEXT NOT NULL,
    remediation TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

### API Endpoints
```
POST   /api/v1/consent                    — Record consent
POST   /api/v1/consent/history            — View consent history
DELETE /api/v1/consent/delete             — Withdraw consent
POST   /api/v1/data-requests             — Submit data access/erasure request
POST   /api/v1/data-requests/get         — Check request status
POST   /api/v1/breach-notifications      — Report breach (internal)
```

> All endpoints are Protobuf-encoded. See `API-CONVENTIONS.md`.

### Priority: P2 — Fix before public launch

---

## SEC-012: No IT Act Section 43A / CERT-In Compliance

**Severity:** MEDIUM
**File:** `28-security.md:153`

### Requirements

| Regulation | Requirement | Status |
|------------|-------------|--------|
| IT Act §43A | Reasonable security practices for sensitive data | Missing |
| CERT-In Directive (2022) | 6-hour incident reporting for cyber incidents | Missing |
| CERT-In Directive | 180-day log retention | Missing |
| CERT-In Directive | Synchronized NTP clocks | Missing |

### Implementation Plan

1. **Incident Response Workflow**
```rust
pub struct IncidentSeverity(pub &'static str);

impl IncidentSeverity {
    pub const CRITICAL: Self = Self("critical"); // 6-hour CERT-In report
    pub const HIGH: Self = Self("high");         // 24-hour internal report
    pub const MEDIUM: Self = Self("medium");     // 72-hour internal report
    pub const LOW: Self = Self("low");           // Weekly summary
}
```

2. **NTP Configuration** — Add to `docker-compose.yml`:
```yaml
services:
  ntp:
    image: cturra/ntp
    restart: always
    cap_add:
      - SYS_TIME
```

3. **Log Retention** — Ensure all logs retained for 180 days minimum (currently 90 days for some)

### Priority: P2 — Fix before public launch

---

## SEC-013: No Aadhaar Act UIDAI Authorization

**Severity:** MEDIUM
**File:** `28-security.md:155`

### Legal Requirement
Under the Aadhaar Act, 2016, any entity collecting Aadhaar numbers must:
1. Be registered as a "Requesting Entity" with UIDAI
2. Use Aadhaar only for identity verification (not as address proof)
3. Implement Aadhaar-based biometric authentication only through registered Authentication User Agencies (AUAs)

### Current State
System stores Aadhaar numbers (hashed) for KYC without UIDAI authorization.

### Options
1. **Obtain UIDAI RE authorization** — Apply at UIDAI portal (6-8 weeks)
2. **Remove Aadhaar storage** — Use Aadhaar-based eKYC via UIDAI API (no local storage)
3. **Replace with alternate ID** — Use PAN, DL, or Voter ID as primary KYC

### Recommendation
Option 2 (eKYC via UIDAI API) — no local Aadhaar storage eliminates SEC-001, SEC-013, and reduces DPDP liability.

### Priority: P2 — Decide before public launch

---

## Summary

| Gap ID | Severity | Title | Fix Effort | Priority |
|--------|----------|-------|------------|----------|
| SEC-001 | CRITICAL | Aadhaar static salt | 1 day | P0 |
| SEC-002 | CRITICAL | MikroTik arbitrary commands | 0.5 day | P0 |
| SEC-003 | CRITICAL | MikroTik TLS bypass | 0.5 day | P0 |
| SEC-004 | CRITICAL | WebSocket no auth | 1 day | P0 |
| SEC-005 | CRITICAL | Swagger in production | 0.5 day | P0 |
| SEC-006 | HIGH | No distributed rate limiting | 1 day | P1 |
| SEC-007 | CRITICAL | RADIUS password encoding | 1 day | P0 |
| SEC-008 | HIGH | RADIUS response validation | 1 day | P1 |
| SEC-009 | HIGH | No refresh token rotation | 1 day | P1 |
| SEC-010 | MEDIUM | Lockout DoS vector | 1 day | P2 |
| SEC-011 | MEDIUM | DPDP Act compliance | 3 days | P2 |
| SEC-012 | MEDIUM | IT Act / CERT-In compliance | 2 days | P2 |
| SEC-013 | MEDIUM | Aadhaar Act authorization | 1 day (decision) | P2 |
| **TOTAL** | **7 CRIT, 3 HIGH, 3 MED** | | **~14 days** | |

---

## Implementation Order

### P0 Sprint (Days 1-3) — Block All Deployments
1. SEC-002: MikroTik command whitelist (0.5 day)
2. SEC-003: MikroTik TLS validation (0.5 day)
3. SEC-005: Swagger environment gate (0.5 day)
4. SEC-004: WebSocket auth (1 day)
5. SEC-007: RADIUS password encoding (1 day)
6. SEC-001: Aadhaar per-record salt (1 day)

### P1 Sprint (Days 4-6) — Block Production
7. SEC-006: Redis-based distributed rate limiting (1 day)
8. SEC-008: RADIUS response authenticator validation (1 day)
9. SEC-009: Refresh token rotation (1 day)

### P2 Sprint (Days 7-12) — Block Public Launch
10. SEC-010: Progressive login delays + CAPTCHA (1 day)
11. SEC-011: DPDP Act compliance tables + APIs (3 days)
12. SEC-012: CERT-In incident workflow + NTP (2 days)
13. SEC-013: Aadhaar Act decision + implementation (1 day)

---

*Document version: 1.0 — 2026-07-21*
*Related: `DESIGN-GAPS-DEEP-ANALYSIS.md` §9.0, `GAP-code-bugs.md` §9.7-9.8*
