use async_trait::async_trait;
use std::collections::HashMap;
use worker::Env;

use crate::error::{BlobError, BlobResult};
use crate::store::{BlobInfo, BlobMetadata, BlobStore, ListOptions, ListResponse, PutOptions};

/// A blob store backed by Cloudflare R2.
pub struct R2BlobStore {
    env: Env,
    bucket_name: String,
}

// `worker::Env` is Send + Sync via unsafe impl in workers-rs.
unsafe impl Send for R2BlobStore {}
unsafe impl Sync for R2BlobStore {}

impl R2BlobStore {
    /// Create a new `R2BlobStore` from a `worker::Env` and the R2 binding name.
    pub fn new(env: Env, bucket_name: impl Into<String>) -> Self {
        Self {
            env,
            bucket_name: bucket_name.into(),
        }
    }

    fn bucket(&self) -> BlobResult<worker::Bucket> {
        self.env
            .bucket(&self.bucket_name)
            .map_err(|e| BlobError::BackendError(format!("R2 binding error: {e}")))
    }

    fn to_metadata(obj: &worker::Object) -> BlobMetadata {
        let http = obj.http_metadata();
        let custom = obj.custom_metadata().unwrap_or_default();
        BlobMetadata {
            content_type: http.content_type.clone(),
            content_disposition: http.content_disposition.clone(),
            content_encoding: http.content_encoding.clone(),
            cache_control: http.cache_control.clone(),
            custom,
        }
    }

    fn to_blob_info(obj: &worker::Object) -> BlobInfo {
        let metadata = Self::to_metadata(obj);
        BlobInfo {
            key: obj.key(),
            size: obj.size() as u64,
            etag: Some(obj.etag()),
            content_type: metadata.content_type.clone(),
            last_modified: Some(obj.uploaded().to_string()),
            metadata: Some(metadata),
        }
    }

    fn build_http_metadata(metadata: &BlobMetadata) -> worker::HttpMetadata {
        let mut http = worker::HttpMetadata::default();
        http.content_type = metadata.content_type.clone();
        http.content_disposition = metadata.content_disposition.clone();
        http.content_encoding = metadata.content_encoding.clone();
        http.cache_control = metadata.cache_control.clone();
        http
    }
}

#[async_trait]
impl BlobStore for R2BlobStore {
    async fn put(&self, key: &str, data: &[u8], options: PutOptions) -> BlobResult<BlobInfo> {
        if options.if_not_exists {
            // Check existence first.
            let b = self.bucket()?;
            if b.head(key)
                .await
                .map_err(|e| BlobError::BackendError(e.to_string()))?
                .is_some()
            {
                return Err(BlobError::AlreadyExists(key.to_string()));
            }
        }

        let b = self.bucket()?;
        let mut builder = b.put(key, worker::Data::Bytes(data.to_vec()));

        if let Some(ref metadata) = options.metadata {
            builder = builder.http_metadata(Self::build_http_metadata(metadata));
            if !metadata.custom.is_empty() {
                builder = builder.custom_metadata(metadata.custom.clone());
            }
        }

        let obj = builder
            .execute()
            .await
            .map_err(|e| BlobError::BackendError(e.to_string()))?;

        Ok(Self::to_blob_info(&obj))
    }

    async fn get(&self, key: &str) -> BlobResult<Vec<u8>> {
        let b = self.bucket()?;
        let obj = b
            .get(key)
            .execute()
            .await
            .map_err(|e| BlobError::BackendError(e.to_string()))?
            .ok_or_else(|| BlobError::NotFound(key.to_string()))?;

        match obj.body() {
            Some(body) => body.bytes().await.map_err(|e| BlobError::BackendError(e.to_string())),
            None => Ok(vec![]),
        }
    }

    async fn get_with_metadata(&self, key: &str) -> BlobResult<(Vec<u8>, BlobMetadata)> {
        let b = self.bucket()?;
        let obj = b
            .get(key)
            .execute()
            .await
            .map_err(|e| BlobError::BackendError(e.to_string()))?
            .ok_or_else(|| BlobError::NotFound(key.to_string()))?;

        let metadata = Self::to_metadata(&obj);

        let data = match obj.body() {
            Some(body) => body.bytes().await.map_err(|e| BlobError::BackendError(e.to_string()))?,
            None => vec![],
        };

        Ok((data, metadata))
    }

    async fn head(&self, key: &str) -> BlobResult<BlobInfo> {
        let b = self.bucket()?;
        let obj = b
            .head(key)
            .await
            .map_err(|e| BlobError::BackendError(e.to_string()))?
            .ok_or_else(|| BlobError::NotFound(key.to_string()))?;

        Ok(Self::to_blob_info(&obj))
    }

    async fn delete(&self, key: &str) -> BlobResult<()> {
        let b = self.bucket()?;
        b.delete(key).await.map_err(|e| BlobError::BackendError(e.to_string()))
    }

    async fn delete_many(&self, keys: &[&str]) -> BlobResult<()> {
        let b = self.bucket()?;
        for key in keys {
            b.delete(key)
                .await
                .map_err(|e| BlobError::BackendError(e.to_string()))?;
        }
        Ok(())
    }

    async fn list(&self, options: ListOptions) -> BlobResult<ListResponse> {
        let b = self.bucket()?;
        let mut builder = b.list();

        if let Some(ref prefix) = options.prefix {
            builder = builder.prefix(prefix);
        }
        if let Some(ref cursor) = options.cursor {
            builder = builder.cursor(cursor);
        }
        if let Some(ref delimiter) = options.delimiter {
            builder = builder.delimiter(delimiter);
        }
        if let Some(limit) = options.limit {
            builder = builder.limit(limit);
        }

        let result = builder
            .execute()
            .await
            .map_err(|e| BlobError::BackendError(e.to_string()))?;

        let objects = result
            .objects()
            .into_iter()
            .map(|obj| {
                let custom = obj.custom_metadata().unwrap_or_default();
                let http = obj.http_metadata();
                BlobInfo {
                    key: obj.key(),
                    size: obj.size() as u64,
                    etag: Some(obj.etag()),
                    content_type: http.content_type.clone(),
                    last_modified: Some(obj.uploaded().to_string()),
                    metadata: Some(BlobMetadata {
                        content_type: http.content_type.clone(),
                        content_disposition: http.content_disposition.clone(),
                        content_encoding: http.content_encoding.clone(),
                        cache_control: http.cache_control.clone(),
                        custom,
                    }),
                }
            })
            .collect();

        Ok(ListResponse {
            objects,
            truncated: result.truncated(),
            cursor: result.cursor(),
            prefixes: result.delimited_prefixes(),
        })
    }
}
