// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateFunctionOutput {
    pub architectures: Vec<String>,
    pub code_sha256: String,
    pub code_size: i64,
    pub dead_letter_config: String,
    pub description: String,
    pub environment: String,
    pub ephemeral_storage: String,
    pub file_system_configs: Vec<String>,
    pub function_arn: String,
    pub function_name: String,
    pub handler: String,
    pub image_config_response: String,
    pub kms_key_arn: String,
    pub last_modified: String,
    pub last_update_status: String,
    pub layers: Vec<String>,
    pub logging_config: String,
    pub master_arn: String,
    pub memory_size: i64,
    pub package_type: String,
    pub revision_id: String,
    pub role: String,
    pub runtime: String,
    pub signing_job_arn: String,
    pub signing_profile_version_arn: String,
    pub snap_start: String,
    pub state: String,
    pub state_reason: String,
    pub state_reason_code: String,
    pub timeout: i64,
    pub tracing_config: String,
    pub version: String,
    pub vpc_config: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetFunctionOutput {
    pub code: String,
    pub concurrency: String,
    pub configuration: String,
    pub tags: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListFunctionsOutput {
    pub functions: Vec<String>,
    pub next_marker: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateFunctionCodeOutput {
    pub architectures: Vec<String>,
    pub code_sha256: String,
    pub code_size: i64,
    pub function_arn: String,
    pub function_name: String,
    pub handler: String,
    pub last_modified: String,
    pub last_update_status: String,
    pub last_update_status_reason: String,
    pub last_update_status_reason_code: String,
    pub revision_id: String,
    pub runtime: String,
    pub state: String,
    pub state_reason: String,
    pub state_reason_code: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateFunctionConfigurationOutput {
    pub architectures: Vec<String>,
    pub code_sha256: String,
    pub code_size: i64,
    pub dead_letter_config: String,
    pub description: String,
    pub environment: String,
    pub ephemeral_storage: String,
    pub file_system_configs: Vec<String>,
    pub function_arn: String,
    pub function_name: String,
    pub handler: String,
    pub image_config_response: String,
    pub kms_key_arn: String,
    pub last_modified: String,
    pub last_update_status: String,
    pub layers: Vec<String>,
    pub logging_config: String,
    pub memory_size: i64,
    pub package_type: String,
    pub revision_id: String,
    pub role: String,
    pub runtime: String,
    pub snap_start: String,
    pub state: String,
    pub state_reason: String,
    pub state_reason_code: String,
    pub timeout: i64,
    pub tracing_config: String,
    pub version: String,
    pub vpc_config: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct InvokeOutput {
    pub executed_version: String,
    pub function_error: String,
    pub log_result: String,
    pub payload: String,
    pub status_code: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PublishVersionOutput {
    pub architectures: Vec<String>,
    pub code_sha256: String,
    pub code_size: i64,
    pub description: String,
    pub function_arn: String,
    pub function_name: String,
    pub handler: String,
    pub last_modified: String,
    pub memory_size: i64,
    pub package_type: String,
    pub revision_id: String,
    pub role: String,
    pub runtime: String,
    pub state: String,
    pub state_reason: String,
    pub state_reason_code: String,
    pub timeout: i64,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListVersionsByFunctionOutput {
    pub next_marker: String,
    pub versions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateAliasOutput {
    pub alias_arn: String,
    pub description: String,
    pub function_version: String,
    pub name: String,
    pub revision_id: String,
    pub routing_config: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetAliasOutput {
    pub alias_arn: String,
    pub description: String,
    pub function_version: String,
    pub name: String,
    pub revision_id: String,
    pub routing_config: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListAliasesOutput {
    pub aliases: Vec<String>,
    pub next_marker: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateAliasOutput {
    pub alias_arn: String,
    pub description: String,
    pub function_version: String,
    pub name: String,
    pub revision_id: String,
    pub routing_config: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateEventSourceMappingOutput {
    pub amazon_managed_kafka_event_source_config: String,
    pub batch_size: i64,
    pub bisect_batch_on_function_error: bool,
    pub destination_config: String,
    pub document_db_event_source_config: String,
    pub event_source_arn: String,
    pub filter_criteria: String,
    pub function_arn: String,
    pub function_response_types: Vec<String>,
    pub last_modified: chrono::DateTime<chrono::Utc>,
    pub last_processing_result: String,
    pub maximum_batching_window_in_seconds: i64,
    pub maximum_record_age_in_seconds: i64,
    pub maximum_retry_attempts: i64,
    pub parallelization_factor: i64,
    pub queues: Vec<String>,
    pub scaling_config: String,
    pub self_managed_event_source: String,
    pub self_managed_kafka_event_source_config: String,
    pub source_access_configurations: Vec<String>,
    pub starting_position: String,
    pub starting_position_timestamp: chrono::DateTime<chrono::Utc>,
    pub state: String,
    pub state_transition_reason: String,
    pub topics: Vec<String>,
    pub tumbling_window_in_seconds: i64,
    pub uuid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteEventSourceMappingOutput {
    pub event_source_arn: String,
    pub function_arn: String,
    pub state: String,
    pub uuid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetEventSourceMappingOutput {
    pub amazon_managed_kafka_event_source_config: String,
    pub batch_size: i64,
    pub bisect_batch_on_function_error: bool,
    pub destination_config: String,
    pub event_source_arn: String,
    pub filter_criteria: String,
    pub function_arn: String,
    pub last_modified: chrono::DateTime<chrono::Utc>,
    pub last_processing_result: String,
    pub maximum_batching_window_in_seconds: i64,
    pub maximum_record_age_in_seconds: i64,
    pub maximum_retry_attempts: i64,
    pub parallelization_factor: i64,
    pub queues: Vec<String>,
    pub scaling_config: String,
    pub self_managed_event_source: String,
    pub source_access_configurations: Vec<String>,
    pub starting_position: String,
    pub state: String,
    pub state_transition_reason: String,
    pub topics: Vec<String>,
    pub tumbling_window_in_seconds: i64,
    pub uuid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListEventSourceMappingsOutput {
    pub event_source_mappings: Vec<String>,
    pub next_marker: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateEventSourceMappingOutput {
    pub batch_size: i64,
    pub bisect_batch_on_function_error: bool,
    pub destination_config: String,
    pub event_source_arn: String,
    pub filter_criteria: String,
    pub function_arn: String,
    pub last_modified: chrono::DateTime<chrono::Utc>,
    pub maximum_batching_window_in_seconds: i64,
    pub maximum_record_age_in_seconds: i64,
    pub maximum_retry_attempts: i64,
    pub parallelization_factor: i64,
    pub scaling_config: String,
    pub source_access_configurations: Vec<String>,
    pub state: String,
    pub state_transition_reason: String,
    pub tumbling_window_in_seconds: i64,
    pub uuid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetPolicyOutput {
    pub policy: String,
    pub revision_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateFunctionUrlConfigOutput {
    pub auth_type: String,
    pub cors: String,
    pub creation_time: String,
    pub function_arn: String,
    pub function_url: String,
    pub invoke_mode: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetFunctionUrlConfigOutput {
    pub auth_type: String,
    pub cors: String,
    pub creation_time: String,
    pub function_arn: String,
    pub function_url: String,
    pub invoke_mode: String,
    pub last_modified_time: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateFunctionUrlConfigOutput {
    pub auth_type: String,
    pub cors: String,
    pub creation_time: String,
    pub function_arn: String,
    pub function_url: String,
    pub invoke_mode: String,
    pub last_modified_time: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListFunctionUrlConfigsOutput {
    pub function_url_configs: Vec<String>,
    pub next_marker: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PutProvisionedConcurrencyConfigOutput {
    pub allocated_provisioned_concurrent_executions: i64,
    pub available_provisioned_concurrent_executions: i64,
    pub last_modified: String,
    pub requested_provisioned_concurrent_executions: i64,
    pub status: String,
    pub status_reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetProvisionedConcurrencyConfigOutput {
    pub allocated_provisioned_concurrent_executions: i64,
    pub available_provisioned_concurrent_executions: i64,
    pub last_modified: String,
    pub requested_provisioned_concurrent_executions: i64,
    pub status: String,
    pub status_reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListProvisionedConcurrencyConfigsOutput {
    pub next_marker: String,
    pub provisioned_concurrency_configs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PublishLayerVersionOutput {
    pub compatible_architectures: Vec<String>,
    pub compatible_runtimes: Vec<String>,
    pub content: String,
    pub created_date: String,
    pub description: String,
    pub layer_arn: String,
    pub layer_version_arn: String,
    pub license_info: String,
    pub version: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetLayerVersionOutput {
    pub compatible_architectures: Vec<String>,
    pub compatible_runtimes: Vec<String>,
    pub content: String,
    pub created_date: String,
    pub description: String,
    pub layer_arn: String,
    pub layer_version_arn: String,
    pub license_info: String,
    pub version: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListLayersOutput {
    pub layers: Vec<String>,
    pub next_marker: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListLayerVersionsOutput {
    pub layer_versions: Vec<String>,
    pub next_marker: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AddLayerVersionPermissionOutput {
    pub revision_id: String,
    pub statement: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetLayerVersionPolicyOutput {
    pub policy: String,
    pub revision_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsLambdaFunction {
    pub account_id: String,
    pub architectures: String,
    pub code_sha256: String,
    pub code_size: i64,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub dead_letter_arn: String,
    pub description: String,
    pub function_arn: String,
    pub function_name: String,
    pub handler: String,
    pub kms_key_arn: String,
    pub last_modified: chrono::DateTime<chrono::Utc>,
    pub memory_size: i64,
    pub package_type: String,
    pub region: String,
    pub role: String,
    pub runtime: String,
    pub state: String,
    pub tags: String,
    pub timeout: i64,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsLambdaFunctionVersion {
    pub code_sha256: String,
    pub code_size: i64,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub description: String,
    pub function_id: String,
    pub handler: String,
    pub memory_size: i64,
    pub runtime: String,
    pub timeout: i64,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsLambdaAlias {
    pub alias_arn: String,
    pub alias_name: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub description: String,
    pub function_id: String,
    pub function_version: String,
    pub routing_config: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsLambdaEventSourceMapping {
    pub batch_size: i64,
    #[serde(default)]
    pub bisect_batch_on_error: bool,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(default)]
    pub enabled: bool,
    pub event_source_arn: String,
    pub function_id: String,
    pub last_processing_result: String,
    pub maximum_batching_window: i64,
    pub maximum_retry_attempts: i64,
    pub state: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub uuid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsLambdaLayer {
    pub account_id: String,
    pub compatible_architectures: String,
    pub compatible_runtimes: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub description: String,
    pub layer_arn: String,
    pub layer_name: String,
    pub latest_version: i64,
    pub license_info: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsLambdaInvocationLog {
    pub billed_duration_ms: i64,
    pub duration_ms: i64,
    pub error: String,
    pub function_id: String,
    pub invocation_type: String,
    #[serde(default = "chrono::Utc::now")]
    pub invoked_at: chrono::DateTime<chrono::Utc>,
    pub max_memory_used_mb: i64,
    pub request_id: String,
    pub status: String,
}

#[async_trait]
pub trait LambdaAction {
    async fn create_function(&self, architectures: String, code: String, code_signing_config_arn: String, dead_letter_config: String, description: String, environment: String, ephemeral_storage: String, file_system_configs: Vec<String>, function_name: String, handler: String, image_config: String, kms_key_arn: String, layers: Vec<String>, logging_config: String, memory_size: i64, package_type: String, publish: bool, region: String, role: String, runtime: String, snap_start: String, tags: String, timeout: i64, tracing_config: String, vpc_config: String) -> Result<CreateFunctionOutput, Box<dyn std::error::Error>>;
    async fn delete_function(&self, function_name: String, qualifier: String, region: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn get_function(&self, function_name: String, qualifier: String, region: String) -> Result<GetFunctionOutput, Box<dyn std::error::Error>>;
    async fn list_functions(&self, function_version: String, marker: String, master_region: String, max_items: i64, region: String) -> Result<ListFunctionsOutput, Box<dyn std::error::Error>>;
    async fn update_function_code(&self, architectures: String, dry_run: bool, function_name: String, image_uri: String, publish: bool, region: String, revision_id: String, s3_bucket: String, s3_key: String, s3_object_version: String, zip_file: String) -> Result<UpdateFunctionCodeOutput, Box<dyn std::error::Error>>;
    async fn update_function_configuration(&self, dead_letter_config: String, description: String, environment: String, ephemeral_storage: String, file_system_configs: Vec<String>, function_name: String, handler: String, image_config: String, kms_key_arn: String, layers: Vec<String>, logging_config: String, memory_size: i64, region: String, revision_id: String, role: String, runtime: String, snap_start: String, timeout: i64, tracing_config: String, vpc_config: String) -> Result<UpdateFunctionConfigurationOutput, Box<dyn std::error::Error>>;
    async fn invoke(&self, client_context: String, function_name: String, invocation_type: String, log_type: String, payload: String, qualifier: String, region: String) -> Result<InvokeOutput, Box<dyn std::error::Error>>;
    async fn invoke_async(&self, function_name: String, invoke_args: String, region: String) -> Result<i64, Box<dyn std::error::Error>>;
    async fn publish_version(&self, code_sha256: String, description: String, function_name: String, region: String, revision_id: String) -> Result<PublishVersionOutput, Box<dyn std::error::Error>>;
    async fn list_versions_by_function(&self, function_name: String, marker: String, max_items: i64, region: String) -> Result<ListVersionsByFunctionOutput, Box<dyn std::error::Error>>;
    async fn create_alias(&self, description: String, function_name: String, function_version: String, name: String, region: String, routing_config: String) -> Result<CreateAliasOutput, Box<dyn std::error::Error>>;
    async fn delete_alias(&self, function_name: String, name: String, region: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn get_alias(&self, function_name: String, name: String, region: String) -> Result<GetAliasOutput, Box<dyn std::error::Error>>;
    async fn list_aliases(&self, function_name: String, function_version: String, marker: String, max_items: i64, region: String) -> Result<ListAliasesOutput, Box<dyn std::error::Error>>;
    async fn update_alias(&self, description: String, function_name: String, function_version: String, name: String, region: String, revision_id: String, routing_config: String) -> Result<UpdateAliasOutput, Box<dyn std::error::Error>>;
    async fn create_event_source_mapping(&self, amazon_managed_kafka_event_source_config: String, batch_size: i64, bisect_batch_on_function_error: bool, destination_config: String, document_db_event_source_config: String, enabled: bool, event_source_arn: String, filter_criteria: String, function_name: String, function_response_types: Vec<String>, maximum_batching_window_in_seconds: i64, maximum_record_age_in_seconds: i64, maximum_retry_attempts: i64, parallelization_factor: i64, queues: Vec<String>, region: String, scaling_config: String, self_managed_event_source: String, self_managed_kafka_event_source_config: String, source_access_configurations: Vec<String>, starting_position: String, starting_position_timestamp: chrono::DateTime<chrono::Utc>, topics: Vec<String>, tumbling_window_in_seconds: i64) -> Result<CreateEventSourceMappingOutput, Box<dyn std::error::Error>>;
    async fn delete_event_source_mapping(&self, region: String, uuid: String) -> Result<DeleteEventSourceMappingOutput, Box<dyn std::error::Error>>;
    async fn get_event_source_mapping(&self, region: String, uuid: String) -> Result<GetEventSourceMappingOutput, Box<dyn std::error::Error>>;
    async fn list_event_source_mappings(&self, event_source_arn: String, function_name: String, marker: String, max_items: i64, region: String) -> Result<ListEventSourceMappingsOutput, Box<dyn std::error::Error>>;
    async fn update_event_source_mapping(&self, batch_size: i64, bisect_batch_on_function_error: bool, destination_config: String, document_db_event_source_config: String, enabled: bool, filter_criteria: String, function_name: String, function_response_types: Vec<String>, maximum_batching_window_in_seconds: i64, maximum_record_age_in_seconds: i64, maximum_retry_attempts: i64, parallelization_factor: i64, region: String, scaling_config: String, source_access_configurations: Vec<String>, tumbling_window_in_seconds: i64, uuid: String) -> Result<UpdateEventSourceMappingOutput, Box<dyn std::error::Error>>;
    async fn add_permission(&self, method: String, event_source_token: String, function_name: String, function_url_auth_type: String, principal: String, principal_org_id: String, qualifier: String, region: String, revision_id: String, source_account: String, source_arn: String, statement_id: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn remove_permission(&self, function_name: String, qualifier: String, region: String, revision_id: String, statement_id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn get_policy(&self, function_name: String, qualifier: String, region: String) -> Result<GetPolicyOutput, Box<dyn std::error::Error>>;
    async fn create_function_url_config(&self, auth_type: String, cors: String, function_name: String, invoke_mode: String, qualifier: String, region: String) -> Result<CreateFunctionUrlConfigOutput, Box<dyn std::error::Error>>;
    async fn delete_function_url_config(&self, function_name: String, qualifier: String, region: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn get_function_url_config(&self, function_name: String, qualifier: String, region: String) -> Result<GetFunctionUrlConfigOutput, Box<dyn std::error::Error>>;
    async fn update_function_url_config(&self, auth_type: String, cors: String, function_name: String, invoke_mode: String, qualifier: String, region: String) -> Result<UpdateFunctionUrlConfigOutput, Box<dyn std::error::Error>>;
    async fn list_function_url_configs(&self, function_name: String, marker: String, max_items: i64, region: String) -> Result<ListFunctionUrlConfigsOutput, Box<dyn std::error::Error>>;
    async fn put_function_concurrency(&self, function_name: String, region: String, reserved_concurrent_executions: i64) -> Result<i64, Box<dyn std::error::Error>>;
    async fn delete_function_concurrency(&self, function_name: String, region: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn get_function_concurrency(&self, function_name: String, region: String) -> Result<i64, Box<dyn std::error::Error>>;
    async fn put_provisioned_concurrency_config(&self, function_name: String, provisioned_concurrent_executions: i64, qualifier: String, region: String) -> Result<PutProvisionedConcurrencyConfigOutput, Box<dyn std::error::Error>>;
    async fn delete_provisioned_concurrency_config(&self, function_name: String, qualifier: String, region: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn get_provisioned_concurrency_config(&self, function_name: String, qualifier: String, region: String) -> Result<GetProvisionedConcurrencyConfigOutput, Box<dyn std::error::Error>>;
    async fn list_provisioned_concurrency_configs(&self, function_name: String, marker: String, max_items: i64, region: String) -> Result<ListProvisionedConcurrencyConfigsOutput, Box<dyn std::error::Error>>;
    async fn publish_layer_version(&self, compatible_architectures: String, compatible_runtimes: Vec<String>, content: String, description: String, layer_name: String, license_info: String, region: String) -> Result<PublishLayerVersionOutput, Box<dyn std::error::Error>>;
    async fn delete_layer_version(&self, layer_name: String, region: String, version_number: i64) -> Result<(), Box<dyn std::error::Error>>;
    async fn get_layer_version(&self, layer_name: String, region: String, version_number: i64) -> Result<GetLayerVersionOutput, Box<dyn std::error::Error>>;
    async fn list_layers(&self, compatible_architecture: String, compatible_runtime: String, marker: String, max_items: i64, region: String) -> Result<ListLayersOutput, Box<dyn std::error::Error>>;
    async fn list_layer_versions(&self, compatible_architecture: String, compatible_runtime: String, layer_name: String, marker: String, max_items: i64, region: String) -> Result<ListLayerVersionsOutput, Box<dyn std::error::Error>>;
    async fn add_layer_version_permission(&self, method: String, layer_name: String, organization_id: String, principal: String, region: String, revision_id: String, statement_id: String, version_number: i64) -> Result<AddLayerVersionPermissionOutput, Box<dyn std::error::Error>>;
    async fn remove_layer_version_permission(&self, layer_name: String, region: String, revision_id: String, statement_id: String, version_number: i64) -> Result<(), Box<dyn std::error::Error>>;
    async fn get_layer_version_policy(&self, layer_name: String, region: String, version_number: i64) -> Result<GetLayerVersionPolicyOutput, Box<dyn std::error::Error>>;
    async fn tag_resource(&self, region: String, resource: String, tags: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn untag_resource(&self, region: String, resource: String, tag_keys: Vec<String>) -> Result<(), Box<dyn std::error::Error>>;
    async fn list_tags(&self, region: String, resource: String) -> Result<String, Box<dyn std::error::Error>>;
}
