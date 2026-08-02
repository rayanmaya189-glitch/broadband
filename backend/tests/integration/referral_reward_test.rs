//! Integration tests for the referral reward flow using testcontainers.
//! Covers: pending referral -> referee registration link -> reward payout
//! (wallet credit + transaction + outbox event) and idempotency.

use crate::common::{TestDatabase, TestFixture};
use aeroxe_backend::modules::referral::application::services::ReferralService;
use aeroxe_backend::modules::referral::domain::entities::{
    CustomerWallet, ReferralProgramActiveModel, ReferralTracking, WalletTransaction,
};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use std::str::FromStr;

async fn create_program(db: &sea_orm::DatabaseConnection) -> i64 {
    let now = chrono::Utc::now();
    let program = ReferralProgramActiveModel {
        name: Set(format!("test-program-{}", rand::random::<u32>())),
        reward_type: Set("fixed".to_string()),
        reward_value: Set(sea_orm::prelude::Decimal::from_str("500").unwrap()),
        max_referrals_per_user: Set(Some(10)),
        valid_from: Set(now.date_naive()),
        valid_until: Set(now.date_naive() + chrono::Duration::days(365)),
        is_active: Set(true),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    };
    program.insert(db).await.expect("create program").id
}

/// Full flow: pending referral -> registered -> rewarded, with wallet + event.
#[ignore]
#[tokio::test]
async fn test_referral_reward_full_flow() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    let branch_id = TestFixture::create_branch(db).await;
    let referrer_id = TestFixture::create_customer(db, branch_id).await;
    let referee_phone = format!("+91{:010}", rand::random::<u64>() % 10_000_000_000);

    let program_id = create_program(db).await;

    ReferralService::create_referral(
        db,
        program_id,
        referrer_id,
        referee_phone.clone(),
        "REF-TEST1".to_string(),
    )
    .await
    .expect("create referral");

    // Register the referee -> pending -> registered, referee_id set.
    let referee_id = TestFixture::create_customer(db, branch_id).await;
    let linked = ReferralService::register_referee(db, &referee_phone, referee_id)
        .await
        .expect("register referee");
    assert!(linked.is_some(), "pending referral should be linked");
    assert_eq!(linked.as_ref().unwrap().status, "registered");
    assert_eq!(linked.as_ref().unwrap().referee_id, Some(referee_id));

    // Reward -> status rewarded, wallet credited, transaction + event written.
    ReferralService::reward_activated_referee(db, referee_id)
        .await
        .expect("reward activated referee");

    let referral = ReferralTracking::find_by_id(linked.unwrap().id)
        .one(db)
        .await
        .expect("load referral")
        .expect("referral exists");
    assert_eq!(referral.status, "rewarded");
    assert_eq!(referral.referrer_reward_status.as_deref(), Some("credited"));
    assert_eq!(
        referral.referrer_reward_amount.unwrap(),
        sea_orm::prelude::Decimal::from_str("500").unwrap()
    );
    assert!(referral.rewarded_at.is_some());

    let wallet = CustomerWallet::find()
        .filter(
            aeroxe_backend::modules::referral::domain::entities::CustomerWalletColumn::CustomerId
                .eq(referrer_id),
        )
        .one(db)
        .await
        .expect("load wallet")
        .expect("wallet exists");
    assert_eq!(
        wallet.balance,
        sea_orm::prelude::Decimal::from_str("500").unwrap()
    );

    let txns = WalletTransaction::find()
        .filter(
            aeroxe_backend::modules::referral::domain::entities::WalletTransactionColumn::WalletId
                .eq(wallet.id),
        )
        .all(db)
        .await
        .expect("load wallet transactions");
    assert_eq!(txns.len(), 1);
    assert_eq!(txns[0].transaction_type, "referral_reward");
    assert_eq!(txns[0].reference_id, Some(referral.id));

    let events = aeroxe_backend::infrastructure::messaging::outbox_entity::Entity::find()
        .filter(
            aeroxe_backend::infrastructure::messaging::outbox_entity::Column::EventType
                .eq("referral.rewarded"),
        )
        .filter(
            aeroxe_backend::infrastructure::messaging::outbox_entity::Column::AggregateId
                .eq(referral.id),
        )
        .all(db)
        .await
        .expect("load outbox events");
    assert_eq!(events.len(), 1, "one referral.rewarded event expected");
}

/// Rewarding the same activated referee twice must not double-credit.
#[ignore]
#[tokio::test]
async fn test_referral_reward_is_idempotent() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    let branch_id = TestFixture::create_branch(db).await;
    let referrer_id = TestFixture::create_customer(db, branch_id).await;
    let referee_phone = format!("+91{:010}", rand::random::<u64>() % 10_000_000_000);

    let program_id = create_program(db).await;
    ReferralService::create_referral(
        db,
        program_id,
        referrer_id,
        referee_phone.clone(),
        "REF-TEST2".to_string(),
    )
    .await
    .expect("create referral");

    let referee_id = TestFixture::create_customer(db, branch_id).await;
    ReferralService::register_referee(db, &referee_phone, referee_id)
        .await
        .expect("register referee");

    ReferralService::reward_activated_referee(db, referee_id)
        .await
        .expect("first reward");
    ReferralService::reward_activated_referee(db, referee_id)
        .await
        .expect("second reward (idempotent)");

    let wallet = CustomerWallet::find()
        .filter(
            aeroxe_backend::modules::referral::domain::entities::CustomerWalletColumn::CustomerId
                .eq(referrer_id),
        )
        .one(db)
        .await
        .expect("load wallet")
        .expect("wallet exists");
    assert_eq!(
        wallet.balance,
        sea_orm::prelude::Decimal::from_str("500").unwrap()
    );

    let txns = WalletTransaction::find()
        .filter(
            aeroxe_backend::modules::referral::domain::entities::WalletTransactionColumn::WalletId
                .eq(wallet.id),
        )
        .all(db)
        .await
        .expect("load wallet transactions");
    assert_eq!(txns.len(), 1, "reward must only be credited once");
}

/// register_referee must return None when no pending referral matches.
#[ignore]
#[tokio::test]
async fn test_register_referee_no_match() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    let branch_id = TestFixture::create_branch(db).await;
    let referee_id = TestFixture::create_customer(db, branch_id).await;

    let linked = ReferralService::register_referee(db, "+910000000000", referee_id)
        .await
        .expect("register referee");
    assert!(linked.is_none(), "no pending referral should be linked");
}
