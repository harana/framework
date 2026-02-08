// Harana Actions - Cloudflare Workflows Module
// This module provides Cloudflare Workflows actions for creating, managing,
// and executing workflow instances and steps.

pub mod output;

use output::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use worker::Env;

fn js_err(e: JsValue) -> String {
    format!("Workflows JS error: {e:?}")
}

/// Get the workflow binding from the environment
fn get_workflow(env: &Env, binding: &str) -> Result<JsValue, String> {
    let val = js_sys::Reflect::get(env, &JsValue::from(binding))
        .map_err(|_| format!("Workflow binding '{binding}' not found"))?;
    if val.is_undefined() {
        return Err(format!("Workflow binding '{binding}' is undefined"));
    }
    Ok(val)
}

/// Create Workflow Instance
pub async fn create(
    env: &Env,
    binding: &str,
    id: Option<&str>,
    params: Option<serde_json::Value>,
) -> Result<CreateOutput, String> {
    let workflow = get_workflow(env, binding)?;

    let create_fn = js_sys::Reflect::get(&workflow, &JsValue::from("create")).map_err(js_err)?;
    let create_fn: js_sys::Function = create_fn
        .dyn_into()
        .map_err(|_| "create is not a function".to_string())?;

    let opts = js_sys::Object::new();
    if let Some(instance_id) = id {
        js_sys::Reflect::set(&opts, &JsValue::from("id"), &JsValue::from(instance_id)).map_err(js_err)?;
    }
    if let Some(p) = &params {
        let params_js =
            serde_wasm_bindgen::to_value(p).map_err(|e| format!("Workflow params serialization error: {e}"))?;
        js_sys::Reflect::set(&opts, &JsValue::from("params"), &params_js).map_err(js_err)?;
    }

    let promise = create_fn.call1(&workflow, &opts).map_err(js_err)?;
    let result = JsFuture::from(js_sys::Promise::from(promise)).await.map_err(js_err)?;

    let instance_id = js_sys::Reflect::get(&result, &JsValue::from("id"))
        .map_err(js_err)?
        .as_string()
        .unwrap_or_default();

    Ok(CreateOutput {
        id: instance_id,
        success: true,
    })
}

/// Create Workflow Instance Batch
pub async fn create_batch(
    env: &Env,
    binding: &str,
    instances: Vec<WorkflowCreateOptions>,
) -> Result<CreateBatchOutput, String> {
    let mut results = Vec::new();

    for instance_opts in &instances {
        match create(env, binding, instance_opts.id.as_deref(), instance_opts.params.clone()).await {
            Ok(output) => {
                results.push(WorkflowInstanceInfo {
                    id: output.id,
                    success: true,
                });
            }
            Err(_e) => {
                results.push(WorkflowInstanceInfo {
                    id: instance_opts.id.clone().unwrap_or_default(),
                    success: false,
                });
            }
        }
    }

    let all_success = results.iter().all(|r| r.success);

    Ok(CreateBatchOutput {
        instances: results,
        success: all_success,
    })
}

/// Get a workflow instance handle
async fn get_instance(env: &Env, binding: &str, id: &str) -> Result<JsValue, String> {
    let workflow = get_workflow(env, binding)?;

    let get_fn = js_sys::Reflect::get(&workflow, &JsValue::from("get")).map_err(js_err)?;
    let get_fn: js_sys::Function = get_fn.dyn_into().map_err(|_| "get is not a function".to_string())?;

    let promise = get_fn.call1(&workflow, &JsValue::from(id)).map_err(js_err)?;
    let instance = JsFuture::from(js_sys::Promise::from(promise)).await.map_err(js_err)?;

    Ok(instance)
}

/// Get Workflow Instance
pub async fn get(env: &Env, binding: &str, id: &str) -> Result<GetOutput, String> {
    let instance = get_instance(env, binding, id).await?;

    let status_fn = js_sys::Reflect::get(&instance, &JsValue::from("status")).map_err(js_err)?;
    let status_fn: js_sys::Function = status_fn
        .dyn_into()
        .map_err(|_| "status is not a function".to_string())?;

    let promise = status_fn.call0(&instance).map_err(js_err)?;
    let status_result = JsFuture::from(js_sys::Promise::from(promise)).await.map_err(js_err)?;

    let status_str = js_sys::Reflect::get(&status_result, &JsValue::from("status"))
        .map_err(js_err)?
        .as_string()
        .unwrap_or_default();

    let error = js_sys::Reflect::get(&status_result, &JsValue::from("error"))
        .ok()
        .and_then(|v| {
            if v.is_undefined() || v.is_null() {
                None
            } else {
                let message = js_sys::Reflect::get(&v, &JsValue::from("message"))
                    .ok()
                    .and_then(|m| m.as_string())
                    .unwrap_or_default();
                let name = js_sys::Reflect::get(&v, &JsValue::from("name"))
                    .ok()
                    .and_then(|n| n.as_string());
                Some(WorkflowError { message, name })
            }
        });

    let output = js_sys::Reflect::get(&status_result, &JsValue::from("output"))
        .ok()
        .and_then(|v| {
            if v.is_undefined() || v.is_null() {
                None
            } else {
                serde_wasm_bindgen::from_value(v).ok()
            }
        });

    Ok(GetOutput {
        id: id.to_string(),
        status: WorkflowInstanceStatus {
            status: status_str,
            error,
            output,
        },
    })
}

/// Get Workflow Instance Status
pub async fn status(env: &Env, binding: &str, id: &str) -> Result<StatusOutput, String> {
    let instance = get_instance(env, binding, id).await?;

    let status_fn = js_sys::Reflect::get(&instance, &JsValue::from("status")).map_err(js_err)?;
    let status_fn: js_sys::Function = status_fn
        .dyn_into()
        .map_err(|_| "status is not a function".to_string())?;

    let promise = status_fn.call0(&instance).map_err(js_err)?;
    let status_result = JsFuture::from(js_sys::Promise::from(promise)).await.map_err(js_err)?;

    let status_str = js_sys::Reflect::get(&status_result, &JsValue::from("status"))
        .map_err(js_err)?
        .as_string()
        .unwrap_or_default();

    let error = js_sys::Reflect::get(&status_result, &JsValue::from("error"))
        .ok()
        .and_then(|v| {
            if v.is_undefined() || v.is_null() {
                None
            } else {
                let message = js_sys::Reflect::get(&v, &JsValue::from("message"))
                    .ok()
                    .and_then(|m| m.as_string())
                    .unwrap_or_default();
                let name = js_sys::Reflect::get(&v, &JsValue::from("name"))
                    .ok()
                    .and_then(|n| n.as_string());
                Some(WorkflowError { message, name })
            }
        });

    let output = js_sys::Reflect::get(&status_result, &JsValue::from("output"))
        .ok()
        .and_then(|v| {
            if v.is_undefined() || v.is_null() {
                None
            } else {
                serde_wasm_bindgen::from_value(v).ok()
            }
        })
        .unwrap_or(serde_json::Value::Null);

    Ok(StatusOutput {
        error,
        output,
        status: status_str,
    })
}

/// Helper to call a simple instance method
async fn call_instance_method(env: &Env, binding: &str, id: &str, method: &str) -> Result<(), String> {
    let instance = get_instance(env, binding, id).await?;

    let method_fn = js_sys::Reflect::get(&instance, &JsValue::from(method)).map_err(js_err)?;
    let method_fn: js_sys::Function = method_fn
        .dyn_into()
        .map_err(|_| format!("{method} is not a function"))?;

    let promise = method_fn.call0(&instance).map_err(js_err)?;
    JsFuture::from(js_sys::Promise::from(promise)).await.map_err(js_err)?;

    Ok(())
}

/// Pause Workflow Instance
pub async fn pause(env: &Env, binding: &str, id: &str) -> Result<PauseOutput, String> {
    call_instance_method(env, binding, id, "pause").await?;
    Ok(PauseOutput { success: true })
}

/// Resume Workflow Instance
pub async fn resume(env: &Env, binding: &str, id: &str) -> Result<ResumeOutput, String> {
    call_instance_method(env, binding, id, "resume").await?;
    Ok(ResumeOutput { success: true })
}

/// Restart Workflow Instance
pub async fn restart(env: &Env, binding: &str, id: &str) -> Result<RestartOutput, String> {
    call_instance_method(env, binding, id, "restart").await?;
    Ok(RestartOutput { success: true })
}

/// Terminate Workflow Instance
pub async fn terminate(env: &Env, binding: &str, id: &str) -> Result<TerminateOutput, String> {
    call_instance_method(env, binding, id, "terminate").await?;
    Ok(TerminateOutput { success: true })
}

/// Send Event To Workflow Instance
pub async fn send_event(
    env: &Env,
    binding: &str,
    id: &str,
    event_type: &str,
    payload: Option<serde_json::Value>,
) -> Result<SendEventOutput, String> {
    let instance = get_instance(env, binding, id).await?;

    let send_fn = js_sys::Reflect::get(&instance, &JsValue::from("sendEvent")).map_err(js_err)?;
    let send_fn: js_sys::Function = send_fn
        .dyn_into()
        .map_err(|_| "sendEvent is not a function".to_string())?;

    let event_obj = js_sys::Object::new();
    js_sys::Reflect::set(&event_obj, &JsValue::from("type"), &JsValue::from(event_type)).map_err(js_err)?;
    if let Some(p) = &payload {
        let payload_js =
            serde_wasm_bindgen::to_value(p).map_err(|e| format!("Event payload serialization error: {e}"))?;
        js_sys::Reflect::set(&event_obj, &JsValue::from("payload"), &payload_js).map_err(js_err)?;
    }

    let promise = send_fn.call1(&instance, &event_obj).map_err(js_err)?;
    JsFuture::from(js_sys::Promise::from(promise)).await.map_err(js_err)?;

    Ok(SendEventOutput { success: true })
}

/// Execute Workflow Step
///
/// Note: Step operations are only available inside a Workflow's `run()` method.
/// They require the `WorkflowStep` context object.
pub async fn step_do(
    _name: &str,
    _callback: serde_json::Value,
    _retries_limit: Option<i32>,
    _retries_delay: Option<&str>,
    _retries_backoff: Option<&str>,
    _timeout: Option<&str>,
) -> Result<StepDoOutput, String> {
    Err("step_do is only available inside a Workflow's run() method. \
         Use step.do_() with the WorkflowStep context."
        .to_string())
}

/// Workflow Step Sleep
///
/// Note: Step operations are only available inside a Workflow's `run()` method.
pub async fn step_sleep(_name: &str, _duration: &str) -> Result<StepSleepOutput, String> {
    Err("step_sleep is only available inside a Workflow's run() method. \
         Use step.sleep() with the WorkflowStep context."
        .to_string())
}

/// Workflow Step Sleep Until
///
/// Note: Step operations are only available inside a Workflow's `run()` method.
pub async fn step_sleep_until(_name: &str, _timestamp: &str) -> Result<StepSleepUntilOutput, String> {
    Err("step_sleep_until is only available inside a Workflow's run() method. \
         Use step.sleep_until() with the WorkflowStep context."
        .to_string())
}

/// Workflow Step Wait For Event
///
/// Note: Step operations are only available inside a Workflow's `run()` method.
pub async fn step_wait_for_event(
    _name: &str,
    _event_type: &str,
    _timeout: Option<&str>,
) -> Result<StepWaitForEventOutput, String> {
    Err(
        "step_wait_for_event is only available inside a Workflow's run() method. \
         Use step.wait_for_event() with the WorkflowStep context."
            .to_string(),
    )
}
