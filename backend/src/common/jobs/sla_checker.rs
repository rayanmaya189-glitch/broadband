use std::time::Duration;

use sqlx::FromRow;
use tokio_util::sync::CancellationToken;
use tracing::{info, warn, error};

use crate::app::SharedState;

const DEFAULT_CHECK_INTERVAL_SECS: u64 = 300;

fn escalation_target(priority: &str) -> Option<&str> {
    match priority {
        "low" => Some("medium"),
        "medium" => Some("high"),
        "high" => Some("critical"),
        "critical" => None,
        _ => None,
    }
}

async fn find_sla_breached_tickets(
    conn: &mut sqlx::PgConnection,
) -> Result<Vec<SLABreachedTicket>, sqlx::Error> {
    sqlx::query_as::<_, SLABreachedTicket>(
        "SELECT id, ticket_number, priority, status, assigned_to, sla_resolution_at
         FROM tickets
         WHERE status IN ('open', 'assigned', 'in_progress')
           AND sla_resolution_at IS NOT NULL
           AND sla_resolution_at < NOW()
         ORDER BY sla_resolution_at ASC
         LIMIT 100",
    )
    .fetch_all(&mut *conn)
    .await
}

async fn auto_escalate_ticket(
    conn: &mut sqlx::PgConnection,
    ticket: &SLABreachedTicket,
) -> Result<(), sqlx::Error> {
    let escalate_to = find_escalation_target_user(conn, ticket.assigned_to).await;
    let new_priority = escalation_target(&ticket.priority).map(|s| s.to_string());

    let result = sqlx::query(
        "UPDATE tickets
         SET escalated_to = $2, priority = COALESCE($3, priority),
             escalated_at = NOW(), status = 'escalated', updated_at = NOW()
         WHERE id = $1",
    )
    .bind(ticket.id)
    .bind(escalate_to)
    .bind(new_priority.as_deref())
    .execute(&mut *conn)
    .await?;

    if result.rows_affected() == 0 {
        return Ok(());
    }

    sqlx::query(
        "INSERT INTO ticket_escalations (ticket_id, from_user_id, to_user_id, from_priority, to_priority, reason)
         VALUES ($1, $2, $3, $4, $5, $6)",
    )
    .bind(ticket.id)
    .bind(1i64)
    .bind(escalate_to)
    .bind(&ticket.priority)
    .bind(new_priority.as_deref())
    .bind(format!("Auto-escalated: SLA breach at {}", ticket.sla_resolution_at.map(|t| t.to_rfc3339()).unwrap_or_default()))
    .execute(&mut *conn)
    .await?;

    sqlx::query(
        "INSERT INTO ticket_status_history (ticket_id, old_status, new_status, changed_by, reason)
         VALUES ($1, $2, 'escalated', 1, $3)",
    )
    .bind(ticket.id)
    .bind(&ticket.status)
    .bind(format!("Auto-escalated due to SLA breach (deadline: {})", ticket.sla_resolution_at.map(|t| t.to_rfc3339()).unwrap_or_default()))
    .execute(&mut *conn)
    .await?;

    info!(ticket_id = ticket.id, ticket_number = %ticket.ticket_number, "Ticket auto-escalated due to SLA breach");
    Ok(())
}

async fn find_escalation_target_user(conn: &mut sqlx::PgConnection, assigned_to: Option<i64>) -> i64 {
    let result: Option<(i64,)> = sqlx::query_as(
        "SELECT u.id FROM users u
         JOIN user_roles ur ON ur.user_id = u.id
         JOIN roles r ON r.id = ur.role_id
         WHERE r.name IN ('noc_engineer', 'network_admin', 'admin', 'super_admin')
           AND u.is_active = true AND u.id != COALESCE($1, 0)
         ORDER BY (SELECT COUNT(*) FROM tickets t WHERE t.assigned_to = u.id AND t.status NOT IN ('resolved', 'closed')) ASC, u.id ASC
         LIMIT 1",
    )
    .bind(assigned_to)
    .fetch_optional(&mut *conn)
    .await
    .ok()
    .flatten();

    result.map(|r| r.0).unwrap_or(1)
}

#[derive(Debug, FromRow)]
struct SLABreachedTicket {
    id: i64,
    ticket_number: String,
    priority: String,
    status: String,
    assigned_to: Option<i64>,
    sla_resolution_at: Option<chrono::DateTime<chrono::Utc>>,
}

pub async fn run_sla_checker(state: SharedState, token: CancellationToken) {
    let interval_secs = std::env::var("SLA_CHECK_INTERVAL_SECS")
        .ok().and_then(|v| v.parse().ok()).unwrap_or(DEFAULT_CHECK_INTERVAL_SECS);
    let mut interval = tokio::time::interval(Duration::from_secs(interval_secs));
    info!(interval_secs = interval_secs, "SLA breach checker background job started");

    loop {
        tokio::select! {
            _ = interval.tick() => {
                let mut tx = match super::rls_bypass::begin_bypass_transaction(&state.db).await {
                    Ok(t) => t,
                    Err(e) => { warn!(error = %e, "Failed to begin RLS bypass transaction"); continue; }
                };
                match find_sla_breached_tickets(&mut tx).await {
                    Ok(tickets) if tickets.is_empty() => {}
                    Ok(tickets) => {
                        let count = tickets.len();
                        info!(count = count, "Found SLA-breached tickets, processing");
                        let mut escalated = 0u64;
                        let mut failed = 0u64;
                        for ticket in &tickets {
                            match auto_escalate_ticket(&mut tx, ticket).await {
                                Ok(()) => escalated += 1,
                                Err(e) => { failed += 1; warn!(ticket_id = ticket.id, error = %e, "Failed to auto-escalate ticket"); }
                            }
                        }
                        info!(total = count, escalated = escalated, failed = failed, "SLA checker batch complete");
                    }
                    Err(e) => error!(error = %e, "Failed to query SLA-breached tickets"),
                }
                if let Err(e) = tx.commit().await { error!(error = %e, "Failed to commit RLS bypass transaction"); }
            }
            _ = token.cancelled() => { info!("SLA breach checker shutting down gracefully"); break; }
        }
    }
}
