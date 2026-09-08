# AeroXe Production Deployment Guide

## Security Checklist

Before deploying to production, ensure ALL items are completed:

### Critical (Must-Do)
- [ ] **JWT Keys**: Generate RSA-2048 key pair and set `JWT_PRIVATE_KEY` / `JWT_PUBLIC_KEY`
- [ ] **Database Password**: Change `POSTGRES_PASSWORD` from default `secret`
- [ ] **Redis Password**: Change `REDIS_PASSWORD` from default
- [ ] **MinIO Credentials**: Change `MINIO_ACCESS_KEY` / `MINIO_SECRET_KEY` from defaults
- [ ] **CORS Origins**: Set `CORS_ORIGINS` to your actual domain(s)
- [ ] **APP_ENV**: Set to `production`
- [ ] **TLS**: Configure nginx reverse proxy with valid SSL certificates
- [ ] **Metrics Token**: Set `METRICS_TOKEN` for Prometheus endpoint

### Recommended
- [ ] **SMTP**: Configure `SMTP_USERNAME` / `SMTP_PASSWORD` for transactional email
- [ ] **SMS**: Configure `MSG91_AUTH_KEY` or `TWILIO_*` for OTP delivery
- [ ] **Sentry**: Set `VITE_SENTRY_DSN` for frontend error tracking
- [ ] **Rate Limiting**: Review and tune rate limit tiers for your traffic patterns
- [ ] **Backup**: Set up automated PostgreSQL backups (pg_dump cron or pgBackRest)

### Optional
- [ ] **RADIUS**: Configure `RADIUS_SECRET` for PPPoE authentication
- [ ] **MikroTik**: Configure `MIKROTIK_PASSWORD` for device management
- [ ] **Huawei OLT**: Configure `HUAWEI_OLT_PASSWORD` for GPON provisioning
- [ ] **Payment Gateways**: Configure Razorpay/PayU/Stripe keys

---

## Deployment Steps

### 1. Generate JWT Keys
```bash
# Generate private key
openssl genpkey -algorithm RSA -out jwt_private.pem -pkeyopt rsa_keygen_bits:2048

# Extract public key
openssl rsa -in jwt_private.pem -pubout -out jwt_public.pem

# Set as env vars (or in .env)
export JWT_PRIVATE_KEY="$(cat jwt_private.pem)"
export JWT_PUBLIC_KEY="$(cat jwt_public.pem)"
```

### 2. Configure Environment
```bash
cd backend
cp .env.example .env
# Edit .env with production values
```

### 3. Deploy with Docker Compose
```bash
# Production (single canonical overlay — see docker-compose.prod.yml)
docker compose -f backend/docker-compose.yml -f docker-compose.prod.yml up -d

# Staging
docker compose -f backend/docker-compose.yml -f backend/docker-compose.staging.yml up -d
```

### 4. Verify Deployment
```bash
# Health check
curl http://localhost:8000/health

# Readiness check (DB, Redis, NATS)
curl http://localhost:8000/ready

# Prometheus metrics (requires METRICS_TOKEN)
curl -H "Authorization: Bearer $METRICS_TOKEN" http://localhost:8000/metrics
```

---

## Architecture

```
                    ┌─────────────┐
                    │   Users     │
                    └──────┬──────┘
                           │
                    ┌──────▼──────┐
                    │ nginx:443   │  TLS termination
                    │ (frontend)  │  Static SPA + API proxy
                    └──────┬──────┘
                           │
              ┌────────────┼────────────┐
              │            │            │
       ┌──────▼──────┐ ┌──▼──┐ ┌──────▼──────┐
       │  Frontend   │ │ API │ │  Swagger UI │
       │  (React)    │ │proxy│ │  (dev only) │
       └─────────────┘ └──┬──┘ └─────────────┘
                          │
                   ┌──────▼──────┐
                   │  Backend    │  Axum + SeaORM
                   │  :8000      │
                   └──────┬──────┘
                          │
          ┌───────────────┼───────────────┐
          │               │               │
   ┌──────▼──────┐ ┌─────▼─────┐ ┌──────▼──────┐
   │ PostgreSQL  │ │   Redis   │ │    NATS     │
   │ (primary)   │ │ (cache)   │ │ (messaging) │
   └─────────────┘ └───────────┘ └─────────────┘
```

---

## Environment Variables Reference

| Variable | Required | Default | Description |
|----------|----------|---------|-------------|
| `APP_ENV` | Yes | `development` | `production` or `development` |
| `DATABASE_URL` | Yes | — | PostgreSQL connection string |
| `REDIS_URL` | Yes | — | Redis connection string (with password) |
| `NATS_URL` | Yes | — | NATS connection string |
| `JWT_PRIVATE_KEY` | Prod | — | RSA private key PEM for signing |
| `JWT_PUBLIC_KEY` | Prod | — | RSA public key PEM for verification |
| `CORS_ORIGINS` | Prod | — | Comma-separated allowed origins |
| `METRICS_TOKEN` | Prod | — | Bearer token for `/metrics` endpoint |
| `TRUST_PROXY` | No | `false` | Trust X-Forwarded-For headers |

See `backend/.env.example` for the full list.

---

## Monitoring

### Prometheus Metrics
- `GET /metrics` — Prometheus scrape endpoint (requires `METRICS_TOKEN`)
- `GET /api/v1/metrics/summary` — JSON summary for dashboards

### Database Backups
Automated `pg_dump` backups run daily at 02:00 via the `backup` service
(defined in `docker-compose.prod.yml`), writing compressed dumps into the
`pgbackups` volume with 14-day rolling retention.

```bash
# List available backups
docker compose -f backend/docker-compose.yml -f docker-compose.prod.yml exec backup ls -1t /backups

# Restore a backup (example)
docker compose -f backend/docker-compose.yml -f docker-compose.prod.yml run --rm backup sh -c \
  "pg_restore -h postgres -U aeroxe -d aeroxe < /backups/aeroxe-20260908-0200.dump"
```

### Key Metrics
- `aeroxe_http_requests_total` — Total HTTP requests
- `aeroxe_http_request_duration_seconds` — Request latency histogram
- `aeroxe_db_connections_active` — Active DB connections
- `aeroxe_worker_cycles_total` — Worker execution counts
- `aeroxe_worker_errors_total` — Worker error counts

### Health Checks
- `GET /health` — Always returns 200 (liveness probe)
- `GET /ready` — Returns 200 if DB + Redis are reachable (readiness probe)

---

## Troubleshooting

### "APP_ENV=production requires JWT_PRIVATE_KEY and JWT_PUBLIC_KEY"
Generate RSA keys (see Step 1 above).

### "APP_ENV=production requires explicit CORS_ORIGINS"
Set `CORS_ORIGINS=https://yourdomain.com`.

### Redis connection refused
Ensure Redis is running with password auth: `redis://:password@host:port`.

### Database migration errors
Run `./migrate up` to apply pending migrations.
