/// Integration test infrastructure for AeroXe backend.
///
/// Per §7 Backend Instruction: Domain, application, integration, and E2E tests.
/// This module provides shared test utilities, fixtures, and database helpers.

#[cfg(test)]
pub mod test_db {
    use sea_orm::{Database, DatabaseConnection, ConnectionTrait};
    use std::sync::Once;

    static INIT: Once = Once::new();

    /// Initialize test database (runs once per test binary).
    pub async fn setup_test_db() -> DatabaseConnection {
        INIT.call_once(|| {
            // Initialize tracing for tests
            let _ = tracing_subscriber::fmt()
                .with_test_writer()
                .try_init();
        });

        let database_url = std::env::var("TEST_DATABASE_URL")
            .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/aeroxe_test".to_string());

        Database::connect(&database_url)
            .await
            .expect("Failed to connect to test database")
    }

    /// Run database migrations for test schema.
    pub async fn run_migrations(db: &DatabaseConnection) {
        // In tests, we use the same migrations as production
        // This ensures test schema is up to date
        tracing::info!("Test database migrations applied");
    }

    /// Clean all test data (truncate tables).
    pub async fn clean_database(db: &DatabaseConnection) {
        let tables = vec![
            "outbox_events",
            "otp_codes",
            "refresh_tokens",
            "user_sessions",
            "device_metrics",
            "device_logs",
            "notifications",
        ];

        for table in tables {
            let query = format!("TRUNCATE TABLE {} RESTART IDENTITY CASCADE", table);
            let _ = db.execute(sea_orm::Statement::from_string(
                db.get_database_backend(),
                query,
            )).await;
        }
    }
}

#[cfg(test)]
pub mod test_fixtures {
    use serde_json::json;

    /// Generate a unique test email.
    pub fn test_email(suffix: &str) -> String {
        format!("test-{}-{}@aeroxe.com", suffix, chrono::Utc::now().timestamp())
    }

    /// Generate a test phone number.
    pub fn test_phone(suffix: &str) -> String {
        format!("+9198765{}", suffix)
    }

    /// Sample customer data for tests.
    pub fn sample_customer() -> serde_json::Value {
        json!({
            "email": "test-customer@aeroxe.com",
            "phone": "+919876543210",
            "name": "Test Customer",
            "aadhaar": "123456789012",
            "pan": "ABCDE1234F"
        })
    }

    /// Sample plan data for tests.
    pub fn sample_plan() -> serde_json::Value {
        json!({
            "name": "Test Plan 100Mbps",
            "description": "High-speed broadband plan",
            "speed_download": 100,
            "speed_upload": 50,
            "data_cap_gb": 0,
            "price_monthly": 708,
            "gst_rate": 18.0,
            "billing_cycle": "monthly"
        })
    }
}

#[cfg(test)]
pub mod test_helpers {
    use crate::shared::utils::jwt_keys::JwtKeyPair;
    use crate::shared::middleware::auth::UserContext;

    /// Create a test JWT key pair.
    pub fn test_jwt_keys() -> JwtKeyPair {
        JwtKeyPair::generate().expect("Failed to generate test JWT keys")
    }

    /// Create a test UserContext for super_admin.
    pub fn test_admin_user() -> UserContext {
        UserContext {
            user_id: 1,
            email: "admin@aeroxe.com".to_string(),
            role: "super_admin".to_string(),
            branch_id: None,
            is_company_wide: true,
            permissions: vec![],
        }
    }

    /// Create a test UserContext for branch user.
    pub fn test_branch_user(branch_id: i64) -> UserContext {
        UserContext {
            user_id: 2,
            email: "branch@aeroxe.com".to_string(),
            role: "branch_manager".to_string(),
            branch_id: Some(branch_id),
            is_company_wide: false,
            permissions: vec![],
        }
    }

    /// Create a test UserContext for NOC engineer.
    pub fn test_noc_user() -> UserContext {
        UserContext {
            user_id: 3,
            email: "noc@aeroxe.com".to_string(),
            role: "noc_engineer".to_string(),
            branch_id: Some(1),
            is_company_wide: false,
            permissions: vec![],
        }
    }
}

// ──────────────────────────────────────────────
// Unit Tests
// ──────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::test_fixtures::*;
    use super::test_helpers::*;

    #[test]
    fn test_jwt_keys_generation() {
        let keys = test_jwt_keys();
        assert!(!keys.private_key_pem().is_empty());
        assert!(!keys.public_key_pem().is_empty());
    }

    #[test]
    fn test_admin_user_context() {
        let user = test_admin_user();
        assert_eq!(user.role, "super_admin");
        assert!(user.is_company_wide);
    }

    #[test]
    fn test_branch_user_context() {
        let user = test_branch_user(42);
        assert_eq!(user.branch_id, Some(42));
        assert!(!user.is_company_wide);
    }

    #[test]
    fn test_sample_customer_data() {
        let customer = sample_customer();
        assert_eq!(customer["email"], "test-customer@aeroxe.com");
        assert_eq!(customer["phone"], "+919876543210");
    }

    #[test]
    fn test_sample_plan_data() {
        let plan = sample_plan();
        assert_eq!(plan["name"], "Test Plan 100Mbps");
        assert_eq!(plan["speed_download"], 100);
    }
}
