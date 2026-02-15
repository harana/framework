// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct App {
    pub name: String,
    pub description: Option<String>,
    pub commands: std::collections::HashMap<String, String>,
    pub defaults: Option<String>,
}

impl App {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

