//! Integration tests for identity module (users, sessions, 2FA, lockout)
//!
//! Tests user registration, authentication flows, session management,
//! two-factor authentication setup, and account lockout mechanics.

mod common;

use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use crate::common::TestDatabase;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Insert a user into the identity schema and return the created model.
async fn insert_test_user(
    db: &sea_orm::DatabaseConnection,
    email: &str,
    phone: &str,
    name: &str,
) -> crate::modules::identity::domain::entities::user::Model {
    use crate::modules::identity::domain::entities::user;

    let now = chrono::Utc::now();
    let active = user::ActiveModel {
        email: Set(email.to_string()),
        phone: Set(phone.to_string()),
        password_hash: Set(Some("$argon2id$v=19$m=4096,t=3,p=1$hash".to_string())),
        name: Set(name.to_string()),
        status: Set("active".to_string()),
        failed_login_attempts: Set(0),
        two_factor_enabled: Set(false),
        phone_verified: Set(false),
        email_verified: Set(false),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    };
    active.insert(db).await.expect("Failed to insert test user")
}

// ===========================================================================
// User registration
// ===========================================================================

#[tokio::test]
async fn test_user_registration() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    let user = insert_test_user(db, "alice@aeroxe.com", "+919000000001", "Alice").await;

    assert!(user.id > 0, "User should get a positive ID");
    assert_eq!(user.email, "alice@aeroxe.com");
    assert_eq!(user.phone, "+919000000001");
    assert_eq!(user.name, "Alice");
    assert_eq!(user.status, "active");
    assert!(!user.two_factor_enabled);
}

#[tokio::test]
async fn test_user_registration_duplicate_email_fails() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    insert_test_user(db, "bob@aeroxe.com", "+919000000002", "Bob").await;

    // Second user with the same email should fail (unique constraint)
    let duplicate = insert_test_user(db, "bob@aeroxe.com", "+919000000099", "Robert").await;
    let result = duplicate.insert(db).await;
    assert!(result.is_err(), "Duplicate email should be rejected");
}

#[tokio::test]
async fn test_user_registration_duplicate_phone_fails() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    insert_test_user(db, "carol@aeroxe.com", "+919000000003", "Carol").await;

    let duplicate = insert_test_user(db, "carol2@aeroxe.com", "+919000000003", "Carol2").await;
    let result = duplicate.insert(db).await;
    assert!(result.is_err(), "Duplicate phone should be rejected");
}

#[tokio::test]
async fn test_user_retrieval_by_email() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    let created = insert_test_user(db, "dave@aeroxe.com", "+919000000004", "Dave").await;

    use crate::modules::identity::domain::entities::user;

    let found = user::Entity::find()
        .filter(user::Column::Email.eq("dave@aeroxe.com"))
        .one(db)
        .await
        .expect("Query failed")
        .expect("User not found");

    assert_eq!(found.id, created.id);
    assert_eq!(found.name, "Dave");
}

#[tokio::test]
async fn test_user_retrieval_by_phone() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    let created = insert_test_user(db, "eve@aeroxe.com", "+919000000005", "Eve").await;

    use crate::modules::identity::domain::entities::user;

    let found = user::Entity::find()
        .filter(user::Column::Phone.eq("+919000000005"))
        .one(db)
        .await
        .expect("Query failed")
        .expect("User not found");

    assert_eq!(found.id, created.id);
}

// ===========================================================================
// Login simulation / last_login_at
// ===========================================================================

#[tokio::test]
async fn test_user_login_updates_last_login_at() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    let created = insert_test_user(db, "frank@aeroxe.com", "+919000000006", "Frank").await;
    assert!(created.last_login_at.is_none(), "Should have no login yet");

    use crate::modules::identity::domain::entities::user;

    let mut active: user::ActiveModel = created.into();
    active.last_login_at = Set(Some(chrono::Utc::now()));
    active.updated_at = Set(chrono::Utc::now());

    let updated = active.update(db).await.expect("Failed to update login time");
    assert!(updated.last_login_at.is_some(), "last_login_at should be set");
}

#[tokio::test]
async fn test_failed_login_increments_counter() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    let created = insert_test_user(db, "grace@aeroxe.com", "+919000000007", "Grace").await;
    assert_eq!(created.failed_login_attempts, 0);

    use crate::modules::identity::domain::entities::user;

    let mut active: user::ActiveModel = created.into();
    active.failed_login_attempts = Set(1);
    active.updated_at = Set(chrono::Utc::now());
    let u1 = active.update(db).await.unwrap();
    assert_eq!(u1.failed_login_attempts, 1);

    let mut active: user::ActiveModel = u1.into();
    active.failed_login_attempts = Set(2);
    active.updated_at = Set(chrono::Utc::now());
    let u2 = active.update(db).await.unwrap();
    assert_eq!(u2.failed_login_attempts, 2);
}

// ===========================================================================
// Account lockout
// ===========================================================================

#[tokio::test]
async fn test_account_lockout_after_failed_attempts() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    let created = insert_test_user(db, "hank@aeroxe.com", "+919000000008", "Hank").await;

    use crate::modules::identity::domain::entities::user;

    // Simulate 5 failed attempts then lockout
    let mut active: user::ActiveModel = created.into();
    active.failed_login_attempts = Set(5);
    active.locked_until = Set(Some(
        chrono::Utc::now() + chrono::Duration::minutes(15),
    ));
    active.updated_at = Set(chrono::Utc::now());

    let locked = active.update(db).await.expect("Failed to lock account");
    assert_eq!(locked.failed_login_attempts, 5);
    assert!(locked.locked_until.is_some(), "Account should have locked_until set");
}

#[tokio::test]
async fn test_account_unlock_after_timeout() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    let created = insert_test_user(db, "ivy@aeroxe.com", "+919000000009", "Ivy").await;

    use crate::modules::identity::domain::entities::user;

    // Lock the account
    let mut active: user::ActiveModel = created.into();
    active.failed_login_attempts = Set(5);
    active.locked_until = Set(Some(
        chrono::Utc::now() + chrono::Duration::seconds(1),
    ));
    active.updated_at = Set(chrono::Utc::now());
    let locked = active.update(db).await.unwrap();

    // Simulate unlock (admin reset or timeout expiry)
    let mut active: user::ActiveModel = locked.into();
    active.failed_login_attempts = Set(0);
    active.locked_until = Set(None);
    active.updated_at = Set(chrono::Utc::now());

    let unlocked = active.update(db).await.expect("Failed to unlock account");
    assert_eq!(unlocked.failed_login_attempts, 0);
    assert!(unlocked.locked_until.is_none());
}

#[tokio::test]
async fn test_account_status_transitions() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    let created = insert_test_user(db, "jake@aeroxe.com", "+919000000010", "Jake").await;
    assert_eq!(created.status, "active");

    use crate::modules::identity::domain::entities::user;

    // active -> suspended
    let mut active: user::ActiveModel = created.into();
    active.status = Set("suspended".to_string());
    active.updated_at = Set(chrono::Utc::now());
    let suspended = active.update(db).await.unwrap();
    assert_eq!(suspended.status, "suspended");

    // suspended -> active
    let mut active: user::ActiveModel = suspended.into();
    active.status = Set("active".to_string());
    active.updated_at = Set(chrono::Utc::now());
    let reactivated = active.update(db).await.unwrap();
    assert_eq!(reactivated.status, "active");

    // active -> deactivated
    let mut active: user::ActiveModel = reactivated.into();
    active.status = Set("deactivated".to_string());
    active.updated_at = Set(chrono::Utc::now());
    let deactivated = active.update(db).await.unwrap();
    assert_eq!(deactivated.status, "deactivated");
}

// ===========================================================================
// Two-factor authentication
// ===========================================================================

#[tokio::test]
async fn test_enable_two_factor_authentication() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    let created = insert_test_user(db, "kate@aeroxe.com", "+919000000011", "Kate").await;
    assert!(!created.two_factor_enabled);

    use crate::modules::identity::domain::entities::user;

    let mut active: user::ActiveModel = created.into();
    active.two_factor_enabled = Set(true);
    active.two_factor_secret = Set(Some("JBSWY3DPEHPK3PXP".to_string()));
    active.two_factor_backup_codes = Set(Some(
        serde_json::to_string(&vec!["ABCD-EFGH", "IJKL-MNOP"])
            .unwrap(),
    ));
    active.updated_at = Set(chrono::Utc::now());

    let updated = active.update(db).await.expect("Failed to enable 2FA");
    assert!(updated.two_factor_enabled);
    assert!(updated.two_factor_secret.is_some());
    assert!(updated.two_factor_backup_codes.is_some());
}

#[tokio::test]
async fn test_disable_two_factor_authentication() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    let created = insert_test_user(db, "leo@aeroxe.com", "+919000000012", "Leo").await;

    use crate::modules::identity::domain::entities::user;

    // Enable 2FA first
    let mut active: user::ActiveModel = created.into();
    active.two_factor_enabled = Set(true);
    active.two_factor_secret = Set(Some("SECRET123".to_string()));
    active.updated_at = Set(chrono::Utc::now());
    let enabled = active.update(db).await.unwrap();
    assert!(enabled.two_factor_enabled);

    // Disable 2FA
    let mut active: user::ActiveModel = enabled.into();
    active.two_factor_enabled = Set(false);
    active.two_factor_secret = Set(None);
    active.two_factor_backup_codes = Set(None);
    active.updated_at = Set(chrono::Utc::now());
    let disabled = active.update(db).await.unwrap();
    assert!(!disabled.two_factor_enabled);
    assert!(disabled.two_factor_secret.is_none());
}

// ===========================================================================
// Email / phone verification
// ===========================================================================

#[tokio::test]
async fn test_email_verification() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    let created = insert_test_user(db, "mia@aeroxe.com", "+919000000013", "Mia").await;
    assert!(!created.email_verified);

    use crate::modules::identity::domain::entities::user;

    let mut active: user::ActiveModel = created.into();
    active.email_verified = Set(true);
    active.updated_at = Set(chrono::Utc::now());

    let verified = active.update(db).await.unwrap();
    assert!(verified.email_verified);
}

#[tokio::test]
async fn test_phone_verification() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    let created = insert_test_user(db, "nora@aeroxe.com", "+919000000014", "Nora").await;
    assert!(!created.phone_verified);

    use crate::modules::identity::domain::entities::user;

    let mut active: user::ActiveModel = created.into();
    active.phone_verified = Set(true);
    active.updated_at = Set(chrono::Utc::now());

    let verified = active.update(db).await.unwrap();
    assert!(verified.phone_verified);
}

// ===========================================================================
// Soft delete
// ===========================================================================

#[tokio::test]
async fn test_user_soft_delete() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    let created = insert_test_user(db, "oscar@aeroxe.com", "+919000000015", "Oscar").await;
    assert!(created.deleted_at.is_none());

    use crate::modules::identity::domain::entities::user;

    let mut active: user::ActiveModel = created.into();
    active.deleted_at = Set(Some(chrono::Utc::now()));
    active.updated_at = Set(chrono::Utc::now());

    let deleted = active.update(db).await.unwrap();
    assert!(deleted.deleted_at.is_some(), "User should be soft-deleted");
}

// ===========================================================================
// Session management
// ===========================================================================

#[tokio::test]
async fn test_create_user_session() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    let user = insert_test_user(db, "pam@aeroxe.com", "+919000000016", "Pam").await;

    use crate::modules::identity::domain::entities::user_session;

    let now = chrono::Utc::now();
    let session = user_session::ActiveModel {
        user_id: Set(user.id),
        refresh_token_hash: Set("sha256_token_hash_abc123".to_string()),
        ip_address: Set(Some("192.168.1.100".to_string())),
        user_agent: Set(Some("Mozilla/5.0 Chrome/120".to_string())),
        expires_at: Set(now + chrono::Duration::days(30)),
        created_at: Set(now),
        ..Default::default()
    };

    let created = session.insert(db).await.expect("Failed to create session");
    assert!(created.id > 0);
    assert_eq!(created.user_id, user.id);
}

#[tokio::test]
async fn test_list_sessions_for_user() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    let user = insert_test_user(db, "quinn@aeroxe.com", "+919000000017", "Quinn").await;

    use crate::modules::identity::domain::entities::user_session;

    let now = chrono::Utc::now();

    // Create 3 sessions
    for i in 0..3 {
        let session = user_session::ActiveModel {
            user_id: Set(user.id),
            refresh_token_hash: Set(format!("token_hash_{}", i)),
            ip_address: Set(Some(format!("192.168.1.{}", 10 + i))),
            expires_at: Set(now + chrono::Duration::days(30)),
            created_at: Set(now),
            ..Default::default()
        };
        session.insert(db).await.expect("Failed to create session");
    }

    let sessions = user_session::Entity::find()
        .filter(user_session::Column::UserId.eq(user.id))
        .all(db)
        .await
        .expect("Failed to query sessions");

    assert_eq!(sessions.len(), 3, "Should find all 3 sessions");
}

#[tokio::test]
async fn test_delete_user_session() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    let user = insert_test_user(db, "rita@aeroxe.com", "+919000000018", "Rita").await;

    use crate::modules::identity::domain::entities::user_session;

    let session = user_session::ActiveModel {
        user_id: Set(user.id),
        refresh_token_hash: Set("token_to_delete".to_string()),
        expires_at: Set(chrono::Utc::now() + chrono::Duration::days(30)),
        created_at: Set(chrono::Utc::now()),
        ..Default::default()
    };

    let created = session.insert(db).await.unwrap();
    let session_id = created.id;

    // Delete the session
    user_session::Entity::delete_by_id(session_id)
        .exec(db)
        .await
        .expect("Failed to delete session");

    let found = user_session::Entity::find_by_id(session_id)
        .one(db)
        .await
        .expect("Query failed");
    assert!(found.is_none(), "Session should be deleted");
}

#[tokio::test]
async fn test_delete_all_sessions_for_user() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    let user = insert_test_user(db, "sam@aeroxe.com", "+919000000019", "Sam").await;

    use crate::modules::identity::domain::entities::user_session;

    let now = chrono::Utc::now();
    for i in 0..3 {
        let session = user_session::ActiveModel {
            user_id: Set(user.id),
            refresh_token_hash: Set(format!("token_{}", i)),
            expires_at: Set(now + chrono::Duration::days(30)),
            created_at: Set(now),
            ..Default::default()
        };
        session.insert(db).await.unwrap();
    }

    // Delete all sessions for this user
    user_session::Entity::delete_many()
        .filter(user_session::Column::UserId.eq(user.id))
        .exec(db)
        .await
        .expect("Failed to delete sessions");

    let remaining = user_session::Entity::find()
        .filter(user_session::Column::UserId.eq(user.id))
        .all(db)
        .await
        .unwrap();
    assert_eq!(remaining.len(), 0, "All sessions should be deleted");
}

// ===========================================================================
// Branch association
// ===========================================================================

#[tokio::test]
async fn test_user_with_branch() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    let branch_id = crate::common::TestFixture::create_branch(db).await;

    use crate::modules::identity::domain::entities::user;

    let now = chrono::Utc::now();
    let active = user::ActiveModel {
        email: Set("tina@aeroxe.com".to_string()),
        phone: Set("+919000000020".to_string()),
        password_hash: Set(Some("hash".to_string())),
        name: Set("Tina".to_string()),
        branch_id: Set(Some(branch_id)),
        status: Set("active".to_string()),
        failed_login_attempts: Set(0),
        two_factor_enabled: Set(false),
        phone_verified: Set(true),
        email_verified: Set(true),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    };

    let user = active.insert(db).await.unwrap();
    assert_eq!(user.branch_id, Some(branch_id));
}

#[tokio::test]
async fn test_user_update_avatar() {
    let test_db = TestDatabase::new().await;
    let db = test_db.connection();

    let created = insert_test_user(db, "uma@aeroxe.com", "+919000000021", "Uma").await;
    assert!(created.avatar_url.is_none());

    use crate::modules::identity::domain::entities::user;

    let mut active: user::ActiveModel = created.into();
    active.avatar_url = Set(Some("https://storage.aeroxe.com/avatars/uma.jpg".to_string()));
    active.updated_at = Set(chrono::Utc::now());

    let updated = active.update(db).await.unwrap();
    assert!(updated.avatar_url.is_some());
    assert!(updated.avatar_url.unwrap().contains("uma.jpg"));
}
