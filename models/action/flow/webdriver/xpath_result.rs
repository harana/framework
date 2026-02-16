// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct XpathResult {
    pub element_count: i64,
    pub error_message: String,
    #[serde(default = "chrono::Utc::now")]
    pub queried_at: chrono::DateTime<chrono::Utc>,
    pub results: String,
    pub session_id: String,
    pub status: String,
    pub xpath: String,
}

