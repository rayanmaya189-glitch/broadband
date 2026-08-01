//! Common test utilities for integration tests
//! Provides testcontainers setup for PostgreSQL (with PostGIS, required by migration 001)
//! and applies the full migration chain (001-022) so tests run against the real schema.

use sea_orm::{DatabaseConnection, Database};
use sea_orm_migration::prelude::*;
use testcontainers::core::{IntoContainerPort, WaitFor};
use testcontainers::{runners::AsyncRunner, ContainerAsync, GenericImage, ImageExt};

/// Test database container
pub struct TestDatabase {
    #[allow(dead_code)]
    pub container: ContainerAsync<GenericImage>,
    #[allow(dead_code)]
    pub connection_string: String,
    pub db: DatabaseConnection,
}

impl TestDatabase {
    /// Create a new test database using testcontainers and apply all migrations
    pub async fn new() -> Self {
        let container = GenericImage::new("postgis/postgis", "16-3.4")
            .with_env_var("POSTGRES_DB", "aeroxe_test")
            .with_env_var("POSTGRES_USER", "test_user")
            .with_env_var("POSTGRES_PASSWORD", "test_password")
            .with_mapped_port(0, 5432.tcp())
            .with_ready_conditions(vec![WaitFor::message_on_stdout(
                "database system is ready to accept connections",
            )])
            .start()
            .await
            .expect("Failed to start PostgreSQL container");

        let host = container
            .get_host_port_ipv4(5432)
            .await
            .expect("Failed to get host port");

        let connection_string = format!(
            "postgres://test_user:test_password@127.0.0.1:{}/aeroxe_test",
            host
        );

        let db = Database::connect(&connection_string)
            .await
            .expect("Failed to connect to test database");

        Self::apply_migrations(&db).await;

        Self {
            container,
            connection_string,
            db,
        }
    }

    /// Apply the sea-orm Migrator chain (001-018) plus the raw SQL migrations
    /// 019-022 (schema movement + missing tables + FKs/indexes + GST columns).
    async fn apply_migrations(db: &DatabaseConnection) {
        aeroxe_backend::migration::Migrator::up(db, None)
            .await
            .expect("Failed to apply sea-orm migrations 001-018");

        let manager = SchemaManager::new(db);
        let raw: [(&str, &str); 5] = [
            (
                "018_create_schemas",
                include_str!("../../migrations/sql/018_create_schemas.sql"),
            ),
            (
                "019_move_tables_to_schemas",
                include_str!("../../migrations/sql/019_move_tables_to_schemas.sql"),
            ),
            (
                "020_create_missing_tables",
                include_str!("../../migrations/sql/020_create_missing_tables.sql"),
            ),
            (
                "021_add_missing_fk_constraints_and_indexes",
                include_str!("../../migrations/sql/021_add_missing_fk_constraints_and_indexes.sql"),
            ),
            (
                "022_add_gst_tax_breakdown",
                include_str!("../../migrations/sql/022_add_gst_tax_breakdown.sql"),
            ),
        ];
        for (name, sql) in raw {
            aeroxe_backend::migration::exec_sql_file(&manager, sql)
                .await
                .unwrap_or_else(|e| panic!("Raw migration {} failed: {}", name, e));
        }

        Self::seed_system_users(db).await;
    }

    /// Seed system users (ids 1-5) so tests that reference fixed user ids
    /// (e.g. audit logs with user_id 1..5) satisfy the users FK constraint.
    async fn seed_system_users(db: &DatabaseConnection) {
        use sea_orm::{ConnectionTrait, Statement};

        let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S");
        let sql = format!(
            "INSERT INTO identity.users (email, phone, name, status, created_at, updated_at)
             VALUES
                ('system.1@aeroxe.test', '+910000000001', 'System One',   'active', '{now}', '{now}'),
                ('system.2@aeroxe.test', '+910000000002', 'System Two',   'active', '{now}', '{now}'),
                ('system.3@aeroxe.test', '+910000000003', 'System Three', 'active', '{now}', '{now}'),
                ('system.4@aeroxe.test', '+910000000004', 'System Four',  'active', '{now}', '{now}'),
                ('system.5@aeroxe.test', '+910000000005', 'System Five',  'active', '{now}', '{now}')"
        );
        db.execute(Statement::from_string(db.get_database_backend(), sql))
            .await
            .expect("Failed to seed system users");
    }

    /// Get a reference to the database connection
    pub fn connection(&self) -> &DatabaseConnection {
        &self.db
    }
}

/// Test fixture for creating test data
pub struct TestFixture;

impl TestFixture {
    /// Create a test branch using raw SQL
    pub async fn create_branch(db: &DatabaseConnection) -> i64 {
        use sea_orm::{ConnectionTrait, Statement};

        let now = chrono::Utc::now();
        let result = db
            .execute(Statement::from_string(
                db.get_database_backend(),
                format!(
                    "INSERT INTO branches.branches (name, slug, code, city, state, is_active, created_at, updated_at) 
                     VALUES ('Test Branch', 'test-branch-{}', 'TST-{}', 'Test City', 'Test State', true, '{}', '{}')
                     RETURNING id",
                    rand::random::<u32>(),
                    rand::random::<u32>(),
                    now.format("%Y-%m-%d %H:%M:%S"),
                    now.format("%Y-%m-%d %H:%M:%S")
                ),
            ))
            .await
            .expect("Failed to create test branch");

        // Extract the ID from the result
        match result.rows_affected() {
            _ => {
                // For simplicity, query the branch we just created
                let row = db
                    .query_one(Statement::from_string(
                        db.get_database_backend(),
                        "SELECT id FROM branches.branches WHERE slug LIKE 'test-branch-%' ORDER BY id DESC LIMIT 1"
                            .to_string(),
                    ))
                    .await
                    .expect("Failed to query branch");

                row.and_then(|r| r.try_get::<i64>("", "id").ok())
                    .unwrap_or(1)
            }
        }
    }

    /// Create a test customer using raw SQL
    pub async fn create_customer(db: &DatabaseConnection, branch_id: i64) -> i64 {
        use sea_orm::{ConnectionTrait, Statement};

        let now = chrono::Utc::now();
        let customer_code = format!("AX-TST-202607-{:04}", rand::random::<u16>() % 10000);

        let result = db
            .execute(Statement::from_string(
                db.get_database_backend(),
                format!(
                    "INSERT INTO customer.customers (customer_code, branch_id, name, phone, status, created_at, updated_at) 
                     VALUES ('{}', {}, 'Test Customer', '+919876543210', 'registered', '{}', '{}')
                     RETURNING id",
                    customer_code,
                    branch_id,
                    now.format("%Y-%m-%d %H:%M:%S"),
                    now.format("%Y-%m-%d %H:%M:%S")
                ),
            ))
            .await
            .expect("Failed to create test customer");

        match result.rows_affected() {
            _ => {
                let row = db
                    .query_one(Statement::from_string(
                        db.get_database_backend(),
                        format!(
                            "SELECT id FROM customer.customers WHERE customer_code = '{}' LIMIT 1",
                            customer_code
                        ),
                    ))
                    .await
                    .expect("Failed to query customer");

                row.and_then(|r| r.try_get::<i64>("", "id").ok())
                    .unwrap_or(1)
            }
        }
    }

    /// Create a test plan using raw SQL
    pub async fn create_plan(db: &DatabaseConnection) -> i64 {
        use sea_orm::{ConnectionTrait, Statement};

        let now = chrono::Utc::now();
        let slug = format!("test-plan-{}", rand::random::<u32>());

        let result = db
            .execute(Statement::from_string(
                db.get_database_backend(),
                format!(
                    "INSERT INTO plans.plans (slug, name, download_mbps, upload_mbps, is_active, created_at, updated_at) 
                     VALUES ('{}', 'Test Plan', 100, 50, true, '{}', '{}')
                     RETURNING id",
                    slug,
                    now.format("%Y-%m-%d %H:%M:%S"),
                    now.format("%Y-%m-%d %H:%M:%S")
                ),
            ))
            .await
            .expect("Failed to create test plan");

        match result.rows_affected() {
            _ => {
                let row = db
                    .query_one(Statement::from_string(
                        db.get_database_backend(),
                        format!(
                            "SELECT id FROM plans.plans WHERE slug = '{}' LIMIT 1",
                            slug
                        ),
                    ))
                    .await
                    .expect("Failed to query plan");

                row.and_then(|r| r.try_get::<i64>("", "id").ok())
                    .unwrap_or(1)
            }
        }
    }

    /// Create a test subscription (with its plan) using raw SQL
    #[allow(dead_code)]
    pub async fn create_subscription(
        db: &DatabaseConnection,
        customer_id: i64,
        branch_id: i64,
    ) -> i64 {
        use sea_orm::{ConnectionTrait, Statement};

        let plan_id = Self::create_plan(db).await;
        let now = chrono::Utc::now();
        let today = now.date_naive();

        let result = db
            .execute(Statement::from_string(
                db.get_database_backend(),
                format!(
                    "INSERT INTO subscription.subscriptions
                         (customer_id, branch_id, plan_id, status, billing_period_months, start_date, auto_renew, created_at, updated_at)
                     VALUES ({}, {}, {}, 'active', 1, '{}', true, '{}', '{}')
                     RETURNING id",
                    customer_id,
                    branch_id,
                    plan_id,
                    today.format("%Y-%m-%d"),
                    now.format("%Y-%m-%d %H:%M:%S"),
                    now.format("%Y-%m-%d %H:%M:%S")
                ),
            ))
            .await
            .expect("Failed to create test subscription");

        match result.rows_affected() {
            _ => {
                let row = db
                    .query_one(Statement::from_string(
                        db.get_database_backend(),
                        format!(
                            "SELECT id FROM subscription.subscriptions WHERE customer_id = {} ORDER BY id DESC LIMIT 1",
                            customer_id
                        ),
                    ))
                    .await
                    .expect("Failed to query subscription");

                row.and_then(|r| r.try_get::<i64>("", "id").ok())
                    .unwrap_or(1)
            }
        }
    }

    /// Create a test user using raw SQL
    pub async fn create_user(db: &DatabaseConnection, branch_id: i64) -> i64 {
        use sea_orm::{ConnectionTrait, Statement};

        let now = chrono::Utc::now();
        let email = format!("user-{}@aeroxe.test", rand::random::<u32>());
        let phone = format!("+91{:010}", rand::random::<u64>() % 10_000_000_000);

        let result = db
            .execute(Statement::from_string(
                db.get_database_backend(),
                format!(
                    "INSERT INTO identity.users (email, phone, name, branch_id, status, created_at, updated_at)
                     VALUES ('{}', '{}', 'Test User', {}, 'active', '{}', '{}')
                     RETURNING id",
                    email,
                    phone,
                    branch_id,
                    now.format("%Y-%m-%d %H:%M:%S"),
                    now.format("%Y-%m-%d %H:%M:%S")
                ),
            ))
            .await
            .expect("Failed to create test user");

        match result.rows_affected() {
            _ => {
                let row = db
                    .query_one(Statement::from_string(
                        db.get_database_backend(),
                        format!("SELECT id FROM identity.users WHERE email = '{}' LIMIT 1", email),
                    ))
                    .await
                    .expect("Failed to query user");

                row.and_then(|r| r.try_get::<i64>("", "id").ok())
                    .unwrap_or(1)
            }
        }
    }

    /// Create a test device model using raw SQL
    #[allow(dead_code)]
    pub async fn create_device_model(db: &DatabaseConnection) -> i64 {
        use sea_orm::{ConnectionTrait, Statement};

        let vendor = format!("vendor-{}", rand::random::<u32>());
        let model = format!("model-{}", rand::random::<u32>());

        let result = db
            .execute(Statement::from_string(
                db.get_database_backend(),
                format!(
                    "INSERT INTO device.device_models (vendor, model, device_type, management_protocol)
                     VALUES ('{}', '{}', 'olt', 'ssh')
                     RETURNING id",
                    vendor, model
                ),
            ))
            .await
            .expect("Failed to create test device model");

        match result.rows_affected() {
            _ => {
                let row = db
                    .query_one(Statement::from_string(
                        db.get_database_backend(),
                        format!(
                            "SELECT id FROM device.device_models WHERE vendor = '{}' AND model = '{}' LIMIT 1",
                            vendor, model
                        ),
                    ))
                    .await
                    .expect("Failed to query device model");

                row.and_then(|r| r.try_get::<i64>("", "id").ok())
                    .unwrap_or(1)
            }
        }
    }
}
