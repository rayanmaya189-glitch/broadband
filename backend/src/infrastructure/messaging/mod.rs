pub mod nats_client;
pub mod outbox;
pub mod outbox_entity;
pub mod subscribers;

use async_nats::Client;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Event envelope for all domain events.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventEnvelope<T> {
    pub event_id: String,
    pub event_type: String,
    pub version: u32,
    pub occurred_at: DateTime<Utc>,
    pub producer: String,
    pub payload: T,
}

impl<T> EventEnvelope<T> {
    pub fn new(event_type: String, producer: String, payload: T) -> Self {
        Self {
            event_id: Uuid::new_v4().to_string(),
            event_type,
            version: 1,
            occurred_at: Utc::now(),
            producer,
            payload,
        }
    }
}

/// NATS subject naming convention: aeroxe.<context>.<entity>.<action>.<version>
pub struct NatsSubjects;

impl NatsSubjects {
    // Customer events
    pub fn customer_created() -> String {
        "aeroxe.customer.created.v1".to_string()
    }
    pub fn customer_activated() -> String {
        "aeroxe.customer.activated.v1".to_string()
    }
    pub fn customer_suspended() -> String {
        "aeroxe.customer.suspended.v1".to_string()
    }
    pub fn customer_reactivated() -> String {
        "aeroxe.customer.reactivated.v1".to_string()
    }
    pub fn customer_terminated() -> String {
        "aeroxe.customer.terminated.v1".to_string()
    }
    pub fn customer_kyc_submitted() -> String {
        "aeroxe.customer.kyc.submitted.v1".to_string()
    }
    pub fn customer_kyc_verified() -> String {
        "aeroxe.customer.kyc.verified.v1".to_string()
    }

    // Subscription events
    pub fn subscription_created() -> String {
        "aeroxe.subscription.created.v1".to_string()
    }
    pub fn subscription_renewed() -> String {
        "aeroxe.subscription.renewed.v1".to_string()
    }
    pub fn subscription_suspended() -> String {
        "aeroxe.subscription.suspended.v1".to_string()
    }
    pub fn subscription_reactivated() -> String {
        "aeroxe.subscription.reactivated.v1".to_string()
    }
    pub fn subscription_cancelled() -> String {
        "aeroxe.subscription.cancelled.v1".to_string()
    }
    pub fn subscription_upgraded() -> String {
        "aeroxe.subscription.upgraded.v1".to_string()
    }
    pub fn subscription_downgraded() -> String {
        "aeroxe.subscription.downgraded.v1".to_string()
    }

    // Billing events
    pub fn invoice_generated() -> String {
        "aeroxe.invoice.generated.v1".to_string()
    }
    pub fn invoice_sent() -> String {
        "aeroxe.invoice.sent.v1".to_string()
    }
    pub fn invoice_paid() -> String {
        "aeroxe.invoice.paid.v1".to_string()
    }
    pub fn invoice_overdue() -> String {
        "aeroxe.invoice.overdue.v1".to_string()
    }
    pub fn invoice_voided() -> String {
        "aeroxe.invoice.voided.v1".to_string()
    }
    pub fn payment_completed() -> String {
        "aeroxe.payment.completed.v1".to_string()
    }
    pub fn payment_failed() -> String {
        "aeroxe.payment.failed.v1".to_string()
    }
    pub fn refund_approved() -> String {
        "aeroxe.refund.approved.v1".to_string()
    }
    pub fn refund_processed() -> String {
        "aeroxe.refund.processed.v1".to_string()
    }

    // Device events
    pub fn device_registered() -> String {
        "aeroxe.device.registered.v1".to_string()
    }
    pub fn device_status_changed() -> String {
        "aeroxe.device.status.changed.v1".to_string()
    }
    pub fn device_discovered() -> String {
        "aeroxe.device.discovered.v1".to_string()
    }

    // Network events
    pub fn vlan_created() -> String {
        "aeroxe.vlan.created.v1".to_string()
    }
    pub fn vlan_deleted() -> String {
        "aeroxe.vlan.deleted.v1".to_string()
    }
    pub fn pppoe_session_started() -> String {
        "aeroxe.pppoe.session.started.v1".to_string()
    }
    pub fn pppoe_session_ended() -> String {
        "aeroxe.pppoe.session.ended.v1".to_string()
    }

    // Ticket events
    pub fn ticket_created() -> String {
        "aeroxe.ticket.created.v1".to_string()
    }
    pub fn ticket_escalated() -> String {
        "aeroxe.ticket.escalated.v1".to_string()
    }
    pub fn ticket_resolved() -> String {
        "aeroxe.ticket.resolved.v1".to_string()
    }

    // Bandwidth events
    pub fn bandwidth_profile_updated() -> String {
        "aeroxe.bandwidth.profile.updated.v1".to_string()
    }
    pub fn bandwidth_profile_applied() -> String {
        "aeroxe.bandwidth.profile.applied.v1".to_string()
    }

    // Plan events
    pub fn plan_created() -> String {
        "aeroxe.plan.created.v1".to_string()
    }
    pub fn plan_updated() -> String {
        "aeroxe.plan.updated.v1".to_string()
    }
    pub fn plan_published() -> String {
        "aeroxe.plan.published.v1".to_string()
    }

    // Lead events
    pub fn lead_created() -> String {
        "aeroxe.lead.created.v1".to_string()
    }
    pub fn lead_converted() -> String {
        "aeroxe.lead.converted.v1".to_string()
    }

    // Referral events
    pub fn referral_created() -> String {
        "aeroxe.referral.created.v1".to_string()
    }
    pub fn referral_rewarded() -> String {
        "aeroxe.referral.rewarded.v1".to_string()
    }

    // Installation events
    pub fn installation_scheduled() -> String {
        "aeroxe.installation.scheduled.v1".to_string()
    }
    pub fn installation_completed() -> String {
        "aeroxe.installation.completed.v1".to_string()
    }

    // Audit events
    pub fn audit_action() -> String {
        "aeroxe.audit.action.v1".to_string()
    }

    // SLA events
    pub fn sla_breach_warning() -> String {
        "aeroxe.sla.breach.warning.v1".to_string()
    }
}

/// Event publisher using NATS.
pub struct EventPublisher {
    nats: Client,
}

impl EventPublisher {
    pub fn new(nats: Client) -> Self {
        Self { nats }
    }

    /// Publish a domain event to NATS with envelope.
    pub async fn publish<T: Serialize>(
        &self,
        subject: &str,
        event_type: &str,
        producer: &str,
        payload: T,
    ) -> anyhow::Result<()> {
        let envelope = EventEnvelope::new(event_type.to_string(), producer.to_string(), payload);

        let json = serde_json::to_vec(&envelope)?;
        self.nats.publish(subject.to_string(), json.into()).await?;
        tracing::info!(event_type = %event_type, subject = %subject, "Published event to NATS");
        Ok(())
    }

    /// Publish raw JSON payload to NATS (for outbox worker).
    pub async fn publish_raw(
        &self,
        subject: &str,
        event_type: &str,
        payload: &serde_json::Value,
    ) -> anyhow::Result<()> {
        let envelope = serde_json::json!({
            "event_id": Uuid::new_v4().to_string(),
            "event_type": event_type,
            "version": 1,
            "occurred_at": Utc::now().to_rfc3339(),
            "producer": "outbox-worker",
            "payload": payload,
        });

        let json = serde_json::to_vec(&envelope)?;
        self.nats.publish(subject.to_string(), json.into()).await?;
        Ok(())
    }
}
