// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// parallel_execution
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ParallelExecution {
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub completed_count: i64,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub failed_count: i64,
    pub max_concurrency: Option<i64>,
    pub status: String,
    pub strategy: String,
    pub timeout_ms: Option<i64>,
    pub total_count: i64,
}

impl ParallelExecution {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// parallel_task
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ParallelTask {
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub error_message: Option<String>,
    /// Reference: parallel_execution.id
    pub execution_id: String,
    pub result: Option<String>,
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    pub status: String,
    pub task_index: i64,
}

impl ParallelTask {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// parallel_result
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ParallelResult {
    pub task_id: String,
    pub success: bool,
    pub result: String,
    pub error: String,
    pub duration_ms: i64,
}

impl ParallelResult {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// parallel_settled_result
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ParallelSettledResult {
    pub task_id: String,
    pub status: String,
    pub value: String,
    pub reason: String,
    pub duration_ms: i64,
}

impl ParallelSettledResult {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// parallel_error
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ParallelError {
    pub task_id: String,
    pub index: i64,
    pub message: String,
    pub code: String,
}

impl ParallelError {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

