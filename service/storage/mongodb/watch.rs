use mongodb::{
    bson::{doc, Bson, Document},
    change_stream::event::{ChangeStreamEvent, OperationType},
    options::{FullDocumentType, FullDocumentBeforeChangeType},
};

use crate::{StorageError, StorageResult};
use super::utils::bson_doc_to_json;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChangeOperation {
    Insert,
    Update,
    Replace,
    Delete,
    Drop,
    Rename,
    DropDatabase,
    Invalidate,
}

impl ChangeOperation {
    /// Returns the MongoDB operation type string.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Insert => "insert",
            Self::Update => "update",
            Self::Replace => "replace",
            Self::Delete => "delete",
            Self::Drop => "drop",
            Self::Rename => "rename",
            Self::DropDatabase => "dropDatabase",
            Self::Invalidate => "invalidate",
        }
    }
}

impl std::fmt::Display for ChangeOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str().to_uppercase())
    }
}

impl std::str::FromStr for ChangeOperation {
    type Err = StorageError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "insert" => Ok(Self::Insert),
            "update" => Ok(Self::Update),
            "replace" => Ok(Self::Replace),
            "delete" => Ok(Self::Delete),
            "drop" => Ok(Self::Drop),
            "rename" => Ok(Self::Rename),
            "drop_database" | "dropdatabase" => Ok(Self::DropDatabase),
            "invalidate" => Ok(Self::Invalidate),
            _ => Err(StorageError::InternalError(format!("Unknown MongoDB operation: {}", s))),
        }
    }
}

impl From<OperationType> for ChangeOperation {
    fn from(op: OperationType) -> Self {
        match op {
            OperationType::Insert => Self::Insert,
            OperationType::Update => Self::Update,
            OperationType::Replace => Self::Replace,
            OperationType::Delete => Self::Delete,
            OperationType::Drop => Self::Drop,
            OperationType::Rename => Self::Rename,
            OperationType::DropDatabase => Self::DropDatabase,
            OperationType::Invalidate => Self::Invalidate,
            _ => Self::Invalidate,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CollectionChangeEvent {
    pub collection_name: String,
    pub database_name: String,
    pub operation: ChangeOperation,
    pub document_id: Option<String>,
    pub full_document: Option<serde_json::Value>,
    pub full_document_before_change: Option<serde_json::Value>,
    pub update_description: Option<UpdateDescription>,
    pub cluster_time: Option<mongodb::bson::Timestamp>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone)]
pub struct UpdateDescription {
    pub updated_fields: Option<serde_json::Value>,
    pub removed_fields: Vec<String>,
    pub truncated_arrays: Vec<String>,
}

impl CollectionChangeEvent {
    /// Creates a new collection change event.
    pub fn new(
        collection_name: impl Into<String>,
        database_name: impl Into<String>,
        operation: ChangeOperation,
        document_id: Option<String>,
    ) -> Self {
        Self {
            collection_name: collection_name.into(),
            database_name: database_name.into(),
            operation,
            document_id,
            full_document: None,
            full_document_before_change: None,
            update_description: None,
            cluster_time: None,
            timestamp: chrono::Utc::now(),
        }
    }

    /// Sets the full document.
    pub fn with_full_document(mut self, doc: Option<serde_json::Value>) -> Self {
        self.full_document = doc;
        self
    }

    /// Sets the full document before change.
    pub fn with_full_document_before_change(mut self, doc: Option<serde_json::Value>) -> Self {
        self.full_document_before_change = doc;
        self
    }

    /// Sets the update description.
    pub fn with_update_description(mut self, desc: Option<UpdateDescription>) -> Self {
        self.update_description = desc;
        self
    }

    /// Sets the cluster time.
    pub fn with_cluster_time(mut self, time: Option<mongodb::bson::Timestamp>) -> Self {
        self.cluster_time = time;
        self
    }

    /// Attempts to deserialize the full document into a specific type.
    pub fn deserialize_document<T: serde::de::DeserializeOwned>(&self) -> Option<T> {
        self.full_document.as_ref().and_then(|d| serde_json::from_value(d.clone()).ok())
    }

    /// Attempts to deserialize the document before change into a specific type.
    pub fn deserialize_document_before_change<T: serde::de::DeserializeOwned>(&self) -> Option<T> {
        self.full_document_before_change.as_ref().and_then(|d| serde_json::from_value(d.clone()).ok())
    }
}

#[derive(Debug, Clone, Default)]
pub struct WatchOptions {
        pub full_document: Option<FullDocumentType>,
        pub full_document_before_change: Option<FullDocumentBeforeChangeType>,
        pub pipeline: Option<Vec<Document>>,
        pub max_await_time_ms: Option<u64>,
        pub batch_size: Option<u32>,
}

impl WatchOptions {
    /// Creates new watch options with default settings.
    pub fn new() -> Self {
        Self::default()
    }

    /// Includes the full document for all change events (not just updates).
    pub fn with_full_document(mut self, full_doc_type: FullDocumentType) -> Self {
        self.full_document = Some(full_doc_type);
        self
    }

    /// Includes the full document before the change (requires MongoDB 6.0+).
    pub fn with_full_document_before_change(mut self, before_change_type: FullDocumentBeforeChangeType) -> Self {
        self.full_document_before_change = Some(before_change_type);
        self
    }

    /// Sets a custom aggregation pipeline to filter/transform events.
    pub fn with_pipeline(mut self, pipeline: Vec<Document>) -> Self {
        self.pipeline = Some(pipeline);
        self
    }

    /// Sets the maximum await time for new changes.
    pub fn with_max_await_time_ms(mut self, ms: u64) -> Self {
        self.max_await_time_ms = Some(ms);
        self
    }

    /// Sets the batch size for the cursor.
    pub fn with_batch_size(mut self, size: u32) -> Self {
        self.batch_size = Some(size);
        self
    }

    /// Creates a pipeline to filter for specific operations.
    pub fn filter_operations(operations: &[ChangeOperation]) -> Vec<Document> {
        let op_strings: Vec<&str> = operations.iter().map(|op| op.as_str()).collect();
        vec![doc! { "$match": { "operationType": { "$in": op_strings } } }]
    }

    /// Creates a pipeline to filter for changes to specific document IDs.
    pub fn filter_document_ids(ids: &[&str]) -> Vec<Document> {
        vec![doc! { "$match": { "documentKey._id": { "$in": ids } } }]
    }
}

/// Converts a MongoDB change stream event to our CollectionChangeEvent.
pub(crate) fn convert_change_event(
    event: ChangeStreamEvent<Document>,
    database_name: &str,
) -> StorageResult<CollectionChangeEvent> {
    let operation = ChangeOperation::from(event.operation_type);
    
    let (collection_name, db_name) = event.ns.as_ref()
        .map(|ns| (
            ns.coll.clone().unwrap_or_else(|| "unknown".to_string()),
            ns.db.clone(),
        ))
        .unwrap_or_else(|| ("unknown".to_string(), database_name.to_string()));

    let document_id = event.document_key
        .as_ref()
        .and_then(|key| key.get("_id"))
        .and_then(|id| match id {
            Bson::ObjectId(oid) => Some(oid.to_hex()),
            Bson::String(s) => Some(s.clone()),
            other => Some(other.to_string()),
        });

    // Convert full document to JSON
    let full_document = event.full_document
        .and_then(|doc| bson_doc_to_json(&doc).ok());

    // Convert full document before change to JSON
    let full_document_before_change = event.full_document_before_change
        .and_then(|doc| bson_doc_to_json(&doc).ok());

    // Convert update description
    let update_description = event.update_description.map(|desc| {
        UpdateDescription {
            updated_fields: bson_doc_to_json(&desc.updated_fields).ok(),
            removed_fields: desc.removed_fields,
            truncated_arrays: desc.truncated_arrays
                .map(|arr| arr.iter().map(|ta| ta.field.clone()).collect())
                .unwrap_or_default(),
        }
    });

    Ok(CollectionChangeEvent {
        collection_name,
        database_name: db_name,
        operation,
        document_id,
        full_document,
        full_document_before_change,
        update_description,
        cluster_time: event.cluster_time,
        timestamp: chrono::Utc::now(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_change_operation_from_str() {
        assert_eq!("INSERT".parse::<ChangeOperation>().unwrap(), ChangeOperation::Insert);
        assert_eq!("UPDATE".parse::<ChangeOperation>().unwrap(), ChangeOperation::Update);
        assert_eq!("REPLACE".parse::<ChangeOperation>().unwrap(), ChangeOperation::Replace);
        assert_eq!("DELETE".parse::<ChangeOperation>().unwrap(), ChangeOperation::Delete);
        assert_eq!("insert".parse::<ChangeOperation>().unwrap(), ChangeOperation::Insert);
        assert!("INVALID".parse::<ChangeOperation>().is_err());
    }

    #[test]
    fn test_change_operation_display() {
        assert_eq!(ChangeOperation::Insert.to_string(), "INSERT");
        assert_eq!(ChangeOperation::Update.to_string(), "UPDATE");
        assert_eq!(ChangeOperation::Replace.to_string(), "REPLACE");
        assert_eq!(ChangeOperation::Delete.to_string(), "DELETE");
    }

    #[test]
    fn test_collection_change_event_creation() {
        let event = CollectionChangeEvent::new(
            "users",
            "mydb",
            ChangeOperation::Insert,
            Some("user-123".to_string()),
        );

        assert_eq!(event.collection_name, "users");
        assert_eq!(event.database_name, "mydb");
        assert_eq!(event.operation, ChangeOperation::Insert);
        assert_eq!(event.document_id, Some("user-123".to_string()));
    }

    #[test]
    fn test_collection_change_event_with_full_document() {
        let event = CollectionChangeEvent::new(
            "users",
            "mydb",
            ChangeOperation::Insert,
            Some("user-123".to_string()),
        )
        .with_full_document(Some(serde_json::json!({"name": "John", "email": "john@example.com"})));

        assert!(event.full_document.is_some());
        
        #[derive(Debug, serde::Deserialize, PartialEq)]
        struct User {
            name: String,
            email: String,
        }

        let user: Option<User> = event.deserialize_document();
        assert!(user.is_some());
        let user = user.unwrap();
        assert_eq!(user.name, "John");
        assert_eq!(user.email, "john@example.com");
    }

    #[test]
    fn test_watch_options_filter_operations() {
        let pipeline = WatchOptions::filter_operations(&[
            ChangeOperation::Insert,
            ChangeOperation::Update,
        ]);
        
        assert_eq!(pipeline.len(), 1);
        assert!(pipeline[0].contains_key("$match"));
    }

    #[test]
    fn test_watch_options_builder() {
        let options = WatchOptions::new()
            .with_full_document(FullDocumentType::UpdateLookup)
            .with_max_await_time_ms(5000)
            .with_batch_size(100);

        assert!(options.full_document.is_some());
        assert_eq!(options.max_await_time_ms, Some(5000));
        assert_eq!(options.batch_size, Some(100));
    }
}
