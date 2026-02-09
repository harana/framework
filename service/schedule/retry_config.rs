use crate::backoff_strategy::BackoffStrategy;
use serde::{Deserialize, Serialize};

/// Configuration for schedule retry behavior.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryConfig {
    pub max_retries: u32,
    pub initial_delay_secs: u64,
    pub max_delay_secs: u64,
    pub multiplier: f64,
    pub strategy: BackoffStrategy,
    pub jitter: bool,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            initial_delay_secs: 10,
            max_delay_secs: 3600,
            multiplier: 2.0,
            strategy: BackoffStrategy::Exponential,
            jitter: true,
        }
    }
}

impl RetryConfig {
    pub fn no_retry() -> Self {
        Self {
            max_retries: 0,
            ..Default::default()
        }
    }

    /// Calculate delay for a given retry attempt
    pub fn delay_for_attempt(&self, attempt: u32) -> std::time::Duration {
        if attempt == 0 {
            return std::time::Duration::from_secs(self.initial_delay_secs);
        }

        let base_delay = match self.strategy {
            BackoffStrategy::Fixed => self.initial_delay_secs as f64,
            BackoffStrategy::Linear => self.initial_delay_secs as f64 + (attempt as f64 * self.multiplier),
            BackoffStrategy::Exponential => self.initial_delay_secs as f64 * self.multiplier.powi(attempt as i32),
        };

        let delay_secs = base_delay.min(self.max_delay_secs as f64);

        let final_delay = if self.jitter {
            // Add up to 25% jitter
            let jitter_factor = 1.0 + (rand_simple() * 0.25);
            delay_secs * jitter_factor
        } else {
            delay_secs
        };

        std::time::Duration::from_secs_f64(final_delay)
    }
}

/// Simple pseudo-random for jitter (avoids external dependency)
fn rand_simple() -> f64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .subsec_nanos();
    (nanos % 1000) as f64 / 1000.0
}
