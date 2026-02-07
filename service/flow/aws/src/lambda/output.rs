use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

// === Action Outputs ===

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFunctionOutput {
    pub function_name: String,
    pub function_arn: String,
    pub runtime: Option<String>,
    pub role: String,
    pub handler: Option<String>,
    pub code_size: i64,
    pub code_sha256: String,
    pub description: Option<String>,
    pub timeout: i32,
    pub memory_size: i32,
    pub last_modified: String,
    pub version: String,
    pub state: String,
    pub state_reason: Option<String>,
    pub state_reason_code: Option<String>,
    pub package_type: String,
    pub architectures: Vec<String>,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteFunctionOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFunctionOutput {
    pub configuration: FunctionConfiguration,
    pub code: Option<FunctionCodeLocation>,
    pub tags: HashMap<String, String>,
    pub concurrency: Option<Concurrency>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListFunctionsOutput {
    pub functions: Vec<FunctionConfiguration>,
    pub next_marker: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateFunctionCodeOutput {
    pub function_name: String,
    pub function_arn: String,
    pub runtime: Option<String>,
    pub handler: Option<String>,
    pub code_size: i64,
    pub code_sha256: String,
    pub last_modified: String,
    pub version: String,
    pub state: String,
    pub state_reason: Option<String>,
    pub state_reason_code: Option<String>,
    pub last_update_status: Option<String>,
    pub last_update_status_reason: Option<String>,
    pub last_update_status_reason_code: Option<String>,
    pub architectures: Vec<String>,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateFunctionConfigurationOutput {
    pub function_name: String,
    pub function_arn: String,
    pub runtime: Option<String>,
    pub role: String,
    pub handler: Option<String>,
    pub code_size: i64,
    pub code_sha256: String,
    pub description: Option<String>,
    pub timeout: i32,
    pub memory_size: i32,
    pub last_modified: String,
    pub version: String,
    pub state: String,
    pub state_reason: Option<String>,
    pub state_reason_code: Option<String>,
    pub package_type: String,
    pub architectures: Vec<String>,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvokeOutput {
    pub status_code: i32,
    pub payload: Option<Vec<u8>>,
    pub function_error: Option<String>,
    pub log_result: Option<String>,
    pub executed_version: String,
}

// === Class Types ===

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionCode {
    pub zip_file: Option<Vec<u8>>,
    pub s3_bucket: Option<String>,
    pub s3_key: Option<String>,
    pub s3_object_version: Option<String>,
    pub image_uri: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionCodeLocation {
    pub repository_type: Option<String>,
    pub location: Option<String>,
    pub image_uri: Option<String>,
    pub resolved_image_uri: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionConfiguration {
    pub function_name: String,
    pub function_arn: String,
    pub runtime: Option<String>,
    pub role: String,
    pub handler: Option<String>,
    pub code_size: i64,
    pub code_sha256: String,
    pub description: Option<String>,
    pub timeout: i32,
    pub memory_size: i32,
    pub last_modified: String,
    pub version: String,
    pub state: String,
    pub state_reason: Option<String>,
    pub state_reason_code: Option<String>,
    pub package_type: String,
    pub architectures: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Concurrency {
    pub reserved_concurrent_executions: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Environment {
    pub variables: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentError {
    pub error_code: Option<String>,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentResponse {
    pub variables: HashMap<String, String>,
    pub error: Option<EnvironmentError>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeadLetterConfig {
    pub target_arn: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VpcConfig {
    pub subnet_ids: Vec<String>,
    pub security_group_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VpcConfigResponse {
    pub subnet_ids: Vec<String>,
    pub security_group_ids: Vec<String>,
    pub vpc_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TracingConfig {
    pub mode: TracingMode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TracingConfigResponse {
    pub mode: TracingMode,
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
    pub entry_point: Vec<String>,
    pub command: Vec<String>,
    pub working_directory: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageConfigResponse {
    pub image_config: Option<ImageConfig>,
    pub error: Option<ImageConfigError>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageConfigError {
    pub error_code: Option<String>,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Layer {
    pub arn: String,
    pub code_size: i64,
    pub signing_profile_version_arn: Option<String>,
    pub signing_job_arn: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub log_format: Option<String>,
    pub application_log_level: Option<String>,
    pub system_log_level: Option<String>,
    pub log_group: Option<String>,
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

// === Enums ===

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Architecture {
    X86_64,
    Arm64,
}

impl Default for Architecture {
    fn default() -> Self {
        Architecture::X86_64
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PackageType {
    Zip,
    Image,
}

impl Default for PackageType {
    fn default() -> Self {
        PackageType::Zip
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InvocationType {
    RequestResponse,
    Event,
    DryRun,
}

impl Default for InvocationType {
    fn default() -> Self {
        InvocationType::RequestResponse
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LogType {
    None,
    Tail,
}

impl Default for LogType {
    fn default() -> Self {
        LogType::None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TracingMode {
    Active,
    PassThrough,
}

// === Internal Storage Types ===

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredFunction {
    pub function_name: String,
    pub function_arn: String,
    pub runtime: Option<String>,
    pub role: String,
    pub handler: Option<String>,
    pub description: Option<String>,
    pub timeout: i32,
    pub memory_size: i32,
    pub code: Vec<u8>,
    pub code_sha256: String,
    pub code_size: i64,
    pub environment: Environment,
    pub vpc_config: Option<VpcConfig>,
    pub dead_letter_config: Option<DeadLetterConfig>,
    pub tracing_config: Option<TracingConfig>,
    pub package_type: PackageType,
    pub architectures: Vec<Architecture>,
    pub layers: Vec<String>,
    pub ephemeral_storage: Option<EphemeralStorage>,
    pub tags: HashMap<String, String>,
    pub version: String,
    pub state: String,
    pub state_reason: Option<String>,
    pub state_reason_code: Option<String>,
    pub reserved_concurrent_executions: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub last_modified: DateTime<Utc>,
    pub region: String,
}
