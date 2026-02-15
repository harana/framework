// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StepCompleted {
    pub wizard_id: String,
    pub step_id: String,
    pub step_number: i64,
    pub step_name: Option<String>,
    pub total_steps: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub completed_at: chrono::DateTime<chrono::Utc>,
}

impl StepCompleted {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StepSkipped {
    pub wizard_id: String,
    pub step_id: String,
    pub step_number: i64,
    pub step_name: Option<String>,
    pub skip_reason: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub skipped_at: chrono::DateTime<chrono::Utc>,
}

impl StepSkipped {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StepNavigated {
    pub wizard_id: String,
    pub from_step: i64,
    pub to_step: i64,
    pub navigation_type: String,
    #[serde(default = "chrono::Utc::now")]
    pub navigated_at: chrono::DateTime<chrono::Utc>,
}

impl StepNavigated {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WizardCompleted {
    pub wizard_id: String,
    pub wizard_name: Option<String>,
    pub total_steps: Option<i64>,
    pub steps_skipped: Option<i64>,
    pub completion_time_ms: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub completed_at: chrono::DateTime<chrono::Utc>,
}

impl WizardCompleted {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WizardCancelled {
    pub wizard_id: String,
    pub wizard_name: Option<String>,
    pub cancelled_at_step: Option<i64>,
    pub total_steps: Option<i64>,
    pub cancel_reason: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub cancelled_at: chrono::DateTime<chrono::Utc>,
}

impl WizardCancelled {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

