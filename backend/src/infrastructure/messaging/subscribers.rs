/// Domain Event Subscribers for cross-module event-driven communication.
///
/// Per §5 Architecture and §24 Events doc: Modules communicate via NATS events
/// rather than direct coupling. Each subscriber handles specific event types
/// and triggers side effects in its module.
///
/// # Architecture
/// - Outbox Pattern: Events are stored in DB within the same transaction,
///   then an outbox worker publishes to NATS for reliable delivery.
/// - Subscribers consume from NATS and trigger module-specific handlers.
/// - All handlers are idempotent (safe to replay).
///
/// # TODO (per §24 Events docs):
/// - subscriber handlers currently log events only
/// - implement actual cross-module side effects:
///   - customer.activated → provision bandwidth, VLAN assignment
///   - subscription.cancelled → terminate PPPoE session, release bandwidth
///   - payment.completed → mark invoice paid, create journal entry
///   - installation.completed → activate customer subscription
///   - device.status.changed → broadcast to WebSocket NOC dashboard

use async_nats::Client;
use futures::StreamExt;
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use tracing::{debug, error, info, warn};

use crate::infrastructure::messaging::EventEnvelope;
use crate::shared::errors::AppError;

/// Subscribe to all domain events and route to appropriate handlers.
pub async fn start_subscribers(
    client: Client,
    db: Arc<DatabaseConnection>,
) -> Result<(), AppError> {
    info!("Starting NATS event subscribers");

    // Subscribe to customer events
    let client_c = client.clone();
    let db_c = db.clone();
    tokio::spawn(async move {
        if let Err(e) = subscribe_customer_events(client_c, db_c).await {
            error!(error = %e, "Customer event subscriber failed");
        }
    });

    // Subscribe to subscription events
    let client_c = client.clone();
    let db_c = db.clone();
    tokio::spawn(async move {
        if let Err(e) = subscribe_subscription_events(client_c, db_c).await {
            error!(error = %e, "Subscription event subscriber failed");
        }
    });

    // Subscribe to billing events
    let client_c = client.clone();
    let db_c = db.clone();
    tokio::spawn(async move {
        if let Err(e) = subscribe_billing_events(client_c, db_c).await {
            error!(error = %e, "Billing event subscriber failed");
        }
    });

    // Subscribe to device events
    let client_c = client.clone();
    let db_c = db.clone();
    tokio::spawn(async move {
        if let Err(e) = subscribe_device_events(client_c, db_c).await {
            error!(error = %e, "Device event subscriber failed");
        }
    });

    // Subscribe to network events
    let client_c = client.clone();
    let db_c = db.clone();
    tokio::spawn(async move {
        if let Err(e) = subscribe_network_events(client_c, db_c).await {
            error!(error = %e, "Network event subscriber failed");
        }
    });

    // Subscribe to ticket events
    let client_c = client.clone();
    let db_c = db.clone();
    tokio::spawn(async move {
        if let Err(e) = subscribe_ticket_events(client_c, db_c).await {
            error!(error = %e, "Ticket event subscriber failed");
        }
    });

    // Subscribe to installation events
    let client_c = client.clone();
    let db_c = db.clone();
    tokio::spawn(async move {
        if let Err(e) = subscribe_installation_events(client_c, db_c).await {
            error!(error = %e, "Installation event subscriber failed");
        }
    });

    info!("All NATS event subscribers started");
    Ok(())
}

// ──────────────────────────────────────────────
// Customer Events
// ──────────────────────────────────────────────

async fn subscribe_customer_events(
    client: Client,
    _db: Arc<DatabaseConnection>,
) -> Result<(), AppError> {
    let mut sub = client
        .subscribe("aeroxe.customer.>".to_string())
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("NATS subscribe failed: {}", e)))?;

    info!(subject = "aeroxe.customer.>", "Subscribed to customer events");

    while let Some(msg) = sub.next().await {
        let subject = msg.subject.clone();
        let payload = msg.payload.clone();

        debug!(subject = %subject, "Received customer event");

        match serde_json::from_slice::<EventEnvelope<serde_json::Value>>(&payload) {
            Ok(envelope) => {
                if let Err(e) = handle_customer_event(&envelope).await {
                    warn!(
                        event_type = %envelope.event_type,
                        error = %e,
                        "Failed to handle customer event"
                    );
                }
            }
            Err(e) => {
                warn!(error = %e, "Failed to deserialize customer event");
            }
        }
    }

    Ok(())
}

async fn handle_customer_event(
    envelope: &EventEnvelope<serde_json::Value>,
) -> Result<(), AppError> {
    match envelope.event_type.as_str() {
        "customer.created" => {
            info!(
                event_id = %envelope.event_id,
                "New customer created - trigger welcome workflow"
            );
            // TODO: Trigger welcome notification, create default subscription
        }
        "customer.activated" => {
            info!(
                event_id = %envelope.event_id,
                "Customer activated - provision network access"
            );
            // TODO: Trigger bandwidth provisioning, VLAN assignment
        }
        "customer.suspended" => {
            info!(
                event_id = %envelope.event_id,
                "Customer suspended - revoke network access"
            );
            // TODO: Trigger bandwidth deprovisioning, notify customer
        }
        "customer.reactivated" => {
            info!(
                event_id = %envelope.event_id,
                "Customer reactivated - restore network access"
            );
            // TODO: Restore bandwidth, send reactivation notification
        }
        "customer.terminated" => {
            info!(
                event_id = %envelope.event_id,
                "Customer terminated - cleanup resources"
            );
            // TODO: Release IP pool, VLAN, cancel pending jobs
        }
        "customer.kyc.submitted" => {
            info!(
                event_id = %envelope.event_id,
                "KYC submitted - trigger verification workflow"
            );
        }
        "customer.kyc.verified" => {
            info!(
                event_id = %envelope.event_id,
                "KYC verified - activate customer account"
            );
        }
        _ => {
            debug!(
                event_type = %envelope.event_type,
                "Unhandled customer event type"
            );
        }
    }
    Ok(())
}

// ──────────────────────────────────────────────
// Subscription Events
// ──────────────────────────────────────────────

async fn subscribe_subscription_events(
    client: Client,
    _db: Arc<DatabaseConnection>,
) -> Result<(), AppError> {
    let mut sub = client
        .subscribe("aeroxe.subscription.>".to_string())
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("NATS subscribe failed: {}", e)))?;

    info!(subject = "aeroxe.subscription.>", "Subscribed to subscription events");

    while let Some(msg) = sub.next().await {
        let subject = msg.subject.clone();
        let payload = msg.payload.clone();

        debug!(subject = %subject, "Received subscription event");

        match serde_json::from_slice::<EventEnvelope<serde_json::Value>>(&payload) {
            Ok(envelope) => {
                if let Err(e) = handle_subscription_event(&envelope).await {
                    warn!(
                        event_type = %envelope.event_type,
                        error = %e,
                        "Failed to handle subscription event"
                    );
                }
            }
            Err(e) => {
                warn!(error = %e, "Failed to deserialize subscription event");
            }
        }
    }

    Ok(())
}

async fn handle_subscription_event(
    envelope: &EventEnvelope<serde_json::Value>,
) -> Result<(), AppError> {
    match envelope.event_type.as_str() {
        "subscription.created" => {
            info!(
                event_id = %envelope.event_id,
                "New subscription - provision bandwidth profile"
            );
            // TODO: Apply bandwidth profile to customer's PPPoE session
        }
        "subscription.suspended" => {
            info!(
                event_id = %envelope.event_id,
                "Subscription suspended - apply rate limit"
            );
            // TODO: Apply bandwidth throttle profile
        }
        "subscription.reactivated" => {
            info!(
                event_id = %envelope.event_id,
                "Subscription reactivated - restore bandwidth"
            );
            // TODO: Restore full bandwidth profile
        }
        "subscription.cancelled" => {
            info!(
                event_id = %envelope.event_id,
                "Subscription cancelled - cleanup resources"
            );
            // TODO: Release bandwidth profile, terminate PPPoE session
        }
        "subscription.upgraded" => {
            info!(
                event_id = %envelope.event_id,
                "Subscription upgraded - apply new bandwidth profile"
            );
            // TODO: Apply upgraded bandwidth profile
        }
        "subscription.downgraded" => {
            info!(
                event_id = %envelope.event_id,
                "Subscription downgraded - apply reduced bandwidth"
            );
            // TODO: Apply downgraded bandwidth profile
        }
        _ => {
            debug!(
                event_type = %envelope.event_type,
                "Unhandled subscription event type"
            );
        }
    }
    Ok(())
}

// ──────────────────────────────────────────────
// Billing Events
// ──────────────────────────────────────────────

async fn subscribe_billing_events(
    client: Client,
    _db: Arc<DatabaseConnection>,
) -> Result<(), AppError> {
    let mut sub = client
        .subscribe("aeroxe.invoice.>".to_string())
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("NATS subscribe failed: {}", e)))?;

    // Also subscribe to payment events
    let mut sub_payment = client
        .subscribe("aeroxe.payment.>".to_string())
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("NATS subscribe failed: {}", e)))?;

    info!(subject = "aeroxe.invoice.> + aeroxe.payment.>", "Subscribed to billing events");

    loop {
        tokio::select! {
            msg = sub.next() => {
                if let Some(msg) = msg {
                    let payload = msg.payload.clone();
                    if let Ok(envelope) = serde_json::from_slice::<EventEnvelope<serde_json::Value>>(&payload) {
                        if let Err(e) = handle_invoice_event(&envelope).await {
                            warn!(event_type = %envelope.event_type, error = %e, "Failed to handle invoice event");
                        }
                    }
                }
            }
            msg = sub_payment.next() => {
                if let Some(msg) = msg {
                    let payload = msg.payload.clone();
                    if let Ok(envelope) = serde_json::from_slice::<EventEnvelope<serde_json::Value>>(&payload) {
                        if let Err(e) = handle_payment_event(&envelope).await {
                            warn!(event_type = %envelope.event_type, error = %e, "Failed to handle payment event");
                        }
                    }
                }
            }
        }
    }
}

async fn handle_invoice_event(
    envelope: &EventEnvelope<serde_json::Value>,
) -> Result<(), AppError> {
    match envelope.event_type.as_str() {
        "invoice.generated" => {
            info!(event_id = %envelope.event_id, "Invoice generated - send to customer");
            // TODO: Trigger notification worker to send invoice email
        }
        "invoice.sent" => {
            info!(event_id = %envelope.event_id, "Invoice sent - track delivery");
        }
        "invoice.overdue" => {
            info!(event_id = %envelope.event_id, "Invoice overdue - trigger payment reminder");
            // TODO: Schedule payment reminder notifications
        }
        "invoice.voided" => {
            info!(event_id = %envelope.event_id, "Invoice voided - update accounting");
            // TODO: Reverse journal entry
        }
        _ => {
            debug!(event_type = %envelope.event_type, "Unhandled invoice event");
        }
    }
    Ok(())
}

async fn handle_payment_event(
    envelope: &EventEnvelope<serde_json::Value>,
) -> Result<(), AppError> {
    match envelope.event_type.as_str() {
        "payment.completed" => {
            info!(event_id = %envelope.event_id, "Payment completed - update invoice status");
            // TODO: Mark invoice as paid, create journal entry, activate subscription
        }
        "payment.failed" => {
            info!(event_id = %envelope.event_id, "Payment failed - notify customer");
            // TODO: Send payment failure notification
        }
        "refund.approved" => {
            info!(event_id = %envelope.event_id, "Refund approved - process refund");
        }
        "refund.processed" => {
            info!(event_id = %envelope.event_id, "Refund processed - update accounting");
        }
        _ => {
            debug!(event_type = %envelope.event_type, "Unhandled payment event");
        }
    }
    Ok(())
}

// ──────────────────────────────────────────────
// Device Events
// ──────────────────────────────────────────────

async fn subscribe_device_events(
    client: Client,
    _db: Arc<DatabaseConnection>,
) -> Result<(), AppError> {
    let mut sub = client
        .subscribe("aeroxe.device.>".to_string())
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("NATS subscribe failed: {}", e)))?;

    info!(subject = "aeroxe.device.>", "Subscribed to device events");

    while let Some(msg) = sub.next().await {
        let subject = msg.subject.clone();
        let payload = msg.payload.clone();

        debug!(subject = %subject, "Received device event");

        match serde_json::from_slice::<EventEnvelope<serde_json::Value>>(&payload) {
            Ok(envelope) => {
                if let Err(e) = handle_device_event(&envelope).await {
                    warn!(
                        event_type = %envelope.event_type,
                        error = %e,
                        "Failed to handle device event"
                    );
                }
            }
            Err(e) => {
                warn!(error = %e, "Failed to deserialize device event");
            }
        }
    }

    Ok(())
}

async fn handle_device_event(
    envelope: &EventEnvelope<serde_json::Value>,
) -> Result<(), AppError> {
    match envelope.event_type.as_str() {
        "device.registered" => {
            info!(event_id = %envelope.event_id, "Device registered - run initial config");
            // TODO: Push initial configuration to device
        }
        "device.status.changed" => {
            info!(event_id = %envelope.event_id, "Device status changed - update NOC dashboard");
            // TODO: Broadcast to WebSocket for real-time NOC updates
        }
        "device.discovered" => {
            info!(event_id = %envelope.event_id, "Device discovered - prompt for registration");
            // TODO: Create discovery approval request
        }
        _ => {
            debug!(event_type = %envelope.event_type, "Unhandled device event");
        }
    }
    Ok(())
}

// ──────────────────────────────────────────────
// Network Events
// ──────────────────────────────────────────────

async fn subscribe_network_events(
    client: Client,
    _db: Arc<DatabaseConnection>,
) -> Result<(), AppError> {
    let mut sub = client
        .subscribe("aeroxe.vlan.>".to_string())
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("NATS subscribe failed: {}", e)))?;

    let mut sub_pppoe = client
        .subscribe("aeroxe.pppoe.>".to_string())
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("NATS subscribe failed: {}", e)))?;

    info!(subject = "aeroxe.vlan.> + aeroxe.pppoe.>", "Subscribed to network events");

    loop {
        tokio::select! {
            msg = sub.next() => {
                if let Some(msg) = msg {
                    if let Ok(envelope) = serde_json::from_slice::<EventEnvelope<serde_json::Value>>(&msg.payload) {
                        info!(event_type = %envelope.event_type, "VLAN event received");
                    }
                }
            }
            msg = sub_pppoe.next() => {
                if let Some(msg) = msg {
                    if let Ok(envelope) = serde_json::from_slice::<EventEnvelope<serde_json::Value>>(&msg.payload) {
                        info!(event_type = %envelope.event_type, "PPPoE event received");
                    }
                }
            }
        }
    }
}

// ──────────────────────────────────────────────
// Ticket Events
// ──────────────────────────────────────────────

async fn subscribe_ticket_events(
    client: Client,
    _db: Arc<DatabaseConnection>,
) -> Result<(), AppError> {
    let mut sub = client
        .subscribe("aeroxe.ticket.>".to_string())
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("NATS subscribe failed: {}", e)))?;

    info!(subject = "aeroxe.ticket.>", "Subscribed to ticket events");

    while let Some(msg) = sub.next().await {
        let payload = msg.payload.clone();
        if let Ok(envelope) = serde_json::from_slice::<EventEnvelope<serde_json::Value>>(&payload) {
            match envelope.event_type.as_str() {
                "ticket.created" => {
                    info!(event_id = %envelope.event_id, "Ticket created - notify assigned team");
                }
                "ticket.escalated" => {
                    info!(event_id = %envelope.event_id, "Ticket escalated - alert management");
                }
                "ticket.resolved" => {
                    info!(event_id = %envelope.event_id, "Ticket resolved - send satisfaction survey");
                }
                _ => {
                    debug!(event_type = %envelope.event_type, "Unhandled ticket event");
                }
            }
        }
    }

    Ok(())
}

// ──────────────────────────────────────────────
// Installation Events
// ──────────────────────────────────────────────

async fn subscribe_installation_events(
    client: Client,
    _db: Arc<DatabaseConnection>,
) -> Result<(), AppError> {
    let mut sub = client
        .subscribe("aeroxe.installation.>".to_string())
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("NATS subscribe failed: {}", e)))?;

    info!(subject = "aeroxe.installation.>", "Subscribed to installation events");

    while let Some(msg) = sub.next().await {
        let payload = msg.payload.clone();
        if let Ok(envelope) = serde_json::from_slice::<EventEnvelope<serde_json::Value>>(&payload) {
            match envelope.event_type.as_str() {
                "installation.scheduled" => {
                    info!(event_id = %envelope.event_id, "Installation scheduled - notify customer");
                }
                "installation.completed" => {
                    info!(event_id = %envelope.event_id, "Installation completed - activate subscription");
                    // TODO: Auto-activate customer subscription after installation
                }
                _ => {
                    debug!(event_type = %envelope.event_type, "Unhandled installation event");
                }
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_envelope_deserialization() {
        let json = serde_json::json!({
            "event_id": "test-123",
            "event_type": "customer.created",
            "version": 1,
            "occurred_at": "2026-07-16T12:00:00Z",
            "producer": "test",
            "payload": {
                "customer_id": 1,
                "name": "Test Customer"
            }
        });

        let envelope: EventEnvelope<serde_json::Value> = serde_json::from_value(json).unwrap();
        assert_eq!(envelope.event_type, "customer.created");
        assert_eq!(envelope.version, 1);
        assert_eq!(envelope.producer, "test");
    }
}
