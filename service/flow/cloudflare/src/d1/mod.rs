// Harana Actions - Cloudflare D1 Module
// This module provides Cloudflare D1 database actions for executing SQL queries
// and managing prepared statements.

pub mod output;

use output::*;
use worker::Env;

fn to_err(e: impl std::fmt::Display) -> String {
    format!("D1 error: {e}")
}

fn js_err(e: wasm_bindgen::JsValue) -> String {
    format!("D1 error: {e:?}")
}

fn json_to_jsvalue(v: serde_json::Value) -> wasm_bindgen::JsValue {
    serde_wasm_bindgen::to_value(&v).unwrap_or(wasm_bindgen::JsValue::NULL)
}

fn bind_params(
    stmt: worker::d1::D1PreparedStatement,
    bind_values: Option<Vec<serde_json::Value>>,
) -> Result<worker::d1::D1PreparedStatement, String> {
    if let Some(values) = bind_values {
        let js_values: Vec<wasm_bindgen::JsValue> = values.into_iter().map(json_to_jsvalue).collect();
        stmt.bind(&js_values).map_err(to_err)
    } else {
        Ok(stmt)
    }
}

fn extract_meta(result: &worker::d1::D1Result) -> (i32, f64, i32, i32, i32) {
    let meta = result.meta().ok().flatten();
    if let Some(meta) = meta {
        (
            meta.changes.unwrap_or(0) as i32,
            meta.duration.unwrap_or(0.0),
            meta.last_row_id.unwrap_or(0) as i32,
            meta.rows_read.unwrap_or(0) as i32,
            meta.rows_written.unwrap_or(0) as i32,
        )
    } else {
        (0, 0.0, 0, 0, 0)
    }
}

/// Execute D1 Query
pub async fn exec(env: &Env, database: &str, sql: &str) -> Result<ExecOutput, String> {
    let db = env.d1(database).map_err(to_err)?;
    let result = db.exec(sql).await.map_err(to_err)?;

    let count = result.count().map_err(js_err)?.unwrap_or(0) as i32;
    let duration = result.duration().map_err(js_err)?.unwrap_or(0.0);

    Ok(ExecOutput {
        changes: count,
        duration,
        last_row_id: 0,
        rows_read: count,
        rows_written: 0,
        success: true,
    })
}

/// Prepare D1 Statement
pub async fn prepare(env: &Env, database: &str, sql: &str) -> Result<PrepareOutput, String> {
    let _db = env.d1(database).map_err(to_err)?;

    Ok(PrepareOutput {
        statement: D1PreparedStatement {
            sql: sql.to_string(),
            params: vec![],
        },
        success: true,
    })
}

/// Run D1 Prepared Statement
pub async fn run(
    env: &Env,
    database: &str,
    sql: &str,
    bind_values: Option<Vec<serde_json::Value>>,
) -> Result<RunOutput, String> {
    let db = env.d1(database).map_err(to_err)?;
    let stmt = db.prepare(sql);
    let stmt = bind_params(stmt, bind_values)?;

    let result = stmt.run().await.map_err(to_err)?;
    let success = result.success();
    let (changes, duration, last_row_id, rows_read, rows_written) = extract_meta(&result);
    let results: Vec<serde_json::Value> = result.results().unwrap_or_default();

    Ok(RunOutput {
        changes,
        duration,
        last_row_id,
        results,
        rows_read,
        rows_written,
        success,
    })
}

/// Query D1 First Row
pub async fn first(
    env: &Env,
    database: &str,
    sql: &str,
    bind_values: Option<Vec<serde_json::Value>>,
    column: Option<&str>,
) -> Result<FirstOutput, String> {
    let db = env.d1(database).map_err(to_err)?;
    let stmt = db.prepare(sql);
    let stmt = bind_params(stmt, bind_values)?;

    let result: Option<serde_json::Value> = stmt.first(column).await.map_err(to_err)?;

    Ok(FirstOutput {
        result: result.unwrap_or(serde_json::Value::Null),
    })
}

/// Query D1 All Rows
pub async fn all(
    env: &Env,
    database: &str,
    sql: &str,
    bind_values: Option<Vec<serde_json::Value>>,
) -> Result<AllOutput, String> {
    let db = env.d1(database).map_err(to_err)?;
    let stmt = db.prepare(sql);
    let stmt = bind_params(stmt, bind_values)?;

    let result = stmt.all().await.map_err(to_err)?;
    let success = result.success();
    let (changes, duration, last_row_id, rows_read, rows_written) = extract_meta(&result);
    let results: Vec<serde_json::Value> = result.results().unwrap_or_default();

    Ok(AllOutput {
        changes,
        duration,
        last_row_id,
        results,
        rows_read,
        rows_written,
        success,
    })
}

/// Query D1 Raw Rows
pub async fn raw(
    env: &Env,
    database: &str,
    sql: &str,
    bind_values: Option<Vec<serde_json::Value>>,
) -> Result<RawOutput, String> {
    let db = env.d1(database).map_err(to_err)?;
    let stmt = db.prepare(sql);
    let stmt = bind_params(stmt, bind_values)?;

    let raw_result: Vec<Vec<serde_json::Value>> = stmt.raw().await.map_err(to_err)?;

    Ok(RawOutput {
        columns: Vec::new(),
        rows: raw_result,
    })
}

/// Execute D1 Batch
pub async fn batch(env: &Env, database: &str, statements: Vec<D1BatchStatement>) -> Result<BatchOutput, String> {
    let db = env.d1(database).map_err(to_err)?;

    let mut prepared = Vec::with_capacity(statements.len());
    for stmt_def in statements {
        let stmt = db.prepare(&stmt_def.sql);
        let stmt = bind_params(stmt, stmt_def.params)?;
        prepared.push(stmt);
    }

    let results = db.batch(prepared).await.map_err(to_err)?;

    let batch_results: Vec<D1BatchResult> = results
        .into_iter()
        .map(|result| {
            let success = result.success();
            let (changes, duration, last_row_id, rows_read, rows_written) = extract_meta(&result);
            let results: Vec<serde_json::Value> = result.results().unwrap_or_default();
            D1BatchResult {
                changes,
                duration,
                last_row_id,
                results,
                rows_read,
                rows_written,
                success,
            }
        })
        .collect();

    Ok(BatchOutput {
        results: batch_results,
        success: true,
    })
}

/// Dump D1 Database
pub async fn dump(env: &Env, database: &str) -> Result<DumpOutput, String> {
    let db = env.d1(database).map_err(to_err)?;
    let data = db.dump().await.map_err(to_err)?;

    Ok(DumpOutput { data, success: true })
}
