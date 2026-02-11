use std::time::{Duration, Instant};

#[derive(Debug, Clone, PartialEq)]
pub enum CircuitBreakerState {
    Closed,
    Open,
    HalfOpen,
}

#[derive(Debug)]
pub struct CircuitBreaker {
    state: CircuitBreakerState,
    failures: u32,
    last_failure: Option<Instant>,
    failure_threshold: u32,
    cooldown: Duration,
}

impl CircuitBreaker {
    pub fn new(failure_threshold: u32, cooldown: Duration) -> Self {
        Self {
            state: CircuitBreakerState::Closed,
            failures: 0,
            last_failure: None,
            failure_threshold,
            cooldown,
        }
    }

    /// Checks if a request is allowed.
    /// Manages state transitions based on cooldown.
    pub fn check(&mut self) -> bool {
        match self.state {
            CircuitBreakerState::Closed => true,
            CircuitBreakerState::Open => {
                if let Some(last) = self.last_failure {
                    if last.elapsed() >= self.cooldown {
                        // Cooldown passed, try one request (Half-Open)
                        self.state = CircuitBreakerState::HalfOpen;
                        return true;
                    }
                }
                false
            }
            CircuitBreakerState::HalfOpen => {
                // In Half-Open state, we only allow one request at a time.
                // The first request transitioned from Open -> HalfOpen and got 'true'.
                // Subsequent checks while still in HalfOpen (meaning the first request hasn't finished)
                // should return false to prevent flooding.
                false
            }
        }
    }

    pub fn report_success(&mut self) {
        if self.state == CircuitBreakerState::HalfOpen {
            self.state = CircuitBreakerState::Closed;
            self.failures = 0;
            self.last_failure = None;
        } else if self.state == CircuitBreakerState::Closed {
            // Also reset failures on success in Closed state
            self.failures = 0;
        }
        // If Open, unexpected success? Keep it open or reset?
        // Usually means a race condition where a request started before it opened finished now.
        // We can arguably close it, or ignore. Safer to ignore or let HalfOpen handle recovery.
    }

    pub fn report_failure(&mut self) {
        match self.state {
            CircuitBreakerState::Closed => {
                self.failures += 1;
                if self.failures >= self.failure_threshold {
                    self.state = CircuitBreakerState::Open;
                    self.last_failure = Some(Instant::now());
                }
            }
            CircuitBreakerState::HalfOpen => {
                // If it fails in HalfOpen, go back to Open immediately
                self.state = CircuitBreakerState::Open;
                self.last_failure = Some(Instant::now());
            }
            CircuitBreakerState::Open => {
                // Update timestamp to extend cooldown
                self.last_failure = Some(Instant::now());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circuit_breaker_flow() {
        let mut cb = CircuitBreaker::new(2, Duration::from_millis(100));

        // Closed initially
        assert!(cb.check());
        cb.report_success();
        assert!(cb.check());

        // 1st failure
        cb.report_failure();
        assert!(cb.check()); // Still closed (threshold is 2) - failure count is 1
        assert_eq!(cb.state, CircuitBreakerState::Closed);

        // 2nd failure
        cb.report_failure(); // failure count is 2 -> Open
        assert_eq!(cb.state, CircuitBreakerState::Open);
        assert!(!cb.check()); // Now open

        // Wait for cooldown
        std::thread::sleep(Duration::from_millis(150));
        assert!(cb.check()); // Should be HalfOpen now (transitioned in check)
        assert_eq!(cb.state, CircuitBreakerState::HalfOpen);

        // Subsequent check in HalfOpen should fail (limit 1)
        assert!(!cb.check());

        // Success in HalfOpen -> Closed
        cb.report_success();
        assert_eq!(cb.state, CircuitBreakerState::Closed);
        assert_eq!(cb.failures, 0);

        // Fail again to Open
        cb.report_failure();
        cb.report_failure();
        assert_eq!(cb.state, CircuitBreakerState::Open);

        // Wait for cooldown
        std::thread::sleep(Duration::from_millis(150));
        assert!(cb.check()); // HalfOpen

        // Failure in HalfOpen -> Open
        cb.report_failure();
        assert_eq!(cb.state, CircuitBreakerState::Open);
        assert!(!cb.check());
    }
}
