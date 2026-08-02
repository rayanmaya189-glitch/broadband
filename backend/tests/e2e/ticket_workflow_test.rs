//! End-to-end test: Support Ticket Workflow
//! Tests: Create ticket → Assign → Escalate → Resolve → Close

use crate::common::{TestDatabase, TestFixture};
use sea_orm::{ActiveModelTrait, Set};

/// Test full ticket lifecycle
#[ignore]
#[tokio::test]
async fn test_ticket_lifecycle() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    let branch_id = TestFixture::create_branch(db).await;
    let customer_id = TestFixture::create_customer(db, branch_id).await;
    let created_by = TestFixture::create_user(db, branch_id).await;
    let assigned_to = TestFixture::create_user(db, branch_id).await;

    use aeroxe_backend::modules::ticket::domain::entities::ticket;

    let now = chrono::Utc::now();

    // Create ticket
    let tkt = ticket::ActiveModel {
        ticket_number: Set(format!("TKT-{:06}", rand::random::<u32>() % 1_000_000)),
        customer_id: Set(Some(customer_id)),
        branch_id: Set(branch_id),
        created_by: Set(created_by),
        subject: Set("Internet not working".to_string()),
        description: Set("Customer reports no connectivity since morning".to_string()),
        category: Set("connectivity".to_string()),
        priority: Set("high".to_string()),
        status: Set("open".to_string()),
        source: Set("system".to_string()),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    };
    let tkt = tkt.insert(db).await.unwrap();
    assert_eq!(tkt.status, "open");
    assert_eq!(tkt.priority, "high");

    // Assign ticket
    let mut active: ticket::ActiveModel = tkt.into();
    active.status = Set("in_progress".to_string());
    active.assigned_to = Set(Some(assigned_to));
    active.updated_at = Set(chrono::Utc::now());
    let tkt = active.update(db).await.unwrap();
    assert_eq!(tkt.status, "in_progress");

    // Escalate
    let mut active: ticket::ActiveModel = tkt.into();
    active.priority = Set("critical".to_string());
    active.updated_at = Set(chrono::Utc::now());
    let tkt = active.update(db).await.unwrap();
    assert_eq!(tkt.priority, "critical");

    // Resolve
    let mut active: ticket::ActiveModel = tkt.into();
    active.status = Set("resolved".to_string());
    active.resolution_notes = Set(Some("Replaced faulty ONT".to_string()));
    active.resolved_at = Set(Some(chrono::Utc::now()));
    active.updated_at = Set(chrono::Utc::now());
    let tkt = active.update(db).await.unwrap();
    assert_eq!(tkt.status, "resolved");

    // Close
    let mut active: ticket::ActiveModel = tkt.into();
    active.status = Set("closed".to_string());
    active.closed_at = Set(Some(chrono::Utc::now()));
    active.updated_at = Set(chrono::Utc::now());
    let tkt = active.update(db).await.unwrap();
    assert_eq!(tkt.status, "closed");
}
