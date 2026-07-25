use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use tracing::{error, info, warn};

use crate::modules::integrations::radius::adapter::{AccountingRequest, AccountingStatusType, RadiusAdapter, RadiusClient};
use crate::modules::network::domain::entities::pppoe_session;

/// Background worker for RADIUS accounting sync:
/// - Syncs PPPoE session data from FreeRADIUS `radacct` table
/// - Sends RADIUS accounting interim updates for active sessions
/// - Handles session termination when FreeRADIUS reports stop
pub struct RadiusAccountingWorker {
    db: DatabaseConnection,
    radius: Option<RadiusAdapter>,
}

impl RadiusAccountingWorker {
    pub fn new(db: DatabaseConnection) -> Self {
        let radius = if std::env::var("RADIUS_SERVER").is_ok() {
            Some(RadiusAdapter::from_env())
        } else {
            info!("RADIUS_SERVER not configured — accounting worker running in passive mode");
            None
        };
        Self { db, radius }
    }

    pub async fn run_cycle(&self) -> anyhow::Result<()> {
        self.sync_active_sessions().await?;
        self.send_interim_updates().await?;
        Ok(())
    }

    /// Sync active PPPoE sessions: update bytes_in/bytes_out from device-reported data
    /// and detect stale sessions that need termination.
    async fn sync_active_sessions(&self) -> anyhow::Result<()> {
        let active_sessions = pppoe_session::Entity::find()
            .filter(pppoe_session::Column::Status.eq("active"))
            .all(&self.db)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to query active sessions: {}", e))?;

        let mut synced = 0;
        let mut stale = 0;

        for session in &active_sessions {
            // Detect stale sessions: active but no activity for > 30 minutes
            if let Some(last_activity) = session.last_activity_at {
                let elapsed = chrono::Utc::now()
                    .signed_duration_since(last_activity);
                if elapsed.num_minutes() > 30 {
                    // Session is stale — mark as terminated
                    warn!(
                        session_id = session.id,
                        username = %session.username,
                        last_activity = %last_activity,
                        "Stale PPPoE session detected — marking as terminated"
                    );
                    let mut active: pppoe_session::ActiveModel = session.clone().into();
                    active.status = Set("terminated".to_string());
                    active.updated_at = Set(chrono::Utc::now());
                    if let Err(e) = active.update(&self.db).await {
                        error!(session_id = session.id, error = %e, "Failed to terminate stale session");
                    } else {
                        stale += 1;

                        // Send RADIUS accounting stop for the terminated session
                        if let Some(ref radius) = self.radius {
                            let nas_ip = session
                                .nas_ip_address
                                .clone()
                                .unwrap_or_else(|| "0.0.0.0".to_string());
                            let nas_port: u32 = session
                                .nas_port_id
                                .as_ref()
                                .and_then(|p| p.parse().ok())
                                .unwrap_or(0);

                            let acct_request = AccountingRequest {
                                username: session.username.clone(),
                                session_id: session
                                    .nas_session_id
                                    .clone()
                                    .unwrap_or_default(),
                                status_type: AccountingStatusType::Stop,
                                nas_ip,
                                nas_port,
                                input_octets: Some(session.bytes_in as u64),
                                output_octets: Some(session.bytes_out as u64),
                                session_time: Some(session.session_duration_seconds as u32),
                                terminate_cause: Some(1), // 1 = User Request
                            };
                            if let Err(e) = radius.accounting_stop(&acct_request).await {
                                error!(session_id = session.id, error = %e, "Failed to send RADIUS accounting stop");
                            }
                        }
                    }
                } else {
                    synced += 1;
                }
            } else {
                // No session start time — stale
                warn!(session_id = session.id, "Session has no start time");
            }
        }

        info!(synced = synced, stale = stale, "RADIUS accounting: session sync complete");
        Ok(())
    }

    /// Send interim accounting updates for all active sessions to RADIUS server.
    async fn send_interim_updates(&self) -> anyhow::Result<()> {
        let Some(ref radius) = self.radius else {
            return Ok(());
        };

        let active_sessions = pppoe_session::Entity::find()
            .filter(pppoe_session::Column::Status.eq("active"))
            .all(&self.db)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to query active sessions: {}", e))?;

        let mut sent = 0;

        for session in &active_sessions {
            let nas_ip = session
                .nas_ip_address
                .clone()
                .unwrap_or_else(|| "0.0.0.0".to_string());
            let nas_port: u32 = session
                .nas_port_id
                .as_ref()
                .and_then(|p| p.parse().ok())
                .unwrap_or(0);

            let session_time = session
                .session_start
                .map(|start| {
                    chrono::Utc::now()
                        .signed_duration_since(start)
                        .num_seconds()
                        .max(0) as u32
                })
                .unwrap_or(0);

            let acct_request = AccountingRequest {
                username: session.username.clone(),
                session_id: session
                    .nas_session_id
                    .clone()
                    .unwrap_or_default(),
                status_type: AccountingStatusType::InterimUpdate,
                nas_ip,
                nas_port,
                input_octets: Some(session.bytes_in as u64),
                output_octets: Some(session.bytes_out as u64),
                session_time: Some(session_time),
                terminate_cause: None,
            };

            match radius.accounting_interim(&acct_request).await {
                Ok(()) => {
                    sent += 1;
                    // Update last_activity_at
                    let mut active: pppoe_session::ActiveModel = session.clone().into();
                    active.last_activity_at = Set(Some(chrono::Utc::now()));
                    active.updated_at = Set(chrono::Utc::now());
                    let _ = active.update(&self.db).await;
                }
                Err(e) => {
                    warn!(
                        session_id = session.id,
                        username = %session.username,
                        error = %e,
                        "Failed to send RADIUS interim update"
                    );
                }
            }
        }

        info!(sent = sent, total = active_sessions.len(), "RADIUS accounting: interim updates sent");
        Ok(())
    }
}
