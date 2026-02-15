// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateFunctionInput {
    pub architectures: String,
    pub code: String,
    pub code_signing_config_arn: String,
    pub dead_letter_config: String,
    pub description: String,
    pub environment: String,
    pub ephemeral_storage: String,
    pub file_system_configs: Vec<String>,
    pub function_name: String,
    pub handler: String,
    pub image_config: String,
    pub kms_key_arn: String,
    pub layers: Vec<String>,
    pub logging_config: String,
    pub memory_size: i64,
    pub package_type: String,
    #[serde(default)]
    pub publish: bool,
    pub region: String,
    pub role: String,
    pub runtime: String,
    pub snap_start: String,
    pub tags: String,
    pub timeout: i64,
    pub tracing_config: String,
    pub vpc_config: String,
}

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
    pub success: bool,
    pub timeout: i64,
    pub tracing_config: String,
    pub version: String,
    pub vpc_config: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteFunctionInput {
    pub function_name: String,
    pub qualifier: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteFunctionOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetFunctionInput {
    pub function_name: String,
    pub qualifier: String,
    pub region: String,
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
pub struct ListFunctionsInput {
    pub function_version: String,
    pub marker: String,
    pub master_region: String,
    pub max_items: i64,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListFunctionsOutput {
    pub functions: Vec<String>,
    pub next_marker: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateFunctionCodeInput {
    pub architectures: String,
    #[serde(default)]
    pub dry_run: bool,
    pub function_name: String,
    pub image_uri: String,
    #[serde(default)]
    pub publish: bool,
    pub region: String,
    pub revision_id: String,
    pub s3_bucket: String,
    pub s3_key: String,
    pub s3_object_version: String,
    pub zip_file: String,
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
    pub success: bool,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateFunctionConfigurationInput {
    pub dead_letter_config: String,
    pub description: String,
    pub environment: String,
    pub ephemeral_storage: String,
    pub file_system_configs: Vec<String>,
    pub function_name: String,
    pub handler: String,
    pub image_config: String,
    pub kms_key_arn: String,
    pub layers: Vec<String>,
    pub logging_config: String,
    pub memory_size: i64,
    pub region: String,
    pub revision_id: String,
    pub role: String,
    pub runtime: String,
    pub snap_start: String,
    pub timeout: i64,
    pub tracing_config: String,
    pub vpc_config: String,
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
    pub success: bool,
    pub timeout: i64,
    pub tracing_config: String,
    pub version: String,
    pub vpc_config: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct InvokeInput {
    pub client_context: String,
    pub function_name: String,
    pub invocation_type: String,
    pub log_type: String,
    pub payload: String,
    pub qualifier: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct InvokeOutput {
    pub executed_version: String,
    pub function_error: String,
    pub log_result: String,
    pub payload: String,
    pub status_code: i64,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct InvokeAsyncInput {
    pub function_name: String,
    pub invoke_args: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct InvokeAsyncOutput {
    pub status: i64,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PublishVersionInput {
    pub code_sha256: String,
    pub description: String,
    pub function_name: String,
    pub region: String,
    pub revision_id: String,
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
    pub success: bool,
    pub timeout: i64,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListVersionsByFunctionInput {
    pub function_name: String,
    pub marker: String,
    pub max_items: i64,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListVersionsByFunctionOutput {
    pub next_marker: String,
    pub versions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateAliasInput {
    pub description: String,
    pub function_name: String,
    pub function_version: String,
    pub name: String,
    pub region: String,
    pub routing_config: String,
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
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteAliasInput {
    pub function_name: String,
    pub name: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteAliasOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetAliasInput {
    pub function_name: String,
    pub name: String,
    pub region: String,
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
pub struct ListAliasesInput {
    pub function_name: String,
    pub function_version: String,
    pub marker: String,
    pub max_items: i64,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListAliasesOutput {
    pub aliases: Vec<String>,
    pub next_marker: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateAliasInput {
    pub description: String,
    pub function_name: String,
    pub function_version: String,
    pub name: String,
    pub region: String,
    pub revision_id: String,
    pub routing_config: String,
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
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateEventSourceMappingInput {
    pub amazon_managed_kafka_event_source_config: String,
    pub batch_size: i64,
    pub bisect_batch_on_function_error: bool,
    pub destination_config: String,
    pub document_db_event_source_config: String,
    #[serde(default)]
    pub enabled: bool,
    pub event_source_arn: String,
    pub filter_criteria: String,
    pub function_name: String,
    pub function_response_types: Vec<String>,
    pub maximum_batching_window_in_seconds: i64,
    pub maximum_record_age_in_seconds: i64,
    pub maximum_retry_attempts: i64,
    pub parallelization_factor: i64,
    pub queues: Vec<String>,
    pub region: String,
    pub scaling_config: String,
    pub self_managed_event_source: String,
    pub self_managed_kafka_event_source_config: String,
    pub source_access_configurations: Vec<String>,
    pub starting_position: String,
    pub starting_position_timestamp: chrono::DateTime<chrono::Utc>,
    pub topics: Vec<String>,
    pub tumbling_window_in_seconds: i64,
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
    pub success: bool,
    pub topics: Vec<String>,
    pub tumbling_window_in_seconds: i64,
    pub uuid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteEventSourceMappingInput {
    pub region: String,
    pub uuid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteEventSourceMappingOutput {
    pub event_source_arn: String,
    pub function_arn: String,
    pub state: String,
    pub success: bool,
    pub uuid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetEventSourceMappingInput {
    pub region: String,
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
pub struct ListEventSourceMappingsInput {
    pub event_source_arn: String,
    pub function_name: String,
    pub marker: String,
    pub max_items: i64,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListEventSourceMappingsOutput {
    pub event_source_mappings: Vec<String>,
    pub next_marker: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateEventSourceMappingInput {
    pub batch_size: i64,
    pub bisect_batch_on_function_error: bool,
    pub destination_config: String,
    pub document_db_event_source_config: String,
    pub enabled: bool,
    pub filter_criteria: String,
    pub function_name: String,
    pub function_response_types: Vec<String>,
    pub maximum_batching_window_in_seconds: i64,
    pub maximum_record_age_in_seconds: i64,
    pub maximum_retry_attempts: i64,
    pub parallelization_factor: i64,
    pub region: String,
    pub scaling_config: String,
    pub source_access_configurations: Vec<String>,
    pub tumbling_window_in_seconds: i64,
    pub uuid: String,
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
    pub success: bool,
    pub tumbling_window_in_seconds: i64,
    pub uuid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AddPermissionInput {
    pub method: String,
    pub event_source_token: String,
    pub function_name: String,
    pub function_url_auth_type: String,
    pub principal: String,
    pub principal_org_id: String,
    pub qualifier: String,
    pub region: String,
    pub revision_id: String,
    pub source_account: String,
    pub source_arn: String,
    pub statement_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AddPermissionOutput {
    pub statement: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RemovePermissionInput {
    pub function_name: String,
    pub qualifier: String,
    pub region: String,
    pub revision_id: String,
    pub statement_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RemovePermissionOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetPolicyInput {
    pub function_name: String,
    pub qualifier: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetPolicyOutput {
    pub policy: String,
    pub revision_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateFunctionUrlConfigInput {
    pub auth_type: String,
    pub cors: String,
    pub function_name: String,
    pub invoke_mode: String,
    pub qualifier: String,
    pub region: String,
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
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteFunctionUrlConfigInput {
    pub function_name: String,
    pub qualifier: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteFunctionUrlConfigOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetFunctionUrlConfigInput {
    pub function_name: String,
    pub qualifier: String,
    pub region: String,
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
pub struct UpdateFunctionUrlConfigInput {
    pub auth_type: String,
    pub cors: String,
    pub function_name: String,
    pub invoke_mode: String,
    pub qualifier: String,
    pub region: String,
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
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListFunctionUrlConfigsInput {
    pub function_name: String,
    pub marker: String,
    pub max_items: i64,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListFunctionUrlConfigsOutput {
    pub function_url_configs: Vec<String>,
    pub next_marker: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PutFunctionConcurrencyInput {
    pub function_name: String,
    pub region: String,
    pub reserved_concurrent_executions: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PutFunctionConcurrencyOutput {
    pub reserved_concurrent_executions: i64,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteFunctionConcurrencyInput {
    pub function_name: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteFunctionConcurrencyOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetFunctionConcurrencyInput {
    pub function_name: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetFunctionConcurrencyOutput {
    pub reserved_concurrent_executions: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PutProvisionedConcurrencyConfigInput {
    pub function_name: String,
    pub provisioned_concurrent_executions: i64,
    pub qualifier: String,
    pub region: String,
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
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteProvisionedConcurrencyConfigInput {
    pub function_name: String,
    pub qualifier: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteProvisionedConcurrencyConfigOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetProvisionedConcurrencyConfigInput {
    pub function_name: String,
    pub qualifier: String,
    pub region: String,
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
pub struct ListProvisionedConcurrencyConfigsInput {
    pub function_name: String,
    pub marker: String,
    pub max_items: i64,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListProvisionedConcurrencyConfigsOutput {
    pub next_marker: String,
    pub provisioned_concurrency_configs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PublishLayerVersionInput {
    pub compatible_architectures: String,
    pub compatible_runtimes: Vec<String>,
    pub content: String,
    pub description: String,
    pub layer_name: String,
    pub license_info: String,
    pub region: String,
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
    pub success: bool,
    pub version: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteLayerVersionInput {
    pub layer_name: String,
    pub region: String,
    pub version_number: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteLayerVersionOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetLayerVersionInput {
    pub layer_name: String,
    pub region: String,
    pub version_number: i64,
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
pub struct ListLayersInput {
    pub compatible_architecture: String,
    pub compatible_runtime: String,
    pub marker: String,
    pub max_items: i64,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListLayersOutput {
    pub layers: Vec<String>,
    pub next_marker: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListLayerVersionsInput {
    pub compatible_architecture: String,
    pub compatible_runtime: String,
    pub layer_name: String,
    pub marker: String,
    pub max_items: i64,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListLayerVersionsOutput {
    pub layer_versions: Vec<String>,
    pub next_marker: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AddLayerVersionPermissionInput {
    pub method: String,
    pub layer_name: String,
    pub organization_id: String,
    pub principal: String,
    pub region: String,
    pub revision_id: String,
    pub statement_id: String,
    pub version_number: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AddLayerVersionPermissionOutput {
    pub revision_id: String,
    pub statement: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RemoveLayerVersionPermissionInput {
    pub layer_name: String,
    pub region: String,
    pub revision_id: String,
    pub statement_id: String,
    pub version_number: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RemoveLayerVersionPermissionOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetLayerVersionPolicyInput {
    pub layer_name: String,
    pub region: String,
    pub version_number: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetLayerVersionPolicyOutput {
    pub policy: String,
    pub revision_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TagResourceInput {
    pub region: String,
    pub resource: String,
    pub tags: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TagResourceOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UntagResourceInput {
    pub region: String,
    pub resource: String,
    pub tag_keys: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UntagResourceOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListTagsInput {
    pub region: String,
    pub resource: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListTagsOutput {
    pub tags: String,
}

#[async_trait]
pub trait LambdaAction {
    async fn create_function(&self, input: CreateFunctionInput) -> Result<CreateFunctionOutput, Box<dyn std::error::Error>>;
    async fn delete_function(&self, input: DeleteFunctionInput) -> Result<DeleteFunctionOutput, Box<dyn std::error::Error>>;
    async fn get_function(&self, input: GetFunctionInput) -> Result<GetFunctionOutput, Box<dyn std::error::Error>>;
    async fn list_functions(&self, input: ListFunctionsInput) -> Result<ListFunctionsOutput, Box<dyn std::error::Error>>;
    async fn update_function_code(&self, input: UpdateFunctionCodeInput) -> Result<UpdateFunctionCodeOutput, Box<dyn std::error::Error>>;
    async fn update_function_configuration(&self, input: UpdateFunctionConfigurationInput) -> Result<UpdateFunctionConfigurationOutput, Box<dyn std::error::Error>>;
    async fn invoke(&self, input: InvokeInput) -> Result<InvokeOutput, Box<dyn std::error::Error>>;
    async fn invoke_async(&self, input: InvokeAsyncInput) -> Result<InvokeAsyncOutput, Box<dyn std::error::Error>>;
    async fn publish_version(&self, input: PublishVersionInput) -> Result<PublishVersionOutput, Box<dyn std::error::Error>>;
    async fn list_versions_by_function(&self, input: ListVersionsByFunctionInput) -> Result<ListVersionsByFunctionOutput, Box<dyn std::error::Error>>;
    async fn create_alias(&self, input: CreateAliasInput) -> Result<CreateAliasOutput, Box<dyn std::error::Error>>;
    async fn delete_alias(&self, input: DeleteAliasInput) -> Result<DeleteAliasOutput, Box<dyn std::error::Error>>;
    async fn get_alias(&self, input: GetAliasInput) -> Result<GetAliasOutput, Box<dyn std::error::Error>>;
    async fn list_aliases(&self, input: ListAliasesInput) -> Result<ListAliasesOutput, Box<dyn std::error::Error>>;
    async fn update_alias(&self, input: UpdateAliasInput) -> Result<UpdateAliasOutput, Box<dyn std::error::Error>>;
    async fn create_event_source_mapping(&self, input: CreateEventSourceMappingInput) -> Result<CreateEventSourceMappingOutput, Box<dyn std::error::Error>>;
    async fn delete_event_source_mapping(&self, input: DeleteEventSourceMappingInput) -> Result<DeleteEventSourceMappingOutput, Box<dyn std::error::Error>>;
    async fn get_event_source_mapping(&self, input: GetEventSourceMappingInput) -> Result<GetEventSourceMappingOutput, Box<dyn std::error::Error>>;
    async fn list_event_source_mappings(&self, input: ListEventSourceMappingsInput) -> Result<ListEventSourceMappingsOutput, Box<dyn std::error::Error>>;
    async fn update_event_source_mapping(&self, input: UpdateEventSourceMappingInput) -> Result<UpdateEventSourceMappingOutput, Box<dyn std::error::Error>>;
    async fn add_permission(&self, input: AddPermissionInput) -> Result<AddPermissionOutput, Box<dyn std::error::Error>>;
    async fn remove_permission(&self, input: RemovePermissionInput) -> Result<RemovePermissionOutput, Box<dyn std::error::Error>>;
    async fn get_policy(&self, input: GetPolicyInput) -> Result<GetPolicyOutput, Box<dyn std::error::Error>>;
    async fn create_function_url_config(&self, input: CreateFunctionUrlConfigInput) -> Result<CreateFunctionUrlConfigOutput, Box<dyn std::error::Error>>;
    async fn delete_function_url_config(&self, input: DeleteFunctionUrlConfigInput) -> Result<DeleteFunctionUrlConfigOutput, Box<dyn std::error::Error>>;
    async fn get_function_url_config(&self, input: GetFunctionUrlConfigInput) -> Result<GetFunctionUrlConfigOutput, Box<dyn std::error::Error>>;
    async fn update_function_url_config(&self, input: UpdateFunctionUrlConfigInput) -> Result<UpdateFunctionUrlConfigOutput, Box<dyn std::error::Error>>;
    async fn list_function_url_configs(&self, input: ListFunctionUrlConfigsInput) -> Result<ListFunctionUrlConfigsOutput, Box<dyn std::error::Error>>;
    async fn put_function_concurrency(&self, input: PutFunctionConcurrencyInput) -> Result<PutFunctionConcurrencyOutput, Box<dyn std::error::Error>>;
    async fn delete_function_concurrency(&self, input: DeleteFunctionConcurrencyInput) -> Result<DeleteFunctionConcurrencyOutput, Box<dyn std::error::Error>>;
    async fn get_function_concurrency(&self, input: GetFunctionConcurrencyInput) -> Result<GetFunctionConcurrencyOutput, Box<dyn std::error::Error>>;
    async fn put_provisioned_concurrency_config(&self, input: PutProvisionedConcurrencyConfigInput) -> Result<PutProvisionedConcurrencyConfigOutput, Box<dyn std::error::Error>>;
    async fn delete_provisioned_concurrency_config(&self, input: DeleteProvisionedConcurrencyConfigInput) -> Result<DeleteProvisionedConcurrencyConfigOutput, Box<dyn std::error::Error>>;
    async fn get_provisioned_concurrency_config(&self, input: GetProvisionedConcurrencyConfigInput) -> Result<GetProvisionedConcurrencyConfigOutput, Box<dyn std::error::Error>>;
    async fn list_provisioned_concurrency_configs(&self, input: ListProvisionedConcurrencyConfigsInput) -> Result<ListProvisionedConcurrencyConfigsOutput, Box<dyn std::error::Error>>;
    async fn publish_layer_version(&self, input: PublishLayerVersionInput) -> Result<PublishLayerVersionOutput, Box<dyn std::error::Error>>;
    async fn delete_layer_version(&self, input: DeleteLayerVersionInput) -> Result<DeleteLayerVersionOutput, Box<dyn std::error::Error>>;
    async fn get_layer_version(&self, input: GetLayerVersionInput) -> Result<GetLayerVersionOutput, Box<dyn std::error::Error>>;
    async fn list_layers(&self, input: ListLayersInput) -> Result<ListLayersOutput, Box<dyn std::error::Error>>;
    async fn list_layer_versions(&self, input: ListLayerVersionsInput) -> Result<ListLayerVersionsOutput, Box<dyn std::error::Error>>;
    async fn add_layer_version_permission(&self, input: AddLayerVersionPermissionInput) -> Result<AddLayerVersionPermissionOutput, Box<dyn std::error::Error>>;
    async fn remove_layer_version_permission(&self, input: RemoveLayerVersionPermissionInput) -> Result<RemoveLayerVersionPermissionOutput, Box<dyn std::error::Error>>;
    async fn get_layer_version_policy(&self, input: GetLayerVersionPolicyInput) -> Result<GetLayerVersionPolicyOutput, Box<dyn std::error::Error>>;
    async fn tag_resource(&self, input: TagResourceInput) -> Result<TagResourceOutput, Box<dyn std::error::Error>>;
    async fn untag_resource(&self, input: UntagResourceInput) -> Result<UntagResourceOutput, Box<dyn std::error::Error>>;
    async fn list_tags(&self, input: ListTagsInput) -> Result<ListTagsOutput, Box<dyn std::error::Error>>;
}
