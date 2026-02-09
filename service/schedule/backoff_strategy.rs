use serde::{Deserialize, Serialize};

/// Strategy for calculating retry delays.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BackoffStrategy {
    Fixed,
    Linear,
    Exponential,
}

impl Default for BackoffStrategy {
    fn default() -> Self {
        Self::Exponential
    }
}
