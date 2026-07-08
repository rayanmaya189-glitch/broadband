# AeroXe Backend — Redis Design

> **Req Ref:** §14 Redis Design

---

## 1. Overview

Redis serves multiple purposes in the platform:

| Use Case | Redis Feature | TTL |
|----------|--------------|-----|
| Session store | Hash + String | 24h (access), 7d (refresh) |
| Rate limiting | Sorted Set (sliding window) | 1 min–1 hour |
| Caching | String (JSON serialized) | 5–60 min |
| Real-time presence | Set | Until disconnect |
| Pub/Sub for WebSocket | Pub/Sub channels | N/A |
| Temporary tokens | String | 5–15 min |
| Feature flags | Hash | No expiry |
| Distributed locks | SET NX | 30s |

## 2. Key Naming Convention

```
{module}:{entity}:{id}:{field}
```

Examples:
```
session:{user_id}:access
session:{user_id}:refresh
rate:{ip}:{endpoint}
cache:plans:active
cache:customer:{id}:profile
presence:device:{id}
lock:billing:generate:{subscription_id}
```

## 3. Session Management

```
# Access token (short-lived)
SET session:{user_id}:access {jwt_token} EX 86400

# Refresh token (long-lived)
SET session:{user_id}:refresh {refresh_hash} EX 604800

# Session metadata
HSET session:{user_id}:meta
    ip "10.0.1.50"
    user_agent "Mozilla/5.0..."
    branch_id "1"
    role "noc_engineer"
    created_at "2026-07-08T10:00:00Z"

# Token blacklist (on logout)
SET blacklist:{token_jti} "1" EX {remaining_ttl}
```

## 4. Rate Limiting

Sliding window rate limiter using sorted sets:

```
# Key: rate:{identifier}:{window}
# Score: timestamp in microseconds
# Value: unique request ID

# Check rate limit
ZRANGEBYSCORE rate:{ip}:1min {now - 60s} {now}
# If count > limit → 429 Too Many Requests

# Add request
ZADD rate:{ip}:1min {now} {request_id}
EXPIRE rate:{ip}:1min 60
```

**Rate limit tiers:**

| Endpoint | Limit | Window | Burst |
|----------|-------|--------|-------|
| Auth (login/OTP) | 5 requests | 1 min | 10 |
| API general | 100 requests | 1 min | 200 |
| API write (POST/PUT) | 30 requests | 1 min | 60 |
| File upload | 10 requests | 5 min | 20 |
| Availability check | 20 requests | 1 min | 40 |
| Public (unauthenticated) | 30 requests | 1 min | 50 |

## 5. Caching Strategy

### Cache-Aside Pattern

```
1. Check cache → if HIT, return cached data
2. Query database
3. Serialize result to JSON
4. SET cache:{key} {json} EX {ttl}
5. Return data
```

### Cache Invalidation

```
# On entity update, invalidate related caches
DEL cache:plans:active
DEL cache:customer:{id}:profile
DEL cache:customer:{id}:subscription

# On entity delete, scan and invalidate
SCAN 0 MATCH cache:customer:{id}:* COUNT 100
```

### Cache TTLs

| Cache Key | TTL | Invalidation |
|-----------|-----|-------------|
| `cache:plans:active` | 10 min | On plan create/update |
| `cache:customer:{id}:profile` | 5 min | On profile update |
| `cache:customer:{id}:subscription` | 5 min | On subscription change |
| `cache:device:{id}:status` | 30s | On device status change |
| `cache:network:vlan:{branch_id}` | 15 min | On VLAN create/update |
| `cache:network:ippool:{branch_id}` | 15 min | On pool update |
| `cache:coverage:pincodes` | 60 min | On coverage area change |
| `cache:rbac:permissions:{role}` | 30 min | On permission change |
| `cache:branch:{id}:config` | 30 min | On branch config change |

## 6. Real-Time Presence

```
# Device online status
SADD presence:devices:online {device_id_1} {device_id_2} ...
EXPIRE presence:devices:online 300  # Refreshed by heartbeat

# Customer online status
SADD presence:customers:online {customer_id_1} ...
EXPIRE presence:customers:online 300

# Per-device last heartbeat
SET presence:device:{id}:heartbeat {timestamp} EX 300
```

## 7. Distributed Locks

Used for concurrent operations (invoice generation, payment processing):

```
# Acquire lock
SET lock:{resource}:{id} {owner_id} NX EX 30

# Release lock (only if owner)
EVAL "
  if redis.call('get', KEYS[1]) == ARGV[1] then
    return redis.call('del', KEYS[1])
  else
    return 0
  end
" 1 lock:{resource}:{id} {owner_id}
```

**Lock resources:**
- `lock:billing:generate:{subscription_id}` — Invoice generation
- `lock:payment:process:{invoice_id}` — Payment processing
- `lock:subscription:renew:{subscription_id}` — Auto-renewal
- `lock:bandwidth:apply:{subscription_id}` — Profile application
- `lock:device:configure:{device_id}` — Device configuration

## 8. Pub/Sub Channels

```
# WebSocket broadcast channels
ws:customer:{customer_id}        # Customer-specific updates
ws:branch:{branch_id}            # Branch-wide updates
ws:noc:alerts                    # NOC dashboard alerts
ws:noc:devices                   # Device status changes
ws:noc:sessions                  # Session status changes

# Internal event channels
event:invoice.generated
event:invoice.paid
event:invoice.overdue
event:customer.activated
event:customer.suspended
event:device.status.changed
event:ticket.created
event:ticket.escalated
```

## 9. Temporary Tokens

```
# Password reset token
SET otp:reset:{phone} {otp_code} EX 300

# Email verification token
SET verify:email:{user_id} {token} EX 86400

# 2FA backup codes
HSET 2fa:backup:{user_id} code_1 "used" code_2 "unused" ...
```

## 10. Feature Flags

```
HSET feature_flags
    maintenance_mode "false"
    registration_enabled "true"
    payment_gateway "razorpay"
    whatsapp_integration "true"
    auto_suspension_enabled "false"

# Read flag
HGET feature_flags maintenance_mode
```

## 11. Redis Configuration

```yaml
# redis.conf
maxmemory 512mb
maxmemory-policy allkeys-lru
save 900 1
save 300 10
save 60 10000
appendonly yes
appendfsync everysec
```

## 12. Connection Pool

```rust
// In config.rs
pub fn redis_pool(url: &str) -> redis::aio::ConnectionManager {
    let client = redis::Client::open(url).unwrap();
    // ConnectionManager handles pooling internally
}
```
