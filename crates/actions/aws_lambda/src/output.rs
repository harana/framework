// Harana Actions - AWS Lambda Module Output Types
// Auto-generated output structs for action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use chrono::{DateTime, Utc};

// create_function
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFunctionOutput {
    pub architectures: Vec<String>,
    pub code_sha256: String,
    pub code_size: i64,
    pub dead_letter_config: Option<DeadLetterConfig>,
    pub description: Option<String>,
    pub environment: Option<Environment>,
    pub ephemeral_storage: Option<EphemeralStorage>,
    pub file_system_configs: Option<Vec<FileSystemConfig>>,
    pub function_arn: String,
    pub function_name: String,
    pub handler: Option<String>,
    pub image_config_response: Option<ImageConfigResponse>,
    pub kms_key_arn: Option<String>,
    pub last_modified: String,
    pub last_update_status: Option<String>,
    pub layers: Option<Vec<Layer>>,
    pub logging_config: Option<LoggingConfig>,
    pub master_arn: Option<String>,
    pub memory_size: i32,
    pub package_type: String,
    pub revision_id: String,
    pub role: String,
    pub runtime: Option<String>,
    pub signing_job_arn: Option<String>,
    pub signing_profile_version_arn: Option<String>,
    pub snap_start: Option<SnapStartResponse>,
    pub state: Option<String>,
    pub state_reason: Option<String>,
    pub state_reason_code: Option<String>,
    pub success: bool,
    pub timeout: i32,
    pub tracing_config: Option<TracingConfigResponse>,
    pub version: String,
    pub vpc_config: Option<VpcConfigResponse>,
}

// delete_function
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteFunctionOutput {
    pub success: bool,
}

// get_function
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFunctionOutput {
    pub code: Option<FunctionCodeLocation>,
    pub concurrency: Option<Concurrency>,
    pub configuration: Option<FunctionConfiguration>,
    pub tags: Option<HashMap<String, String>>,
}

// list_functions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListFunctionsOutput {
    pub functions: Vec<FunctionConfiguration>,
    pub next_marker: Option<String>,
}

// update_function_code
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateFunctionCodeOutput {
    pub architectures: Vec<String>,
    pub code_sha256: String,
    pub code_size: i64,
    pub function_arn: String,
    pub function_name: String,
    pub handler: Option<String>,
    pub last_modified: String,
    pub last_update_status: Option<String>,
    pub last_update_status_reason: Option<String>,
    pub last_update_status_reason_code: Option<String>,
    pub revision_id: String,
    pub runtime: Option<String>,
    pub state: Option<String>,
    pub state_reason: Option<String>,
    pub state_reason_code: Option<String>,
    pub success: bool,
    pub version: String,
}

// update_function_configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateFunctionConfigurationOutput {
    pub architectures: Vec<String>,
    pub code_sha256: String,
    pub code_size: i64,
    pub dead_letter_config: Option<DeadLetterConfig>,
    pub description: Option<String>,
    pub environment: Option<Environment>,
    pub ephemeral_storage: Option<EphemeralStorage>,
    pub file_system_configs: Option<Vec<FileSystemConfig>>,
    pub function_arn: String,
    pub function_name: String,
    pub handler: Option<String>,
    pub image_config_response: Option<ImageConfigResponse>,
    pub kms_key_arn: Option<String>,
    pub last_modified: String,
    pub last_update_status: Option<String>,
    pub layers: Option<Vec<Layer>>,
    pub logging_config: Option<LoggingConfig>,
    pub memory_size: i32,
    pub package_type: String,
    pub revision_id: String,
    pub role: String,
    pub runtime: Option<String>,
    pub snap_start: Option<SnapStartResponse>,
    pub state: Option<String>,
    pub state_reason: Option<String>,
    pub state_reason_code: Option<String>,
    pub success: bool,
    pub timeout: i32,
    pub tracing_config: Option<TracingConfigResponse>,
    pub version: String,
    pub vpc_config: Option<VpcConfigResponse>,
}

// invoke
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvokeOutput {
    pub executed_version: Option<String>,
    pub function_error: Option<String>,
    pub log_result: Option<String>,
    pub payload: Option<Vec<u8>>,
    pub status_code: i32,
}

// Helper structs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionCode {
    pub image_uri: Option<String>,
    pub s3_bucket: Option<String>,
    pub s3_key: Option<String>,
    pub s3_object_version: Option<String>,
    pub zip_file: Option<Vec<u8>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionCodeLocation {
    pub image_uri: Option<String>,
    pub location: Option<String>,
    pub repository_type: Option<String>,
    pub resolved_image_uri: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionConfiguration {
    pub architectures: Option<Vec<String>>,
    pub code_sha256: Option<String>,
    pub code_size: Option<i64>,
    pub dead_letter_config: Option<DeadLetterConfig>,
    pub description: Option<String>,
    pub environment: Option<Environment>,
    pub ephemeral_storage: Option<EphemeralStorage>,
    pub file_system_configs: Option<Vec<FileSystemConfig>>,
    pub function_arn: Option<String>,
    pub function_name: Option<String>,
    pub handler: Option<String>,
    pub image_config_response: Option<ImageConfigResponse>,
    pub kms_key_arn: Option<String>,
    pub last_modified: Option<String>,
    pub last_update_status: Option<String>,
    pub last_update_status_reason: Option<String>,
    pub last_update_status_reason_code: Option<String>,
    pub layers: Option<Vec<Layer>>,
    pub logging_config: Option<LoggingConfig>,
    pub master_arn: Option<String>,
    pub memory_size: Option<i32>,
    pub package_type: Option<String>,
    pub revision_id: Option<String>,
    pub role: Option<String>,
    pub runtime: Option<String>,
    pub signing_job_arn: Option<String>,
    pub signing_profile_version_arn: Option<String>,
    pub snap_start: Option<SnapStartResponse>,
    pub state: Option<String>,
    pub state_reason: Option<String>,
    pub state_reason_code: Option<String>,
    pub timeout: Option<i32>,
    pub tracing_config: Option<TracingConfigResponse>,
    pub version: Option<String>,
    pub vpc_config: Option<VpcConfigResponse>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeadLetterConfig {
    pub target_arn: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Environment {
    pub variables: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EphemeralStorage {
    pub size: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileSystemConfig {
    pub arn: String,
    pub local_mount_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageConfig {
    pub command: Option<Vec<String>>,
    pub entry_point: Option<Vec<String>>,
    pub working_directory: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageConfigResponse {
    pub error: Option<ImageConfigError>,
    pub image_config: Option<ImageConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageConfigError {
    pub error_code: Option<String>,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Layer {
    pub arn: Option<String>,
    pub code_size: Option<i64>,
    pub signing_job_arn: Option<String>,
    pub signing_profile_version_arn: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub application_log_level: Option<String>,
    pub log_format: Option<String>,
    pub log_group: Option<String>,
    pub system_log_level: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapStart {
    pub apply_on: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapStartResponse {
    pub apply_on: Option<String>,
    pub optimization_status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TracingConfig {
    pub mode: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TracingConfigResponse {
    pub mode: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VpcConfig {
    pub ipv6_allowed_for_dual_stack: Option<bool>,
    pub security_group_ids: Option<Vec<String>>,
    pub subnet_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VpcConfigResponse {
    pub ipv6_allowed_for_dual_stack: Option<bool>,
    pub security_group_ids: Option<Vec<String>>,
    pub subnet_ids: Option<Vec<String>>,
    pub vpc_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Concurrency {
    pub reserved_concurrent_executions: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LambdaTags {
    pub tags: HashMap<String, String>,
}
