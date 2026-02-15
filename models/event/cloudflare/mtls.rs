// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareMtlsFetchInvoked {
    pub certificate_binding: String,
    pub url: String,
    pub method: String,
    pub status_code: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub invoked_at: chrono::DateTime<chrono::Utc>,
}

impl CloudflareMtlsFetchInvoked {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareMtlsFetchFailed {
    pub certificate_binding: String,
    pub url: String,
    pub error_message: String,
    #[serde(default = "chrono::Utc::now")]
    pub failed_at: chrono::DateTime<chrono::Utc>,
}

impl CloudflareMtlsFetchFailed {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

