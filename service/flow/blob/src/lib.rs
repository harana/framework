pub mod output;

#[cfg(test)]
mod tests;

use dashmap::DashMap;
use once_cell::sync::Lazy;
use chrono::{Utc, Duration};
use uuid::Uuid;

use output::*;

// In-memory storage for blobs
// Key format: "bucket:key"
static BLOBS: Lazy<DashMap<String, StoredBlob>> = Lazy::new(DashMap::new);

fn storage_key(bucket: &str, key: &str) -> String {
    format!("{}:{}", bucket, key)
}

fn generate_etag(content: &[u8]) -> String {
    // Simple etag generation based on content hash
    format!("\"{:x}\"", md5_hash(content))
}

fn md5_hash(data: &[u8]) -> u64 {
    // Simple hash for demo purposes
    use std::hash::{Hash, Hasher};
    use std::collections::hash_map::DefaultHasher;
    let mut hasher = DefaultHasher::new();
    data.hash(&mut hasher);
    hasher.finish()
}

fn generate_url(bucket: &str, key: &str) -> String {
    format!("https://{}.blob.storage.example.com/{}", bucket, key)
}

/// Upload a blob to storage
pub async fn upload_blob(
    bucket: String,
    key: String,
    content: Vec<u8>,
    content_type: Option<String>,
    metadata: Option<BlobMetadata>,
) -> UploadBlobOutput {
    let etag = generate_etag(&content);
    let url = generate_url(&bucket, &key);
    let now = Utc::now();

    let stored = StoredBlob {
        bucket: bucket.clone(),
        key: key.clone(),
        content,
        content_type: content_type.unwrap_or_else(|| "application/octet-stream".to_string()),
        etag: etag.clone(),
        metadata: metadata.unwrap_or_default(),
        created: now,
        modified: now,
    };

    BLOBS.insert(storage_key(&bucket, &key), stored);

    UploadBlobOutput { url, etag }
}

/// Download a blob from storage
pub async fn download_blob(bucket: String, key: String) -> DownloadBlobOutput {
    if let Some(entry) = BLOBS.get(&storage_key(&bucket, &key)) {
        DownloadBlobOutput {
            content: entry.content.clone(),
            content_type: entry.content_type.clone(),
            size: entry.content.len() as i64,
            metadata: entry.metadata.clone(),
        }
    } else {
        DownloadBlobOutput {
            content: Vec::new(),
            content_type: String::new(),
            size: 0,
            metadata: BlobMetadata::default(),
        }
    }
}

/// Delete a blob from storage
pub async fn delete_blob(bucket: String, key: String) -> DeleteBlobOutput {
    let success = BLOBS.remove(&storage_key(&bucket, &key)).is_some();
    DeleteBlobOutput { success }
}

/// List blobs in a bucket
pub async fn list_blobs(
    bucket: String,
    prefix: Option<String>,
    max_results: Option<i64>,
    continuation_token: Option<String>,
) -> ListBlobsOutput {
    let max_results = max_results.unwrap_or(1000) as usize;
    let prefix = prefix.unwrap_or_default();
    
    let bucket_prefix = format!("{}:", bucket);
    
    let mut blobs: Vec<BlobInfo> = BLOBS
        .iter()
        .filter(|entry| {
            let key = entry.key();
            key.starts_with(&bucket_prefix) && {
                let blob_key = &key[bucket_prefix.len()..];
                blob_key.starts_with(&prefix)
            }
        })
        .map(|entry| {
            let blob = entry.value();
            BlobInfo {
                key: blob.key.clone(),
                size: blob.content.len() as i64,
                etag: blob.etag.clone(),
                last_modified: blob.modified,
                content_type: blob.content_type.clone(),
            }
        })
        .collect();

    // Sort by key for consistent pagination
    blobs.sort_by(|a, b| a.key.cmp(&b.key));

    // Apply continuation token (skip entries until we find the token)
    let start_idx = if let Some(token) = continuation_token {
        blobs.iter().position(|b| b.key > token).unwrap_or(0)
    } else {
        0
    };

    let end_idx = (start_idx + max_results).min(blobs.len());
    let result_blobs = blobs[start_idx..end_idx].to_vec();
    
    let next_token = if end_idx < blobs.len() {
        result_blobs.last().map(|b| b.key.clone())
    } else {
        None
    };

    ListBlobsOutput {
        blobs: result_blobs,
        next_token,
    }
}

/// Check if a blob exists
pub async fn exists(bucket: String, key: String) -> ExistsOutput {
    let exists = BLOBS.contains_key(&storage_key(&bucket, &key));
    ExistsOutput { exists }
}

/// Get blob metadata
pub async fn get_blob_metadata(bucket: String, key: String) -> GetBlobMetadataOutput {
    if let Some(entry) = BLOBS.get(&storage_key(&bucket, &key)) {
        GetBlobMetadataOutput {
            content_type: entry.content_type.clone(),
            size: entry.content.len() as i64,
            etag: entry.etag.clone(),
            created: entry.created,
            modified: entry.modified,
            metadata: entry.metadata.clone(),
        }
    } else {
        GetBlobMetadataOutput {
            content_type: String::new(),
            size: 0,
            etag: String::new(),
            created: Utc::now(),
            modified: Utc::now(),
            metadata: BlobMetadata::default(),
        }
    }
}

/// Copy a blob to a new location
pub async fn copy_blob(
    source_bucket: String,
    source_key: String,
    dest_bucket: String,
    dest_key: String,
) -> CopyBlobOutput {
    let source_storage_key = storage_key(&source_bucket, &source_key);
    
    if let Some(source) = BLOBS.get(&source_storage_key) {
        let etag = generate_etag(&source.content);
        let now = Utc::now();
        
        let dest = StoredBlob {
            bucket: dest_bucket.clone(),
            key: dest_key.clone(),
            content: source.content.clone(),
            content_type: source.content_type.clone(),
            etag: etag.clone(),
            metadata: source.metadata.clone(),
            created: now,
            modified: now,
        };

        BLOBS.insert(storage_key(&dest_bucket, &dest_key), dest);

        CopyBlobOutput {
            success: true,
            etag,
        }
    } else {
        CopyBlobOutput {
            success: false,
            etag: String::new(),
        }
    }
}

/// Generate a presigned URL for blob access
pub async fn generate_presigned_url(
    bucket: String,
    key: String,
    operation: Option<PresignedUrlOperation>,
    expires_in: Option<i64>,
) -> GeneratePresignedUrlOutput {
    let operation = operation.unwrap_or(PresignedUrlOperation::Get);
    let expires_in_secs = expires_in.unwrap_or(3600);
    let expires_at = Utc::now() + Duration::seconds(expires_in_secs);
    
    let op_str = match operation {
        PresignedUrlOperation::Get => "GET",
        PresignedUrlOperation::Put => "PUT",
    };
    
    // Generate a signed URL (mock)
    let signature = Uuid::new_v4().to_string().replace("-", "")[..16].to_string();
    let url = format!(
        "https://{}.blob.storage.example.com/{}?X-Operation={}&X-Expires={}&X-Signature={}",
        bucket,
        key,
        op_str,
        expires_at.timestamp(),
        signature
    );

    GeneratePresignedUrlOutput { url, expires_at }
}

// Utility functions for testing
#[cfg(test)]
pub fn clear_all_data() {
    BLOBS.clear();
}
