// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Flow {
    pub name: String,
    pub description: Option<String>,
    pub steps: Vec<String>,
}

impl Flow {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FlowStep {
    pub actor_id: Option<String>,
    pub content: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub feed_id: String,
    #[serde(default)]
    pub is_complete: bool,
    pub target_id: Option<String>,
    pub target_name: Option<String>,
    pub target_type: Option<String>,
    pub type: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl FlowStep {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

