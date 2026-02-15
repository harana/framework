// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TextClicked {
    pub text_id: String,
    pub text_content: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub clicked_at: chrono::DateTime<chrono::Utc>,
}

impl TextClicked {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TextLinkClicked {
    pub link_id: String,
    pub link_label: Option<String>,
    pub link_href: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub clicked_at: chrono::DateTime<chrono::Utc>,
}

impl TextLinkClicked {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TextSelected {
    pub text_id: String,
    pub selected_text: Option<String>,
    pub selection_start: Option<i64>,
    pub selection_end: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub selected_at: chrono::DateTime<chrono::Utc>,
}

impl TextSelected {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TextCopied {
    pub text_id: String,
    pub copied_text: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub copied_at: chrono::DateTime<chrono::Utc>,
}

impl TextCopied {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CodeCopied {
    pub code_id: String,
    pub code_content: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub copied_at: chrono::DateTime<chrono::Utc>,
}

impl CodeCopied {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

