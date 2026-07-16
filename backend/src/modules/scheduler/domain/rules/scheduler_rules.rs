/// Scheduler business rules and invariants
pub struct SchedulerRules;

impl SchedulerRules {
    /// Maximum concurrent jobs
    pub const MAX_CONCURRENT_JOBS: usize = 10;

    /// Job timeout (minutes)
    pub const JOB_TIMEOUT_MINUTES: u64 = 60;

    /// Maximum retries for failed jobs
    pub const MAX_RETRIES: u32 = 3;

    /// Common cron expressions
    pub const DAILY_1AM: &'static str = "0 1 * * *";
    pub const HOURLY: &'static str = "0 * * * *";
    pub const EVERY_5_MINUTES: &'static str = "*/5 * * * *";

    /// Check if cron expression is valid format
    pub fn is_valid_cron(expr: &str) -> bool {
        let parts: Vec<&str> = expr.split_whitespace().collect();
        parts.len() == 5
    }
}
