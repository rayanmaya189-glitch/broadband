# Customer Module — ISP Design Gaps

**Module:** `customer`, `tickets`, `notifications`
**Cross-reference:** `DESIGN-GAPS-DEEP-ANALYSIS.md` (ISP-CUST-C01 through ISP-CUST-L01)

---

## Critical Gaps

### ISP-CUST-C01: No Customer Self-Service Portal

**Files Affected:**
- `src/modules/customer/application/service.rs`
- `src/routes/mod.rs` (no `/customer/me/*` routes)

**Current State:** `customer` module has admin-facing CRUD only. Zero self-service endpoints.

**Required Route Group:**
```
/api/v1/customer/me/
  GET    /profile              — Get own profile
  PUT    /profile              — Update own profile
  GET    /subscription         — Get current subscription
  GET    /invoices             — List own invoices
  GET    /invoices/:id         — Get invoice detail
  POST   /invoices/:id/pay     — Initiate payment
  GET    /usage                — Get bandwidth usage (daily/hourly)
  GET    /usage/realtime       — Get real-time speed
  GET    /sessions             — List active sessions
  POST   /tickets              — Create support ticket
  GET    /tickets              — List own tickets
  GET    /tickets/:id          — Get ticket detail
  POST   /tickets/:id/comments — Add comment
  POST   /plan-change          — Request plan upgrade/downgrade
  POST   /service/pause        — Pause service temporarily
  POST   /service/resume       — Resume paused service
  GET    /referral-code        — Get own referral code
  POST   /kyc/upload           — Upload KYC documents
  GET    /notifications        — List notifications
  PUT    /notifications/preferences — Update notification preferences
```

**Customer Auth Separation:**
```rust
// Customer portal uses separate auth middleware
// Customer JWT contains: { sub, role: "customer", customer_id, subscription_id }
// Staff JWT contains: { sub, role, permissions, branch_id }

pub async fn require_customer_auth(
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Claims, AppError> {
    let token = extract_bearer_token(&headers)?;
    let claims = verify_customer_jwt(&state.jwt_keys, &token)?;

    // Customer can only access their own data
    Ok(claims)
}
```

**Usage Dashboard API:**
```rust
pub async fn get_usage(
    State(state): State<AppState>,
    claims: Claims,
    Query(params): Query<UsageParams>,
) -> Result<Json<UsageResponse>> {
    // params.period: "daily" | "weekly" | "monthly"
    // params.from: DateTime
    // params.to: DateTime

    let usage = state.db.query(
        "SELECT date_trunc($1, session_start) as period,
                SUM(bytes_in) as total_in,
                SUM(bytes_out) as total_out,
                SUM(session_duration) as total_duration
         FROM customer_sessions
         WHERE customer_id = $2
         AND session_start BETWEEN $3 AND $4
         GROUP BY period
         ORDER BY period",
    ).fetch_all?;

    Ok(Json(UsageResponse {
        usage,
        current_speed: get_current_speed(&state, &claims).await?,
        plan_limit: get_plan_limit(&state, &claims).await?,
    }))
}
```

---

## High Gaps

### ISP-CUST-H01: No WhatsApp Two-Way Bot

**Current State:** `whatsapp/mod.rs` sends notifications only. No inbound handling.

**Required Implementation:**
1. WhatsApp Business API webhook endpoint for inbound messages
2. Message parser: text commands, button responses, list selections
3. Bot flow:
   - "Hi" → Welcome message with menu
   - "1" → View balance
   - "2" → Raise ticket
   - "3" → Check usage
   - "4" → Pay bill
4. Integration with customer module for data retrieval
5. Human handoff for complex queries

**New Route:**
```
POST /api/v1/whatsapp/webhook        — Receive inbound WhatsApp messages
POST /api/v1/whatsapp/webhook/verify — Webhook verification (GET)
```

---

### ISP-CUST-H02: No SLA Enforcement

**Current State:** `escalate_ticket()` only changes status string.

**Required Implementation:**
```rust
// SLA Definition
pub struct SlaDefinition {
    pub id: Uuid,
    pub priority: TicketPriority,
    pub customer_tier: CustomerTier,
    pub response_time_minutes: i32,  // First response SLA
    pub resolution_time_minutes: i32, // Resolution SLA
    pub escalation_matrix: Vec<EscalationLevel>,
}

pub struct EscalationLevel {
    pub level: i32,        // 1, 2, 3
    pub after_minutes: i32,
    pub assign_to_role: String,
    pub notify_channels: Vec<String>,
}

// SLA Monitoring
pub struct SlaMonitorWorker {
    db: DatabaseConnection,
    notification_service: NotificationService,
}

impl SlaMonitorWorker {
    pub async fn run(&self) {
        // Check every minute
        let open_tickets = self.get_open_tickets().await;
        for ticket in open_tickets {
            let sla = self.get_sla_definition(&ticket).await?;
            let elapsed = (Utc::now() - ticket.created_at).num_minutes();

            // Check response SLA
            if ticket.first_response_at.is_none() && elapsed > sla.response_time_minutes {
                self.escalate_ticket(&ticket, "response_sla_breach").await?;
                self.send_alert(&ticket, "Response SLA breached").await?;
            }

            // Check resolution SLA
            if elapsed > sla.resolution_time_minutes {
                self.escalate_ticket(&ticket, "resolution_sla_breach").await?;
                self.send_alert(&ticket, "Resolution SLA breached").await?;
            }

            // Check escalation matrix
            for level in &sla.escalation_matrix {
                if elapsed >= level.after_minutes && !ticket.escalated_to_level(level.level) {
                    self.escalate_to_level(&ticket, level).await?;
                }
            }
        }
    }
}
```

**New Entities:**
```rust
pub struct SlaDefinition {
    pub id: Uuid,
    pub plan_id: Option<Uuid>,     // Per-plan SLA
    pub priority: TicketPriority,
    pub response_time_minutes: i32,
    pub resolution_time_minutes: i32,
    pub created_at: DateTime<Utc>,
}

pub struct SlaMeasurement {
    pub id: Uuid,
    pub ticket_id: Uuid,
    pub sla_definition_id: Uuid,
    pub first_response_at: Option<DateTime<Utc>>,
    pub resolution_at: Option<DateTime<Utc>>,
    pub response_breached: bool,
    pub resolution_breached: bool,
    pub response_time_seconds: Option<i64>,
    pub resolution_time_seconds: Option<i64>,
}
```

---

### ISP-CUST-H03: No Field Tech Mobile API

**Required Endpoints:**
```
/api/v1/field-ops/
  GET    /my-assignments         — List assigned installations
  POST   /assignments/:id/checkin — GPS check-in (lat, lng, timestamp)
  POST   /assignments/:id/checkout — GPS check-out
  POST   /assignments/:id/photo  — Upload installation photo
  POST   /assignments/:id/barcode — Scan equipment barcode
  GET    /route                  — Get optimized route for day
  POST   /assignments/:id/complete — Mark complete with notes
  GET    /offline-sync           — Get data for offline mode
```

**Offline Sync Data:**
```json
{
  "assignments": [...],
  "customer_info": {...},
  "equipment_inventory": [...],
  "installation_checklist": [...],
  "cached_at": "2026-07-21T10:00:00Z",
  "expires_at": "2026-07-21T22:00:00Z"
}
```

---

### ISP-CUST-H04: No Customer Communication Preferences

**Required Entity:**
```rust
pub struct NotificationPreference {
    pub id: Uuid,
    pub customer_id: Uuid,
    pub channel: NotificationChannel, // email, sms, whatsapp, push
    pub event_type: String,           // invoice.generated, ticket.updated, etc.
    pub enabled: bool,
    pub quiet_hours_start: Option<NaiveTime>,
    pub quiet_hours_end: Option<NaiveTime>,
    pub created_at: DateTime<Utc>,
}
```

---

### ISP-CUST-H05: No Subscription Downgrade Protection

**Required Validation:**
```rust
pub async fn validate_downgrade(
    &self,
    subscription_id: Uuid,
    target_plan_id: Uuid,
) -> Result<DowngradeValidation> {
    let subscription = self.get_subscription(subscription_id).await?;
    let target_plan = self.get_plan(target_plan_id).await?;
    let current_usage = self.get_current_usage(subscription_id).await?;

    // Check if current usage exceeds target plan limit
    if current_usage.download_speed > target_plan.download_limit {
        return Ok(DowngradeValidation {
            allowed: false,
            reason: format!(
                "Current usage ({} Mbps) exceeds target plan limit ({} Mbps). \
                 Please wait for usage to drop below limit or contact support.",
                current_usage.download_speed, target_plan.download_limit
            ),
            suggested_action: DowngradeAction::ScheduleForNextCycle,
        });
    }

    // Check for active large transfers
    let active_transfers = self.check_active_transfers(subscription_id).await?;
    if active_transfers {
        return Ok(DowngradeValidation {
            allowed: false,
            reason: "Active data transfer detected. Downgrade will be applied after transfer completes.".into(),
            suggested_action: DowngradeAction::ScheduleAfterTransfer,
        });
    }

    Ok(DowngradeValidation {
        allowed: true,
        reason: "Downgrade approved".into(),
        suggested_action: DowngradeAction::Immediate,
    })
}
```

---

## Medium Gaps

### ISP-CUST-M01: No Fraud Detection
- Required: Concurrent login detection, MAC spoofing check, speed bypass detection
- New entity: `fraud_alert` with detection rules

### ISP-CUST-M02: No Data Retention Enforcement
- Required: Background worker to purge soft-deleted records after retention period
- Configurable per entity type (customers: 7yr, tickets: 2yr, sessions: 1yr)

---

## New Dependencies Required

```toml
# Cargo.toml additions
reqwest = { version = "0.12", features = ["json"] }  # WhatsApp API
```

---

## Database Schema Additions

```sql
-- Customer portal tables
CREATE TABLE customer_portal_users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    customer_id UUID NOT NULL REFERENCES customers(id),
    pin_hash TEXT NOT NULL,  -- 4-digit PIN for mobile app
    biometric_enabled BOOLEAN DEFAULT FALSE,
    last_login_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE notification_preferences (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    customer_id UUID NOT NULL REFERENCES customers(id),
    channel TEXT NOT NULL,  -- email, sms, whatsapp, push
    event_type TEXT NOT NULL,  -- invoice.generated, ticket.updated, etc.
    enabled BOOLEAN DEFAULT TRUE,
    quiet_hours_start TIME,
    quiet_hours_end TIME,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(customer_id, channel, event_type)
);

CREATE TABLE sla_definitions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    plan_id UUID REFERENCES plans(id),
    priority TEXT NOT NULL,
    response_time_minutes INTEGER NOT NULL,
    resolution_time_minutes INTEGER NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE sla_measurements (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    ticket_id UUID NOT NULL REFERENCES tickets(id),
    sla_definition_id UUID NOT NULL REFERENCES sla_definitions(id),
    first_response_at TIMESTAMPTZ,
    resolution_at TIMESTAMPTZ,
    response_breached BOOLEAN DEFAULT FALSE,
    resolution_breached BOOLEAN DEFAULT FALSE,
    response_time_seconds INTEGER,
    resolution_time_seconds INTEGER,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE fraud_alerts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    customer_id UUID REFERENCES customers(id),
    alert_type TEXT NOT NULL,  -- concurrent_login, mac_spoof, speed_bypass
    severity TEXT NOT NULL,    -- low, medium, high, critical
    details JSONB NOT NULL,
    status TEXT DEFAULT 'open',  -- open, investigating, resolved, false_positive
    resolved_by UUID REFERENCES users(id),
    resolved_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW()
);
```
