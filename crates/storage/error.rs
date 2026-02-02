// Harana Components - Storage Error Types

use std::fmt;

/// Storage operation errors.
#[derive(Debug, Clone)]
pub enum StorageError {
    NotFound { entity_type: String, id: String },
    DuplicateKey { entity_type: String, id: String },
    ConnectionError(String),
    QueryError(String),
    SerializationError(String),
    ValidationError(String),
    InternalError(String),
    NotSupported(String),
}

impl fmt::Display for StorageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound { entity_type, id } => write!(f, "{entity_type} with id '{id}' not found"),
            Self::DuplicateKey { entity_type, id } => write!(f, "{entity_type} with id '{id}' already exists"),
            Self::ConnectionError(msg) => write!(f, "Connection error: {msg}"),
            Self::QueryError(msg) => write!(f, "Query error: {msg}"),
            Self::SerializationError(msg) => write!(f, "Serialization error: {msg}"),
            Self::ValidationError(msg) => write!(f, "Validation error: {msg}"),
            Self::InternalError(msg) => write!(f, "Internal error: {msg}"),
            Self::NotSupported(msg) => write!(f, "Operation not supported: {msg}"),
        }
    }
}

impl std::error::Error for StorageError {}

pub type StorageResult<T> = Result<T, StorageError>;
