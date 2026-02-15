use async_trait::async_trait;

use crate::{BlobError, BlobInfo, BlobMetadata, BlobResult, ListOptions, ListResponse, PutOptions};

#[async_trait]
pub trait BlobService: Send + Sync {
    async fn put(&self, key: &str, data: &[u8], options: PutOptions) -> BlobResult<BlobInfo>;
    async fn get(&self, key: &str) -> BlobResult<Vec<u8>>;

    async fn get_with_metadata(&self, key: &str) -> BlobResult<(Vec<u8>, BlobMetadata)> {
        let data = self.get(key).await?;
        let info = self.head(key).await?;
        let metadata = info.metadata.unwrap_or_default();
        Ok((data, metadata))
    }

    async fn head(&self, key: &str) -> BlobResult<BlobInfo>;

    async fn delete(&self, key: &str) -> BlobResult<()>;

    async fn delete_many(&self, keys: &[&str]) -> BlobResult<()> {
        for key in keys {
            self.delete(key).await?;
        }
        Ok(())
    }

    async fn list(&self, options: ListOptions) -> BlobResult<ListResponse>;

    async fn exists(&self, key: &str) -> BlobResult<bool> {
        match self.head(key).await {
            Ok(_) => Ok(true),
            Err(BlobError::NotFound(_)) => Ok(false),
            Err(e) => Err(e),
        }
    }

    async fn copy(&self, src_key: &str, dst_key: &str) -> BlobResult<BlobInfo> {
        let (data, metadata) = self.get_with_metadata(src_key).await?;
        let opts = PutOptions::new().with_metadata(metadata);
        self.put(dst_key, &data, opts).await
    }

    async fn rename(&self, src_key: &str, dst_key: &str) -> BlobResult<BlobInfo> {
        let info = self.copy(src_key, dst_key).await?;
        self.delete(src_key).await?;
        Ok(info)
    }
}
