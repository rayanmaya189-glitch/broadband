//! Integration tests for ticket module using testcontainers
//!
//! Covers ticket CRUD, assignment, escalation, resolution flow,
//! comments, priority filtering, and status transitions.

mod common;

use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use crate::common::{TestDatabase, TestFixture};

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

async fn create_ticket(
    db: &sea_orm::DatabaseConnection,
    branch_id: i64,
    customer_id: Option<i64>,
    subject: &str,
    priority: &str,
    status: &str,
) -> crate::modules::ticket::domain::entities::ticket::Model {
    use crate::modules::ticket::domain::entities::ticket;

    let now = chrono::Utc::now();
    let active = ticket::ActiveModel {
        branch_id: Set(branch_id),
        customer_id: Set(customer_id),
        subject: Set(subject.to_string()),
        description: Set(format!("Description for {}", subject)),
        category: Set("connectivity".to_string()),
        priority: Set(priority.to_string()),
        status: Set(status.to_string()),
        source: Set("portal".to_string()),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    };
    active.insert(db).await.expect("Failed to create ticket")
}

// ===========================================================================
// CRUD operations
// ===========================================================================

#[tokio::test]
async fn test_create_ticket() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    let branch_id = TestFixture::create_branch(db).await;
    let customer_id = TestFixture::create_customer(db, branch_id).await;

    let ticket = create_ticket(db, branch_id, Some(customer_id), "Connectivity issue", "high", "open").await;

    assert!(ticket.id > 0, "Ticket should have positive ID");
    assert_eq!(ticket.subject, "Connectivity issue");
    assert_eq!(ticket.priority, "high");
    assert_eq!(ticket.status, "open");
    assert_eq!(ticket.source, "portal");
    assert_eq!(ticket.category, "connectivity");
}

#[tokio::test]
async fn test_retrieve_ticket_by_id() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    let branch_id = TestFixture::create_branch(db).await;
    let created = create_ticket(db, branch_id, None, "Retrieve test", "medium", "open").await;

    let found = crate::modules::ticket::domain::entities::ticket::Entity::find_by_id(created.id)
        .one(db)
        .await
        .expect("Query failed")
        .expect("Ticket not found");

    assert_eq!(found.id, created.id);
    assert_eq!(found.subject, "Retrieve test");
}

#[tokio::test]
async fn test_list_tickets_by_status() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    let branch_id = TestFixture::create_branch(db).await;

    create_ticket(db, branch_id, None, "Open 1", "low", "open").await;
    create_ticket(db, branch_id, None, "Open 2", "medium", "open").await;
    create_ticket(db, branch_id, None, "Resolved 1", "high", "resolved").await;

    use crate::modules::ticket::domain::entities::ticket;

    let open_tickets = ticket::Entity::find()
        .filter(ticket::Column::Status.eq("open"))
        .all(db)
        .await
        .expect("Failed to query");

    assert_eq!(open_tickets.len(), 2);

    let resolved_tickets = ticket::Entity::find()
        .filter(ticket::Column::Status.eq("resolved"))
        .all(db)
        .await
        .expect("Failed to query");

    assert_eq!(resolved_tickets.len(), 1);
}

#[tokio::test]
async fn test_list_tickets_by_branch() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    let branch_a = TestFixture::create_branch(db).await;
    let branch_b = TestFixture::create_branch(db).await;

    create_ticket(db, branch_a, None, "Branch A ticket", "low", "open").await;
    create_ticket(db, branch_a, None, "Branch A ticket 2", "low", "open").await;
    create_ticket(db, branch_b, None, "Branch B ticket", "low", "open").await;

    use crate::modules::ticket::domain::entities::ticket;

    let branch_a_tickets = ticket::Entity::find()
        .filter(ticket::Column::BranchId.eq(branch_a))
        .all(db)
        .await
        .expect("Failed to query");

    assert_eq!(branch_a_tickets.len(), 2);
}

#[tokio::test]
async fn test_update_ticket() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    let branch_id = TestFixture::create_branch(db).await;
    let mut ticket = create_ticket(db, branch_id, None, "Update test", "low", "open").await;

    use crate::modules::ticket::domain::entities::ticket;

    let mut active: ticket::ActiveModel = ticket.into();
    active.subject = Set("Updated subject".to_string());
    active.priority = Set("critical".to_string());
    active.updated_at = Set(chrono::Utc::now());

    let updated = active.update(db).await.expect("Failed to update ticket");
    assert_eq!(updated.subject, "Updated subject");
    assert_eq!(updated.priority, "critical");
}

#[tokio::test]
async fn test_delete_ticket() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    let branch_id = TestFixture::create_branch(db).await;
    let ticket = create_ticket(db, branch_id, None, "Delete test", "low", "open").await;

    use crate::modules::ticket::domain::entities::ticket;

    ticket::Entity::delete_by_id(ticket.id)
        .exec(db)
        .await
        .expect("Failed to delete ticket");

    let found = ticket::Entity::find_by_id(ticket.id)
        .one(db)
        .await
        .expect("Query failed");
    assert!(found.is_none(), "Ticket should be deleted");
}

// ===========================================================================
// Status transitions
// ===========================================================================

#[tokio::test]
async fn test_ticket_open_to_in_progress() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    let branch_id = TestFixture::create_branch(db).await;
    let ticket = create_ticket(db, branch_id, None, "Status test", "medium", "open").await;

    use crate::modules::ticket::domain::entities::ticket;

    let mut active: ticket::ActiveModel = ticket.into();
    active.status = Set("in_progress".to_string());
    active.updated_at = Set(chrono::Utc::now());

    let updated = active.update(db).await.unwrap();
    assert_eq!(updated.status, "in_progress");
}

#[tokio::test]
async fn test_ticket_full_resolution_flow() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    let branch_id = TestFixture::create_branch(db).await;
    let ticket = create_ticket(db, branch_id, None, "Full flow", "high", "open").await;

    use crate::modules::ticket::domain::entities::ticket;

    // open -> in_progress
    let mut active: ticket::ActiveModel = ticket.into();
    active.status = Set("in_progress".to_string());
    active.first_response_at = Set(Some(chrono::Utc::now()));
    active.updated_at = Set(chrono::Utc::now());
    let in_progress = active.update(db).await.unwrap();
    assert_eq!(in_progress.status, "in_progress");
    assert!(in_progress.first_response_at.is_some());

    // in_progress -> resolved
    let mut active: ticket::ActiveModel = in_progress.into();
    active.status = Set("resolved".to_string());
    active.resolution_notes = Set(Some("Issue was caused by a misconfigured VLAN. Fixed.".to_string()));
    active.resolved_at = Set(Some(chrono::Utc::now()));
    active.updated_at = Set(chrono::Utc::now());
    let resolved = active.update(db).await.unwrap();
    assert_eq!(resolved.status, "resolved");
    assert!(resolved.resolution_notes.is_some());
    assert!(resolved.resolved_at.is_some());

    // resolved -> closed
    let mut active: ticket::ActiveModel = resolved.into();
    active.status = Set("closed".to_string());
    active.closed_at = Set(Some(chrono::Utc::now()));
    active.satisfaction_rating = Set(Some(5));
    active.updated_at = Set(chrono::Utc::now());
    let closed = active.update(db).await.unwrap();
    assert_eq!(closed.status, "closed");
    assert!(closed.closed_at.is_some());
    assert_eq!(closed.satisfaction_rating, Some(5));
}

#[tokio::test]
async fn test_ticket_reopen() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    let branch_id = TestFixture::create_branch(db).await;
    let ticket = create_ticket(db, branch_id, None, "Reopen test", "medium", "open").await;

    use crate::modules::ticket::domain::entities::ticket;

    // open -> in_progress -> resolved
    let mut active: ticket::ActiveModel = ticket.into();
    active.status = Set("in_progress".to_string());
    active.updated_at = Set(chrono::Utc::now());
    let in_prog = active.update(db).await.unwrap();

    let mut active: ticket::ActiveModel = in_prog.into();
    active.status = Set("resolved".to_string());
    active.resolution_notes = Set(Some("Temp fix".to_string()));
    active.resolved_at = Set(Some(chrono::Utc::now()));
    active.updated_at = Set(chrono::Utc::now());
    let resolved = active.update(db).await.unwrap();

    // Reopen: resolved -> in_progress
    let mut active: ticket::ActiveModel = resolved.into();
    active.status = Set("in_progress".to_string());
    active.resolved_at = Set(None);
    active.resolution_notes = Set(None);
    active.updated_at = Set(chrono::Utc::now());
    let reopened = active.update(db).await.unwrap();
    assert_eq!(reopened.status, "in_progress");
    assert!(reopened.resolved_at.is_none());
}

// ===========================================================================
// Assignment
// ===========================================================================

#[tokio::test]
async fn test_ticket_assignment() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    let branch_id = TestFixture::create_branch(db).await;
    let ticket = create_ticket(db, branch_id, None, "Assignment test", "high", "open").await;
    assert!(ticket.assigned_to.is_none());

    use crate::modules::ticket::domain::entities::ticket;

    let mut active: ticket::ActiveModel = ticket.into();
    active.assigned_to = Set(Some(42)); // agent user id
    active.status = Set("in_progress".to_string());
    active.updated_at = Set(chrono::Utc::now());

    let assigned = active.update(db).await.unwrap();
    assert_eq!(assigned.assigned_to, Some(42));
    assert_eq!(assigned.status, "in_progress");
}

#[tokio::test]
async fn test_ticket_reassignment() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    let branch_id = TestFixture::create_branch(db).await;
    let ticket = create_ticket(db, branch_id, None, "Reassign test", "high", "open").await;

    use crate::modules::ticket::domain::entities::ticket;

    // Assign to agent 10
    let mut active: ticket::ActiveModel = ticket.into();
    active.assigned_to = Set(Some(10));
    active.updated_at = Set(chrono::Utc::now());
    let t1 = active.update(db).await.unwrap();
    assert_eq!(t1.assigned_to, Some(10));

    // Reassign to agent 20
    let mut active: ticket::ActiveModel = t1.into();
    active.assigned_to = Set(Some(20));
    active.updated_at = Set(chrono::Utc::now());
    let t2 = active.update(db).await.unwrap();
    assert_eq!(t2.assigned_to, Some(20));
}

// ===========================================================================
// Escalation
// ===========================================================================

#[tokio::test]
async fn test_ticket_escalation() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    let branch_id = TestFixture::create_branch(db).await;
    let ticket = create_ticket(db, branch_id, None, "Escalation test", "medium", "open").await;

    use crate::modules::ticket::domain::entities::ticket;

    let mut active: ticket::ActiveModel = ticket.into();
    active.escalated_to = Set(Some(99)); // manager user id
    active.priority = Set("critical".to_string());
    active.updated_at = Set(chrono::Utc::now());

    let escalated = active.update(db).await.unwrap();
    assert_eq!(escalated.escalated_to, Some(99));
    assert_eq!(escalated.priority, "critical");
}

#[tokio::test]
async fn test_ticket_escalation_with_assignment() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    let branch_id = TestFixture::create_branch(db).await;
    let ticket = create_ticket(db, branch_id, None, "Escalated assignment", "high", "open").await;

    use crate::modules::ticket::domain::entities::ticket;

    // Assign to first-level agent
    let mut active: ticket::ActiveModel = ticket.into();
    active.assigned_to = Set(Some(10));
    active.status = Set("in_progress".to_string());
    active.updated_at = Set(chrono::Utc::now());
    let t1 = active.update(db).await.unwrap();

    // Escalate to second-level
    let mut active: ticket::ActiveModel = t1.into();
    active.escalated_to = Set(Some(50));
    active.assigned_to = Set(Some(50));
    active.priority = Set("critical".to_string());
    active.updated_at = Set(chrono::Utc::now());
    let escalated = active.update(db).await.unwrap();

    assert_eq!(escalated.assigned_to, Some(50));
    assert_eq!(escalated.escalated_to, Some(50));
    assert_eq!(escalated.priority, "critical");
}

// ===========================================================================
// Ticket comments
// ===========================================================================

#[tokio::test]
async fn test_add_ticket_comment() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    let branch_id = TestFixture::create_branch(db).await;
    let ticket = create_ticket(db, branch_id, None, "Comment test", "medium", "open").await;

    use crate::modules::ticket::domain::entities::ticket_comment;

    let now = chrono::Utc::now();
    let comment = ticket_comment::ActiveModel {
        ticket_id: Set(ticket.id),
        user_id: Set(Some(1)),
        is_customer: Set(false),
        comment: Set("Investigating the issue now.".to_string()),
        is_internal: Set(false),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    };

    let created = comment.insert(db).await.expect("Failed to add comment");
    assert!(created.id > 0);
    assert_eq!(created.ticket_id, ticket.id);
    assert_eq!(created.comment, "Investigating the issue now.");
    assert!(!created.is_internal);
}

#[tokio::test]
async fn test_add_internal_comment() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    let branch_id = TestFixture::create_branch(db).await;
    let ticket = create_ticket(db, branch_id, None, "Internal comment test", "medium", "open").await;

    use crate::modules::ticket::domain::entities::ticket_comment;

    let now = chrono::Utc::now();
    let comment = ticket_comment::ActiveModel {
        ticket_id: Set(ticket.id),
        user_id: Set(Some(1)),
        is_customer: Set(false),
        comment: Set("Escalated to NOC team".to_string()),
        is_internal: Set(true),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    };

    let created = comment.insert(db).await.expect("Failed to add comment");
    assert!(created.is_internal);
}

#[tokio::test]
async fn test_list_ticket_comments() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    let branch_id = TestFixture::create_branch(db).await;
    let ticket = create_ticket(db, branch_id, None, "Multi comment test", "medium", "open").await;

    use crate::modules::ticket::domain::entities::ticket_comment;

    let now = chrono::Utc::now();
    for i in 0..3 {
        let comment = ticket_comment::ActiveModel {
            ticket_id: Set(ticket.id),
            user_id: Set(Some(1)),
            is_customer: Set(i == 0),
            comment: Set(format!("Comment {}", i)),
            is_internal: Set(false),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };
        comment.insert(db).await.unwrap();
    }

    let comments = ticket_comment::Entity::find()
        .filter(ticket_comment::Column::TicketId.eq(ticket.id))
        .all(db)
        .await
        .expect("Failed to query comments");

    assert_eq!(comments.len(), 3);
    assert!(comments[0].is_customer, "First comment should be from customer");
}

#[tokio::test]
async fn test_add_comment_with_attachments() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    let branch_id = TestFixture::create_branch(db).await;
    let ticket = create_ticket(db, branch_id, None, "Attachment test", "medium", "open").await;

    use crate::modules::ticket::domain::entities::ticket_comment;

    let now = chrono::Utc::now();
    let attachments = serde_json::json!([
        {"name": "screenshot.png", "url": "https://storage.aeroxe.com/tickets/img.png"}
    ]);

    let comment = ticket_comment::ActiveModel {
        ticket_id: Set(ticket.id),
        user_id: Set(Some(1)),
        is_customer: Set(true),
        comment: Set("See attached screenshot".to_string()),
        is_internal: Set(false),
        attachments: Set(Some(attachments)),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    };

    let created = comment.insert(db).await.unwrap();
    assert!(created.attachments.is_some());
}

// ===========================================================================
// Priority filtering
// ===========================================================================

#[tokio::test]
async fn test_filter_tickets_by_priority() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    let branch_id = TestFixture::create_branch(db).await;

    create_ticket(db, branch_id, None, "Critical 1", "critical", "open").await;
    create_ticket(db, branch_id, None, "Critical 2", "critical", "open").await;
    create_ticket(db, branch_id, None, "High 1", "high", "open").await;
    create_ticket(db, branch_id, None, "Medium 1", "medium", "open").await;

    use crate::modules::ticket::domain::entities::ticket;

    let critical = ticket::Entity::find()
        .filter(ticket::Column::Priority.eq("critical"))
        .all(db)
        .await
        .unwrap();
    assert_eq!(critical.len(), 2);

    let high = ticket::Entity::find()
        .filter(ticket::Column::Priority.eq("high"))
        .all(db)
        .await
        .unwrap();
    assert_eq!(high.len(), 1);
}

#[tokio::test]
async fn test_filter_tickets_by_category() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    let branch_id = TestFixture::create_branch(db).await;

    use crate::modules::ticket::domain::entities::ticket;
    let now = chrono::Utc::now();

    // Create tickets with different categories
    for (cat, idx) in [("connectivity", 1), ("billing", 2), ("connectivity", 3)] {
        let active = ticket::ActiveModel {
            branch_id: Set(branch_id),
            subject: Set(format!("Ticket {}", idx)),
            description: Set("Test".to_string()),
            category: Set(cat.to_string()),
            priority: Set("medium".to_string()),
            status: Set("open".to_string()),
            source: Set("portal".to_string()),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };
        active.insert(db).await.unwrap();
    }

    let connectivity = ticket::Entity::find()
        .filter(ticket::Column::Category.eq("connectivity"))
        .all(db)
        .await
        .unwrap();
    assert_eq!(connectivity.len(), 2);
}

#[tokio::test]
async fn test_filter_tickets_by_source() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    let branch_id = TestFixture::create_branch(db).await;

    use crate::modules::ticket::domain::entities::ticket;
    let now = chrono::Utc::now();

    for (source, idx) in [("phone", 1), ("portal", 2), ("phone", 3), ("email", 4)] {
        let active = ticket::ActiveModel {
            branch_id: Set(branch_id),
            subject: Set(format!("Ticket {}", idx)),
            description: Set("Test".to_string()),
            category: Set("connectivity".to_string()),
            priority: Set("low".to_string()),
            status: Set("open".to_string()),
            source: Set(source.to_string()),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };
        active.insert(db).await.unwrap();
    }

    let phone_tickets = ticket::Entity::find()
        .filter(ticket::Column::Source.eq("phone"))
        .all(db)
        .await
        .unwrap();
    assert_eq!(phone_tickets.len(), 2);
}

// ===========================================================================
// Error cases
// ===========================================================================

#[tokio::test]
async fn test_ticket_invalid_branch_id() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    use crate::modules::ticket::domain::entities::ticket;
    let now = chrono::Utc::now();

    let active = ticket::ActiveModel {
        branch_id: Set(999999),
        subject: Set("Bad branch".to_string()),
        description: Set("Test".to_string()),
        category: Set("connectivity".to_string()),
        priority: Set("medium".to_string()),
        status: Set("open".to_string()),
        source: Set("portal".to_string()),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    };

    let result = active.insert(db).await;
    assert!(result.is_err(), "Should fail with non-existent branch_id");
}

#[tokio::test]
async fn test_ticket_with_customer_reference() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    let branch_id = TestFixture::create_branch(db).await;
    let customer_id = TestFixture::create_customer(db, branch_id).await;

    let ticket = create_ticket(
        db,
        branch_id,
        Some(customer_id),
        "Customer issue",
        "high",
        "open",
    )
    .await;

    assert_eq!(ticket.customer_id, Some(customer_id));
}

#[tokio::test]
async fn test_ticket_without_customer() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    let branch_id = TestFixture::create_branch(db).await;
    let ticket = create_ticket(db, branch_id, None, "No customer", "low", "open").await;

    assert!(ticket.customer_id.is_none());
}

// ===========================================================================
// Concurrent ticket creation
// ===========================================================================

#[tokio::test]
async fn test_concurrent_ticket_creation() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    let branch_id = TestFixture::create_branch(db).await;

    let mut handles = Vec::new();
    for i in 0..5 {
        let db_clone = db.clone();
        let branch_id = branch_id;
        let handle = tokio::spawn(async move {
            use crate::modules::ticket::domain::entities::ticket;
            let now = chrono::Utc::now();
            let active = ticket::ActiveModel {
                branch_id: Set(branch_id),
                subject: Set(format!("Concurrent ticket {}", i)),
                description: Set("Test".to_string()),
                category: Set("connectivity".to_string()),
                priority: Set("medium".to_string()),
                status: Set("open".to_string()),
                source: Set("portal".to_string()),
                created_at: Set(now),
                updated_at: Set(now),
                ..Default::default()
            };
            active.insert(&db_clone).await.expect("Failed to create ticket")
        });
        handles.push(handle);
    }

    let results: Vec<_> = futures::future::join_all(handles)
        .await
        .into_iter()
        .collect::<Result<_, _>>()
        .expect("One or more tasks failed");

    assert_eq!(results.len(), 5);
    for r in &results {
        assert!(r.id > 0);
    }
}
