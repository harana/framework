// Harana Components - Events Error Types

use std::fmt;

#[derive(Debug, Clone)]
pub enum EventError {
    NotFound { event_id: String },
    ChannelNotFound { channel: String },
    SubscriptionNotFound { subscription_id: String },
    AlreadyAcknowledged { event_id: String },
    DuplicateEvent { event_id: String },
    SerializationError(String),
    StorageError(String),
    ChannelFull { channel: String },
    ChannelClosed { channel: String },
    InvalidFilter(String),
    Timeout,
    InternalError(String),
}

impl fmt::Display for EventError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound { event_id } => write!(f, "Event with id '{}' not found", event_id),
            Self::ChannelNotFound { channel } => write!(f, "Channel '{}' not found", channel),
            Self::SubscriptionNotFound { subscription_id } => {
                write!(f, "Subscription '{}' not found", subscription_id)
            }
            Self::AlreadyAcknowledged { event_id } => {
                write!(f, "Event '{}' already acknowledged", event_id)
            }
            Self::DuplicateEvent { event_id } => write!(f, "Duplicate event '{}'", event_id),
            Self::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
            Self::StorageError(msg) => write!(f, "Storage error: {}", msg),
            Self::ChannelFull { channel } => write!(f, "Channel '{}' is full", channel),
            Self::ChannelClosed { channel } => write!(f, "Channel '{}' is closed", channel),
            Self::InvalidFilter(msg) => write!(f, "Invalid filter: {}", msg),
            Self::Timeout => write!(f, "Operation timed out"),
            Self::InternalError(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for EventError {}

impl From<harana_components_storage::StorageError> for EventError {
    fn from(err: harana_components_storage::StorageError) -> Self {
        EventError::StorageError(err.to_string())
    }
}

impl From<serde_json::Error> for EventError {
    fn from(err: serde_json::Error) -> Self {
        EventError::SerializationError(err.to_string())
    }
}

pub type EventResult<T> = Result<T, EventError>;
