// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FeedItemLiked {
    pub feed_id: String,
    pub item_id: String,
    pub item_type: String,
    pub user_id: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub liked_at: chrono::DateTime<chrono::Utc>,
}

impl FeedItemLiked {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FeedItemCommented {
    pub feed_id: String,
    pub item_id: String,
    pub comment_id: String,
    pub item_type: String,
    pub user_id: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub commented_at: chrono::DateTime<chrono::Utc>,
}

impl FeedItemCommented {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FeedItemShared {
    pub feed_id: String,
    pub item_id: String,
    pub item_type: String,
    pub share_destination: String,
    pub user_id: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub shared_at: chrono::DateTime<chrono::Utc>,
}

impl FeedItemShared {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FeedScrolled {
    pub feed_id: String,
    pub scroll_direction: String,
    pub scroll_depth_percent: Option<i64>,
    pub items_viewed: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub scrolled_at: chrono::DateTime<chrono::Utc>,
}

impl FeedScrolled {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct LoadMoreClicked {
    pub feed_id: String,
    pub current_count: Option<i64>,
    pub page_number: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub clicked_at: chrono::DateTime<chrono::Utc>,
}

impl LoadMoreClicked {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

