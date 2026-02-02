// Harana Components - SQL Watch Types
// Types and traits for watching table changes via database notifications.

use async_trait::async_trait;

use crate::{StorageError, StorageResult};

/// Represents a change event from a database table.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChangeOperation {
    Insert,
    Update,
    Delete,
}

impl std::fmt::Display for ChangeOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChangeOperation::Insert => write!(f, "INSERT"),
            ChangeOperation::Update => write!(f, "UPDATE"),
            ChangeOperation::Delete => write!(f, "DELETE"),
        }
    }
}

impl std::str::FromStr for ChangeOperation {
    type Err = StorageError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "INSERT" => Ok(ChangeOperation::Insert),
            "UPDATE" => Ok(ChangeOperation::Update),
            "DELETE" => Ok(ChangeOperation::Delete),
            _ => Err(StorageError::InternalError(format!("Unknown operation: {}", s))),
        }
    }
}

/// A change event notification from a watched table.
#[derive(Debug, Clone)]
pub struct TableChangeEvent {
        pub table_name: String,
        pub operation: ChangeOperation,
        pub row_id: Option<String>,
        pub payload: Option<serde_json::Value>,
        pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl TableChangeEvent {
    /// Creates a new table change event.
    pub fn new(
        table_name: impl Into<String>,
        operation: ChangeOperation,
        row_id: Option<String>,
        payload: Option<serde_json::Value>,
    ) -> Self {
        Self {
            table_name: table_name.into(),
            operation,
            row_id,
            payload,
            timestamp: chrono::Utc::now(),
        }
    }

    /// Attempts to deserialize the payload into a specific type.
    pub fn deserialize_payload<T: serde::de::DeserializeOwned>(&self) -> Option<T> {
        self.payload.as_ref().and_then(|p| serde_json::from_value(p.clone()).ok())
    }
}

/// Trait for backends that support watching for table changes.
#[async_trait]
pub trait WatchableBackend: Send + Sync {
    /// Sets up the necessary database triggers and notification channels for watching changes.
    async fn setup_change_notifications(&self) -> StorageResult<()>;

    /// Removes the database triggers and notification channels.
    async fn teardown_change_notifications(&self) -> StorageResult<()>;

    /// Returns a stream of change events for the table.
    fn watch_changes(
        &self,
    ) -> std::pin::Pin<Box<dyn futures::Stream<Item = StorageResult<TableChangeEvent>> + Send + '_>>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_change_operation_from_str() {
        assert_eq!("INSERT".parse::<ChangeOperation>().unwrap(), ChangeOperation::Insert);
        assert_eq!("UPDATE".parse::<ChangeOperation>().unwrap(), ChangeOperation::Update);
        assert_eq!("DELETE".parse::<ChangeOperation>().unwrap(), ChangeOperation::Delete);
        assert_eq!("insert".parse::<ChangeOperation>().unwrap(), ChangeOperation::Insert);
        assert!("INVALID".parse::<ChangeOperation>().is_err());
    }

    #[test]
    fn test_change_operation_display() {
        assert_eq!(ChangeOperation::Insert.to_string(), "INSERT");
        assert_eq!(ChangeOperation::Update.to_string(), "UPDATE");
        assert_eq!(ChangeOperation::Delete.to_string(), "DELETE");
    }

    #[test]
    fn test_table_change_event_creation() {
        let event = TableChangeEvent::new(
            "users",
            ChangeOperation::Insert,
            Some("user-123".to_string()),
            Some(serde_json::json!({"name": "John", "email": "john@example.com"})),
        );

        assert_eq!(event.table_name, "users");
        assert_eq!(event.operation, ChangeOperation::Insert);
        assert_eq!(event.row_id, Some("user-123".to_string()));
        assert!(event.payload.is_some());
    }

    #[test]
    fn test_table_change_event_deserialize_payload() {
        #[derive(Debug, serde::Deserialize, PartialEq)]
        struct User {
            name: String,
            email: String,
        }

        let event = TableChangeEvent::new(
            "users",
            ChangeOperation::Insert,
            Some("user-123".to_string()),
            Some(serde_json::json!({"name": "John", "email": "john@example.com"})),
        );

        let user: Option<User> = event.deserialize_payload();
        assert!(user.is_some());
        let user = user.unwrap();
        assert_eq!(user.name, "John");
        assert_eq!(user.email, "john@example.com");
    }

    #[test]
    fn test_table_change_event_deserialize_payload_none() {
        let event = TableChangeEvent::new(
            "users",
            ChangeOperation::Delete,
            Some("user-123".to_string()),
            None,
        );

        let user: Option<serde_json::Value> = event.deserialize_payload();
        assert!(user.is_none());
    }
}
