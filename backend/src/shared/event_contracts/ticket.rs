use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TicketCreatedV1 {
    pub ticket_id: i64,
    pub ticket_number: String,
    pub branch_id: i64,
    pub category: String,
    pub priority: String,
    pub subject: String,
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TicketAssignedV1 {
    pub ticket_id: i64,
    pub assigned_to: i64,
    pub assigned_by: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TicketEscalatedV1 {
    pub ticket_id: i64,
    pub from_user_id: i64,
    pub to_user_id: i64,
    pub reason: String,
    pub new_priority: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TicketResolvedV1 {
    pub ticket_id: i64,
    pub resolved_by: i64,
    pub resolution_notes: Option<String>,
    pub resolution_time_minutes: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlaBreachWarningV1 {
    pub ticket_id: i64,
    pub breach_type: String,
    pub sla_at: String,
    pub current_time: String,
}
