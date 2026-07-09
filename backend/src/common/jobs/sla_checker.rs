use std::time::Duration;

use sqlx::{FromRow, PgPool};
use tokio_util::sync::CancellationToken;
use tracing::{info, warn, error};

use crate::app::SharedState;

/// Default SLA escalation check interval (5 minutes).
const DEFAULT_CHECK_INTERVAL_SECS: u64 = 300;

/// Priority escalation ladder when SLA is breached.
/// Maps current priority → next higher priority for escalation.
fn escalation_target(priority: &str) -> Option<&str> {
    match priority {
        "low" => Some("medium"),
        "medium" => Some("high"),
        "high" => Some("critical"),
        "critical" => None, // Already at max — notify only
        _ => None,
    }
}

/// Find tickets that have breached their SLA resolution deadline
/// and haven't been escalated yet (status is still open/assigned/in_progress).
async fn find_sla_breached_tickets(
    pool: &PgPool,
) -> Result<Vec<SLABreachedTicket>, sqlx::Error> {
    let rows = sqlx::query_as::<_, SLABreachedTicket>(
        "SELECT id, ticket_number, priority, status, assigned_to,
                sla_resolution_at
         FROM tickets
         WHERE status IN ('open', 'assigned', 'in_progress')
           AND sla_resolution_at IS NOT NULL
           AND sla_resolution_at < NOW()
         ORDER BY sla_resolution_at ASC
         LIMIT 100",
    )
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

/// Record an SLA breach in ticket_status_history and escalate the ticket.
async fn auto_escalate_ticket(
    pool: &PgPool,
    ticket: &SLABreachedTicket,
) -> Result<(), sqlx::Error> {
    // Determine escalation target — use a system user (ID 1 = super_admin) if unassigned
    let escalate_to = find_escalation_target_user(pool, ticket.assigned_to).await;

    // Determine new priority
    let new_priority = escalation_target(&ticket.priority).map(|s| s.to_string());

    // Update the ticket — use parameterized queries for all values
    let result = sqlx::query(
        "UPDATE tickets
         SET escalated_to = $2,
             priority = COALESCE($3, priority),
             escalated_at = NOW(),
             status = 'escalated',
             updated_at = NOW()
         WHERE id = $1",
    )
    .bind(ticket.id)
    .bind(escalate_to)
    .bind(new_priority.as_deref())
    .execute(pool)
    .await?;

    // If no rows were affected, the ticket was already escalated — skip audit records
    if result.rows_affected() == 0 {
        return Ok(());
    }

    // Record the escalation
    sqlx::query(
        "INSERT INTO ticket_escalations
             (ticket_id, from_user_id, to_user_id, from_priority, to_priority, reason)
         VALUES ($1, $2, $3, $4, $5, $6)",
    )
    .bind(ticket.id)
    .bind(1i64) // System user
    .bind(escalate_to)
    .bind(&ticket.priority)
    .bind(new_priority.as_deref())
    .bind(format!(
        "Auto-escalated: SLA resolution deadline breached at {}",
        ticket.sla_resolution_at.map(|t| t.to_rfc3339()).unwrap_or_default()
    ))
    .execute(pool)
    .await?;

    // Record status history
    sqlx::query(
        "INSERT INTO ticket_status_history
             (ticket_id, old_status, new_status, changed_by, reason)
         VALUES ($1, $2, 'escalated', 1, $3)",
    )
    .bind(ticket.id)
    .bind(&ticket.status)
    .bind(format!(
        "Auto-escalated due to SLA breach (deadline: {})",
        ticket.sla_resolution_at.map(|t| t.to_rfc3339()).unwrap_or_default()
    ))
    .execute(pool)
    .await?;

    info!(
        ticket_id = ticket.id,
        ticket_number = %ticket.ticket_number,
        priority = %ticket.priority,
        new_priority = ?new_priority,
        escalate_to = escalate_to,
        "Ticket auto-escalated due to SLA breach"
    );

    Ok(())
}

/// Find the best user to escalate to: a noc_engineer or admin, preferring
/// users with fewer active tickets.
async fn find_escalation_target_user(
    pool: &PgPool,
    assigned_to: Option<i64>,
) -> i64 {
    let result: Option<(i64,)> = sqlx::query_as(
        "SELECT u.id
         FROM users u
         JOIN user_roles ur ON ur.user_id = u.id
         JOIN roles r ON r.id = ur.role_id
         WHERE r.name IN ('noc_engineer', 'network_admin', 'admin', 'super_admin')
           AND u.is_active = true
           AND u.id != COALESCE($1, 0)
         ORDER BY (
           SELECT COUNT(*) FROM tickets t
           WHERE t.assigned_to = u.id
             AND t.status NOT IN ('resolved', 'closed')
         ) ASC, u.id ASC
         LIMIT 1",
    )
    .bind(assigned_to)
    .fetch_optional(pool)
    .await
    .ok()
    .flatten();

    result.map(|r| r.0).unwrap_or(1) // Fallback to super_admin (ID 1)
}

/// Row type for SLA-breached ticket query.
#[derive(Debug, FromRow)]
struct SLABreachedTicket {
    id: i64,
    ticket_number: String,
    priority: String,
    status: String,
    assigned_to: Option<i64>,
    sla_resolution_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// Main SLA checker loop — runs as a background tokio task.
///
/// Periodically queries for tickets that have breached their SLA resolution
/// deadline and auto-escalates them with appropriate priority bumps.
pub async fn run_sla_checker(state: SharedState, token: CancellationToken) {
    let interval_secs = std::env::var("SLA_CHECK_INTERVAL_SECS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(DEFAULT_CHECK_INTERVAL_SECS);

    let mut interval = tokio::time::interval(Duration::from_secs(interval_secs));

    info!(
        interval_secs = interval_secs,
        "SLA breach checker background job started"
    );

    loop {
        tokio::select! {
            _ = interval.tick() => {
                match find_sla_breached_tickets(&state.db).await {
                    Ok(tickets) if tickets.is_empty() => {
                        // No breached tickets — quiet log
                    }
                    Ok(tickets) => {
                        let count = tickets.len();
                        info!(count = count, "Found SLA-breached tickets, processing");

                        let mut escalated = 0u64;
                        let mut failed = 0u64;

                        for ticket in &tickets {
                            match auto_escalate_ticket(&state.db, ticket).await {
                                Ok(()) => escalated += 1,
                                Err(e) => {
                                    failed += 1;
                                    warn!(
                                        ticket_id = ticket.id,
                                        ticket_number = %ticket.ticket_number,
                                        error = %e,
                                        "Failed to auto-escalate ticket"
                                    );
                                }
                            }
                        }

                        info!(
                            total = count,
                            escalated = escalated,
                            failed = failed,
                            "SLA checker batch complete"
                        );
                    }
                    Err(e) => {
                        error!(error = %e, "Failed to query SLA-breached tickets");
                    }
                }
            }
            _ = token.cancelled() => {
                info!("SLA breach checker shutting down gracefully");
                break;
            }
        }
    }
}
