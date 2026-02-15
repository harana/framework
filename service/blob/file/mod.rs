use async_trait::async_trait;
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};
use tokio::fs;

use crate::error::{BlobError, BlobResult};
use crate::model::{BlobInfo, BlobMetadata, ListOptions, ListResponse, PutOptions};
use crate::service::BlobService;

const METADATA_SUFFIX: &str = ".meta.json";

pub struct FileBlobService {
    base_dir: PathBuf,
}

impl FileBlobService {
    pub async fn new(base_dir: impl Into<PathBuf>) -> BlobResult<Self> {
        let base_dir = base_dir.into();
        fs::create_dir_all(&base_dir)
            .await
            .map_err(|e| BlobError::IoError(format!("Failed to create base dir: {e}")))?;
        Ok(Self { base_dir })
    }

    fn blob_path(&self, key: &str) -> PathBuf {
        self.base_dir.join(key)
    }

    fn meta_path(&self, key: &str) -> PathBuf {
        self.base_dir.join(format!("{key}{METADATA_SUFFIX}"))
    }

    async fn write_metadata(&self, key: &str, metadata: &BlobMetadata) -> BlobResult<()> {
        let json = serde_json::to_string_pretty(metadata).map_err(|e| BlobError::SerializationError(e.to_string()))?;
        fs::write(self.meta_path(key), json)
            .await
            .map_err(|e| BlobError::IoError(e.to_string()))?;
        Ok(())
    }

    async fn read_metadata(&self, key: &str) -> BlobResult<BlobMetadata> {
        let path = self.meta_path(key);
        match fs::read_to_string(&path).await {
            Ok(json) => serde_json::from_str(&json).map_err(|e| BlobError::SerializationError(e.to_string())),
            Err(_) => Ok(BlobMetadata::default()),
        }
    }

    fn compute_etag(data: &[u8]) -> String {
        let hash = Sha256::digest(data);
        format!("{:x}", hash)
    }
}

#[async_trait]
impl BlobService for FileBlobService {
    async fn put(&self, key: &str, data: &[u8], options: PutOptions) -> BlobResult<BlobInfo> {
        let path = self.blob_path(key);

        if options.if_not_exists && path.exists() {
            return Err(BlobError::AlreadyExists(key.to_string()));
        }

        // Ensure parent directories exist.
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .await
                .map_err(|e| BlobError::IoError(e.to_string()))?;
        }

        fs::write(&path, data)
            .await
            .map_err(|e| BlobError::IoError(e.to_string()))?;

        let metadata = options.metadata.unwrap_or_default();
        self.write_metadata(key, &metadata).await?;

        let etag = Self::compute_etag(data);

        Ok(BlobInfo {
            key: key.to_string(),
            size: data.len() as u64,
            etag: Some(etag),
            content_type: metadata.content_type.clone(),
            last_modified: Some(chrono::Utc::now().to_rfc3339()),
            metadata: Some(metadata),
        })
    }

    async fn get(&self, key: &str) -> BlobResult<Vec<u8>> {
        let path = self.blob_path(key);
        fs::read(&path).await.map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                BlobError::NotFound(key.to_string())
            } else {
                BlobError::IoError(e.to_string())
            }
        })
    }

    async fn get_with_metadata(&self, key: &str) -> BlobResult<(Vec<u8>, BlobMetadata)> {
        let data = self.get(key).await?;
        let metadata = self.read_metadata(key).await?;
        Ok((data, metadata))
    }

    async fn head(&self, key: &str) -> BlobResult<BlobInfo> {
        let path = self.blob_path(key);
        let file_meta = fs::metadata(&path).await.map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                BlobError::NotFound(key.to_string())
            } else {
                BlobError::IoError(e.to_string())
            }
        })?;

        let metadata = self.read_metadata(key).await?;

        let last_modified = file_meta
            .modified()
            .ok()
            .and_then(|t| {
                t.duration_since(std::time::UNIX_EPOCH)
                    .ok()
                    .map(|d| chrono::DateTime::from_timestamp(d.as_secs() as i64, d.subsec_nanos()))
            })
            .flatten()
            .map(|dt| dt.to_rfc3339());

        Ok(BlobInfo {
            key: key.to_string(),
            size: file_meta.len(),
            etag: None,
            content_type: metadata.content_type.clone(),
            last_modified,
            metadata: Some(metadata),
        })
    }

    async fn delete(&self, key: &str) -> BlobResult<()> {
        let path = self.blob_path(key);
        match fs::remove_file(&path).await {
            Ok(()) => {}
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                // Idempotent – not an error.
            }
            Err(e) => return Err(BlobError::IoError(e.to_string())),
        }
        // Remove sidecar metadata if present.
        let _ = fs::remove_file(self.meta_path(key)).await;
        Ok(())
    }

    async fn list(&self, options: ListOptions) -> BlobResult<ListResponse> {
        let prefix = options.prefix.as_deref().unwrap_or("");
        let limit = options.limit.unwrap_or(1000) as usize;

        let mut objects = Vec::new();
        let mut prefixes = Vec::new();

        collect_entries(
            &self.base_dir,
            &self.base_dir,
            prefix,
            options.delimiter.as_deref(),
            &mut objects,
            &mut prefixes,
        )
        .await?;

        objects.sort_by(|a, b| a.key.cmp(&b.key));
        prefixes.sort();
        prefixes.dedup();

        // Handle cursor-based pagination (cursor = last key seen).
        if let Some(ref cursor) = options.cursor {
            objects.retain(|o| o.key.as_str() > cursor.as_str());
        }

        let truncated = objects.len() > limit;
        let cursor = if truncated {
            objects.get(limit - 1).map(|o| o.key.clone())
        } else {
            None
        };

        objects.truncate(limit);

        Ok(ListResponse {
            objects,
            truncated,
            cursor,
            prefixes,
        })
    }

    async fn copy(&self, src_key: &str, dst_key: &str) -> BlobResult<BlobInfo> {
        let src_path = self.blob_path(src_key);
        let dst_path = self.blob_path(dst_key);

        if !src_path.exists() {
            return Err(BlobError::NotFound(src_key.to_string()));
        }

        if let Some(parent) = dst_path.parent() {
            fs::create_dir_all(parent)
                .await
                .map_err(|e| BlobError::IoError(e.to_string()))?;
        }

        fs::copy(&src_path, &dst_path)
            .await
            .map_err(|e| BlobError::IoError(e.to_string()))?;

        // Copy metadata sidecar if it exists.
        let src_meta = self.meta_path(src_key);
        if src_meta.exists() {
            let dst_meta = self.meta_path(dst_key);
            fs::copy(&src_meta, &dst_meta)
                .await
                .map_err(|e| BlobError::IoError(e.to_string()))?;
        }

        self.head(dst_key).await
    }
}

async fn collect_entries(
    base: &Path,
    dir: &Path,
    prefix: &str,
    delimiter: Option<&str>,
    objects: &mut Vec<BlobInfo>,
    prefixes: &mut Vec<String>,
) -> BlobResult<()> {
    let mut entries = fs::read_dir(dir).await.map_err(|e| BlobError::IoError(e.to_string()))?;

    while let Some(entry) = entries
        .next_entry()
        .await
        .map_err(|e| BlobError::IoError(e.to_string()))?
    {
        let path = entry.path();

        if path
            .file_name()
            .and_then(|n| n.to_str())
            .map_or(false, |n| n.ends_with(METADATA_SUFFIX))
        {
            continue;
        }

        let relative = path
            .strip_prefix(base)
            .unwrap_or(&path)
            .to_string_lossy()
            .replace('\\', "/");

        if path.is_dir() {
            if let Some(delim) = delimiter {
                let dir_prefix = format!("{relative}{delim}");
                if dir_prefix.starts_with(prefix) {
                    prefixes.push(dir_prefix);
                }
            } else {
                Box::pin(collect_entries(base, &path, prefix, delimiter, objects, prefixes)).await?;
            }
        } else if relative.starts_with(prefix) {
            let file_meta = fs::metadata(&path)
                .await
                .map_err(|e| BlobError::IoError(e.to_string()))?;

            let last_modified = file_meta
                .modified()
                .ok()
                .and_then(|t| {
                    t.duration_since(std::time::UNIX_EPOCH)
                        .ok()
                        .map(|d| chrono::DateTime::from_timestamp(d.as_secs() as i64, d.subsec_nanos()))
                })
                .flatten()
                .map(|dt| dt.to_rfc3339());

            objects.push(BlobInfo {
                key: relative,
                size: file_meta.len(),
                etag: None,
                content_type: None,
                last_modified,
                metadata: None,
            });
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests;
