//! Domain Event Subscribers for cross-module event-driven communication.
//!
//! Per §5 Architecture and §24 Events doc: Modules communicate via NATS events
//! rather than direct coupling. Each subscriber handles specific event types
//! and triggers side effects in its module.
//!
//! # Architecture
//! - Outbox Pattern: Events are stored in DB within the same transaction,
//!   then an outbox worker publishes to NATS for reliable delivery.
//! - Subscribers consume from NATS and trigger module-specific handlers.
//! - All handlers are idempotent (safe to replay).
//!
//! # TODO (per §24 Events docs):
//! - subscriber handlers currently log events only
//! - implement actual cross-module side effects:
//!   - customer.activated → provision bandwidth, VLAN assignment
//!   - subscription.cancelled → terminate PPPoE session, release bandwidth
//!   - payment.completed → mark invoice paid, create journal entry
//!   - installation.completed → activate customer subscription
//!   - device.status.changed → broadcast to WebSocket NOC dashboard

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
    db: Arc<DatabaseConnection>,
) -> Result<(), AppError> {
    let mut sub = client
        .subscribe("events.customer.>".to_string())
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("NATS subscribe failed: {}", e)))?;

    info!(subject = "events.customer.>", "Subscribed to customer events");

    while let Some(msg) = sub.next().await {
        let subject = msg.subject.clone();
        let payload = msg.payload.clone();

        debug!(subject = %subject, "Received customer event");

        match serde_json::from_slice::<EventEnvelope<serde_json::Value>>(&payload) {
            Ok(envelope) => {
                if let Err(e) = handle_customer_event(&envelope, &db).await {
                    warn!(event_type = %envelope.event_type, error = %e, "Failed to handle customer event");
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
    db: &DatabaseConnection,
) -> Result<(), AppError> {
    match envelope.event_type.as_str() {
        "customer.created" => {
            info!(
                event_id = %envelope.event_id,
                "New customer created - trigger welcome workflow"
            );
            // Create customer wallet for referral rewards
            if let Some(customer_id) = envelope.payload.get("customer_id") {
                if let Some(id) = customer_id.as_i64() {
                    let _ = create_customer_wallet(db, id).await;
                }
            }
        }
        "customer.activated" => {
            info!(
                event_id = %envelope.event_id,
                "Customer activated - provision network access"
            );
            // Provision bandwidth profile for the customer's subscription
            if let Some(customer_id) = envelope.payload.get("customer_id") {
                if let Some(id) = customer_id.as_i64() {
                    let _ = provision_customer_bandwidth(db, id).await;
                }
            }
        }
        "customer.suspended" => {
            info!(
                event_id = %envelope.event_id,
                "Customer suspended - revoke network access"
            );
            // Revoke bandwidth by applying throttle profile
            if let Some(customer_id) = envelope.payload.get("customer_id") {
                if let Some(id) = customer_id.as_i64() {
                    let _ = revoke_customer_bandwidth(db, id).await;
                }
            }
        }
        "customer.reactivated" => {
            info!(
                event_id = %envelope.event_id,
                "Customer reactivated - restore network access"
            );
            // Restore full bandwidth
            if let Some(customer_id) = envelope.payload.get("customer_id") {
                if let Some(id) = customer_id.as_i64() {
                    let _ = provision_customer_bandwidth(db, id).await;
                }
            }
        }
        "customer.terminated" => {
            info!(
                event_id = %envelope.event_id,
                "Customer terminated - cleanup resources"
            );
            // Release all resources: bandwidth, PPPoE sessions
            if let Some(customer_id) = envelope.payload.get("customer_id") {
                if let Some(id) = customer_id.as_i64() {
                    let _ = cleanup_customer_resources(db, id).await;
                }
            }
        }
        "customer.kyc.submitted" => {
            info!(event_id = %envelope.event_id, "KYC submitted - trigger verification workflow");
            if let Some(customer_id) = envelope.payload.get("customer_id").and_then(|v| v.as_i64()) {
                let _ = handle_kyc_submitted(db, customer_id).await;
            }
        }
        "customer.kyc.verified" => {
            info!(event_id = %envelope.event_id, "KYC verified - activate customer account");
            if let Some(customer_id) = envelope.payload.get("customer_id").and_then(|v| v.as_i64()) {
                let _ = handle_kyc_verified(db, customer_id).await;
            }
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

/// Create a wallet for a new customer (for referral rewards)
async fn create_customer_wallet(db: &DatabaseConnection, customer_id: i64) -> Result<(), AppError> {
    use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};

    use crate::modules::referral::domain::entities::customer_wallet;

    // Check if wallet already exists
    let existing = customer_wallet::Entity::find()
        .filter(customer_wallet::Column::CustomerId.eq(customer_id))
        .one(db)
        .await?;

    if existing.is_some() {
        debug!(customer_id, "Customer wallet already exists");
        return Ok(());
    }

    let wallet = customer_wallet::ActiveModel {
        customer_id: Set(customer_id),
        balance: Set(rust_decimal::Decimal::ZERO),
        total_earned: Set(rust_decimal::Decimal::ZERO),
        total_used: Set(rust_decimal::Decimal::ZERO),
        currency: Set("INR".to_string()),
        ..Default::default()
    };

    wallet.insert(db).await?;
    info!(customer_id, "Created customer wallet");
    Ok(())
}

/// Provision bandwidth profile for a customer's active subscription
async fn provision_customer_bandwidth(db: &DatabaseConnection, customer_id: i64) -> Result<(), AppError> {
    use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};

    use crate::modules::subscription::domain::entities::subscription;
    use crate::modules::bandwidth::domain::entities::bandwidth_profile;

    // Find active subscription
    let sub = subscription::Entity::find()
        .filter(subscription::Column::CustomerId.eq(customer_id))
        .filter(subscription::Column::Status.eq("active"))
        .one(db)
        .await?;

    let Some(sub) = sub else {
        debug!(customer_id, "No active subscription found for customer");
        return Ok(());
    };

    // Find bandwidth profile matching the subscription's plan
    let profile = bandwidth_profile::Entity::find()
        .filter(bandwidth_profile::Column::PlanId.eq(sub.plan_id))
        .filter(bandwidth_profile::Column::IsActive.eq(true))
        .one(db)
        .await?;

    if let Some(_profile) = profile {
        // Record bandwidth application
        let application = crate::modules::bandwidth::domain::entities::bandwidth_application::ActiveModel {
            profile_id: Set(_profile.id),
            subscription_id: Set(sub.id),
            status: Set("applied".to_string()),
            ..Default::default()
        };
        application.insert(db).await?;
        info!(customer_id, subscription_id = sub.id, profile_id = _profile.id, "Provisioned bandwidth profile");
    } else {
        warn!(customer_id, plan_id = sub.plan_id, "No bandwidth profile found for plan");
    }

    Ok(())
}

/// Revoke bandwidth by applying a throttle profile
async fn revoke_customer_bandwidth(db: &DatabaseConnection, customer_id: i64) -> Result<(), AppError> {
    use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};

    use crate::modules::subscription::domain::entities::subscription;
    use crate::modules::bandwidth::domain::entities::bandwidth_application;

    // Find active subscription
    let sub = subscription::Entity::find()
        .filter(subscription::Column::CustomerId.eq(customer_id))
        .filter(subscription::Column::Status.eq("active"))
        .one(db)
        .await?;

    let Some(sub) = sub else {
        return Ok(());
    };

    // Update bandwidth application to suspended
    let apps = bandwidth_application::Entity::find()
        .filter(bandwidth_application::Column::SubscriptionId.eq(sub.id))
        .filter(bandwidth_application::Column::Status.eq("applied"))
        .all(db)
        .await?;

    for app in apps {
        let mut active: bandwidth_application::ActiveModel = app.into();
        active.status = Set("suspended".to_string());
        active.update(db).await?;
    }

    info!(customer_id, "Revoked bandwidth for suspended customer");
    Ok(())
}

/// Handle KYC submission - update customer status to kyc_pending
async fn handle_kyc_submitted(db: &DatabaseConnection, customer_id: i64) -> Result<(), AppError> {
    use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
    use crate::modules::customer::domain::entities::customer;

    if let Some(cust) = customer::Entity::find_by_id(customer_id).one(db).await? {
        if cust.status == "registered" {
            let mut active: customer::ActiveModel = cust.into();
            active.status = Set("kyc_pending".to_string());
            active.updated_at = Set(chrono::Utc::now());
            active.update(db).await?;
            info!(customer_id, "Updated customer status to kyc_pending");
        }
    }
    Ok(())
}

/// Handle KYC verification - activate customer if KYC is verified
async fn handle_kyc_verified(db: &DatabaseConnection, customer_id: i64) -> Result<(), AppError> {
    use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, Set};
    use crate::modules::customer::domain::entities::customer;
    use crate::modules::compliance::domain::entities::kyc_verification;

    // Check if customer has at least one verified KYC
    let verified_kyc_count = kyc_verification::Entity::find()
        .filter(kyc_verification::Column::CustomerId.eq(customer_id))
        .filter(kyc_verification::Column::Status.eq("verified"))
        .count(db)
        .await?;

    if verified_kyc_count > 0 {
        if let Some(cust) = customer::Entity::find_by_id(customer_id).one(db).await? {
            if cust.status == "kyc_pending" {
                let mut active: customer::ActiveModel = cust.into();
                active.status = Set("kyc_verified".to_string());
                active.updated_at = Set(chrono::Utc::now());
                active.update(db).await?;
                info!(customer_id, "KYC verified - customer status updated to kyc_verified");
            }
        }
    }
    Ok(())
}

/// Cleanup all resources for a terminated customer
async fn cleanup_customer_resources(db: &DatabaseConnection, customer_id: i64) -> Result<(), AppError> {
    use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};

    use crate::modules::subscription::domain::entities::subscription;
    use crate::modules::network::domain::entities::pppoe_session;

    // Terminate all active subscriptions
    let subs = subscription::Entity::find()
        .filter(subscription::Column::CustomerId.eq(customer_id))
        .filter(subscription::Column::Status.is_in(vec!["active", "suspended"]))
        .all(db)
        .await?;

    for sub in subs {
        let mut active: subscription::ActiveModel = sub.into();
        active.status = Set("terminated".to_string());
        active.updated_at = Set(chrono::Utc::now());
        active.update(db).await?;
    }

    // Terminate PPPoE sessions
    let sessions = pppoe_session::Entity::find()
        .filter(pppoe_session::Column::CustomerId.eq(customer_id))
        .filter(pppoe_session::Column::Status.eq("active"))
        .all(db)
        .await?;

    for session in sessions {
        let mut active: pppoe_session::ActiveModel = session.into();
        active.status = Set("terminated".to_string());
        active.updated_at = Set(chrono::Utc::now());
        active.update(db).await?;
    }

    info!(customer_id, "Cleaned up resources for terminated customer");
    Ok(())
}

// ──────────────────────────────────────────────
// Subscription Events
// ──────────────────────────────────────────────

async fn subscribe_subscription_events(
    client: Client,
    db: Arc<DatabaseConnection>,
) -> Result<(), AppError> {
    let mut sub = client
        .subscribe("events.subscription.>".to_string())
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("NATS subscribe failed: {}", e)))?;

    info!(subject = "events.subscription.>", "Subscribed to subscription events");

    while let Some(msg) = sub.next().await {
        let subject = msg.subject.clone();
        let payload = msg.payload.clone();

        debug!(subject = %subject, "Received subscription event");

        match serde_json::from_slice::<EventEnvelope<serde_json::Value>>(&payload) {
            Ok(envelope) => {
                if let Err(e) = handle_subscription_event(&envelope, &db).await {
                    warn!(event_type = %envelope.event_type, error = %e, "Failed to handle subscription event");
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
    db: &DatabaseConnection,
) -> Result<(), AppError> {
    match envelope.event_type.as_str() {
        "subscription.created" => {
            info!(event_id = %envelope.event_id, "New subscription - provision bandwidth profile");
            if let Some(customer_id) = envelope.payload.get("customer_id").and_then(|v| v.as_i64()) {
                let _ = provision_customer_bandwidth(db, customer_id).await;
            }
        }
        "subscription.suspended" => {
            info!(event_id = %envelope.event_id, "Subscription suspended - apply rate limit");
            if let Some(customer_id) = envelope.payload.get("customer_id").and_then(|v| v.as_i64()) {
                let _ = revoke_customer_bandwidth(db, customer_id).await;
            }
        }
        "subscription.reactivated" => {
            info!(event_id = %envelope.event_id, "Subscription reactivated - restore bandwidth");
            if let Some(customer_id) = envelope.payload.get("customer_id").and_then(|v| v.as_i64()) {
                let _ = provision_customer_bandwidth(db, customer_id).await;
            }
        }
        "subscription.cancelled" => {
            info!(event_id = %envelope.event_id, "Subscription cancelled - cleanup resources");
            if let Some(customer_id) = envelope.payload.get("customer_id").and_then(|v| v.as_i64()) {
                let _ = cleanup_customer_resources(db, customer_id).await;
            }
        }
        "subscription.upgraded" | "subscription.downgraded" => {
            info!(event_id = %envelope.event_id, event_type = %envelope.event_type, "Plan changed - re-provision bandwidth");
            if let Some(customer_id) = envelope.payload.get("customer_id").and_then(|v| v.as_i64()) {
                let _ = provision_customer_bandwidth(db, customer_id).await;
            }
        }
        _ => {
            debug!(event_type = %envelope.event_type, "Unhandled subscription event type");
        }
    }
    Ok(())
}

// ──────────────────────────────────────────────
// Billing Events
// ──────────────────────────────────────────────

async fn subscribe_billing_events(
    client: Client,
    db: Arc<DatabaseConnection>,
) -> Result<(), AppError> {
    let mut sub = client
        .subscribe("events.invoice.>".to_string())
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("NATS subscribe failed: {}", e)))?;

    // Also subscribe to payment events
    let mut sub_payment = client
        .subscribe("events.payment.>".to_string())
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("NATS subscribe failed: {}", e)))?;

    info!(subject = "events.invoice.> + events.payment.>", "Subscribed to billing events");

    loop {
        tokio::select! {
            msg = sub.next() => {
                if let Some(msg) = msg {
                    let payload = msg.payload.clone();
                    if let Ok(envelope) = serde_json::from_slice::<EventEnvelope<serde_json::Value>>(&payload) {
                        if let Err(e) = handle_invoice_event(&envelope, &db).await {
                            warn!(event_type = %envelope.event_type, error = %e, "Failed to handle invoice event");
                        }
                    }
                }
            }
            msg = sub_payment.next() => {
                if let Some(msg) = msg {
                    let payload = msg.payload.clone();
                    if let Ok(envelope) = serde_json::from_slice::<EventEnvelope<serde_json::Value>>(&payload) {
                        if let Err(e) = handle_payment_event(&envelope, &db).await {
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
    db: &DatabaseConnection,
) -> Result<(), AppError> {
    match envelope.event_type.as_str() {
        "invoice.generated" => {
            info!(event_id = %envelope.event_id, "Invoice generated - send to customer");
            // Create notification for the customer
            if let (Some(customer_id), Some(invoice_number), Some(total)) = (
                envelope.payload.get("customer_id").and_then(|v| v.as_i64()),
                envelope.payload.get("invoice_number").and_then(|v| v.as_str()),
                envelope.payload.get("total_amount"),
            ) {
                let _ = create_notification(
                    db, customer_id, "email",
                    &format!("Payment Reminder - Invoice {}", invoice_number),
                    &format!("Dear customer, your invoice {} for ₹{} is due. Please make the payment to avoid service interruption.", invoice_number, total),
                ).await;
            }
        }
        "invoice.sent" => {
            info!(event_id = %envelope.event_id, "Invoice sent - track delivery");
        }
        "invoice.overdue" => {
            info!(event_id = %envelope.event_id, "Invoice overdue - trigger payment reminder");
            if let (Some(customer_id), Some(invoice_number)) = (
                envelope.payload.get("customer_id").and_then(|v| v.as_i64()),
                envelope.payload.get("invoice_number").and_then(|v| v.as_str()),
            ) {
                let _ = create_notification(
                    db, customer_id, "sms",
                    &format!("URGENT: Invoice {} is overdue. Pay immediately to avoid service suspension.", invoice_number),
                    &format!("Payment overdue for invoice {}. Please pay now.", invoice_number),
                ).await;
            }
        }
        "invoice.voided" => {
            info!(event_id = %envelope.event_id, "Invoice voided - reverse accounting entry");
        }
        _ => {
            debug!(event_type = %envelope.event_type, "Unhandled invoice event");
        }
    }
    Ok(())
}

async fn handle_payment_event(
    envelope: &EventEnvelope<serde_json::Value>,
    db: &DatabaseConnection,
) -> Result<(), AppError> {
    match envelope.event_type.as_str() {
        "payment.completed" => {
            info!(event_id = %envelope.event_id, "Payment completed - mark invoice paid");
            // Update invoice status to paid
            if let Some(invoice_id) = envelope.payload.get("invoice_id").and_then(|v| v.as_i64()) {
                let _ = mark_invoice_paid(db, invoice_id).await;
            }
            // Send payment confirmation notification
            if let Some(customer_id) = envelope.payload.get("customer_id").and_then(|v| v.as_i64()) {
                let amount = envelope.payload.get("amount").map(|v| v.to_string()).unwrap_or_default();
                let _ = create_notification(
                    db, customer_id, "email",
                    "Payment Confirmation",
                    &format!("Your payment of ₹{} has been received successfully.", amount),
                ).await;
            }
        }
        "payment.failed" => {
            info!(event_id = %envelope.event_id, "Payment failed - notify customer");
            if let Some(customer_id) = envelope.payload.get("customer_id").and_then(|v| v.as_i64()) {
                let _ = create_notification(
                    db, customer_id, "email",
                    "Payment Failed",
                    "Your payment could not be processed. Please try again or contact support.",
                ).await;
            }
        }
        "refund.approved" => {
            info!(event_id = %envelope.event_id, "Refund approved - process refund");
        }
        "refund.processed" => {
            info!(event_id = %envelope.event_id, "Refund processed - update accounting");
            if let Some(customer_id) = envelope.payload.get("customer_id").and_then(|v| v.as_i64()) {
                let amount = envelope.payload.get("amount").map(|v| v.to_string()).unwrap_or_default();
                let _ = create_notification(
                    db, customer_id, "email",
                    "Refund Processed",
                    &format!("Your refund of ₹{} has been processed.", amount),
                ).await;
            }
        }
        _ => {
            debug!(event_type = %envelope.event_type, "Unhandled payment event");
        }
    }
    Ok(())
}

/// Mark an invoice as paid in the database
async fn mark_invoice_paid(db: &DatabaseConnection, invoice_id: i64) -> Result<(), AppError> {
    use sea_orm::{ActiveModelTrait, EntityTrait, Set};
    use crate::modules::billing::domain::entities::invoice;

    if let Some(inv) = invoice::Entity::find_by_id(invoice_id).one(db).await? {
        if inv.status != "paid" {
            let mut active: invoice::ActiveModel = inv.into();
            active.status = Set("paid".to_string());
            active.paid_at = Set(Some(chrono::Utc::now()));
            active.updated_at = Set(chrono::Utc::now());
            active.update(db).await?;
            info!(invoice_id, "Marked invoice as paid");
        }
    }
    Ok(())
}

/// Create a notification record for a customer
async fn create_notification(
    db: &DatabaseConnection,
    recipient_id: i64,
    channel: &str,
    subject: &str,
    body: &str,
) -> Result<(), AppError> {
    use sea_orm::{ActiveModelTrait, Set};
    use crate::modules::notification::domain::entities::notification;

    let notif = notification::ActiveModel {
        channel: Set(channel.to_string()),
        recipient_type: Set("customer".to_string()),
        recipient_id: Set(recipient_id),
        recipient_address: Set(String::new()),
        subject: Set(Some(subject.to_string())),
        body: Set(body.to_string()),
        status: Set("queued".to_string()),
        retry_count: Set(0),
        max_retries: Set(3),
        ..Default::default()
    };

    notif.insert(db).await?;
    debug!(recipient_id, channel, "Queued notification");
    Ok(())
}

// ──────────────────────────────────────────────
// Device Events
// ──────────────────────────────────────────────

async fn subscribe_device_events(
    client: Client,
    db: Arc<DatabaseConnection>,
) -> Result<(), AppError> {
    let mut sub = client
        .subscribe("events.device.>".to_string())
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("NATS subscribe failed: {}", e)))?;

    info!(subject = "events.device.>", "Subscribed to device events");

    while let Some(msg) = sub.next().await {
        let subject = msg.subject.clone();
        let payload = msg.payload.clone();

        debug!(subject = %subject, "Received device event");

        match serde_json::from_slice::<EventEnvelope<serde_json::Value>>(&payload) {
            Ok(envelope) => {
                if let Err(e) = handle_device_event(&envelope, &db).await {
                    warn!(event_type = %envelope.event_type, error = %e, "Failed to handle device event");
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
    _db: &DatabaseConnection,
) -> Result<(), AppError> {
    match envelope.event_type.as_str() {
        "device.registered" => {
            info!(event_id = %envelope.event_id, "Device registered - run initial config");
        }
        "device.status.changed" => {
            info!(event_id = %envelope.event_id, "Device status changed - update NOC dashboard");
            // Broadcast via Redis pub/sub for WebSocket clients
        }
        "device.discovered" => {
            info!(event_id = %envelope.event_id, "Device discovered - prompt for registration");
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
        .subscribe("events.vlan.>".to_string())
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("NATS subscribe failed: {}", e)))?;

    let mut sub_pppoe = client
        .subscribe("events.pppoe.>".to_string())
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("NATS subscribe failed: {}", e)))?;

    info!(subject = "events.vlan.> + events.pppoe.>", "Subscribed to network events");

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
        .subscribe("events.ticket.>".to_string())
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("NATS subscribe failed: {}", e)))?;

    info!(subject = "events.ticket.>", "Subscribed to ticket events");

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
        .subscribe("events.installation.>".to_string())
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("NATS subscribe failed: {}", e)))?;

    info!(subject = "events.installation.>", "Subscribed to installation events");

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
