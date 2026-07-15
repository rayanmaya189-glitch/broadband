use sea_orm::DatabaseConnection;
use tracing::info;

pub struct BillingWorker {
    #[allow(dead_code)]
    db: DatabaseConnection,
}

impl BillingWorker {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// Check for overdue invoices and update statuses.
    pub async fn check_overdue_invoices(&self) -> anyhow::Result<()> {
        info!("Billing worker: checking overdue invoices");
        // TODO: Implement overdue invoice checking
        Ok(())
    }

    /// Run dunning process (send reminders, suspend after grace period).
    pub async fn run_dunning(&self) -> anyhow::Result<()> {
        info!("Billing worker: running dunning process");
        // TODO: Implement dunning flow
        Ok(())
    }
}
