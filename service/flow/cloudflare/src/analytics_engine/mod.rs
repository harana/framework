// Harana Actions - Cloudflare Analytics Engine Module
// This module provides Cloudflare Analytics Engine actions for writing and querying data points.

pub mod output;

use output::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use worker::Env;

fn to_err(e: impl std::fmt::Display) -> String {
    format!("Analytics Engine error: {e}")
}

/// Write Analytics Engine Data Point
pub async fn write_data_point(
    env: &Env,
    binding: &str,
    blobs: Option<Vec<String>>,
    doubles: Option<Vec<f64>>,
    indexes: Option<Vec<String>>,
) -> Result<WriteDataPointOutput, String> {
    // Access the analytics engine dataset binding via raw JS interop
    let binding_val = js_sys::Reflect::get(env, &JsValue::from(binding))
        .map_err(|_| format!("Analytics Engine binding '{binding}' not found"))?;

    if binding_val.is_undefined() {
        return Err(format!("Analytics Engine binding '{binding}' is undefined"));
    }

    // Build the data point object
    let data_point = js_sys::Object::new();

    if let Some(blobs) = blobs {
        let blobs_array = js_sys::Array::new();
        for blob in blobs {
            blobs_array.push(&JsValue::from_str(&blob));
        }
        js_sys::Reflect::set(&data_point, &JsValue::from("blobs"), &blobs_array)
            .map_err(|e| to_err(format!("{e:?}")))?;
    }

    if let Some(doubles) = doubles {
        let doubles_array = js_sys::Array::new();
        for double in doubles {
            doubles_array.push(&JsValue::from_f64(double));
        }
        js_sys::Reflect::set(&data_point, &JsValue::from("doubles"), &doubles_array)
            .map_err(|e| to_err(format!("{e:?}")))?;
    }

    if let Some(indexes) = indexes {
        let indexes_array = js_sys::Array::new();
        for index in indexes {
            indexes_array.push(&JsValue::from_str(&index));
        }
        js_sys::Reflect::set(&data_point, &JsValue::from("indexes"), &indexes_array)
            .map_err(|e| to_err(format!("{e:?}")))?;
    }

    // Call writeDataPoint on the binding
    let write_fn =
        js_sys::Reflect::get(&binding_val, &JsValue::from("writeDataPoint")).map_err(|e| to_err(format!("{e:?}")))?;

    let write_fn: js_sys::Function = write_fn
        .dyn_into()
        .map_err(|_| "writeDataPoint is not a function".to_string())?;

    write_fn
        .call1(&binding_val, &data_point)
        .map_err(|e| to_err(format!("{e:?}")))?;

    Ok(WriteDataPointOutput { success: true })
}

/// Query Analytics Engine
pub async fn query(
    _env: &Env,
    _binding: &str,
    _query: &str,
    _time_start: Option<&str>,
    _time_end: Option<&str>,
) -> Result<QueryOutput, String> {
    // Note: Analytics Engine querying is done via the Cloudflare REST API,
    // not directly through the Workers binding. The binding only supports writing data points.
    Err(
        "Analytics Engine querying is not available via the Workers binding. Use the Cloudflare REST API instead."
            .to_string(),
    )
}
