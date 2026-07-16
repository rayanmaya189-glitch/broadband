//! Common test utilities for integration tests
//! Provides testcontainers setup for PostgreSQL

use sea_orm::{DatabaseConnection, Database};
use testcontainers::{runners::AsyncRunner, ContainerAsync, ImageExt};
use testcontainers_modules::postgres::Postgres;

/// Test database container
pub struct TestDatabase {
    #[allow(dead_code)]
    pub container: ContainerAsync<Postgres>,
    #[allow(dead_code)]
    pub connection_string: String,
    pub db: DatabaseConnection,
}

impl TestDatabase {
    /// Create a new test database using testcontainers
    pub async fn new() -> Self {
        let container = Postgres::default()
            .with_tag("16-alpine")
            .with_env("POSTGRES_DB", "aeroxe_test")
            .with_env("POSTGRES_USER", "test_user")
            .with_env("POSTGRES_PASSWORD", "test_password")
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

        // Wait a bit for PostgreSQL to be ready
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;

        let db = Database::connect(&connection_string)
            .await
            .expect("Failed to connect to test database");

        Self {
            container,
            connection_string,
            db,
        }
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
        use sea_orm::{Statement, ConnectionTrait};
        
        let now = chrono::Utc::now();
        let result = db.execute(Statement::from_string(
            db.get_database_backend(),
            format!(
                "INSERT INTO branches (name, slug, code, city, state, is_active, created_at, updated_at) 
                 VALUES ('Test Branch', 'test-branch-{}', 'TST', 'Test City', 'Test State', true, '{}', '{}')
                 RETURNING id",
                rand::random::<u32>(),
                now.format("%Y-%m-%d %H:%M:%S"),
                now.format("%Y-%m-%d %H:%M:%S")
            )
        )).await.expect("Failed to create test branch");
        
        // Extract the ID from the result
        match result.rows_affected() {
            _ => {
                // For simplicity, query the branch we just created
                let row = db.query_one(Statement::from_string(
                    db.get_database_backend(),
                    "SELECT id FROM branches WHERE slug LIKE 'test-branch-%' ORDER BY id DESC LIMIT 1".to_string(),
                )).await.expect("Failed to query branch");
                
                row.and_then(|r| r.try_get::<i64>("", "id").ok())
                    .unwrap_or(1)
            }
        }
    }

    /// Create a test customer using raw SQL
    pub async fn create_customer(db: &DatabaseConnection, branch_id: i64) -> i64 {
        use sea_orm::{Statement, ConnectionTrait};
        
        let now = chrono::Utc::now();
        let customer_code = format!("AX-TST-202607-{:04}", rand::random::<u16>() % 10000);
        
        let result = db.execute(Statement::from_string(
            db.get_database_backend(),
            format!(
                "INSERT INTO customers (customer_code, branch_id, name, phone, status, created_at, updated_at) 
                 VALUES ('{}', {}, 'Test Customer', '+919876543210', 'registered', '{}', '{}')
                 RETURNING id",
                customer_code,
                branch_id,
                now.format("%Y-%m-%d %H:%M:%S"),
                now.format("%Y-%m-%d %H:%M:%S")
            )
        )).await.expect("Failed to create test customer");
        
        match result.rows_affected() {
            _ => {
                let row = db.query_one(Statement::from_string(
                    db.get_database_backend(),
                    format!("SELECT id FROM customers WHERE customer_code = '{}' LIMIT 1", customer_code)
                )).await.expect("Failed to query customer");
                
                row.and_then(|r| r.try_get::<i64>("", "id").ok())
                    .unwrap_or(1)
            }
        }
    }

    /// Create a test plan using raw SQL
    pub async fn create_plan(db: &DatabaseConnection) -> i64 {
        use sea_orm::{Statement, ConnectionTrait};
        
        let now = chrono::Utc::now();
        let slug = format!("test-plan-{}", rand::random::<u32>());
        
        let result = db.execute(Statement::from_string(
            db.get_database_backend(),
            format!(
                "INSERT INTO plans (slug, name, download_mbps, upload_mbps, is_active, created_at, updated_at) 
                 VALUES ('{}', 'Test Plan', 100, 50, true, '{}', '{}')
                 RETURNING id",
                slug,
                now.format("%Y-%m-%d %H:%M:%S"),
                now.format("%Y-%m-%d %H:%M:%S")
            )
        )).await.expect("Failed to create test plan");
        
        match result.rows_affected() {
            _ => {
                let row = db.query_one(Statement::from_string(
                    db.get_database_backend(),
                    format!("SELECT id FROM plans WHERE slug = '{}' LIMIT 1", slug)
                )).await.expect("Failed to query plan");
                
                row.and_then(|r| r.try_get::<i64>("", "id").ok())
                    .unwrap_or(1)
            }
        }
    }
}
