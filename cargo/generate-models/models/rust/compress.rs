// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// compression_job
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CompressionJob {
    pub algorithm: String,
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub compressed_size: Option<i64>,
    pub compression_level: i64,
    pub compression_ratio: Option<f64>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub direction: String,
    pub error_message: Option<String>,
    pub original_size: Option<i64>,
    pub status: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl CompressionJob {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// compressed_data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CompressedData {
    pub algorithm: String,
    pub original_size: i64,
    pub compressed_size: i64,
    pub compression_ratio: f64,
    pub data: String,
}

impl CompressedData {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

