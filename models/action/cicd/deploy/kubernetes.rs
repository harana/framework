// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[async_trait]
pub trait KubernetesAction {
    async fn deploy_kubernetes(&self) -> Result<(), Box<dyn std::error::Error>>;
    async fn deploy_kubernetes_rolling_update(&self) -> Result<(), Box<dyn std::error::Error>>;
    async fn deploy_kubernetes_helm(&self) -> Result<(), Box<dyn std::error::Error>>;
    async fn deploy_kubernetes_kubectl(&self) -> Result<(), Box<dyn std::error::Error>>;
    async fn deploy_kubernetes_kustomize(&self) -> Result<(), Box<dyn std::error::Error>>;
    async fn deploy_kubernetes_health_check(&self) -> Result<(), Box<dyn std::error::Error>>;
}
