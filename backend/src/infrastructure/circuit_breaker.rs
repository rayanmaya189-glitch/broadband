use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;
use std::time::{Duration, Instant};
use tracing::{info, warn};

/// Circuit breaker state for external service calls.
///
/// States:
/// - **Closed** (normal): requests pass through. Failures are counted.
/// - **Open** (tripped): requests are rejected immediately. After `reset_timeout`,
///   transitions to Half-Open.
/// - **Half-Open**: one probe request is allowed through.
///   - Success → Closed
///   - Failure → Open
#[derive(Debug)]
pub struct CircuitBreaker {
    name: String,
    failure_threshold: u64,
    reset_timeout: Duration,
    state: AtomicU64,       // 0=closed, 1=open, 2=half-open
    failure_count: AtomicU64,
    last_failure: Mutex<Option<Instant>>,
}

/// Result of a circuit breaker check.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CircuitState {
    Closed,
    Open,
    HalfOpen,
}

impl CircuitBreaker {
    pub fn new(name: impl Into<String>, failure_threshold: u64, reset_timeout: Duration) -> Self {
        Self {
            name: name.into(),
            failure_threshold,
            reset_timeout,
            state: AtomicU64::new(0),
            failure_count: AtomicU64::new(0),
            last_failure: Mutex::new(None),
        }
    }

    /// Check if a request should be allowed.
    pub fn can_proceed(&self) -> bool {
        let state = self.current_state();
        match state {
            CircuitState::Closed => true,
            CircuitState::Open => {
                // Check if reset timeout has elapsed
                let last = *self.last_failure.lock().unwrap_or_else(|e| e.into_inner());
                if let Some(last) = last {
                    if last.elapsed() >= self.reset_timeout {
                        self.state.store(2, Ordering::Relaxed); // half-open
                        info!(name = %self.name, "Circuit breaker: transitioning to half-open");
                        return true;
                    }
                }
                false
            }
            CircuitState::HalfOpen => true, // Allow one probe
        }
    }

    /// Record a successful call.
    pub fn record_success(&self) {
        let prev = self.state.swap(0, Ordering::Relaxed);
        self.failure_count.store(0, Ordering::Relaxed);
        if prev != 0 {
            info!(name = %self.name, "Circuit breaker: closed (recovered)");
        }
    }

    /// Record a failed call.
    pub fn record_failure(&self) {
        let count = self.failure_count.fetch_add(1, Ordering::Relaxed) + 1;
        *self.last_failure.lock().unwrap_or_else(|e| e.into_inner()) = Some(Instant::now());

        if count >= self.failure_threshold {
            let prev = self.state.swap(1, Ordering::Relaxed); // open
            if prev != 1 {
                warn!(
                    name = %self.name,
                    failures = count,
                    "Circuit breaker: OPEN (tripped)"
                );
            }
        }
    }

    pub fn current_state(&self) -> CircuitState {
        match self.state.load(Ordering::Relaxed) {
            0 => CircuitState::Closed,
            1 => CircuitState::Open,
            _ => CircuitState::HalfOpen,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn circuit_breaker_starts_closed() {
        let cb = CircuitBreaker::new("test", 3, Duration::from_secs(30));
        assert!(cb.can_proceed());
        assert_eq!(cb.current_state(), CircuitState::Closed);
    }

    #[test]
    fn circuit_breaker_trips_after_threshold() {
        let cb = CircuitBreaker::new("test", 3, Duration::from_secs(30));
        cb.record_failure();
        cb.record_failure();
        assert!(cb.can_proceed()); // 2 failures, still closed
        cb.record_failure();
        assert!(!cb.can_proceed()); // 3 failures, open
        assert_eq!(cb.current_state(), CircuitState::Open);
    }

    #[test]
    fn circuit_breaker_resets_on_success() {
        let cb = CircuitBreaker::new("test", 3, Duration::from_secs(30));
        cb.record_failure();
        cb.record_success();
        assert_eq!(cb.failure_count.load(Ordering::Relaxed), 0);
        assert_eq!(cb.current_state(), CircuitState::Closed);
    }

    #[test]
    fn circuit_breaker_half_open_after_timeout() {
        let cb = CircuitBreaker::new("test", 1, Duration::from_millis(10));
        cb.record_failure();
        assert!(!cb.can_proceed());
        std::thread::sleep(Duration::from_millis(20));
        assert!(cb.can_proceed()); // half-open
    }
}
