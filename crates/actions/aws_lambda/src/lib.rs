pub mod output;

#[cfg(test)]
mod tests;

use dashmap::DashMap;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use chrono::Utc;
use uuid::Uuid;

use output::*;

// In-memory storage
static FUNCTIONS: Lazy<DashMap<String, StoredFunction>> = Lazy::new(DashMap::new);

fn generate_arn(region: &str, function_name: &str) -> String {
    format!(
        "arn:aws:lambda:{}:123456789012:function:{}",
        region, function_name
    )
}

fn compute_sha256(data: &[u8]) -> String {
    use std::hash::{Hash, Hasher};
    use std::collections::hash_map::DefaultHasher;
    let mut hasher = DefaultHasher::new();
    data.hash(&mut hasher);
    format!("{:x}", hasher.finish())
}

/// Create a Lambda function
pub async fn create_function(
    function_name: String,
    role: String,
    code: FunctionCode,
    runtime: Option<String>,
    handler: Option<String>,
    description: Option<String>,
    timeout: Option<i32>,
    memory_size: Option<i32>,
    environment: Option<Environment>,
    vpc_config: Option<VpcConfig>,
    dead_letter_config: Option<DeadLetterConfig>,
    tracing_config: Option<TracingConfig>,
    package_type: Option<PackageType>,
    architectures: Option<Vec<Architecture>>,
    layers: Option<Vec<String>>,
    ephemeral_storage: Option<EphemeralStorage>,
    tags: Option<HashMap<String, String>>,
    publish: Option<bool>,
    region: Option<String>,
) -> CreateFunctionOutput {
    let region = region.unwrap_or_else(|| "us-east-1".to_string());
    let function_arn = generate_arn(&region, &function_name);
    let now = Utc::now();
    
    let code_bytes = code.zip_file.unwrap_or_default();
    let code_sha256 = compute_sha256(&code_bytes);
    let code_size = code_bytes.len() as i64;
    let package_type = package_type.unwrap_or(PackageType::Zip);
    let architectures = architectures.unwrap_or_else(|| vec![Architecture::X86_64]);
    let timeout = timeout.unwrap_or(3);
    let memory_size = memory_size.unwrap_or(128);
    let _publish = publish.unwrap_or(false);

    let stored = StoredFunction {
        function_name: function_name.clone(),
        function_arn: function_arn.clone(),
        runtime: runtime.clone(),
        role: role.clone(),
        handler: handler.clone(),
        description: description.clone(),
        timeout,
        memory_size,
        code: code_bytes,
        code_sha256: code_sha256.clone(),
        code_size,
        environment: environment.unwrap_or_default(),
        vpc_config,
        dead_letter_config,
        tracing_config,
        package_type,
        architectures: architectures.clone(),
        layers: layers.unwrap_or_default(),
        ephemeral_storage,
        tags: tags.unwrap_or_default(),
        version: "$LATEST".to_string(),
        state: "Active".to_string(),
        state_reason: None,
        state_reason_code: None,
        reserved_concurrent_executions: None,
        created_at: now,
        last_modified: now,
        region,
    };

    FUNCTIONS.insert(function_name.clone(), stored);

    let arch_strings: Vec<String> = architectures
        .iter()
        .map(|a| match a {
            Architecture::X86_64 => "x86_64".to_string(),
            Architecture::Arm64 => "arm64".to_string(),
        })
        .collect();

    CreateFunctionOutput {
        function_name,
        function_arn,
        runtime,
        role,
        handler,
        code_size,
        code_sha256,
        description,
        timeout,
        memory_size,
        last_modified: now.to_rfc3339(),
        version: "$LATEST".to_string(),
        state: "Active".to_string(),
        state_reason: None,
        state_reason_code: None,
        package_type: format!("{:?}", package_type),
        architectures: arch_strings,
        success: true,
    }
}

/// Delete a Lambda function
pub async fn delete_function(
    function_name: String,
    _qualifier: Option<String>,
    _region: Option<String>,
) -> DeleteFunctionOutput {
    let success = FUNCTIONS.remove(&function_name).is_some();
    DeleteFunctionOutput { success }
}

/// Get a Lambda function
pub async fn get_function(
    function_name: String,
    _qualifier: Option<String>,
    _region: Option<String>,
) -> GetFunctionOutput {
    if let Some(entry) = FUNCTIONS.get(&function_name) {
        let f = entry.value();
        let arch_strings: Vec<String> = f
            .architectures
            .iter()
            .map(|a| match a {
                Architecture::X86_64 => "x86_64".to_string(),
                Architecture::Arm64 => "arm64".to_string(),
            })
            .collect();

        GetFunctionOutput {
            configuration: FunctionConfiguration {
                function_name: f.function_name.clone(),
                function_arn: f.function_arn.clone(),
                runtime: f.runtime.clone(),
                role: f.role.clone(),
                handler: f.handler.clone(),
                code_size: f.code_size,
                code_sha256: f.code_sha256.clone(),
                description: f.description.clone(),
                timeout: f.timeout,
                memory_size: f.memory_size,
                last_modified: f.last_modified.to_rfc3339(),
                version: f.version.clone(),
                state: f.state.clone(),
                state_reason: f.state_reason.clone(),
                state_reason_code: f.state_reason_code.clone(),
                package_type: format!("{:?}", f.package_type),
                architectures: arch_strings,
            },
            code: Some(FunctionCodeLocation {
                repository_type: Some("S3".to_string()),
                location: Some(format!("https://awslambda-{}-tasks.s3.amazonaws.com/...", f.region)),
                image_uri: None,
                resolved_image_uri: None,
            }),
            tags: f.tags.clone(),
            concurrency: f.reserved_concurrent_executions.map(|r| Concurrency {
                reserved_concurrent_executions: Some(r),
            }),
        }
    } else {
        GetFunctionOutput {
            configuration: FunctionConfiguration {
                function_name: function_name.clone(),
                function_arn: String::new(),
                runtime: None,
                role: String::new(),
                handler: None,
                code_size: 0,
                code_sha256: String::new(),
                description: None,
                timeout: 0,
                memory_size: 0,
                last_modified: String::new(),
                version: String::new(),
                state: "Not Found".to_string(),
                state_reason: None,
                state_reason_code: None,
                package_type: String::new(),
                architectures: Vec::new(),
            },
            code: None,
            tags: HashMap::new(),
            concurrency: None,
        }
    }
}

/// List Lambda functions
pub async fn list_functions(
    max_items: Option<i32>,
    _marker: Option<String>,
    _master_region: Option<String>,
    _function_version: Option<String>,
    _region: Option<String>,
) -> ListFunctionsOutput {
    let max_items = max_items.unwrap_or(50) as usize;

    let functions: Vec<FunctionConfiguration> = FUNCTIONS
        .iter()
        .take(max_items)
        .map(|entry| {
            let f = entry.value();
            let arch_strings: Vec<String> = f
                .architectures
                .iter()
                .map(|a| match a {
                    Architecture::X86_64 => "x86_64".to_string(),
                    Architecture::Arm64 => "arm64".to_string(),
                })
                .collect();

            FunctionConfiguration {
                function_name: f.function_name.clone(),
                function_arn: f.function_arn.clone(),
                runtime: f.runtime.clone(),
                role: f.role.clone(),
                handler: f.handler.clone(),
                code_size: f.code_size,
                code_sha256: f.code_sha256.clone(),
                description: f.description.clone(),
                timeout: f.timeout,
                memory_size: f.memory_size,
                last_modified: f.last_modified.to_rfc3339(),
                version: f.version.clone(),
                state: f.state.clone(),
                state_reason: f.state_reason.clone(),
                state_reason_code: f.state_reason_code.clone(),
                package_type: format!("{:?}", f.package_type),
                architectures: arch_strings,
            }
        })
        .collect();

    ListFunctionsOutput {
        functions,
        next_marker: None,
    }
}

/// Update Lambda function code
pub async fn update_function_code(
    function_name: String,
    zip_file: Option<Vec<u8>>,
    s3_bucket: Option<String>,
    s3_key: Option<String>,
    s3_object_version: Option<String>,
    image_uri: Option<String>,
    architectures: Option<Vec<Architecture>>,
    publish: Option<bool>,
    _dry_run: Option<bool>,
    _revision_id: Option<String>,
    _region: Option<String>,
) -> UpdateFunctionCodeOutput {
    let mut success = false;
    let mut output = UpdateFunctionCodeOutput {
        function_name: function_name.clone(),
        function_arn: String::new(),
        runtime: None,
        handler: None,
        code_size: 0,
        code_sha256: String::new(),
        last_modified: String::new(),
        version: String::new(),
        state: "Not Found".to_string(),
        state_reason: None,
        state_reason_code: None,
        last_update_status: None,
        last_update_status_reason: None,
        last_update_status_reason_code: None,
        architectures: Vec::new(),
        success: false,
    };

    if let Some(mut entry) = FUNCTIONS.get_mut(&function_name) {
        let now = Utc::now();
        
        // Update code
        if let Some(code) = zip_file {
            entry.code = code.clone();
            entry.code_sha256 = compute_sha256(&code);
            entry.code_size = code.len() as i64;
        }
        
        // Update architectures if provided
        if let Some(arch) = architectures {
            entry.architectures = arch;
        }
        
        entry.last_modified = now;

        let arch_strings: Vec<String> = entry
            .architectures
            .iter()
            .map(|a| match a {
                Architecture::X86_64 => "x86_64".to_string(),
                Architecture::Arm64 => "arm64".to_string(),
            })
            .collect();

        output.function_arn = entry.function_arn.clone();
        output.runtime = entry.runtime.clone();
        output.handler = entry.handler.clone();
        output.code_size = entry.code_size;
        output.code_sha256 = entry.code_sha256.clone();
        output.last_modified = now.to_rfc3339();
        output.version = entry.version.clone();
        output.state = entry.state.clone();
        output.last_update_status = Some("Successful".to_string());
        output.architectures = arch_strings;
        success = true;
    }

    output.success = success;
    output
}

/// Update Lambda function configuration
pub async fn update_function_configuration(
    function_name: String,
    runtime: Option<String>,
    role: Option<String>,
    handler: Option<String>,
    description: Option<String>,
    timeout: Option<i32>,
    memory_size: Option<i32>,
    environment: Option<Environment>,
    vpc_config: Option<VpcConfig>,
    dead_letter_config: Option<DeadLetterConfig>,
    tracing_config: Option<TracingConfig>,
    layers: Option<Vec<String>>,
    ephemeral_storage: Option<EphemeralStorage>,
    _revision_id: Option<String>,
    _region: Option<String>,
) -> UpdateFunctionConfigurationOutput {
    let mut success = false;
    let mut output = UpdateFunctionConfigurationOutput {
        function_name: function_name.clone(),
        function_arn: String::new(),
        runtime: None,
        role: String::new(),
        handler: None,
        code_size: 0,
        code_sha256: String::new(),
        description: None,
        timeout: 0,
        memory_size: 0,
        last_modified: String::new(),
        version: String::new(),
        state: "Not Found".to_string(),
        state_reason: None,
        state_reason_code: None,
        package_type: String::new(),
        architectures: Vec::new(),
        success: false,
    };

    if let Some(mut entry) = FUNCTIONS.get_mut(&function_name) {
        let now = Utc::now();
        
        if let Some(r) = runtime {
            entry.runtime = Some(r);
        }
        if let Some(r) = role {
            entry.role = r;
        }
        if let Some(h) = handler {
            entry.handler = Some(h);
        }
        if let Some(d) = description {
            entry.description = Some(d);
        }
        if let Some(t) = timeout {
            entry.timeout = t;
        }
        if let Some(m) = memory_size {
            entry.memory_size = m;
        }
        if let Some(e) = environment {
            entry.environment = e;
        }
        if let Some(v) = vpc_config {
            entry.vpc_config = Some(v);
        }
        if let Some(d) = dead_letter_config {
            entry.dead_letter_config = Some(d);
        }
        if let Some(t) = tracing_config {
            entry.tracing_config = Some(t);
        }
        if let Some(l) = layers {
            entry.layers = l;
        }
        if let Some(e) = ephemeral_storage {
            entry.ephemeral_storage = Some(e);
        }
        
        entry.last_modified = now;

        let arch_strings: Vec<String> = entry
            .architectures
            .iter()
            .map(|a| match a {
                Architecture::X86_64 => "x86_64".to_string(),
                Architecture::Arm64 => "arm64".to_string(),
            })
            .collect();

        output.function_arn = entry.function_arn.clone();
        output.runtime = entry.runtime.clone();
        output.role = entry.role.clone();
        output.handler = entry.handler.clone();
        output.description = entry.description.clone();
        output.timeout = entry.timeout;
        output.memory_size = entry.memory_size;
        output.code_size = entry.code_size;
        output.code_sha256 = entry.code_sha256.clone();
        output.last_modified = now.to_rfc3339();
        output.version = entry.version.clone();
        output.state = entry.state.clone();
        output.package_type = format!("{:?}", entry.package_type);
        output.architectures = arch_strings;
        success = true;
    }

    output.success = success;
    output
}

/// Invoke a Lambda function
pub async fn invoke(
    function_name: String,
    payload: Option<Vec<u8>>,
    invocation_type: Option<InvocationType>,
    log_type: Option<LogType>,
    _client_context: Option<String>,
    _qualifier: Option<String>,
    _region: Option<String>,
) -> InvokeOutput {
    let invocation_type = invocation_type.unwrap_or(InvocationType::RequestResponse);
    let log_type = log_type.unwrap_or(LogType::None);

    if let Some(_entry) = FUNCTIONS.get(&function_name) {
        // Simulate function execution
        let response_payload = match invocation_type {
            InvocationType::RequestResponse => {
                // Echo back the payload for demo purposes
                payload.map(|p| {
                    serde_json::json!({
                        "statusCode": 200,
                        "body": String::from_utf8_lossy(&p).to_string()
                    })
                    .to_string()
                    .into_bytes()
                })
            }
            InvocationType::Event => None, // Async invocation, no response
            InvocationType::DryRun => None, // Validation only
        };

        let log_result = match log_type {
            LogType::Tail => {
                Some(base64::Engine::encode(
                    &base64::engine::general_purpose::STANDARD,
                    "START RequestId: abc-123\nEND RequestId: abc-123\nREPORT RequestId: abc-123",
                ))
            }
            LogType::None => None,
        };

        InvokeOutput {
            status_code: match invocation_type {
                InvocationType::RequestResponse => 200,
                InvocationType::Event => 202,
                InvocationType::DryRun => 204,
            },
            payload: response_payload,
            function_error: None,
            log_result,
            executed_version: "$LATEST".to_string(),
        }
    } else {
        InvokeOutput {
            status_code: 404,
            payload: None,
            function_error: Some("Function not found".to_string()),
            log_result: None,
            executed_version: String::new(),
        }
    }
}

// Utility functions for testing
#[cfg(test)]
pub fn clear_all_data() {
    FUNCTIONS.clear();
}
