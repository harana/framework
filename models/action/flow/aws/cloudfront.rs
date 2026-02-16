// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateDistributionOutput {
    pub distribution_id: String,
    pub domain_name: String,
    pub arn: String,
    pub status: String,
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
    pub price_schema: String,
    pub last_modified_time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateDistributionOutput {
    pub distribution_id: String,
    pub status: String,
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
pub struct CreateInvalidationOutput {
    pub invalidation_id: String,
    pub status: String,
    pub create_time: chrono::DateTime<chrono::Utc>,
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
pub struct ListInvalidationsOutput {
    pub invalidations: Vec<String>,
    pub next_marker: String,
    pub is_truncated: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateCachePolicyOutput {
    pub cache_policy_id: String,
    pub etag: String,
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
pub struct ListCachePoliciesOutput {
    pub cache_policies: Vec<String>,
    pub next_marker: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateOriginRequestPolicyOutput {
    pub origin_request_policy_id: String,
    pub etag: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateResponseHeadersPolicyOutput {
    pub response_headers_policy_id: String,
    pub etag: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateFunctionOutput {
    pub function_name: String,
    pub function_arn: String,
    pub etag: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PublishFunctionOutput {
    pub function_arn: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetDistributionConfigOutput {
    pub distribution_config: String,
    pub etag: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsCloudfrontDistribution {
    pub account_id: String,
    pub aliases: String,
    pub arn: String,
    pub certificate_arn: String,
    pub comment: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub default_root_object: String,
    pub distribution_id: String,
    pub domain_name: String,
    #[serde(default)]
    pub enabled: bool,
    pub http_version: String,
    pub last_modified_time: chrono::DateTime<chrono::Utc>,
    pub origin_domain_name: String,
    pub origin_id: String,
    pub price_schema: String,
    pub region: String,
    pub ssl_support_method: String,
    pub status: String,
    pub tags: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsCloudfrontInvalidation {
    #[serde(default = "chrono::Utc::now")]
    pub create_time: chrono::DateTime<chrono::Utc>,
    pub distribution_id: String,
    pub invalidation_id: String,
    pub paths: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsCloudfrontCachePolicy {
    pub account_id: String,
    pub cache_policy_id: String,
    pub comment: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub default_ttl: i64,
    pub etag: String,
    pub max_ttl: i64,
    pub min_ttl: i64,
    pub name: String,
    pub parameters_in_cache_key: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsCloudfrontOriginRequestPolicy {
    pub account_id: String,
    pub comment: String,
    pub cookies_config: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub etag: String,
    pub headers_config: String,
    pub name: String,
    pub origin_request_policy_id: String,
    pub query_strings_config: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsCloudfrontFunction {
    pub account_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub etag: String,
    pub function_arn: String,
    pub function_name: String,
    pub runtime: String,
    pub status: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[async_trait]
pub trait CloudfrontAction {
    async fn create_distribution(&self, origin_domain_name: String, origin_id: String, default_root_object: String, enabled: bool, price_schema: String, aliases: Vec<String>, certificate_arn: String, ssl_support_method: String, minimum_protocol_version: String, default_cache_behavior: String, cache_behaviors: Vec<String>, custom_error_responses: Vec<String>, comment: String, tags: String) -> Result<CreateDistributionOutput, Box<dyn std::error::Error>>;
    async fn get_distribution(&self, distribution_id: String) -> Result<GetDistributionOutput, Box<dyn std::error::Error>>;
    async fn update_distribution(&self, distribution_id: String, if_match: String, enabled: bool, default_root_object: String, aliases: Vec<String>, default_cache_behavior: String, cache_behaviors: Vec<String>, custom_error_responses: Vec<String>, comment: String) -> Result<UpdateDistributionOutput, Box<dyn std::error::Error>>;
    async fn delete_distribution(&self, distribution_id: String, if_match: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn list_distributions(&self, max_items: i64, marker: String) -> Result<ListDistributionsOutput, Box<dyn std::error::Error>>;
    async fn create_invalidation(&self, distribution_id: String, paths: Vec<String>, caller_reference: String) -> Result<CreateInvalidationOutput, Box<dyn std::error::Error>>;
    async fn get_invalidation(&self, distribution_id: String, invalidation_id: String) -> Result<GetInvalidationOutput, Box<dyn std::error::Error>>;
    async fn list_invalidations(&self, distribution_id: String, max_items: i64, marker: String) -> Result<ListInvalidationsOutput, Box<dyn std::error::Error>>;
    async fn create_cache_policy(&self, name: String, default_ttl: i64, max_ttl: i64, min_ttl: i64, parameters_in_cache_key_and_forwarded_to_origin: String, comment: String) -> Result<CreateCachePolicyOutput, Box<dyn std::error::Error>>;
    async fn get_cache_policy(&self, cache_policy_id: String) -> Result<GetCachePolicyOutput, Box<dyn std::error::Error>>;
    async fn delete_cache_policy(&self, cache_policy_id: String, if_match: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn list_cache_policies(&self, type: String, max_items: i64, marker: String) -> Result<ListCachePoliciesOutput, Box<dyn std::error::Error>>;
    async fn create_origin_request_policy(&self, name: String, headers_config: String, cookies_config: String, query_strings_config: String, comment: String) -> Result<CreateOriginRequestPolicyOutput, Box<dyn std::error::Error>>;
    async fn create_response_headers_policy(&self, name: String, cors_config: String, security_headers_config: String, custom_headers_config: String, comment: String) -> Result<CreateResponseHeadersPolicyOutput, Box<dyn std::error::Error>>;
    async fn create_function(&self, name: String, runtime: String, function_code: String, comment: String) -> Result<CreateFunctionOutput, Box<dyn std::error::Error>>;
    async fn publish_function(&self, name: String, if_match: String) -> Result<PublishFunctionOutput, Box<dyn std::error::Error>>;
    async fn get_distribution_config(&self, distribution_id: String) -> Result<GetDistributionConfigOutput, Box<dyn std::error::Error>>;
    async fn tag_resource(&self, resource_arn: String, tags: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn list_tags_for_resource(&self, resource_arn: String) -> Result<String, Box<dyn std::error::Error>>;
}
