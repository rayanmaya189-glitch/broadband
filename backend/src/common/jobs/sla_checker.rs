//! SLA breach checker — auto-escalates tickets past their SLA deadline.
//!
//! Converted from raw sqlx to SeaORM for consistency with the rest of the codebase.

use std::time::Duration;

use sea_orm::*;
use tokio_util::sync::CancellationToken;
use tracing::{info, warn, error};

use crate::app::SharedState;
use crate::modules::ticket::model::ticket_entity::{self, Entity as TicketEntity};
use crate::modules::ticket::model::ticket_escalation_entity::{self};
use crate::modules::ticket::model::ticket_status_history_entity::{self};
use crate::modules::role::model::role_entity::{self, Entity as RoleEntity};
use crate::modules::role::model::user_role_entity::{self, Entity as UserRoleEntity};

const DEFAULT_CHECK_INTERVAL_SECS: u64 = 300;

fn escalation_target(priority: &str) -> Option<&'static str> {
    match priority {
        "low" => Some("medium"),
        "medium" => Some("high"),
        "high" => Some("critical"),
        "critical" => None,
        _ => None,
    }
}

/// Find tickets whose SLA resolution deadline has passed.
async fn find_sla_breached_tickets(
    db: &DatabaseConnection,
) -> Result<Vec<ticket_entity::Model>, DbErr> {
    let now = chrono::Utc::now();
    let statuses = vec!["open".to_string(), "assigned".to_string(), "in_progress".to_string()];

    TicketEntity::find()
        .filter(ticket_entity::Column::Status.is_in(statuses))
        .filter(ticket_entity::Column::SlaResolutionAt.is_not_null())
        .filter(ticket_entity::Column::SlaResolutionAt.lt(now))
        .order_by_asc(ticket_entity::Column::SlaResolutionAt)
        .limit(100)
        .all(db)
        .await
}

/// Find the best escalation target: a user with noc_engineer/network_admin/admin/super_admin role
/// who has the fewest open tickets.
async fn find_escalation_target_user(
    db: &DatabaseConnection,
    assigned_to: Option<i64>,
) -> i64 {
    let eligible_roles = vec![
        "noc_engineer".to_string(),
        "network_admin".to_string(),
        "admin".to_string(),
        "super_admin".to_string(),
    ];

    let eligible_role_ids: Vec<i64> = match RoleEntity::find()
        .filter(role_entity::Column::Name.is_in(eligible_roles))
        .all(db)
        .await
    {
        Ok(roles) => roles.into_iter().map(|r| r.id).collect(),
        Err(_) => return 1,
    };

    if eligible_role_ids.is_empty() {
        return 1;
    }

    let mut user_select = UserRoleEntity::find()
        .filter(user_role_entity::Column::RoleId.is_in(eligible_role_ids.clone()))
        .filter(user_role_entity::Column::IsActive.eq(true));

    if let Some(uid) = assigned_to {
        user_select = user_select.filter(user_role_entity::Column::UserId.ne(uid));
    }

    let user_roles = match user_select.all(db).await {
        Ok(ur) => ur,
        Err(_) => return 1,
    };

    let candidate_ids: Vec<i64> = user_roles.into_iter().map(|ur| ur.user_id).collect();

    if candidate_ids.is_empty() {
        return 1;
    }

    let mut best_id = candidate_ids[0];
    let mut best_count = i64::MAX;

    for user_id in &candidate_ids {
        let count = TicketEntity::find()
            .filter(ticket_entity::Column::AssignedTo.eq(*user_id))
            .filter(ticket_entity::Column::Status.is_not_in(vec![
                "resolved".to_string(),
                "closed".to_string(),
            ]))
            .count(db)
            .await
            .unwrap_or(0) as i64;

        if count < best_count {
            best_count = count;
            best_id = *user_id;
        }
    }

    best_id
}

/// Escalate a single ticket: update ticket, create escalation record, add status history.
async fn auto_escalate_ticket(
    db: &DatabaseConnection,
    ticket: &ticket_entity::Model,
) -> Result<(), DbErr> {
    let escalate_to = find_escalation_target_user(db, ticket.assigned_to).await;
    let new_priority = escalation_target(&ticket.priority);

    let now = chrono::Utc::now();
    let mut ticket_active: ticket_entity::ActiveModel = ticket.clone().into();
    ticket_active.escalated_to = Set(Some(escalate_to));
    if let Some(p) = new_priority {
        ticket_active.priority = Set(p.to_string());
    }
    ticket_active.status = Set("escalated".to_string());
    ticket_active.updated_at = Set(now.into());
    ticket_active.update(db).await?;

    let escalation_active = ticket_escalation_entity::ActiveModel {
        ticket_id: Set(ticket.id),
        from_user_id: Set(ticket.assigned_to.unwrap_or(1)),
        to_user_id: Set(escalate_to),
        from_priority: Set(Some(ticket.priority.clone())),
        to_priority: Set(new_priority.map(|s| s.to_string())),
        reason: Set(format!(
            "Auto-escalated: SLA breach at {}",
            ticket.sla_resolution_at
                .map(|t| t.to_rfc3339())
                .unwrap_or_default()
        )),
        escalated_at: Set(now.into()),
        ..Default::default()
    };
    escalation_active.insert(db).await?;

    let history_active = ticket_status_history_entity::ActiveModel {
        ticket_id: Set(ticket.id),
        old_status: Set(Some(ticket.status.clone())),
        new_status: Set("escalated".to_string()),
        changed_by: Set(1),
        reason: Set(Some(format!(
            "Auto-escalated due to SLA breach (deadline: {})",
            ticket.sla_resolution_at
                .map(|t| t.to_rfc3339())
                .unwrap_or_default()
        ))),
        ..Default::default()
    };
    history_active.insert(db).await?;

    info!(
        ticket_id = ticket.id,
        ticket_number = %ticket.ticket_number,
        "Ticket auto-escalated due to SLA breach"
    );
    Ok(())
}

pub async fn run_sla_checker(state: SharedState, token: CancellationToken) {
    let interval_secs = std::env::var("SLA_CHECK_INTERVAL_SECS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(DEFAULT_CHECK_INTERVAL_SECS);
    let mut interval = tokio::time::interval(Duration::from_secs(interval_secs));
    info!(interval_secs = interval_secs, "SLA breach checker background job started");

    loop {
        tokio::select! {
            _ = interval.tick() => {
                // Set RLS bypass for background job queries
                if let Err(e) = super::set_rls_bypass(&state.db).await {
                    warn!(error = %e, "Failed to set RLS bypass context");
                    continue;
                }

                match find_sla_breached_tickets(&state.db).await {
                    Ok(tickets) if tickets.is_empty() => {}
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
                                    warn!(ticket_id = ticket.id, error = %e, "Failed to auto-escalate ticket");
                                }
                            }
                        }
                        info!(total = count, escalated = escalated, failed = failed, "SLA checker batch complete");
                    }
                    Err(e) => error!(error = %e, "Failed to query SLA-breached tickets"),
                }
            }
            _ = token.cancelled() => {
                info!("SLA breach checker shutting down gracefully");
                break;
            }
        }
    }
}
