// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsSqsQueue {
    pub account_id: String,
    #[serde(default)]
    pub content_based_deduplication: bool,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub dead_letter_target_arn: Option<String>,
    pub delay_seconds: i64,
    #[serde(default)]
    pub is_fifo: bool,
    pub max_message_size: i64,
    pub message_retention_seconds: i64,
    pub queue_arn: Option<String>,
    pub queue_name: String,
    pub queue_url: Option<String>,
    pub receive_wait_time_seconds: i64,
    pub redrive_max_receive_count: Option<i64>,
    pub region: Option<String>,
    pub tags: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub visibility_timeout: i64,
}

impl AwsSqsQueue {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsSqsMessage {
    pub body: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub delay_seconds: Option<i64>,
    pub message_attributes: Option<String>,
    pub message_deduplication_id: Option<String>,
    pub message_group_id: Option<String>,
    pub message_id: String,
    pub queue_id: String,
    pub receipt_handle: Option<String>,
    pub receive_count: i64,
    pub sequence_number: Option<String>,
    pub status: String,
    #[serde(default = "chrono::Utc::now")]
    pub visible_at: chrono::DateTime<chrono::Utc>,
}

impl AwsSqsMessage {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsSqsQueuePermission {
    pub actions: String,
    pub aws_account_ids: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub label: String,
    pub queue_id: String,
}

impl AwsSqsQueuePermission {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

