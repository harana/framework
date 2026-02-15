//! Error types for the application framework.

use std::fmt;

/// Application error type
#[derive(Debug)]
pub enum AppError {
    /// Configuration error
    ConfigError(String),
    
    /// HTTP service error
    HttpError(Box<dyn std::error::Error + Send + Sync>),
    
    /// Workflow error
    WorkflowError(String),
    
    /// Event error
    EventError(String),
    
    /// Schedule error
    ScheduleError(String),
    
    /// Runtime error
    RuntimeError(String),
    
    /// Lifecycle hook error
    LifecycleError(String),
    
    /// Generic error
    Other(Box<dyn std::error::Error + Send + Sync>),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::ConfigError(msg) => write!(f, "Configuration error: {}", msg),
            AppError::HttpError(err) => write!(f, "HTTP service error: {}", err),
            AppError::WorkflowError(msg) => write!(f, "Workflow error: {}", msg),
            AppError::EventError(msg) => write!(f, "Event error: {}", msg),
            AppError::ScheduleError(msg) => write!(f, "Schedule error: {}", msg),
            AppError::RuntimeError(msg) => write!(f, "Runtime error: {}", msg),
            AppError::LifecycleError(msg) => write!(f, "Lifecycle error: {}", msg),
            AppError::Other(err) => write!(f, "Error: {}", err),
        }
    }
}

impl std::error::Error for AppError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            AppError::HttpError(err) => Some(err.as_ref()),
            AppError::Other(err) => Some(err.as_ref()),
            _ => None,
        }
    }
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError::RuntimeError(err.to_string())
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::RuntimeError(err.to_string())
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::ConfigError(err.to_string())
    }
}
