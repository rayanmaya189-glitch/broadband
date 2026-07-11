use crate::modules::ticket::model::ticket_entity;
use crate::modules::ticket::model::ticket_comment_entity;
use crate::modules::ticket::response::ticket_response::{TicketResponse, TicketCommentResponse};

pub fn ticket_to_response(ticket: &ticket_entity::Model) -> TicketResponse {
    TicketResponse {
        id: ticket.id,
        ticket_number: ticket.ticket_number.clone(),
        branch_id: ticket.branch_id,
        customer_id: ticket.customer_id,
        subscription_id: ticket.subscription_id,
        created_by: ticket.created_by,
        assigned_to: ticket.assigned_to,
        escalated_to: ticket.escalated_to,
        category: ticket.category.clone(),
        subcategory: ticket.subcategory.clone(),
        priority: ticket.priority.clone(),
        status: ticket.status.clone(),
        subject: ticket.subject.clone(),
        description: ticket.description.clone(),
        source: ticket.source.clone(),
        resolution_notes: ticket.resolution_notes.clone(),
        sla_response_at: ticket.sla_response_at.map(|v| v.into()),
        sla_resolution_at: ticket.sla_resolution_at.map(|v| v.into()),
        first_response_at: ticket.first_response_at.map(|v| v.into()),
        resolved_at: ticket.resolved_at.map(|v| v.into()),
        closed_at: ticket.closed_at.map(|v| v.into()),
        reopen_count: ticket.reopen_count,
        satisfaction_rating: ticket.satisfaction_rating,
        satisfaction_feedback: ticket.satisfaction_feedback.clone(),
        created_at: ticket.created_at.into(),
        updated_at: ticket.updated_at.into(),
        creator_name: None,
        assignee_name: None,
        branch_name: None,
        customer_name: None,
    }
}

pub fn comment_to_response(comment: &ticket_comment_entity::Model) -> TicketCommentResponse {
    TicketCommentResponse {
        id: comment.id,
        ticket_id: comment.ticket_id,
        user_id: comment.user_id,
        is_customer: comment.is_customer,
        comment: comment.comment.clone(),
        is_internal: comment.is_internal,
        attachments: comment.attachments.as_ref().map(|j| j.clone()),
        created_at: comment.created_at.into(),
        user_name: None,
    }
}
