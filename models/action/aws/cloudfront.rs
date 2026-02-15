// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateDistributionInput {
    pub origin_domain_name: String,
    pub origin_id: String,
    pub default_root_object: String,
    #[serde(default)]
    pub enabled: bool,
    pub price_class: String,
    pub aliases: Vec<String>,
    pub certificate_arn: String,
    pub ssl_support_method: String,
    pub minimum_protocol_version: String,
    pub default_cache_behavior: String,
    pub cache_behaviors: Vec<String>,
    pub custom_error_responses: Vec<String>,
    pub comment: String,
    pub tags: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateDistributionOutput {
    pub distribution_id: String,
    pub domain_name: String,
    pub arn: String,
    pub status: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetDistributionInput {
    pub distribution_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetDistributionOutput {
    pub distribution_id: String,
    pub domain_name: String,
    pub arn: String,
    pub status: String,
    pub enabled: bool,
    pub aliases: Vec<String>,
    pub origins: Vec<String>,
    pub default_cache_behavior: String,
    pub cache_behaviors: Vec<String>,
    pub price_class: String,
    pub last_modified_time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateDistributionInput {
    pub distribution_id: String,
    pub if_match: String,
    pub enabled: bool,
    pub default_root_object: String,
    pub aliases: Vec<String>,
    pub default_cache_behavior: String,
    pub cache_behaviors: Vec<String>,
    pub custom_error_responses: Vec<String>,
    pub comment: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateDistributionOutput {
    pub distribution_id: String,
    pub status: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteDistributionInput {
    pub distribution_id: String,
    pub if_match: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteDistributionOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListDistributionsInput {
    pub max_items: i64,
    pub marker: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListDistributionsOutput {
    pub distributions: Vec<String>,
    pub next_marker: String,
    pub is_truncated: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateInvalidationInput {
    pub distribution_id: String,
    pub paths: Vec<String>,
    pub caller_reference: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateInvalidationOutput {
    pub invalidation_id: String,
    pub status: String,
    pub create_time: chrono::DateTime<chrono::Utc>,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetInvalidationInput {
    pub distribution_id: String,
    pub invalidation_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetInvalidationOutput {
    pub invalidation_id: String,
    pub status: String,
    pub create_time: chrono::DateTime<chrono::Utc>,
    pub paths: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListInvalidationsInput {
    pub distribution_id: String,
    pub max_items: i64,
    pub marker: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListInvalidationsOutput {
    pub invalidations: Vec<String>,
    pub next_marker: String,
    pub is_truncated: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateCachePolicyInput {
    pub name: String,
    pub default_ttl: i64,
    pub max_ttl: i64,
    pub min_ttl: i64,
    pub parameters_in_cache_key_and_forwarded_to_origin: String,
    pub comment: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateCachePolicyOutput {
    pub cache_policy_id: String,
    pub etag: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetCachePolicyInput {
    pub cache_policy_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetCachePolicyOutput {
    pub cache_policy_id: String,
    pub name: String,
    pub default_ttl: i64,
    pub max_ttl: i64,
    pub min_ttl: i64,
    pub parameters_in_cache_key: String,
    pub etag: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteCachePolicyInput {
    pub cache_policy_id: String,
    pub if_match: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteCachePolicyOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListCachePoliciesInput {
    pub type: String,
    pub max_items: i64,
    pub marker: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListCachePoliciesOutput {
    pub cache_policies: Vec<String>,
    pub next_marker: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateOriginRequestPolicyInput {
    pub name: String,
    pub headers_config: String,
    pub cookies_config: String,
    pub query_strings_config: String,
    pub comment: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateOriginRequestPolicyOutput {
    pub origin_request_policy_id: String,
    pub etag: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateResponseHeadersPolicyInput {
    pub name: String,
    pub cors_config: String,
    pub security_headers_config: String,
    pub custom_headers_config: String,
    pub comment: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateResponseHeadersPolicyOutput {
    pub response_headers_policy_id: String,
    pub etag: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateFunctionInput {
    pub name: String,
    pub runtime: String,
    pub function_code: String,
    pub comment: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateFunctionOutput {
    pub function_name: String,
    pub function_arn: String,
    pub etag: String,
    pub status: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PublishFunctionInput {
    pub name: String,
    pub if_match: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PublishFunctionOutput {
    pub function_arn: String,
    pub status: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetDistributionConfigInput {
    pub distribution_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetDistributionConfigOutput {
    pub distribution_config: String,
    pub etag: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TagResourceInput {
    pub resource_arn: String,
    pub tags: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TagResourceOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListTagsForResourceInput {
    pub resource_arn: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListTagsForResourceOutput {
    pub tags: String,
}

#[async_trait]
pub trait CloudfrontAction {
    async fn create_distribution(&self, input: CreateDistributionInput) -> Result<CreateDistributionOutput, Box<dyn std::error::Error>>;
    async fn get_distribution(&self, input: GetDistributionInput) -> Result<GetDistributionOutput, Box<dyn std::error::Error>>;
    async fn update_distribution(&self, input: UpdateDistributionInput) -> Result<UpdateDistributionOutput, Box<dyn std::error::Error>>;
    async fn delete_distribution(&self, input: DeleteDistributionInput) -> Result<DeleteDistributionOutput, Box<dyn std::error::Error>>;
    async fn list_distributions(&self, input: ListDistributionsInput) -> Result<ListDistributionsOutput, Box<dyn std::error::Error>>;
    async fn create_invalidation(&self, input: CreateInvalidationInput) -> Result<CreateInvalidationOutput, Box<dyn std::error::Error>>;
    async fn get_invalidation(&self, input: GetInvalidationInput) -> Result<GetInvalidationOutput, Box<dyn std::error::Error>>;
    async fn list_invalidations(&self, input: ListInvalidationsInput) -> Result<ListInvalidationsOutput, Box<dyn std::error::Error>>;
    async fn create_cache_policy(&self, input: CreateCachePolicyInput) -> Result<CreateCachePolicyOutput, Box<dyn std::error::Error>>;
    async fn get_cache_policy(&self, input: GetCachePolicyInput) -> Result<GetCachePolicyOutput, Box<dyn std::error::Error>>;
    async fn delete_cache_policy(&self, input: DeleteCachePolicyInput) -> Result<DeleteCachePolicyOutput, Box<dyn std::error::Error>>;
    async fn list_cache_policies(&self, input: ListCachePoliciesInput) -> Result<ListCachePoliciesOutput, Box<dyn std::error::Error>>;
    async fn create_origin_request_policy(&self, input: CreateOriginRequestPolicyInput) -> Result<CreateOriginRequestPolicyOutput, Box<dyn std::error::Error>>;
    async fn create_response_headers_policy(&self, input: CreateResponseHeadersPolicyInput) -> Result<CreateResponseHeadersPolicyOutput, Box<dyn std::error::Error>>;
    async fn create_function(&self, input: CreateFunctionInput) -> Result<CreateFunctionOutput, Box<dyn std::error::Error>>;
    async fn publish_function(&self, input: PublishFunctionInput) -> Result<PublishFunctionOutput, Box<dyn std::error::Error>>;
    async fn get_distribution_config(&self, input: GetDistributionConfigInput) -> Result<GetDistributionConfigOutput, Box<dyn std::error::Error>>;
    async fn tag_resource(&self, input: TagResourceInput) -> Result<TagResourceOutput, Box<dyn std::error::Error>>;
    async fn list_tags_for_resource(&self, input: ListTagsForResourceInput) -> Result<ListTagsForResourceOutput, Box<dyn std::error::Error>>;
}
