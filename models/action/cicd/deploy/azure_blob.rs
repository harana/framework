// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[async_trait]
pub trait AzureBlobAction {
    async fn deploy_azure_blob(&self) -> Result<(), Box<dyn std::error::Error>>;
    async fn deploy_azure_blob_cdn(&self) -> Result<(), Box<dyn std::error::Error>>;
    async fn deploy_azure_blob_credentials(&self) -> Result<(), Box<dyn std::error::Error>>;
}
