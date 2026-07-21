# Workers & Infrastructure — ISP Design Gaps

**Module:** `workers`, `integrations`
**Cross-reference:** `DESIGN-GAPS-DEEP-ANALYSIS.md` (ISP-INFRA-C01, ISP-NET-H01)

---

## Critical Gaps

### ISP-INFRA-C01: Missing Workers

**Current Workers (8):**
| Worker | Interval | Status |
|--------|----------|--------|
| BillingWorker | 5 min | ✅ Real |
| NotificationWorker | 30 sec | ✅ Real |
| DeviceSyncWorker | 2 min | ✅ Real |
| BandwidthWorker | 1 min | ✅ Real |
| SchedulerWorker | 30 sec | ✅ Real |
| OutboxWorker | Event-driven | ✅ Real |
| OutboxCleanupWorker | 1 hour | ✅ Real |
| PartitionWorker | Hourly | ✅ Real |

**Missing Workers (9):**

#### 1. ProvisioningWorker (CRITICAL)
```
Trigger: subscription.activated event
Steps:
  1. Create PPPoE credentials on RADIUS server
  2. Push bandwidth profile to BNG/MikroTik
  3. Configure ONT VLAN/QoS on Huawei OLT
  4. Verify connectivity (SNMP ping)
  5. Publish customer.provisioned event
  6. Notify customer: "Your service is now active!"
Failure Handling:
  - Retry 3x with exponential backoff (5s, 30s, 5min)
  - On final failure: rollback completed steps, alert NOC
  - Mark provisioning_job as failed
```

#### 2. RadiusAccountingWorker (CRITICAL)
```
Trigger: Continuous (UDP listener on port 1813)
Steps:
  1. Receive RADIUS Accounting-Request packets
  2. Parse Acct-Session-Id, Acct-Status-Type
  3. Correlate to pppoe_sessions table
  4. Update bytes_in (Acct-Input-Octets)
  5. Update bytes_out (Acct-Output-Octets)
  6. Update session_end on Accounting-Stop
  7. Publish accounting events to NATS
```

#### 3. CdrIngestionWorker (CRITICAL)
```
Trigger: Scheduled (every hour) or file upload
Steps:
  1. Scan for new CDR files (CSV/binary)
  2. Parse CDR records (timestamp, session_id, bytes_in, bytes_out)
  3. Correlate to customer sessions
  4. Insert into bandwidth_usage table
  5. Update customer usage aggregates
  6. Flag anomalies (sudden spike, zero usage)
```

#### 4. UsageMeteringWorker (CRITICAL)
```
Trigger: Every 5 minutes
Steps:
  1. Query active PPPoE sessions
  2. Get current bytes_in/bytes_out from RADIUS accounting
  3. Calculate delta since last metering
  4. Insert into bandwidth_usage (partitioned by day)
  5. Check FUP limits per customer
  6. If FUP exceeded: apply throttle or notify customer
```

#### 5. SlaMonitorWorker (HIGH)
```
Trigger: Every 1 minute
Steps:
  1. Query open tickets
  2. For each ticket: get SLA definition
  3. Check response SLA (first_response_at)
  4. Check resolution SLA
  5. Check escalation matrix
  6. If breached: escalate, notify, create alert
```

#### 6. MassIncidentWorker (HIGH)
```
Trigger: Every 5 minutes
Steps:
  1. Query device health alerts in last 5 minutes
  2. Correlate by location/area/OLT
  3. If 3+ devices in same area are down: create mass incident
  4. Identify affected customers (by device → subscription)
  5. Bulk notify affected customers
  6. Create single ticket for mass incident
```

#### 7. FraudDetectionWorker (MEDIUM)
```
Trigger: Every 15 minutes
Steps:
  1. Check for concurrent logins (same credentials, different IPs)
  2. Check for MAC address anomalies
  3. Check for speed bypass attempts
  4. Check for unusual usage patterns
  5. If suspicious: create fraud_alert, notify security team
```

#### 8. ReportGenerationWorker (MEDIUM)
```
Trigger: Daily at 06:00 IST
Steps:
  1. Generate daily revenue report
  2. Generate subscriber growth report
  3. Generate churn report
  4. Generate bandwidth utilization report
  5. Generate SLA compliance report
  6. Store reports for admin dashboard
```

#### 9. BackupWorker (MEDIUM)
```
Trigger: Daily at 02:00 IST
Steps:
  1. pg_dump full database
  2. Compress backup file
  3. Upload to MinIO/S3
  4. Verify backup integrity
  5. Delete backups older than 30 days
  6. Notify admin of backup status
```

---

## Worker Implementation Template

```rust
// src/workers/provisioning_worker.rs

use tokio::time::{interval, Duration};
use tracing::{info, error, warn};

pub struct ProvisioningWorker {
    db: DatabaseConnection,
    radius: Option<Arc<dyn RadiusClient>>,
    mikrotik: Option<Arc<dyn MikrotikAdapter>>,
    huawei: Option<Arc<dyn HuaweiOltAdapter>>,
    event_publisher: EventPublisher,
    shutdown: broadcast::Sender<()>,
}

impl ProvisioningWorker {
    pub fn new(
        db: DatabaseConnection,
        radius: Option<Arc<dyn RadiusClient>>,
        mikrotik: Option<Arc<dyn MikrotikAdapter>>,
        huawei: Option<Arc<dyn HuaweiOltAdapter>>,
        event_publisher: EventPublisher,
    ) -> Self {
        let (shutdown, _) = broadcast::channel(1);
        Self { db, radius, mikrotik, huawei, event_publisher, shutdown }
    }

    pub async fn run(&self) {
        let mut interval = interval(Duration::from_secs(30));
        let mut shutdown_rx = self.shutdown.subscribe();

        info!("ProvisioningWorker started");

        loop {
            tokio::select! {
                _ = interval.tick() => {
                    if let Err(e) = self.process_pending_jobs().await {
                        error!("ProvisioningWorker error: {}", e);
                    }
                }
                _ = shutdown_rx.recv() => {
                    info!("ProvisioningWorker shutting down");
                    break;
                }
            }
        }
    }

    async fn process_pending_jobs(&self) -> Result<()> {
        let pending_jobs = ProvisioningJob::find_pending(&self.db).await?;

        for job in pending_jobs {
            match self.execute_provisioning(&job).await {
                Ok(_) => {
                    job.mark_completed(&self.db).await?;
                    self.event_publisher.publish(
                        "customer.provisioned",
                        &serde_json::json!({
                            "customer_id": job.customer_id,
                            "subscription_id": job.subscription_id,
                        }),
                    ).await?;
                }
                Err(e) => {
                    warn!("Provisioning failed for job {}: {}", job.id, e);
                    if job.retry_count < 3 {
                        job.increment_retry(&self.db).await?;
                    } else {
                        job.mark_failed(&self.db, &e.to_string()).await?;
                        self.alert_noc(&job, &e).await?;
                    }
                }
            }
        }

        Ok(())
    }
}
```

---

## Integration Adapter Gaps

### Missing Dependencies (Cargo.toml)

```toml
# Required additions
snmp = "0.9"                    # SNMP polling
ipnetwork = "0.20"              # CIDR parsing
reqwest = { version = "0.12", features = ["json"] }  # HTTP client
printpdf = "0.7"                # PDF generation
```

### Missing Integrations

| Integration | Purpose | Priority |
|-------------|---------|----------|
| SNMP Library | Poll legacy devices | CRITICAL |
| ZTE OLT Adapter | ZTE GPON management | HIGH |
| TR-069/CWMP | Remote ONT management | HIGH |
| GSTN API | E-invoice IRN generation | MEDIUM |
| Bank Statement Parser | Payment reconciliation | MEDIUM |

### Connection Pool Templates

```rust
// src/infrastructure/pools/mikrotik_pool.rs

pub struct MikrotikPool {
    clients: Vec<Arc<MikrotikClient>>,
    config: MikrotikConfig,
    max_connections: usize,
}

impl MikrotikPool {
    pub async fn new(config: MikrotikConfig, max_connections: usize) -> Result<Self> {
        let mut clients = Vec::new();
        for _ in 0..max_connections {
            let client = MikrotikClient::new(&config).await?;
            clients.push(Arc::new(client));
        }
        Ok(Self { clients, config, max_connections })
    }

    pub async fn get_client(&self) -> Result<Arc<MikrotikClient>> {
        // Round-robin or least-connections strategy
        todo!()
    }
}

// src/infrastructure/pools/ssh_pool.rs

pub struct SshPool {
    sessions: HashMap<IpAddr, Vec<SshSession>>,
    max_per_host: usize,
}

impl SshPool {
    pub async fn get_session(&self, host: IpAddr) -> Result<SshSession> {
        // Return idle session or create new if under limit
        todo!()
    }

    pub async fn return_session(&self, session: SshSession) {
        // Return session to pool
        todo!()
    }
}
```

---

## Worker Registration in main.rs

```rust
// Add to main.rs worker spawning section:

// Provisioning Worker
if let (Some(radius), Some(mikrotik), Some(huawei)) = (&radius_client, &mikrotik_pool, &huawei_pool) {
    let provisioning_worker = ProvisioningWorker::new(
        db.clone(),
        radius.clone(),
        mikrotik.clone(),
        huawei.clone(),
        event_publisher.clone(),
    );
    tokio::spawn(async move { provisioning_worker.run().await });
}

// SLA Monitor Worker
let sla_worker = SlaMonitorWorker::new(db.clone(), notification_service.clone());
tokio::spawn(async move { sla_worker.run().await });

// Fraud Detection Worker
let fraud_worker = FraudDetectionWorker::new(db.clone(), notification_service.clone());
tokio::spawn(async move { fraud_worker.run().await });
```

---

## Monitoring & Observability

### Worker Health Metrics

```rust
// Each worker should expose:
// - worker_jobs_processed_total (counter)
// - worker_jobs_failed_total (counter)
// - worker_last_success_timestamp (gauge)
// - worker_active_jobs (gauge)

use prometheus::{IntCounter, Gauge, IntGauge};

pub struct WorkerMetrics {
    pub jobs_processed: IntCounter,
    pub jobs_failed: IntCounter,
    pub last_success: Gauge,
    pub active_jobs: IntGauge,
}

impl WorkerMetrics {
    pub fn new(worker_name: &str) -> Self {
        Self {
            jobs_processed: IntCounter::new(
                format!("worker_{}_jobs_processed_total", worker_name),
                "Total jobs processed"
            ).unwrap(),
            jobs_failed: IntCounter::new(
                format!("worker_{}_jobs_failed_total", worker_name),
                "Total jobs failed"
            ).unwrap(),
            last_success: Gauge::new(
                format!("worker_{}_last_success_timestamp", worker_name),
                "Last successful job timestamp"
            ).unwrap(),
            active_jobs: IntGauge::new(
                format!("worker_{}_active_jobs", worker_name),
                "Currently active jobs"
            ).unwrap(),
        }
    }
}
```
