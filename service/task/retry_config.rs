use serde::{Deserialize, Serialize};
use crate::backoff::BackoffStrategy;

/// Configuration for task retry behavior.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryConfig {
    pub max_retries: u32,
    pub retry_delay_secs: u64,
    pub max_delay_secs: u64,
    pub backoff_strategy: BackoffStrategy,
    pub retry_on_errors: Option<Vec<String>>,
    pub no_retry_on_errors: Option<Vec<String>>,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            retry_delay_secs: 5,
            max_delay_secs: 3600,
            backoff_strategy: BackoffStrategy::Exponential,
            retry_on_errors: None,
            no_retry_on_errors: None,
        }
    }
}

impl RetryConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn no_retries() -> Self {
        Self {
            max_retries: 0,
            ..Default::default()
        }
    }

    pub fn with_max_retries(mut self, max: u32) -> Self {
        self.max_retries = max;
        self
    }

    pub fn with_delay(mut self, secs: u64) -> Self {
        self.retry_delay_secs = secs;
        self
    }

    pub fn with_max_delay(mut self, secs: u64) -> Self {
        self.max_delay_secs = secs;
        self
    }

    pub fn with_backoff(mut self, strategy: BackoffStrategy) -> Self {
        self.backoff_strategy = strategy;
        self
    }

    pub fn calculate_delay(&self, attempt: u32) -> u64 {
        let base_delay = self.retry_delay_secs;
        let delay = match self.backoff_strategy {
            BackoffStrategy::Fixed => base_delay,
            BackoffStrategy::Linear => base_delay * (attempt as u64 + 1),
            BackoffStrategy::Exponential => base_delay * 2u64.pow(attempt),
        };
        delay.min(self.max_delay_secs)
    }
}
