// Harana Actions - Log Module
// This module provides log actions and functionality using tracing.

pub mod output;

use std::collections::HashMap;
use serde_json::Value;
use tracing::{debug as tracing_debug, error as tracing_error, info as tracing_info, warn as tracing_warn};
use output::*;

/// Format context as a string for logging
fn format_context(context: &Option<HashMap<String, Value>>) -> String {
    match context {
        Some(ctx) => serde_json::to_string(ctx).unwrap_or_default(),
        None => String::new(),
    }
}

/// Log Debug Message
pub async fn debug(
    message: &str,
    context: Option<HashMap<String, Value>>,
) -> Result<DebugOutput, String> {
    let ctx = format_context(&context);
    
    if ctx.is_empty() {
        tracing_debug!("{}", message);
    } else {
        tracing_debug!(context = %ctx, "{}", message);
    }
    
    Ok(DebugOutput { success: true })
}

/// Log Error Message
pub async fn error(
    message: &str,
    context: Option<HashMap<String, Value>>,
    error_info: Option<HashMap<String, Value>>,
) -> Result<ErrorOutput, String> {
    let ctx = format_context(&context);
    let err = format_context(&error_info);
    
    match (ctx.is_empty(), err.is_empty()) {
        (true, true) => tracing_error!("{}", message),
        (false, true) => tracing_error!(context = %ctx, "{}", message),
        (true, false) => tracing_error!(error = %err, "{}", message),
        (false, false) => tracing_error!(context = %ctx, error = %err, "{}", message),
    }
    
    Ok(ErrorOutput { success: true })
}

/// Log Info Message
pub async fn info(
    message: &str,
    context: Option<HashMap<String, Value>>,
) -> Result<InfoOutput, String> {
    let ctx = format_context(&context);
    
    if ctx.is_empty() {
        tracing_info!("{}", message);
    } else {
        tracing_info!(context = %ctx, "{}", message);
    }
    
    Ok(InfoOutput { success: true })
}

/// Log Structured Data
pub async fn structured(
    level: &str,
    data: HashMap<String, Value>,
) -> Result<StructuredOutput, String> {
    let json = serde_json::to_string(&data).unwrap_or_default();
    
    match level.to_lowercase().as_str() {
        "debug" => tracing_debug!(data = %json, "structured log"),
        "info" => tracing_info!(data = %json, "structured log"),
        "warn" | "warning" => tracing_warn!(data = %json, "structured log"),
        "error" => tracing_error!(data = %json, "structured log"),
        _ => tracing_info!(data = %json, "structured log"),
    }
    
    Ok(StructuredOutput { success: true })
}

/// Log Warning Message
pub async fn warn(
    message: &str,
    context: Option<HashMap<String, Value>>,
) -> Result<WarnOutput, String> {
    let ctx = format_context(&context);
    
    if ctx.is_empty() {
        tracing_warn!("{}", message);
    } else {
        tracing_warn!(context = %ctx, "{}", message);
    }
    
    Ok(WarnOutput { success: true })
}

#[cfg(test)]
mod tests;
