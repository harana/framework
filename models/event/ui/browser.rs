// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BrowserVisibilityHidden {
    pub session_id: String,
    pub page_url: String,
    #[serde(default = "chrono::Utc::now")]
    pub hidden_at: chrono::DateTime<chrono::Utc>,
}

impl BrowserVisibilityHidden {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BrowserVisibilityVisible {
    pub session_id: String,
    pub page_url: String,
    pub hidden_duration_ms: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub visible_at: chrono::DateTime<chrono::Utc>,
}

impl BrowserVisibilityVisible {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BrowserPageHide {
    pub session_id: String,
    pub page_url: String,
    #[serde(default)]
    pub persisted: bool,
    #[serde(default = "chrono::Utc::now")]
    pub hidden_at: chrono::DateTime<chrono::Utc>,
}

impl BrowserPageHide {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BrowserPageShow {
    pub session_id: String,
    pub page_url: String,
    #[serde(default)]
    pub persisted: bool,
    #[serde(default = "chrono::Utc::now")]
    pub shown_at: chrono::DateTime<chrono::Utc>,
}

impl BrowserPageShow {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BrowserBeforeUnload {
    pub session_id: String,
    pub page_url: String,
    #[serde(default = "chrono::Utc::now")]
    pub triggered_at: chrono::DateTime<chrono::Utc>,
}

impl BrowserBeforeUnload {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BrowserOnline {
    pub session_id: String,
    pub page_url: String,
    pub offline_duration_ms: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub online_at: chrono::DateTime<chrono::Utc>,
}

impl BrowserOnline {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BrowserOffline {
    pub session_id: String,
    pub page_url: String,
    #[serde(default = "chrono::Utc::now")]
    pub offline_at: chrono::DateTime<chrono::Utc>,
}

impl BrowserOffline {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BrowserException {
    pub session_id: String,
    pub page_url: String,
    pub error_message: String,
    pub error_type: Option<String>,
    pub stack_trace: Option<String>,
    pub filename: Option<String>,
    pub line_number: Option<i64>,
    pub column_number: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub occurred_at: chrono::DateTime<chrono::Utc>,
}

impl BrowserException {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BrowserPromiseRejection {
    pub session_id: String,
    pub page_url: String,
    pub reason: String,
    pub promise_id: Option<String>,
    pub stack_trace: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub occurred_at: chrono::DateTime<chrono::Utc>,
}

impl BrowserPromiseRejection {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BrowserLcp {
    pub session_id: String,
    pub page_url: String,
    pub value_ms: f64,
    pub element: Option<String>,
    pub element_id: Option<String>,
    pub rating: String,
    #[serde(default = "chrono::Utc::now")]
    pub recorded_at: chrono::DateTime<chrono::Utc>,
}

impl BrowserLcp {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BrowserFid {
    pub session_id: String,
    pub page_url: String,
    pub value_ms: f64,
    pub event_type: Option<String>,
    pub event_target: Option<String>,
    pub rating: String,
    #[serde(default = "chrono::Utc::now")]
    pub recorded_at: chrono::DateTime<chrono::Utc>,
}

impl BrowserFid {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BrowserCls {
    pub session_id: String,
    pub page_url: String,
    pub value: f64,
    pub sources: Option<String>,
    pub rating: String,
    #[serde(default = "chrono::Utc::now")]
    pub recorded_at: chrono::DateTime<chrono::Utc>,
}

impl BrowserCls {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BrowserLongTask {
    pub session_id: String,
    pub page_url: String,
    pub duration_ms: f64,
    pub start_time_ms: Option<f64>,
    pub attribution: Option<String>,
    pub container_type: Option<String>,
    pub container_id: Option<String>,
    pub container_name: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub occurred_at: chrono::DateTime<chrono::Utc>,
}

impl BrowserLongTask {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BrowserResize {
    pub session_id: String,
    pub page_url: String,
    pub width: i64,
    pub height: i64,
    pub previous_width: Option<i64>,
    pub previous_height: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub resized_at: chrono::DateTime<chrono::Utc>,
}

impl BrowserResize {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BrowserOrientationChange {
    pub session_id: String,
    pub page_url: String,
    pub orientation: String,
    pub angle: Option<i64>,
    pub previous_orientation: String,
    #[serde(default = "chrono::Utc::now")]
    pub changed_at: chrono::DateTime<chrono::Utc>,
}

impl BrowserOrientationChange {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BrowserFullscreenEnter {
    pub session_id: String,
    pub page_url: String,
    pub element: Option<String>,
    pub element_id: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub entered_at: chrono::DateTime<chrono::Utc>,
}

impl BrowserFullscreenEnter {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BrowserFullscreenExit {
    pub session_id: String,
    pub page_url: String,
    pub duration_ms: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub exited_at: chrono::DateTime<chrono::Utc>,
}

impl BrowserFullscreenExit {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BrowserStorageChange {
    pub session_id: String,
    pub page_url: String,
    pub storage_type: String,
    pub key: String,
    pub old_value: Option<String>,
    pub new_value: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub changed_at: chrono::DateTime<chrono::Utc>,
}

impl BrowserStorageChange {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BrowserFocusGained {
    pub session_id: String,
    pub page_url: String,
    pub blur_duration_ms: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub focused_at: chrono::DateTime<chrono::Utc>,
}

impl BrowserFocusGained {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BrowserFocusLost {
    pub session_id: String,
    pub page_url: String,
    #[serde(default = "chrono::Utc::now")]
    pub lost_at: chrono::DateTime<chrono::Utc>,
}

impl BrowserFocusLost {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BrowserIdleStart {
    pub session_id: String,
    pub page_url: String,
    pub idle_threshold_ms: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub started_at: chrono::DateTime<chrono::Utc>,
}

impl BrowserIdleStart {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BrowserIdleEnd {
    pub session_id: String,
    pub page_url: String,
    pub idle_duration_ms: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub ended_at: chrono::DateTime<chrono::Utc>,
}

impl BrowserIdleEnd {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BrowserBack {
    pub session_id: String,
    pub page_url: String,
    pub previous_url: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub navigated_at: chrono::DateTime<chrono::Utc>,
}

impl BrowserBack {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BrowserForward {
    pub session_id: String,
    pub page_url: String,
    pub next_url: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub navigated_at: chrono::DateTime<chrono::Utc>,
}

impl BrowserForward {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BrowserScrollDepth25 {
    pub session_id: String,
    pub page_url: String,
    pub scroll_top: Option<i64>,
    pub page_height: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub reached_at: chrono::DateTime<chrono::Utc>,
}

impl BrowserScrollDepth25 {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BrowserScrollDepth50 {
    pub session_id: String,
    pub page_url: String,
    pub scroll_top: Option<i64>,
    pub page_height: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub reached_at: chrono::DateTime<chrono::Utc>,
}

impl BrowserScrollDepth50 {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BrowserScrollDepth75 {
    pub session_id: String,
    pub page_url: String,
    pub scroll_top: Option<i64>,
    pub page_height: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub reached_at: chrono::DateTime<chrono::Utc>,
}

impl BrowserScrollDepth75 {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BrowserScrollDepth100 {
    pub session_id: String,
    pub page_url: String,
    pub scroll_top: Option<i64>,
    pub page_height: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub reached_at: chrono::DateTime<chrono::Utc>,
}

impl BrowserScrollDepth100 {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BrowserElementVisible {
    pub session_id: String,
    pub page_url: String,
    pub element: String,
    pub element_id: Option<String>,
    pub element_class: Option<String>,
    pub visibility_ratio: Option<f64>,
    #[serde(default = "chrono::Utc::now")]
    pub visible_at: chrono::DateTime<chrono::Utc>,
}

impl BrowserElementVisible {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BrowserRageClick {
    pub session_id: String,
    pub page_url: String,
    pub element: Option<String>,
    pub element_id: Option<String>,
    pub element_class: Option<String>,
    pub click_count: i64,
    pub x: Option<i64>,
    pub y: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub detected_at: chrono::DateTime<chrono::Utc>,
}

impl BrowserRageClick {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BrowserMediaPlay {
    pub session_id: String,
    pub page_url: String,
    pub media_type: String,
    pub media_url: Option<String>,
    pub media_id: Option<String>,
    pub media_title: Option<String>,
    pub duration_seconds: Option<f64>,
    pub current_time_seconds: Option<f64>,
    #[serde(default = "chrono::Utc::now")]
    pub played_at: chrono::DateTime<chrono::Utc>,
}

impl BrowserMediaPlay {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BrowserMediaPause {
    pub session_id: String,
    pub page_url: String,
    pub media_type: String,
    pub media_url: Option<String>,
    pub media_id: Option<String>,
    pub media_title: Option<String>,
    pub duration_seconds: Option<f64>,
    pub current_time_seconds: Option<f64>,
    #[serde(default = "chrono::Utc::now")]
    pub paused_at: chrono::DateTime<chrono::Utc>,
}

impl BrowserMediaPause {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BrowserMediaEnded {
    pub session_id: String,
    pub page_url: String,
    pub media_type: String,
    pub media_url: Option<String>,
    pub media_id: Option<String>,
    pub media_title: Option<String>,
    pub duration_seconds: Option<f64>,
    pub watch_duration_seconds: Option<f64>,
    #[serde(default = "chrono::Utc::now")]
    pub ended_at: chrono::DateTime<chrono::Utc>,
}

impl BrowserMediaEnded {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BrowserMediaProgress {
    pub session_id: String,
    pub page_url: String,
    pub media_type: String,
    pub media_url: Option<String>,
    pub media_id: Option<String>,
    pub media_title: Option<String>,
    pub duration_seconds: Option<f64>,
    pub current_time_seconds: Option<f64>,
    pub progress_percent: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub recorded_at: chrono::DateTime<chrono::Utc>,
}

impl BrowserMediaProgress {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

