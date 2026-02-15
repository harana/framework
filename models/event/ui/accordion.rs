// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AccordionExpanded {
    pub accordion_id: String,
    pub section_id: String,
    pub section_title: Option<String>,
    pub section_index: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub expanded_at: chrono::DateTime<chrono::Utc>,
}

impl AccordionExpanded {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AccordionCollapsed {
    pub accordion_id: String,
    pub section_id: String,
    pub section_title: Option<String>,
    pub section_index: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub collapsed_at: chrono::DateTime<chrono::Utc>,
}

impl AccordionCollapsed {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AccordionSectionClicked {
    pub accordion_id: String,
    pub section_id: String,
    pub section_title: Option<String>,
    pub section_index: Option<i64>,
    #[serde(default)]
    pub was_expanded: bool,
    #[serde(default = "chrono::Utc::now")]
    pub clicked_at: chrono::DateTime<chrono::Utc>,
}

impl AccordionSectionClicked {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

