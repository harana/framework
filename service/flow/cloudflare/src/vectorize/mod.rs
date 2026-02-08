// Harana Actions - Cloudflare Vectorize Module
// This module provides Cloudflare Vectorize actions for managing vector indexes,
// inserting/querying vectors, and managing metadata indexes.

pub mod output;

use output::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use worker::Env;

fn js_err(e: JsValue) -> String {
    format!("Vectorize JS error: {e:?}")
}

/// Get the vectorize index binding from the environment
fn get_index(env: &Env, index: &str) -> Result<JsValue, String> {
    let val = js_sys::Reflect::get(env, &JsValue::from(index))
        .map_err(|_| format!("Vectorize binding '{index}' not found"))?;
    if val.is_undefined() {
        return Err(format!("Vectorize binding '{index}' is undefined"));
    }
    Ok(val)
}

/// Convert VectorizeVector to JS object
fn vector_to_js(v: &VectorizeVector) -> Result<JsValue, String> {
    let obj = js_sys::Object::new();

    js_sys::Reflect::set(&obj, &JsValue::from("id"), &JsValue::from(&v.id)).map_err(js_err)?;

    let values = js_sys::Float32Array::new_with_length(v.values.len() as u32);
    values.copy_from(&v.values);
    js_sys::Reflect::set(&obj, &JsValue::from("values"), &values).map_err(js_err)?;

    if let Some(ns) = &v.namespace {
        js_sys::Reflect::set(&obj, &JsValue::from("namespace"), &JsValue::from(ns)).map_err(js_err)?;
    }

    if let Some(meta) = &v.metadata {
        let meta_js =
            serde_wasm_bindgen::to_value(meta).map_err(|e| format!("Vectorize metadata serialization error: {e}"))?;
        js_sys::Reflect::set(&obj, &JsValue::from("metadata"), &meta_js).map_err(js_err)?;
    }

    Ok(obj.into())
}

/// Convert vectors to JS array
fn vectors_to_js_array(vectors: &[VectorizeVector]) -> Result<js_sys::Array, String> {
    let arr = js_sys::Array::new();
    for v in vectors {
        arr.push(&vector_to_js(v)?);
    }
    Ok(arr)
}

/// Extract mutation result from JS value
fn extract_mutation_result(result: &JsValue) -> Result<(i32, String, bool), String> {
    let count = js_sys::Reflect::get(result, &JsValue::from("count"))
        .map_err(js_err)?
        .as_f64()
        .unwrap_or(0.0) as i32;
    let mutation_id = js_sys::Reflect::get(result, &JsValue::from("mutationId"))
        .map_err(js_err)?
        .as_string()
        .unwrap_or_default();
    // Some operations may not return success directly
    Ok((count, mutation_id, true))
}

/// Insert Vectors
pub async fn insert(env: &Env, index: &str, vectors: Vec<VectorizeVector>) -> Result<InsertOutput, String> {
    let idx = get_index(env, index)?;
    let vectors_js = vectors_to_js_array(&vectors)?;

    let insert_fn = js_sys::Reflect::get(&idx, &JsValue::from("insert")).map_err(js_err)?;
    let insert_fn: js_sys::Function = insert_fn
        .dyn_into()
        .map_err(|_| "insert is not a function".to_string())?;

    let promise = insert_fn.call1(&idx, &vectors_js).map_err(js_err)?;
    let result = JsFuture::from(js_sys::Promise::from(promise)).await.map_err(js_err)?;

    let (count, mutation_id, success) = extract_mutation_result(&result)?;

    Ok(InsertOutput {
        count,
        mutation_id,
        success,
    })
}

/// Upsert Vectors
pub async fn upsert(env: &Env, index: &str, vectors: Vec<VectorizeVector>) -> Result<UpsertOutput, String> {
    let idx = get_index(env, index)?;
    let vectors_js = vectors_to_js_array(&vectors)?;

    let upsert_fn = js_sys::Reflect::get(&idx, &JsValue::from("upsert")).map_err(js_err)?;
    let upsert_fn: js_sys::Function = upsert_fn
        .dyn_into()
        .map_err(|_| "upsert is not a function".to_string())?;

    let promise = upsert_fn.call1(&idx, &vectors_js).map_err(js_err)?;
    let result = JsFuture::from(js_sys::Promise::from(promise)).await.map_err(js_err)?;

    let (count, mutation_id, success) = extract_mutation_result(&result)?;

    Ok(UpsertOutput {
        count,
        mutation_id,
        success,
    })
}

/// Build query options JS object
fn build_query_options(
    top_k: Option<i32>,
    namespace: Option<&str>,
    filter: Option<&VectorizeFilter>,
    return_values: Option<bool>,
    return_metadata: Option<&str>,
) -> Result<JsValue, String> {
    let opts = js_sys::Object::new();

    if let Some(k) = top_k {
        js_sys::Reflect::set(&opts, &JsValue::from("topK"), &JsValue::from(k)).map_err(js_err)?;
    }
    if let Some(ns) = namespace {
        js_sys::Reflect::set(&opts, &JsValue::from("namespace"), &JsValue::from(ns)).map_err(js_err)?;
    }
    if let Some(f) = filter {
        let filter_js = serde_wasm_bindgen::to_value(&f.conditions)
            .map_err(|e| format!("Vectorize filter serialization error: {e}"))?;
        js_sys::Reflect::set(&opts, &JsValue::from("filter"), &filter_js).map_err(js_err)?;
    }
    if let Some(rv) = return_values {
        js_sys::Reflect::set(&opts, &JsValue::from("returnValues"), &JsValue::from(rv)).map_err(js_err)?;
    }
    if let Some(rm) = return_metadata {
        js_sys::Reflect::set(&opts, &JsValue::from("returnMetadata"), &JsValue::from(rm)).map_err(js_err)?;
    }

    Ok(opts.into())
}

/// Extract matches from query result
fn extract_matches(result: &JsValue) -> Result<Vec<VectorizeMatch>, String> {
    let matches_js = js_sys::Reflect::get(result, &JsValue::from("matches")).map_err(js_err)?;

    let matches_arr: js_sys::Array = matches_js
        .dyn_into()
        .map_err(|_| "matches is not an array".to_string())?;

    let mut matches = Vec::new();
    for i in 0..matches_arr.length() {
        let m = matches_arr.get(i);
        let id = js_sys::Reflect::get(&m, &JsValue::from("id"))
            .map_err(js_err)?
            .as_string()
            .unwrap_or_default();
        let score = js_sys::Reflect::get(&m, &JsValue::from("score"))
            .map_err(js_err)?
            .as_f64()
            .unwrap_or(0.0);

        let values_js = js_sys::Reflect::get(&m, &JsValue::from("values")).ok();
        let values = values_js.and_then(|v| {
            if v.is_undefined() || v.is_null() {
                None
            } else {
                let arr: js_sys::Float32Array = v.dyn_into().ok()?;
                let mut vec = vec![0f32; arr.length() as usize];
                arr.copy_to(&mut vec);
                Some(vec)
            }
        });

        let namespace = js_sys::Reflect::get(&m, &JsValue::from("namespace"))
            .ok()
            .and_then(|v| v.as_string());

        let metadata = js_sys::Reflect::get(&m, &JsValue::from("metadata")).ok().and_then(|v| {
            if v.is_undefined() || v.is_null() {
                None
            } else {
                serde_wasm_bindgen::from_value(v).ok()
            }
        });

        matches.push(VectorizeMatch {
            id,
            score,
            values,
            namespace,
            metadata,
        });
    }

    Ok(matches)
}

/// Query Vectors
pub async fn query(
    env: &Env,
    index: &str,
    vector: Vec<f32>,
    top_k: Option<i32>,
    namespace: Option<&str>,
    filter: Option<VectorizeFilter>,
    return_values: Option<bool>,
    return_metadata: Option<&str>,
) -> Result<QueryOutput, String> {
    let idx = get_index(env, index)?;

    let vector_js = js_sys::Float32Array::new_with_length(vector.len() as u32);
    vector_js.copy_from(&vector);

    let opts = build_query_options(top_k, namespace, filter.as_ref(), return_values, return_metadata)?;

    let query_fn = js_sys::Reflect::get(&idx, &JsValue::from("query")).map_err(js_err)?;
    let query_fn: js_sys::Function = query_fn.dyn_into().map_err(|_| "query is not a function".to_string())?;

    let promise = query_fn.call2(&idx, &vector_js, &opts).map_err(js_err)?;
    let result = JsFuture::from(js_sys::Promise::from(promise)).await.map_err(js_err)?;

    let matches = extract_matches(&result)?;
    let count = js_sys::Reflect::get(&result, &JsValue::from("count"))
        .map_err(js_err)?
        .as_f64()
        .unwrap_or(matches.len() as f64) as i32;

    Ok(QueryOutput { count, matches })
}

/// Query Vectors By ID
pub async fn query_by_id(
    env: &Env,
    index: &str,
    vector_id: &str,
    top_k: Option<i32>,
    namespace: Option<&str>,
    filter: Option<VectorizeFilter>,
    return_values: Option<bool>,
    return_metadata: Option<&str>,
) -> Result<QueryByIdOutput, String> {
    // First get the vector by ID, then use it to query
    let idx = get_index(env, index)?;

    // Build get-by-ids request to retrieve the vector
    let ids_arr = js_sys::Array::new();
    ids_arr.push(&JsValue::from(vector_id));

    let get_fn = js_sys::Reflect::get(&idx, &JsValue::from("getByIds")).map_err(js_err)?;
    let get_fn: js_sys::Function = get_fn
        .dyn_into()
        .map_err(|_| "getByIds is not a function".to_string())?;

    let promise = get_fn.call1(&idx, &ids_arr).map_err(js_err)?;
    let get_result = JsFuture::from(js_sys::Promise::from(promise)).await.map_err(js_err)?;

    let vectors_arr: js_sys::Array = get_result
        .dyn_into()
        .map_err(|_| "getByIds result is not an array".to_string())?;

    if vectors_arr.length() == 0 {
        return Err(format!("Vector '{vector_id}' not found"));
    }

    let vec_obj = vectors_arr.get(0);
    let values_js = js_sys::Reflect::get(&vec_obj, &JsValue::from("values")).map_err(js_err)?;
    let float_arr: js_sys::Float32Array = values_js
        .dyn_into()
        .map_err(|_| "values is not a Float32Array".to_string())?;
    let mut vector = vec![0f32; float_arr.length() as usize];
    float_arr.copy_to(&mut vector);

    // Now query with the vector
    let vector_js = js_sys::Float32Array::new_with_length(vector.len() as u32);
    vector_js.copy_from(&vector);

    let opts = build_query_options(top_k, namespace, filter.as_ref(), return_values, return_metadata)?;

    let query_fn = js_sys::Reflect::get(&idx, &JsValue::from("query")).map_err(js_err)?;
    let query_fn: js_sys::Function = query_fn.dyn_into().map_err(|_| "query is not a function".to_string())?;

    let promise = query_fn.call2(&idx, &vector_js, &opts).map_err(js_err)?;
    let result = JsFuture::from(js_sys::Promise::from(promise)).await.map_err(js_err)?;

    let matches = extract_matches(&result)?;
    let count = js_sys::Reflect::get(&result, &JsValue::from("count"))
        .map_err(js_err)?
        .as_f64()
        .unwrap_or(matches.len() as f64) as i32;

    Ok(QueryByIdOutput { count, matches })
}

/// Get Vectors By ID
pub async fn get_by_ids(env: &Env, index: &str, ids: Vec<String>) -> Result<GetByIdsOutput, String> {
    let idx = get_index(env, index)?;

    let ids_arr = js_sys::Array::new();
    for id in &ids {
        ids_arr.push(&JsValue::from(id));
    }

    let get_fn = js_sys::Reflect::get(&idx, &JsValue::from("getByIds")).map_err(js_err)?;
    let get_fn: js_sys::Function = get_fn
        .dyn_into()
        .map_err(|_| "getByIds is not a function".to_string())?;

    let promise = get_fn.call1(&idx, &ids_arr).map_err(js_err)?;
    let result = JsFuture::from(js_sys::Promise::from(promise)).await.map_err(js_err)?;

    let vectors_arr: js_sys::Array = result
        .dyn_into()
        .map_err(|_| "getByIds result is not an array".to_string())?;

    let mut vectors = Vec::new();
    for i in 0..vectors_arr.length() {
        let v = vectors_arr.get(i);
        let id = js_sys::Reflect::get(&v, &JsValue::from("id"))
            .map_err(js_err)?
            .as_string()
            .unwrap_or_default();

        let values_js = js_sys::Reflect::get(&v, &JsValue::from("values")).map_err(js_err)?;
        let float_arr: js_sys::Float32Array = values_js
            .dyn_into()
            .map_err(|_| "values is not a Float32Array".to_string())?;
        let mut values = vec![0f32; float_arr.length() as usize];
        float_arr.copy_to(&mut values);

        let namespace = js_sys::Reflect::get(&v, &JsValue::from("namespace"))
            .ok()
            .and_then(|v| v.as_string());

        let metadata = js_sys::Reflect::get(&v, &JsValue::from("metadata")).ok().and_then(|v| {
            if v.is_undefined() || v.is_null() {
                None
            } else {
                serde_wasm_bindgen::from_value(v).ok()
            }
        });

        vectors.push(VectorizeVector {
            id,
            values,
            namespace,
            metadata,
        });
    }

    Ok(GetByIdsOutput { vectors })
}

/// Delete Vectors By ID
pub async fn delete_by_ids(env: &Env, index: &str, ids: Vec<String>) -> Result<DeleteByIdsOutput, String> {
    let idx = get_index(env, index)?;

    let ids_arr = js_sys::Array::new();
    for id in &ids {
        ids_arr.push(&JsValue::from(id));
    }

    let delete_fn = js_sys::Reflect::get(&idx, &JsValue::from("deleteByIds")).map_err(js_err)?;
    let delete_fn: js_sys::Function = delete_fn
        .dyn_into()
        .map_err(|_| "deleteByIds is not a function".to_string())?;

    let promise = delete_fn.call1(&idx, &ids_arr).map_err(js_err)?;
    let result = JsFuture::from(js_sys::Promise::from(promise)).await.map_err(js_err)?;

    let (count, mutation_id, success) = extract_mutation_result(&result)?;

    Ok(DeleteByIdsOutput {
        count,
        mutation_id,
        success,
    })
}

/// Describe Vectorize Index
pub async fn describe(env: &Env, index: &str) -> Result<DescribeOutput, String> {
    let idx = get_index(env, index)?;

    let describe_fn = js_sys::Reflect::get(&idx, &JsValue::from("describe")).map_err(js_err)?;
    let describe_fn: js_sys::Function = describe_fn
        .dyn_into()
        .map_err(|_| "describe is not a function".to_string())?;

    let promise = describe_fn.call0(&idx).map_err(js_err)?;
    let result = JsFuture::from(js_sys::Promise::from(promise)).await.map_err(js_err)?;

    let configured_dimensions = js_sys::Reflect::get(&result, &JsValue::from("dimensions"))
        .map_err(js_err)?
        .as_f64()
        .unwrap_or(0.0) as i32;
    let description = js_sys::Reflect::get(&result, &JsValue::from("description"))
        .map_err(js_err)?
        .as_string()
        .unwrap_or_default();
    let metric = js_sys::Reflect::get(&result, &JsValue::from("metric"))
        .map_err(js_err)?
        .as_string()
        .unwrap_or_default();
    let name = js_sys::Reflect::get(&result, &JsValue::from("name"))
        .map_err(js_err)?
        .as_string()
        .unwrap_or_default();
    let vectors_count = js_sys::Reflect::get(&result, &JsValue::from("vectorsCount"))
        .map_err(js_err)?
        .as_f64()
        .unwrap_or(0.0) as i32;

    Ok(DescribeOutput {
        configured_dimensions,
        description,
        metric,
        name,
        vectors_count,
    })
}

/// Get Vectorize Index Info
pub async fn info(env: &Env, index: &str) -> Result<InfoOutput, String> {
    // info() is similar to describe() in the Vectorize API
    let result = describe(env, index).await?;

    Ok(InfoOutput {
        dimensions: result.configured_dimensions,
        metric: result.metric,
        name: result.name,
        vectors_count: result.vectors_count,
    })
}

/// Create Metadata Index
pub async fn create_metadata_index(
    env: &Env,
    index: &str,
    property_name: &str,
    index_type: Option<&str>,
) -> Result<CreateMetadataIndexOutput, String> {
    let idx = get_index(env, index)?;

    let create_fn = js_sys::Reflect::get(&idx, &JsValue::from("createMetadataIndex")).map_err(js_err)?;
    let create_fn: js_sys::Function = create_fn
        .dyn_into()
        .map_err(|_| "createMetadataIndex is not a function".to_string())?;

    let opts = js_sys::Object::new();
    js_sys::Reflect::set(&opts, &JsValue::from("propertyName"), &JsValue::from(property_name)).map_err(js_err)?;
    if let Some(t) = index_type {
        js_sys::Reflect::set(&opts, &JsValue::from("type"), &JsValue::from(t)).map_err(js_err)?;
    }

    let promise = create_fn.call1(&idx, &opts).map_err(js_err)?;
    JsFuture::from(js_sys::Promise::from(promise)).await.map_err(js_err)?;

    Ok(CreateMetadataIndexOutput { success: true })
}

/// Delete Metadata Index
pub async fn delete_metadata_index(
    env: &Env,
    index: &str,
    property_name: &str,
) -> Result<DeleteMetadataIndexOutput, String> {
    let idx = get_index(env, index)?;

    let delete_fn = js_sys::Reflect::get(&idx, &JsValue::from("deleteMetadataIndex")).map_err(js_err)?;
    let delete_fn: js_sys::Function = delete_fn
        .dyn_into()
        .map_err(|_| "deleteMetadataIndex is not a function".to_string())?;

    let promise = delete_fn.call1(&idx, &JsValue::from(property_name)).map_err(js_err)?;
    JsFuture::from(js_sys::Promise::from(promise)).await.map_err(js_err)?;

    Ok(DeleteMetadataIndexOutput { success: true })
}

/// List Metadata Indexes
pub async fn list_metadata_indexes(env: &Env, index: &str) -> Result<ListMetadataIndexesOutput, String> {
    let idx = get_index(env, index)?;

    let list_fn = js_sys::Reflect::get(&idx, &JsValue::from("listMetadataIndexes")).map_err(js_err)?;
    let list_fn: js_sys::Function = list_fn
        .dyn_into()
        .map_err(|_| "listMetadataIndexes is not a function".to_string())?;

    let promise = list_fn.call0(&idx).map_err(js_err)?;
    let result = JsFuture::from(js_sys::Promise::from(promise)).await.map_err(js_err)?;

    let indexes_arr: js_sys::Array = js_sys::Reflect::get(&result, &JsValue::from("indexes"))
        .unwrap_or(result.clone())
        .dyn_into()
        .unwrap_or_else(|_| js_sys::Array::new());

    let mut metadata_indexes = Vec::new();
    for i in 0..indexes_arr.length() {
        let idx_obj = indexes_arr.get(i);
        let property_name = js_sys::Reflect::get(&idx_obj, &JsValue::from("propertyName"))
            .map_err(js_err)?
            .as_string()
            .unwrap_or_default();
        let index_type = js_sys::Reflect::get(&idx_obj, &JsValue::from("type"))
            .map_err(js_err)?
            .as_string()
            .unwrap_or_default();

        metadata_indexes.push(VectorizeMetadataIndex {
            property_name,
            index_type,
        });
    }

    Ok(ListMetadataIndexesOutput { metadata_indexes })
}
