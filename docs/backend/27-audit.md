# AeroXe Backend — Audit Module

> **Req Ref:** §2.10 Audit Tracking

---

## 1. Overview

Every permission check and action is logged to provide a complete, immutable audit trail. Used for security compliance, incident investigation, and regulatory requirements. Partitioned monthly for performance.

## 2. Database Tables

```sql
CREATE TABLE audit_logs (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT REFERENCES users(id),
    user_email VARCHAR(255),
    user_role VARCHAR(100),
    action VARCHAR(255) NOT NULL,
    resource_type VARCHAR(100),
    resource_id UUID,
    ip_address INET,
    user_agent TEXT,
    result VARCHAR(20) NOT NULL CHECK (result IN ('granted', 'denied', 'expired')),
    old_data JSONB,
    new_data JSONB,
    metadata JSONB,
    created_at TIMESTAMPTZ DEFAULT NOW()
) PARTITION BY RANGE (created_at);

-- Create monthly partitions
CREATE TABLE audit_logs_2026_07 PARTITION OF audit_logs
    FOR VALUES FROM ('2026-07-01') TO ('2026-08-01');
```

**Indexes:**
```sql
CREATE INDEX idx_audit_logs_user_id ON audit_logs(user_id);
CREATE INDEX idx_audit_logs_action ON audit_logs(action);
CREATE INDEX idx_audit_logs_created_at ON audit_logs(created_at);
CREATE INDEX idx_audit_logs_resource ON audit_logs(resource_type, resource_id);
CREATE INDEX idx_audit_logs_result ON audit_logs(result);
```

## 3. Audit Log Structure

```json
{
  "audit_id": "uuid",
  "timestamp": "2026-07-08T14:30:00Z",
  "user_id": 42,
  "user_email": "admin@aeroxe.com",
  "user_role": "network_admin",
  "action": "device.router.restart",
  "resource_type": "device",
  "resource_id": "550e8400-e29b-41d4-a716-446655440000",
  "ip_address": "10.0.1.50",
  "user_agent": "Mozilla/5.0...",
  "result": "granted",
  "old_data": null,
  "new_data": { "status": "online" },
  "metadata": {
    "device_name": "Jalgaon-CityCenter-R01",
    "reason": "Customer reported connectivity issue"
  }
}
```

## 4. Audit Middleware

```rust
pub struct AuditMiddleware;

impl<S> Middleware<S> for AuditMiddleware {
    async fn call(&self, req: Request, next: Next<S>) -> Response {
        let start = Instant::now();
        let method = req.method().clone();
        let uri = req.uri().clone();
        let user = req.extensions().get::<UserContext>().cloned();
        let ip = extract_ip(&req);
        let user_agent = extract_user_agent(&req);

        let response = next.run(req).await;

        // Log the action asynchronously
        let action = format!("{} {}", method, uri);
        let result = if response.status().is_success() {
            "granted"
        } else if response.status() == StatusCode::FORBIDDEN {
            "denied"
        } else {
            "success"
        };

        tokio::spawn(async move {
            insert_audit_log(InsertAuditLog {
                user_id: user.as_ref().map(|u| u.id),
                user_email: user.as_ref().map(|u| u.email.clone()),
                user_role: user.as_ref().map(|u| u.role.clone()),
                action,
                resource_type: extract_resource_type(&uri),
                resource_id: extract_resource_id(&uri),
                ip_address: ip,
                user_agent,
                result: result.to_string(),
                metadata: json!({ "duration_ms": start.elapsed().as_millis() }),
            }).await.ok();
        });

        response
    }
}
```

## 5. API Endpoints

> **API Convention:** Protobuf-first. No GET, no PUT, no path variables, no query strings. See `API-CONVENTIONS.md`.

| Method | Path | Required Role | Description |
|--------|------|--------------|-------------|
| `POST` | `/api/v1/audit/logs/list` | audit.log.view | Search audit logs |
| `POST` | `/api/v1/audit/logs/get` | audit.log.view | Get specific log entry |
| `POST` | `/api/v1/audit/export` | audit.log.export | Export logs (CSV/JSON) |
| `POST` | `/api/v1/audit/user/list` | audit.log.view | User activity log |
| `POST` | `/api/v1/audit/resource/list` | audit.log.view | Resource history |

## 6. Query Filters

```sql
-- Search audit logs with filters
SELECT * FROM audit_logs
WHERE ($1::BIGINT IS NULL OR user_id = $1)
  AND ($2::TEXT IS NULL OR action LIKE '%' || $2 || '%')
  AND ($3::TEXT IS NULL OR resource_type = $3)
  AND ($4::TEXT IS NULL OR result = $4)
  AND created_at >= $5
  AND created_at <= $6
ORDER BY created_at DESC
LIMIT $7 OFFSET $8;
```

## 7. Retention Policy

- Hot data (current month): full index, fast queries
- Warm data (1-6 months): compressed partitions
- Cold data (6-12 months): archived to S3/MinIO
- Legal retention: 7 years minimum

## 8. Background Jobs

```
Daily at 2 AM IST:
  - Create next month's partition
  - Compress partitions older than 30 days
  - Archive partitions older than 6 months

Weekly:
  - Verify partition integrity
  - Clean up orphaned partitions
```

## 9. RBAC Permissions

```
audit.log.view
audit.log.export
audit.log.search
```
