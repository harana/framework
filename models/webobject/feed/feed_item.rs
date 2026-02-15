// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FeedItemEvent {
    Click,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FeedItem {
    pub actor_avatar: Option<String>,
    pub actor_href: Option<String>,
    pub actor_name: String,
    pub content: Option<String>,
    pub feed_id: String,
    pub icon: Option<String>,
    #[serde(default)]
    pub is_complete: bool,
    pub order: i64,
    pub target_href: Option<String>,
    pub target_name: Option<String>,
    pub timestamp: Option<String>,
    pub type: String,
}

