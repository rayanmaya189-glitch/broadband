//! Integration tests: BillingService business flows
//! Exercises the real service layer (GST computation, payment settlement,
//! voiding, refunds, line items, auto-generation) against PostgreSQL.

use crate::common::{TestDatabase, TestFixture};
use aeroxe_backend::modules::billing::application::services::BillingService;
use aeroxe_backend::shared::errors::AppError;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};

/// Helper: create branch + customer + subscription + pricing-backed plan.
async fn setup(db: &sea_orm::DatabaseConnection) -> (i64, i64, i64, i64) {
    let branch_id = TestFixture::create_branch(db).await;
    let customer_id = TestFixture::create_customer(db, branch_id).await;
    let plan_id = TestFixture::create_plan(db).await;

    // Insert active pricing for the plan (needed by auto-generate/renew paths).
    use aeroxe_backend::modules::plans::domain::entities::plan_pricing;
    let now = chrono::Utc::now();
    plan_pricing::ActiveModel {
        plan_id: Set(plan_id),
        billing_period_months: Set(1),
        price: Set(dec!(599.00)),
        is_active: Set(true),
        created_at: Set(now),
        ..Default::default()
    }
    .insert(db)
    .await
    .expect("Failed to insert plan pricing");

    // Create the subscription on the priced plan so billing paths resolve pricing.
    let subscription_id = aeroxe_backend::modules::subscription::application::services::SubscriptionService::create_subscription(
        db,
        customer_id,
        branch_id,
        plan_id,
        1,
    )
    .await
    .expect("Failed to create subscription")
    .id;
    (branch_id, customer_id, plan_id, subscription_id)
}

/// Invoice creation computes intra-state GST (9% CGST + 9% SGST) and a due date.
#[ignore]
#[tokio::test]
async fn test_create_invoice_applies_gst() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();
    let (branch_id, customer_id, _plan_id, subscription_id) = setup(db).await;

    let start = chrono::NaiveDate::from_ymd_opt(2026, 7, 1).unwrap();
    let end = chrono::NaiveDate::from_ymd_opt(2026, 7, 31).unwrap();

    let inv = BillingService::create_invoice(
        db,
        customer_id,
        branch_id,
        subscription_id,
        start,
        end,
        dec!(599.00),
    )
    .await
    .expect("Invoice creation failed");

    assert!(
        inv.invoice_number.starts_with("INV-"),
        "unexpected invoice number"
    );
    assert_eq!(inv.subtotal, dec!(599.00));
    assert_eq!(inv.cgst_amount, dec!(53.91));
    assert_eq!(inv.sgst_amount, dec!(53.91));
    assert_eq!(inv.igst_amount, Decimal::ZERO);
    assert_eq!(inv.tax_amount, dec!(107.82));
    assert_eq!(inv.total_amount, dec!(706.82));
    assert_eq!(inv.status, "pending");
    assert_eq!(inv.due_date, end + chrono::Duration::days(15));
}

/// A full payment settles the invoice to 'paid' and stamps paid_at.
#[ignore]
#[tokio::test]
async fn test_record_payment_marks_invoice_paid() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();
    let (branch_id, customer_id, _plan_id, subscription_id) = setup(db).await;

    let inv = BillingService::create_invoice(
        db,
        customer_id,
        branch_id,
        subscription_id,
        chrono::NaiveDate::from_ymd_opt(2026, 7, 1).unwrap(),
        chrono::NaiveDate::from_ymd_opt(2026, 7, 31).unwrap(),
        dec!(599.00),
    )
    .await
    .unwrap();

    let payment = BillingService::record_payment(
        db,
        inv.id,
        customer_id,
        branch_id,
        inv.total_amount,
        "upi".to_string(),
    )
    .await
    .expect("Payment recording failed");

    assert!(payment.payment_number.starts_with("PAY-"));
    assert_eq!(payment.status, "completed");

    let paid = BillingService::get_invoice(db, inv.id).await.unwrap();
    assert_eq!(paid.status, "paid");
    assert!(paid.paid_at.is_some());
}

/// A partial payment leaves the invoice in 'partial' status.
#[ignore]
#[tokio::test]
async fn test_partial_payment_leaves_invoice_partial() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();
    let (branch_id, customer_id, _plan_id, subscription_id) = setup(db).await;

    let inv = BillingService::create_invoice(
        db,
        customer_id,
        branch_id,
        subscription_id,
        chrono::NaiveDate::from_ymd_opt(2026, 7, 1).unwrap(),
        chrono::NaiveDate::from_ymd_opt(2026, 7, 31).unwrap(),
        dec!(599.00),
    )
    .await
    .unwrap();

    BillingService::record_payment(
        db,
        inv.id,
        customer_id,
        branch_id,
        dec!(300.00),
        "cash".to_string(),
    )
    .await
    .expect("Partial payment failed");

    let partial = BillingService::get_invoice(db, inv.id).await.unwrap();
    assert_eq!(partial.status, "partial");
    assert!(partial.paid_at.is_none());
}

/// A pending invoice can be sent and then voided; a paid invoice cannot be voided.
#[ignore]
#[tokio::test]
async fn test_send_and_void_invoice_guards() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();
    let (branch_id, customer_id, _plan_id, subscription_id) = setup(db).await;

    let inv = BillingService::create_invoice(
        db,
        customer_id,
        branch_id,
        subscription_id,
        chrono::NaiveDate::from_ymd_opt(2026, 7, 1).unwrap(),
        chrono::NaiveDate::from_ymd_opt(2026, 7, 31).unwrap(),
        dec!(599.00),
    )
    .await
    .unwrap();

    let sent = BillingService::send_invoice(db, inv.id).await.unwrap();
    assert_eq!(sent.status, "sent");

    let voided = BillingService::void_invoice(db, inv.id, "created by mistake")
        .await
        .unwrap();
    assert_eq!(voided.status, "voided");

    // A paid invoice must reject voiding.
    let inv2 = BillingService::create_invoice(
        db,
        customer_id,
        branch_id,
        subscription_id,
        chrono::NaiveDate::from_ymd_opt(2026, 8, 1).unwrap(),
        chrono::NaiveDate::from_ymd_opt(2026, 8, 31).unwrap(),
        dec!(599.00),
    )
    .await
    .unwrap();
    BillingService::record_payment(
        db,
        inv2.id,
        customer_id,
        branch_id,
        inv2.total_amount,
        "upi".to_string(),
    )
    .await
    .unwrap();

    let err = BillingService::void_invoice(db, inv2.id, "oops")
        .await
        .unwrap_err();
    assert!(
        matches!(err, AppError::Validation(_)),
        "paid invoice should not be voidable"
    );
}

/// list_overdue_invoices returns only unpaid invoices past their due date.
#[ignore]
#[tokio::test]
async fn test_list_overdue_invoices_filters() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();
    let (branch_id, customer_id, _plan_id, subscription_id) = setup(db).await;

    let overdue = BillingService::create_invoice(
        db,
        customer_id,
        branch_id,
        subscription_id,
        chrono::NaiveDate::from_ymd_opt(2026, 6, 1).unwrap(),
        chrono::NaiveDate::from_ymd_opt(2026, 6, 30).unwrap(),
        dec!(599.00),
    )
    .await
    .unwrap();
    // due_date = period_end + 15d = 2026-07-15, which is in the past relative to 2026-08.
    assert!(overdue.due_date < chrono::NaiveDate::from_ymd_opt(2026, 8, 1).unwrap());

    // A future-dated invoice must NOT be reported overdue.
    BillingService::create_invoice(
        db,
        customer_id,
        branch_id,
        subscription_id,
        chrono::NaiveDate::from_ymd_opt(2026, 9, 1).unwrap(),
        chrono::NaiveDate::from_ymd_opt(2026, 9, 30).unwrap(),
        dec!(599.00),
    )
    .await
    .unwrap();

    let due = BillingService::list_overdue_invoices(db, Some(branch_id))
        .await
        .expect("Overdue listing failed");

    // Billing dates are relative to real "today", so only assert membership:
    // the past-due invoice is present, and any future-due invoice is not.
    assert!(
        due.iter().any(|i| i.id == overdue.id),
        "past-due invoice should be listed as overdue"
    );
    assert!(
        due.iter()
            .all(|i| i.billing_period_end < chrono::Utc::now().date_naive()),
        "no future invoice should be flagged overdue"
    );
}

/// Adding and removing line items recomputes invoice totals with GST.
#[ignore]
#[tokio::test]
async fn test_line_items_recalculate_totals() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();
    let (branch_id, customer_id, _plan_id, subscription_id) = setup(db).await;

    let inv = BillingService::create_invoice(
        db,
        customer_id,
        branch_id,
        subscription_id,
        chrono::NaiveDate::from_ymd_opt(2026, 7, 1).unwrap(),
        chrono::NaiveDate::from_ymd_opt(2026, 7, 31).unwrap(),
        dec!(599.00),
    )
    .await
    .unwrap();

    let item = BillingService::add_line_item(
        db,
        inv.id,
        "Installation charge".to_string(),
        dec!(1.0),
        dec!(1000.00),
        None,
    )
    .await
    .expect("Adding line item failed");
    assert!(item.tax_rate > Decimal::ZERO);

    let updated = BillingService::get_invoice(db, inv.id).await.unwrap();
    assert_eq!(updated.subtotal, dec!(1000.00));
    assert_eq!(updated.tax_amount, dec!(180.00));
    assert_eq!(updated.total_amount, dec!(1180.00));

    // Removing the item zeroes the invoice.
    BillingService::remove_line_item(db, inv.id, item.id)
        .await
        .expect("Removing line item failed");
    let emptied = BillingService::get_invoice(db, inv.id).await.unwrap();
    assert_eq!(emptied.subtotal, Decimal::ZERO);
    assert_eq!(emptied.total_amount, Decimal::ZERO);
}

/// A wallet refund is processed in-transaction: refund marked processed, the
/// customer wallet is credited, and a GST credit note is created.
#[ignore]
#[tokio::test]
async fn test_refund_workflow_credits_wallet_and_creates_credit_note() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();
    let (branch_id, customer_id, _plan_id, subscription_id) = setup(db).await;

    let inv = BillingService::create_invoice(
        db,
        customer_id,
        branch_id,
        subscription_id,
        chrono::NaiveDate::from_ymd_opt(2026, 7, 1).unwrap(),
        chrono::NaiveDate::from_ymd_opt(2026, 7, 31).unwrap(),
        dec!(599.00),
    )
    .await
    .unwrap();

    let payment = BillingService::record_payment(
        db,
        inv.id,
        customer_id,
        branch_id,
        inv.total_amount,
        "wallet".to_string(),
    )
    .await
    .unwrap();

    let refund = BillingService::request_refund(
        db,
        payment.id,
        inv.id,
        customer_id,
        dec!(100.00),
        "Service disruption".to_string(),
        1,
    )
    .await
    .expect("Refund request failed");
    assert_eq!(refund.status, "pending");

    let processed = BillingService::approve_refund(db, refund.id, 1)
        .await
        .expect("Refund approval failed");
    assert_eq!(processed.status, "processed");

    // Wallet should now hold exactly the refund amount.
    use aeroxe_backend::modules::referral::domain::entities::customer_wallet;
    let wallet = customer_wallet::Entity::find()
        .filter(customer_wallet::Column::CustomerId.eq(customer_id))
        .one(db)
        .await
        .unwrap()
        .expect("Wallet should exist after refund");
    assert_eq!(wallet.balance, dec!(100.00));

    // A credit note must have been created against the original invoice.
    use aeroxe_backend::modules::billing::domain::entities::credit_debit_note;
    let notes = credit_debit_note::Entity::find()
        .filter(credit_debit_note::Column::OriginalInvoiceId.eq(inv.id))
        .all(db)
        .await
        .unwrap();
    assert_eq!(notes.len(), 1, "expected one credit note for the refund");
    assert_eq!(notes[0].note_type, "credit");
    assert_eq!(notes[0].total_amount, dec!(118.00)); // 100 + 18% GST
}

/// auto_generate_invoices invoices due subscriptions once (idempotent on rerun).
#[ignore]
#[tokio::test]
async fn test_auto_generate_invoices_is_idempotent() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();
    let (branch_id, customer_id, _plan_id, subscription_id) = setup(db).await;

    // Force the subscription due for billing today.
    use aeroxe_backend::modules::subscription::domain::entities::{subscription, Subscription};
    let sub = Subscription::find_by_id(subscription_id)
        .one(db)
        .await
        .unwrap()
        .unwrap();
    let mut active: subscription::ActiveModel = sub.into();
    active.next_billing_date = Set(Some(chrono::Utc::now().date_naive()));
    active.updated_at = Set(chrono::Utc::now());
    active.update(db).await.unwrap();

    let first = BillingService::auto_generate_invoices(db)
        .await
        .expect("Auto-generation failed");
    assert_eq!(first, 1, "exactly one invoice should be generated");

    // Second run must not duplicate the invoice.
    let second = BillingService::auto_generate_invoices(db).await.unwrap();
    assert_eq!(second, 0, "rerun must not duplicate invoices");

    // Verify the generated invoice used the plan price + GST.
    use aeroxe_backend::modules::billing::domain::entities::Invoice;
    let invoices = Invoice::find()
        .filter(
            aeroxe_backend::modules::billing::domain::entities::invoice::Column::SubscriptionId
                .eq(subscription_id),
        )
        .all(db)
        .await
        .unwrap();
    assert_eq!(invoices.len(), 1);
    assert_eq!(invoices[0].subtotal, dec!(599.00));
    assert_eq!(invoices[0].total_amount, dec!(706.82));
    let _ = branch_id;
    let _ = customer_id;
}

/// Security deposits can be collected and refunded.
#[ignore]
#[tokio::test]
async fn test_security_deposit_flow() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();
    let (branch_id, customer_id, _plan_id, _subscription_id) = setup(db).await;

    let deposit = BillingService::collect_security_deposit(
        db,
        customer_id,
        branch_id,
        dec!(2000.00),
        "equipment".to_string(),
    )
    .await
    .expect("Collect deposit failed");
    assert_eq!(deposit.status, "held");

    let refunded = BillingService::refund_security_deposit(
        db,
        deposit.id,
        dec!(2000.00),
        "Installation completed".to_string(),
    )
    .await
    .expect("Refund deposit failed");
    assert_eq!(refunded.status, "refunded");
    assert_eq!(refunded.refund_amount, Some(dec!(2000.00)));

    // Over-refunding must be rejected.
    let deposit2 = BillingService::collect_security_deposit(
        db,
        customer_id,
        branch_id,
        dec!(1000.00),
        "equipment".to_string(),
    )
    .await
    .unwrap();
    let err =
        BillingService::refund_security_deposit(db, deposit2.id, dec!(2000.00), "nope".to_string())
            .await
            .unwrap_err();
    assert!(matches!(err, AppError::Validation(_)));
}
