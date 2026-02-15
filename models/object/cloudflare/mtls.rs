// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareMtlsCertificate {
    pub account_id: String,
    pub binding: String,
    pub certificate_name: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    pub issuer: Option<String>,
    pub serial_number: Option<String>,
    pub status: String,
    pub subject: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl CloudflareMtlsCertificate {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

