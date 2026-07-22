# AeroXe Backend — DevOps & Deployment

> **Req Ref:** §20 DevOps and Production Deployment  
> **Gap analysis:** See `DESIGN-GAPS-DEEP-ANALYSIS.md` and relevant `GAP-*.md` files.

---

## 1. Overview

Production-ready deployment using Docker, Kubernetes, CI/CD pipelines, and comprehensive monitoring with Prometheus + Grafana. Designed for high availability with zero-downtime deployments.

## 2. Docker Configuration

### Dockerfile (Multi-stage Build)
```dockerfile
# Build stage
FROM rust:1.78-slim as builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src/ src/
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/aeroxe-backend /usr/local/bin/
EXPOSE 8000
CMD ["aeroxe-backend"]
```

### Docker Compose (Development)
```yaml
version: '3.8'
services:
  backend:
    build: .
    ports: ["8000:8000"]
    environment:
      DATABASE_URL: postgresql://aeroxe:secret@postgres:5432/aeroxe
      REDIS_URL: redis://redis:6379
      NATS_URL: nats://nats:4222
    depends_on: [postgres, redis, nats, minio]

  postgres:
    image: postgis/postgis:16-3.4
    environment:
      POSTGRES_DB: aeroxe
      POSTGRES_USER: aeroxe
      POSTGRES_PASSWORD: secret
    volumes: [pgdata:/var/lib/postgresql/data]
    ports: ["5432:5432"]

  redis:
    image: redis:7-alpine
    command: redis-server --appendonly yes
    volumes: [redisdata:/data]
    ports: ["6379:6379"]

  nats:
    image: nats:2.10-alpine
    command: --jetstream --store_dir /data
    volumes: [natsdata:/data]
    ports: ["4222:4222", "8222:8222"]

  minio:
    image: minio/minio
    command: server /data --console-address ":9001"
    environment:
      MINIO_ROOT_USER: minioadmin
      MINIO_ROOT_PASSWORD: minioadmin
    volumes: [miniodata:/data]
    ports: ["9000:9000", "9001:9001"]

volumes:
  pgdata:
  redisdata:
  natsdata:
  miniodata:
```

## 3. Kubernetes Deployment

### Namespace
```yaml
apiVersion: v1
kind: Namespace
metadata:
  name: aeroxe-prod
```

### Backend Deployment
```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: aeroxe-backend
  namespace: aeroxe-prod
spec:
  replicas: 3
  strategy:
    type: RollingUpdate
    rollingUpdate:
      maxUnavailable: 1
      maxSurge: 1
  selector:
    matchLabels:
      app: aeroxe-backend
  template:
    spec:
      containers:
      - name: backend
        image: registry.aeroxe.com/backend:latest
        ports:
        - containerPort: 8000
        resources:
          requests:
            memory: "256Mi"
            cpu: "250m"
          limits:
            memory: "512Mi"
            cpu: "500m"
        envFrom:
        - secretRef:
            name: aeroxe-secrets
        livenessProbe:
          httpGet:
            path: /health
            port: 8000
          initialDelaySeconds: 10
          periodSeconds: 30
        readinessProbe:
          httpGet:
            path: /ready
            port: 8000
          initialDelaySeconds: 5
          periodSeconds: 10
```

### Service & Ingress
```yaml
apiVersion: v1
kind: Service
metadata:
  name: aeroxe-backend
  namespace: aeroxe-prod
spec:
  selector:
    app: aeroxe-backend
  ports:
  - port: 8000
    targetPort: 8000
  type: ClusterIP

---
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: aeroxe-api
  namespace: aeroxe-prod
  annotations:
    nginx.ingress.kubernetes.io/ssl-redirect: "true"
    cert-manager.io/cluster-issuer: letsencrypt-prod
spec:
  tls:
  - hosts: [api.aeroxebroadband.com]
    secretName: aeroxe-api-tls
  rules:
  - host: api.aeroxebroadband.com
    http:
      paths:
      - path: /
        pathType: Prefix
        backend:
          service:
            name: aeroxe-backend
            port:
              number: 8000
```

## 4. CI/CD Pipeline (GitHub Actions)

```yaml
name: CI/CD
on:
  push:
    branches: [main, dev]
  pull_request:
    branches: [main]

jobs:
  test:
    runs-on: ubuntu-latest
    services:
      postgres:
        image: postgis/postgis:16-3.4
        env:
          POSTGRES_DB: aeroxe_test
          POSTGRES_USER: aeroxe
          POSTGRES_PASSWORD: test
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo test
      - run: cargo clippy -- -D warnings
      - run: cargo fmt --check

  build:
    needs: test
    if: github.ref == 'refs/heads/main'
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Build Docker image
        run: docker build -t aeroxe-backend:${{ github.sha }} .
      - name: Push to registry
        run: |
          docker tag aeroxe-backend:${{ github.sha }} registry.aeroxe.com/backend:${{ github.sha }}
          docker push registry.aeroxe.com/backend:${{ github.sha }}

  deploy:
    needs: build
    runs-on: ubuntu-latest
    steps:
      - name: Deploy to Kubernetes
        run: |
          kubectl set image deployment/aeroxe-backend \
            backend=registry.aeroxe.com/backend:${{ github.sha }} \
            -n aeroxe-prod
          kubectl rollout status deployment/aeroxe-backend -n aeroxe-prod
```

## 5. Monitoring Stack

### Prometheus Configuration
```yaml
scrape_configs:
  - job_name: 'aeroxe-backend'
    static_configs:
      - targets: ['backend:8000']
    metrics_path: '/metrics'
    scrape_interval: 15s

  - job_name: 'postgres'
    static_configs:
      - targets: ['postgres-exporter:9187']

  - job_name: 'redis'
    static_configs:
      - targets: ['redis-exporter:9121']

  - job_name: 'nats'
    static_configs:
      - targets: ['nats:8222']
```

### Key Metrics
| Metric | Description | Alert Threshold |
|--------|-------------|-----------------|
| `http_requests_total` | Total HTTP requests | - |
| `http_request_duration_seconds` | Request latency | p99 > 2s |
| `http_requests_errors_total` | Error rate | > 5% for 5min |
| `db_connections_active` | Active DB connections | > 80% pool |
| `redis_connections_active` | Active Redis connections | > 80% pool |
| `nats_messages_published` | Events published | - |
| `invoices_generated_total` | Invoices/day | - |
| `active_subscriptions` | Current subscribers | - |
| `device_online_count` | Online devices | Drop > 10% |

### Grafana Dashboards
- API Performance (latency, throughput, errors)
- Database (connections, query time, cache hit rate)
- Business Metrics (revenue, subscribers, churn)
- Network Health (device status, bandwidth usage)
- Infrastructure (CPU, memory, disk, network)

## 6. Alert Rules

```yaml
groups:
  - name: aeroxe-alerts
    rules:
      - alert: HighErrorRate
        expr: rate(http_requests_errors_total[5m]) > 0.05
        for: 5m
        labels:
          severity: critical
        annotations:
          summary: "High error rate detected"

      - alert: HighLatency
        expr: histogram_quantile(0.99, rate(http_request_duration_seconds_bucket[5m])) > 2
        for: 5m
        labels:
          severity: warning

      - alert: DatabaseConnectionsHigh
        expr: db_connections_active / db_connections_max > 0.8
        for: 2m
        labels:
          severity: warning

      - alert: DeviceOffline
        expr: device_online_count < device_online_count offset 1h * 0.9
        for: 10m
        labels:
          severity: critical
```

## 7. Backup Strategy

| Component | Method | Frequency | Retention |
|-----------|--------|-----------|-----------|
| PostgreSQL | pg_dump + WAL | Daily + continuous | 30 days |
| Redis | RDB + AOF | Every 6 hours | 7 days |
| NATS JetStream | File snapshot | Daily | 14 days |
| MinIO | Replication | Real-time | 30 days |
| Config/Secrets | Git (encrypted) | On change | Indefinite |

## 8. Environment Tiers

| Environment | Purpose | Infrastructure |
|------------|---------|---------------|
| Local | Development | Docker Compose |
| Staging | Pre-production testing | Kubernetes (small) |
| Production | Live system | Kubernetes (HA) |

## 9. RBAC Permissions

```
devops.config.view
devops.config.update
devops.deploy.trigger
devops.backup.view
devops.backup.restore
devops.monitoring.view
devops.alerts.manage
```
