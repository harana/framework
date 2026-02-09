use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::BlobResult;

/// Metadata associated with a blob object.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BlobMetadata {
    pub content_type: Option<String>,
    pub content_disposition: Option<String>,
    pub content_encoding: Option<String>,
    pub cache_control: Option<String>,
    pub custom: HashMap<String, String>,
}

impl BlobMetadata {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_content_type(mut self, content_type: impl Into<String>) -> Self {
        self.content_type = Some(content_type.into());
        self
    }

    pub fn with_content_disposition(mut self, disposition: impl Into<String>) -> Self {
        self.content_disposition = Some(disposition.into());
        self
    }

    pub fn with_content_encoding(mut self, encoding: impl Into<String>) -> Self {
        self.content_encoding = Some(encoding.into());
        self
    }

    pub fn with_cache_control(mut self, cache_control: impl Into<String>) -> Self {
        self.cache_control = Some(cache_control.into());
        self
    }

    pub fn with_custom(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.custom.insert(key.into(), value.into());
        self
    }
}

/// Options for putting a blob.
#[derive(Debug, Clone, Default)]
pub struct PutOptions {
    pub metadata: Option<BlobMetadata>,
    /// If true, fail when a blob with the same key already exists.
    pub if_not_exists: bool,
}

impl PutOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_metadata(mut self, metadata: BlobMetadata) -> Self {
        self.metadata = Some(metadata);
        self
    }

    pub fn with_content_type(mut self, content_type: impl Into<String>) -> Self {
        let meta = self.metadata.get_or_insert_with(BlobMetadata::default);
        meta.content_type = Some(content_type.into());
        self
    }

    pub fn if_not_exists(mut self) -> Self {
        self.if_not_exists = true;
        self
    }
}

/// Options for listing blobs.
#[derive(Debug, Clone, Default)]
pub struct ListOptions {
    pub prefix: Option<String>,
    pub cursor: Option<String>,
    pub limit: Option<u32>,
    pub delimiter: Option<String>,
}

impl ListOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.prefix = Some(prefix.into());
        self
    }

    pub fn with_cursor(mut self, cursor: impl Into<String>) -> Self {
        self.cursor = Some(cursor.into());
        self
    }

    pub fn with_limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn with_delimiter(mut self, delimiter: impl Into<String>) -> Self {
        self.delimiter = Some(delimiter.into());
        self
    }
}

/// Information about a stored blob.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlobInfo {
    pub key: String,
    pub size: u64,
    pub etag: Option<String>,
    pub content_type: Option<String>,
    pub last_modified: Option<String>,
    pub metadata: Option<BlobMetadata>,
}

/// Response from a list operation.
#[derive(Debug, Clone)]
pub struct ListResponse {
    pub objects: Vec<BlobInfo>,
    pub truncated: bool,
    pub cursor: Option<String>,
    pub prefixes: Vec<String>,
}

/// The trait all blob store backends must implement.
#[async_trait]
pub trait BlobStore: Send + Sync {
    /// Store bytes under the given key.
    async fn put(&self, key: &str, data: &[u8], options: PutOptions) -> BlobResult<BlobInfo>;

    /// Retrieve the bytes stored under the given key.
    async fn get(&self, key: &str) -> BlobResult<Vec<u8>>;

    /// Retrieve the bytes and metadata for the given key.
    async fn get_with_metadata(&self, key: &str) -> BlobResult<(Vec<u8>, BlobMetadata)> {
        let data = self.get(key).await?;
        let info = self.head(key).await?;
        let metadata = info.metadata.unwrap_or_default();
        Ok((data, metadata))
    }

    /// Retrieve metadata/info without downloading the body.
    async fn head(&self, key: &str) -> BlobResult<BlobInfo>;

    /// Delete the blob with the given key.
    async fn delete(&self, key: &str) -> BlobResult<()>;

    /// Delete multiple blobs.
    async fn delete_many(&self, keys: &[&str]) -> BlobResult<()> {
        for key in keys {
            self.delete(key).await?;
        }
        Ok(())
    }

    /// List blobs matching the given options.
    async fn list(&self, options: ListOptions) -> BlobResult<ListResponse>;

    /// Check whether a blob exists.
    async fn exists(&self, key: &str) -> BlobResult<bool> {
        match self.head(key).await {
            Ok(_) => Ok(true),
            Err(crate::BlobError::NotFound(_)) => Ok(false),
            Err(e) => Err(e),
        }
    }

    /// Copy a blob from one key to another.
    async fn copy(&self, src_key: &str, dst_key: &str) -> BlobResult<BlobInfo> {
        let (data, metadata) = self.get_with_metadata(src_key).await?;
        let opts = PutOptions::new().with_metadata(metadata);
        self.put(dst_key, &data, opts).await
    }

    /// Move (rename) a blob from one key to another.
    async fn rename(&self, src_key: &str, dst_key: &str) -> BlobResult<BlobInfo> {
        let info = self.copy(src_key, dst_key).await?;
        self.delete(src_key).await?;
        Ok(info)
    }
}
