// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// aws_lambda_function
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsLambdaFunction {
    pub account_id: String,
    pub architectures: Option<String>,
    pub code_sha256: Option<String>,
    pub code_size: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub dead_letter_arn: Option<String>,
    pub description: Option<String>,
    pub function_arn: Option<String>,
    pub function_name: String,
    pub handler: Option<String>,
    pub kms_key_arn: Option<String>,
    pub last_modified: Option<chrono::DateTime<chrono::Utc>>,
    pub memory_size: i64,
    pub package_type: String,
    pub region: Option<String>,
    pub role: String,
    pub runtime: Option<String>,
    pub state: String,
    pub tags: Option<String>,
    pub timeout: i64,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub version: Option<String>,
}

impl AwsLambdaFunction {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// aws_lambda_function_version
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsLambdaFunctionVersion {
    pub code_sha256: Option<String>,
    pub code_size: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub description: Option<String>,
    /// Reference: aws_lambda_function.id
    pub function_id: String,
    pub handler: Option<String>,
    pub memory_size: Option<i64>,
    pub runtime: Option<String>,
    pub timeout: Option<i64>,
    pub version: String,
}

impl AwsLambdaFunctionVersion {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// aws_lambda_alias
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsLambdaAlias {
    pub alias_arn: Option<String>,
    pub alias_name: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub description: Option<String>,
    /// Reference: aws_lambda_function.id
    pub function_id: String,
    pub function_version: String,
    pub routing_config: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl AwsLambdaAlias {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// aws_lambda_event_source_mapping
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsLambdaEventSourceMapping {
    pub batch_size: Option<i64>,
    #[serde(default)]
    pub bisect_batch_on_error: bool,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(default)]
    pub enabled: bool,
    pub event_source_arn: String,
    /// Reference: aws_lambda_function.id
    pub function_id: String,
    pub last_processing_result: Option<String>,
    pub maximum_batching_window: Option<i64>,
    pub maximum_retry_attempts: Option<i64>,
    pub state: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub uuid: Option<String>,
}

impl AwsLambdaEventSourceMapping {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// aws_lambda_layer
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsLambdaLayer {
    pub account_id: String,
    pub compatible_architectures: Option<String>,
    pub compatible_runtimes: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub description: Option<String>,
    pub layer_arn: Option<String>,
    pub layer_name: String,
    pub latest_version: i64,
    pub license_info: Option<String>,
    pub region: Option<String>,
}

impl AwsLambdaLayer {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// aws_lambda_invocation_log
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsLambdaInvocationLog {
    pub billed_duration_ms: Option<i64>,
    pub duration_ms: Option<i64>,
    pub error: Option<String>,
    /// Reference: aws_lambda_function.id
    pub function_id: String,
    pub invocation_type: String,
    #[serde(default = "chrono::Utc::now")]
    pub invoked_at: chrono::DateTime<chrono::Utc>,
    pub max_memory_used_mb: Option<i64>,
    pub request_id: Option<String>,
    pub status: String,
}

impl AwsLambdaInvocationLog {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

