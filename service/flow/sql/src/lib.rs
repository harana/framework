// Harana Actions - Sql Module
// This module provides sql actions and functionality.

pub mod output;

use std::collections::HashMap;
use serde_json::Value;
use uuid::Uuid;
use dashmap::DashMap;
use once_cell::sync::Lazy;
use parking_lot::RwLock;
use chrono::Utc;
use output::*;

// In-memory storage for simulated database
#[derive(Debug, Clone)]
struct TableSchema {
    columns: Vec<ColumnDef>,
    primary_key: Vec<String>,
    indexes: Vec<IndexDef>,
}

#[derive(Debug, Clone)]
struct ColumnDef {
    name: String,
    data_type: String,
    nullable: bool,
    default_value: Option<Value>,
}

#[derive(Debug, Clone)]
struct IndexDef {
    name: String,
    columns: Vec<String>,
    unique: bool,
}

#[derive(Debug, Clone)]
struct Transaction {
    id: String,
    isolation_level: String,
    operations: Vec<TransactionOp>,
}

#[derive(Debug, Clone)]
enum TransactionOp {
    Insert { table: String, row: HashMap<String, Value> },
    Update { table: String, conditions: String, values: HashMap<String, Value> },
    Delete { table: String, conditions: String },
}

// Global in-memory database state
static TABLES: Lazy<DashMap<String, Vec<HashMap<String, Value>>>> = Lazy::new(DashMap::new);
static SCHEMAS: Lazy<DashMap<String, TableSchema>> = Lazy::new(DashMap::new);
static TRANSACTIONS: Lazy<DashMap<String, Transaction>> = Lazy::new(DashMap::new);
static AUTO_INCREMENT: Lazy<RwLock<i64>> = Lazy::new(|| RwLock::new(1));

fn get_next_id() -> i64 {
    let mut id = AUTO_INCREMENT.write();
    let current = *id;
    *id += 1;
    current
}

fn get_database_key(database: Option<&str>, name: &str) -> String {
    match database {
        Some(db) => format!("{}.{}", db, name),
        None => name.to_string(),
    }
}

/// Execute Batch SQL
pub async fn batch(
    queries: Vec<HashMap<String, Value>>,
    _database: Option<&str>,
) -> Result<BatchOutput, String> {
    let mut success_count = 0;
    let mut failed_count = 0;
    let mut results = Vec::new();

    for query_info in queries {
        let query = query_info.get("query")
            .and_then(|v| v.as_str())
            .ok_or("Missing 'query' in batch item")?;

        let params = query_info.get("parameters")
            .and_then(|v| v.as_array())
            .map(|arr| arr.clone());

        // Try to execute the query
        let result = execute(query, params, None).await;
        
        let mut result_map = HashMap::new();
        match result {
            Ok(output) => {
                success_count += 1;
                result_map.insert("success".to_string(), Value::Bool(true));
                result_map.insert("affected_rows".to_string(), Value::Number(output.affected_rows.into()));
            }
            Err(e) => {
                failed_count += 1;
                result_map.insert("success".to_string(), Value::Bool(false));
                result_map.insert("error".to_string(), Value::String(e));
            }
        }
        results.push(result_map);
    }

    Ok(BatchOutput {
        success_count,
        failed_count,
        results,
        success: failed_count == 0,
    })
}

/// Begin Transaction
pub async fn begin_transaction(
    isolation_level: Option<&str>,
    _database: Option<&str>,
) -> Result<BeginTransactionOutput, String> {
    let transaction_id = Uuid::new_v4().to_string();
    let level = isolation_level.unwrap_or("READ_COMMITTED").to_string();

    let transaction = Transaction {
        id: transaction_id.clone(),
        isolation_level: level,
        operations: Vec::new(),
    };

    TRANSACTIONS.insert(transaction_id.clone(), transaction);

    Ok(BeginTransactionOutput {
        success: true,
        transaction_id,
    })
}

/// Bulk Insert
pub async fn bulk_insert(
    columns: Vec<String>,
    table_name: &str,
    rows: Vec<Value>,
    database: Option<&str>,
) -> Result<BulkInsertOutput, String> {
    let key = get_database_key(database, table_name);

    if !TABLES.contains_key(&key) {
        return Err(format!("Table '{}' does not exist", table_name));
    }

    let mut affected_rows = 0;
    let mut table_data = TABLES.get_mut(&key).unwrap();

    for row in rows {
        let row_arr = row.as_array()
            .ok_or("Each row must be an array")?;

        if row_arr.len() != columns.len() {
            return Err("Row column count does not match column definition".to_string());
        }

        let mut row_map = HashMap::new();
        for (i, col) in columns.iter().enumerate() {
            row_map.insert(col.clone(), row_arr[i].clone());
        }

        // Add auto-generated id if not present
        if !row_map.contains_key("id") {
            row_map.insert("id".to_string(), Value::Number(get_next_id().into()));
        }

        table_data.push(row_map);
        affected_rows += 1;
    }

    Ok(BulkInsertOutput {
        success: true,
        affected_rows,
    })
}

/// Call Stored Procedure
pub async fn call_procedure(
    procedure_name: &str,
    _parameters: Option<Vec<Value>>,
    _database: Option<&str>,
) -> Result<CallProcedureOutput, String> {
    // Simulated stored procedure execution
    // In production, this would execute actual stored procedures

    let mut out_parameters = HashMap::new();
    let result_sets = Vec::new();

    // Mock some common procedures
    match procedure_name {
        "get_version" => {
            out_parameters.insert("version".to_string(), Value::String("1.0.0".to_string()));
        }
        "get_current_time" => {
            out_parameters.insert("current_time".to_string(), Value::String(Utc::now().to_rfc3339()));
        }
        _ => {
            out_parameters.insert("result".to_string(), Value::String(format!("Procedure '{}' executed", procedure_name)));
        }
    }

    Ok(CallProcedureOutput {
        out_parameters,
        result_sets,
        success: true,
    })
}

/// Commit Transaction
pub async fn commit(
    transaction_id: &str,
    _database: Option<&str>,
) -> Result<CommitOutput, String> {
    let transaction = TRANSACTIONS.remove(transaction_id)
        .ok_or_else(|| format!("Transaction '{}' not found", transaction_id))?;

    // In a real implementation, this would apply all operations atomically
    // For simulation, operations were already applied

    drop(transaction);

    Ok(CommitOutput { success: true })
}

/// Create Index
pub async fn create_index(
    columns: Vec<String>,
    index_name: &str,
    table_name: &str,
    unique: Option<bool>,
    database: Option<&str>,
) -> Result<CreateIndexOutput, String> {
    let key = get_database_key(database, table_name);

    let mut schema = SCHEMAS.get_mut(&key)
        .ok_or_else(|| format!("Table '{}' does not exist", table_name))?;

    // Check if index already exists
    if schema.indexes.iter().any(|idx| idx.name == index_name) {
        return Err(format!("Index '{}' already exists", index_name));
    }

    let index = IndexDef {
        name: index_name.to_string(),
        columns,
        unique: unique.unwrap_or(false),
    };

    schema.indexes.push(index);

    Ok(CreateIndexOutput { success: true })
}

/// Create Table
pub async fn create_table(
    table_name: &str,
    columns: Vec<HashMap<String, Value>>,
    database: Option<&str>,
    if_not_exists: Option<bool>,
) -> Result<CreateTableOutput, String> {
    let key = get_database_key(database, table_name);

    if TABLES.contains_key(&key) {
        if if_not_exists.unwrap_or(false) {
            return Ok(CreateTableOutput { success: true });
        }
        return Err(format!("Table '{}' already exists", table_name));
    }

    let mut column_defs = Vec::new();
    let mut primary_key = Vec::new();

    for col in columns {
        let name = col.get("name")
            .and_then(|v| v.as_str())
            .ok_or("Column must have 'name'")?
            .to_string();

        let data_type = col.get("type")
            .and_then(|v| v.as_str())
            .unwrap_or("TEXT")
            .to_string();

        let nullable = col.get("nullable")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);

        let default_value = col.get("default").cloned();

        let is_primary = col.get("primary_key")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        if is_primary {
            primary_key.push(name.clone());
        }

        column_defs.push(ColumnDef {
            name,
            data_type,
            nullable,
            default_value,
        });
    }

    let schema = TableSchema {
        columns: column_defs,
        primary_key,
        indexes: Vec::new(),
    };

    SCHEMAS.insert(key.clone(), schema);
    TABLES.insert(key, Vec::new());

    Ok(CreateTableOutput { success: true })
}

/// Execute SQL Delete
pub async fn delete(
    query: &str,
    _parameters: Option<Vec<Value>>,
    database: Option<&str>,
) -> Result<DeleteOutput, String> {
    // Parse simple DELETE FROM table WHERE condition
    let query_upper = query.to_uppercase();

    if !query_upper.starts_with("DELETE") {
        return Err("Query must be a DELETE statement".to_string());
    }

    // Extract table name (simplified parsing)
    let parts: Vec<&str> = query.split_whitespace().collect();
    let from_idx = parts.iter().position(|&p| p.to_uppercase() == "FROM")
        .ok_or("Missing FROM clause")?;

    let table_name = parts.get(from_idx + 1)
        .ok_or("Missing table name")?;

    let key = get_database_key(database, table_name);

    if !TABLES.contains_key(&key) {
        return Err(format!("Table '{}' does not exist", table_name));
    }

    let mut table_data = TABLES.get_mut(&key).unwrap();

    // Simple WHERE clause handling (very basic simulation)
    let initial_count = table_data.len();

    if let Some(where_idx) = parts.iter().position(|&p| p.to_uppercase() == "WHERE") {
        if let (Some(col), Some(op), Some(val)) = (parts.get(where_idx + 1), parts.get(where_idx + 2), parts.get(where_idx + 3)) {
            let clean_val = val.trim_matches(|c| c == '\'' || c == '"');
            if *op == "=" {
                table_data.retain(|row| {
                    row.get(*col).map(|v| {
                        match v {
                            Value::String(s) => s != clean_val,
                            Value::Number(n) => n.to_string() != clean_val,
                            _ => true,
                        }
                    }).unwrap_or(true)
                });
            }
        }
    } else {
        // DELETE all rows
        table_data.clear();
    }

    let affected_rows = (initial_count - table_data.len()) as i32;

    Ok(DeleteOutput {
        affected_rows,
        success: true,
    })
}

/// Drop Index
pub async fn drop_index(
    index_name: &str,
    table_name: Option<&str>,
    if_exists: Option<bool>,
    database: Option<&str>,
) -> Result<DropIndexOutput, String> {
    // Find and remove the index
    let mut found = false;

    if let Some(tbl) = table_name {
        let key = get_database_key(database, tbl);
        if let Some(mut schema) = SCHEMAS.get_mut(&key) {
            let initial_len = schema.indexes.len();
            schema.indexes.retain(|idx| idx.name != index_name);
            found = schema.indexes.len() < initial_len;
        }
    } else {
        // Search all tables
        for mut entry in SCHEMAS.iter_mut() {
            let initial_len = entry.indexes.len();
            entry.indexes.retain(|idx| idx.name != index_name);
            if entry.indexes.len() < initial_len {
                found = true;
                break;
            }
        }
    }

    if !found && !if_exists.unwrap_or(false) {
        return Err(format!("Index '{}' does not exist", index_name));
    }

    Ok(DropIndexOutput { success: true })
}

/// Drop Table
pub async fn drop_table(
    table_name: &str,
    database: Option<&str>,
    if_exists: Option<bool>,
) -> Result<DropTableOutput, String> {
    let key = get_database_key(database, table_name);

    if !TABLES.contains_key(&key) {
        if if_exists.unwrap_or(false) {
            return Ok(DropTableOutput { success: true });
        }
        return Err(format!("Table '{}' does not exist", table_name));
    }

    TABLES.remove(&key);
    SCHEMAS.remove(&key);

    Ok(DropTableOutput { success: true })
}

/// Execute SQL Query
pub async fn execute(
    query: &str,
    _parameters: Option<Vec<Value>>,
    database: Option<&str>,
) -> Result<ExecuteOutput, String> {
    let query_upper = query.trim().to_uppercase();

    if query_upper.starts_with("INSERT") {
        let result = insert(query, _parameters, database).await?;
        return Ok(ExecuteOutput {
            last_insert_id: result.last_insert_id,
            success: result.success,
            affected_rows: result.affected_rows,
        });
    }

    if query_upper.starts_with("UPDATE") {
        let result = update(query, database, _parameters).await?;
        return Ok(ExecuteOutput {
            last_insert_id: "".to_string(),
            success: result.success,
            affected_rows: result.affected_rows,
        });
    }

    if query_upper.starts_with("DELETE") {
        let result = delete(query, _parameters, database).await?;
        return Ok(ExecuteOutput {
            last_insert_id: "".to_string(),
            success: result.success,
            affected_rows: result.affected_rows,
        });
    }

    // Generic execution for other statements
    Ok(ExecuteOutput {
        last_insert_id: "".to_string(),
        success: true,
        affected_rows: 0,
    })
}

/// Get Table Schema
pub async fn get_schema(
    table_name: &str,
    database: Option<&str>,
) -> Result<GetSchemaOutput, String> {
    let key = get_database_key(database, table_name);

    let schema = SCHEMAS.get(&key)
        .ok_or_else(|| format!("Table '{}' does not exist", table_name))?;

    let columns: Vec<HashMap<String, Value>> = schema.columns.iter().map(|col| {
        let mut map = HashMap::new();
        map.insert("name".to_string(), Value::String(col.name.clone()));
        map.insert("type".to_string(), Value::String(col.data_type.clone()));
        map.insert("nullable".to_string(), Value::Bool(col.nullable));
        if let Some(ref default) = col.default_value {
            map.insert("default".to_string(), default.clone());
        }
        map
    }).collect();

    let indexes: Vec<HashMap<String, Value>> = schema.indexes.iter().map(|idx| {
        let mut map = HashMap::new();
        map.insert("name".to_string(), Value::String(idx.name.clone()));
        map.insert("columns".to_string(), Value::Array(idx.columns.iter().map(|c| Value::String(c.clone())).collect()));
        map.insert("unique".to_string(), Value::Bool(idx.unique));
        map
    }).collect();

    Ok(GetSchemaOutput {
        columns,
        primary_key: schema.primary_key.clone(),
        indexes,
    })
}

/// Execute SQL Insert
pub async fn insert(
    query: &str,
    _parameters: Option<Vec<Value>>,
    database: Option<&str>,
) -> Result<InsertOutput, String> {
    // Parse simple INSERT INTO table (columns) VALUES (values)
    let query_upper = query.to_uppercase();

    if !query_upper.starts_with("INSERT") {
        return Err("Query must be an INSERT statement".to_string());
    }

    // Extract table name (simplified)
    let into_pos = query_upper.find("INTO")
        .ok_or("Missing INTO clause")?;

    let after_into = &query[into_pos + 4..].trim_start();
    let table_end = after_into.find(|c: char| c == '(' || c.is_whitespace())
        .unwrap_or(after_into.len());
    let table_name = &after_into[..table_end];

    let key = get_database_key(database, table_name);

    if !TABLES.contains_key(&key) {
        return Err(format!("Table '{}' does not exist", table_name));
    }

    let new_id = get_next_id();

    // Create a simple row with the new id
    let mut row = HashMap::new();
    row.insert("id".to_string(), Value::Number(new_id.into()));

    // Parse columns and values if present (simplified)
    if let (Some(cols_start), Some(vals_start)) = (query.find('('), query.to_uppercase().find("VALUES")) {
        let cols_end = query[cols_start..].find(')').map(|i| cols_start + i);
        if let Some(ce) = cols_end {
            let cols_str = &query[cols_start + 1..ce];
            let cols: Vec<&str> = cols_str.split(',').map(|s| s.trim()).collect();

            // Find values
            let vals_section = &query[vals_start + 6..];
            if let (Some(vs), Some(ve)) = (vals_section.find('('), vals_section.rfind(')')) {
                let vals_str = &vals_section[vs + 1..ve];
                let vals: Vec<&str> = vals_str.split(',').map(|s| s.trim().trim_matches(|c| c == '\'' || c == '"')).collect();

                for (i, col) in cols.iter().enumerate() {
                    if let Some(val) = vals.get(i) {
                        let parsed_val = if let Ok(n) = val.parse::<i64>() {
                            Value::Number(n.into())
                        } else if let Ok(f) = val.parse::<f64>() {
                            Value::Number(serde_json::Number::from_f64(f).unwrap_or(0.into()))
                        } else if *val == "NULL" || *val == "null" {
                            Value::Null
                        } else {
                            Value::String(val.to_string())
                        };
                        row.insert(col.to_string(), parsed_val);
                    }
                }
            }
        }
    }

    TABLES.get_mut(&key).unwrap().push(row);

    Ok(InsertOutput {
        affected_rows: 1,
        success: true,
        last_insert_id: new_id.to_string(),
    })
}

/// List Tables
pub async fn list_tables(
    database: Option<&str>,
    _schema: Option<&str>,
) -> Result<ListTablesOutput, String> {
    let prefix = database.map(|db| format!("{}.", db));

    let tables: Vec<String> = TABLES.iter()
        .filter_map(|entry| {
            let key = entry.key();
            match &prefix {
                Some(p) => {
                    if key.starts_with(p) {
                        Some(key[p.len()..].to_string())
                    } else {
                        None
                    }
                }
                None => {
                    if !key.contains('.') {
                        Some(key.clone())
                    } else {
                        None
                    }
                }
            }
        })
        .collect();

    Ok(ListTablesOutput { tables })
}

/// Execute Prepared Statement
pub async fn prepared_statement(
    statement: &str,
    database: Option<&str>,
    parameters: Option<Vec<Value>>,
) -> Result<PreparedStatementOutput, String> {
    // Replace placeholders with parameters
    let mut query = statement.to_string();

    if let Some(params) = &parameters {
        for (i, param) in params.iter().enumerate() {
            let placeholder = format!("${}", i + 1);
            let value_str = match param {
                Value::String(s) => format!("'{}'", s),
                Value::Number(n) => n.to_string(),
                Value::Bool(b) => b.to_string(),
                Value::Null => "NULL".to_string(),
                _ => param.to_string(),
            };
            query = query.replace(&placeholder, &value_str);
        }
    }

    // Execute the resolved query
    let query_upper = query.trim().to_uppercase();

    if query_upper.starts_with("SELECT") {
        let result = select(&query, None, None, database, None).await?;
        return Ok(PreparedStatementOutput {
            success: true,
            rows: result.rows,
            affected_rows: 0,
        });
    }

    let exec_result = execute(&query, None, database).await?;

    Ok(PreparedStatementOutput {
        success: exec_result.success,
        rows: Vec::new(),
        affected_rows: exec_result.affected_rows,
    })
}

/// Rollback Transaction
pub async fn rollback(
    transaction_id: &str,
    _database: Option<&str>,
) -> Result<RollbackOutput, String> {
    let _ = TRANSACTIONS.remove(transaction_id)
        .ok_or_else(|| format!("Transaction '{}' not found", transaction_id))?;

    // In a real implementation, this would undo all operations
    // For simulation, we just remove the transaction record

    Ok(RollbackOutput { success: true })
}

/// Execute SQL Select
pub async fn select(
    query: &str,
    limit: Option<i32>,
    offset: Option<i32>,
    database: Option<&str>,
    _parameters: Option<Vec<Value>>,
) -> Result<SelectOutput, String> {
    let query_upper = query.to_uppercase();

    if !query_upper.starts_with("SELECT") {
        return Err("Query must be a SELECT statement".to_string());
    }

    // Extract table name (simplified parsing)
    let from_pos = query_upper.find("FROM")
        .ok_or("Missing FROM clause")?;

    let after_from = query[from_pos + 4..].trim_start();
    let table_end = after_from.find(|c: char| c.is_whitespace())
        .unwrap_or(after_from.len());
    let table_name = &after_from[..table_end];

    let key = get_database_key(database, table_name);

    let table_data = TABLES.get(&key)
        .ok_or_else(|| format!("Table '{}' does not exist", table_name))?;

    let mut rows: Vec<HashMap<String, Value>> = table_data.clone();

    // Apply WHERE clause (very basic)
    if let Some(where_pos) = query_upper.find("WHERE") {
        let where_clause = &query[where_pos + 5..];
        let parts: Vec<&str> = where_clause.split_whitespace().collect();
        if parts.len() >= 3 {
            let col = parts[0];
            let op = parts[1];
            let val = parts[2].trim_matches(|c| c == '\'' || c == '"');

            rows = rows.into_iter().filter(|row| {
                if let Some(row_val) = row.get(col) {
                    match op {
                        "=" => {
                            match row_val {
                                Value::String(s) => s == val,
                                Value::Number(n) => n.to_string() == val,
                                _ => false,
                            }
                        }
                        "!=" | "<>" => {
                            match row_val {
                                Value::String(s) => s != val,
                                Value::Number(n) => n.to_string() != val,
                                _ => true,
                            }
                        }
                        _ => true,
                    }
                } else {
                    false
                }
            }).collect();
        }
    }

    // Apply offset and limit
    let offset = offset.unwrap_or(0).max(0) as usize;
    let limit = limit.map(|l| l.max(0) as usize);

    rows = rows.into_iter().skip(offset).collect();
    if let Some(lim) = limit {
        rows = rows.into_iter().take(lim).collect();
    }

    // Get column names
    let column_names: Vec<String> = if let Some(schema) = SCHEMAS.get(&key) {
        schema.columns.iter().map(|c| c.name.clone()).collect()
    } else if let Some(first_row) = rows.first() {
        first_row.keys().cloned().collect()
    } else {
        Vec::new()
    };

    let count = rows.len() as i32;

    Ok(SelectOutput {
        column_names,
        rows,
        count,
    })
}

/// Truncate Table
pub async fn truncate_table(
    table_name: &str,
    database: Option<&str>,
) -> Result<TruncateTableOutput, String> {
    let key = get_database_key(database, table_name);

    if !TABLES.contains_key(&key) {
        return Err(format!("Table '{}' does not exist", table_name));
    }

    TABLES.get_mut(&key).unwrap().clear();

    Ok(TruncateTableOutput { success: true })
}

/// Execute SQL Update
pub async fn update(
    query: &str,
    database: Option<&str>,
    _parameters: Option<Vec<Value>>,
) -> Result<UpdateOutput, String> {
    let query_upper = query.to_uppercase();

    if !query_upper.starts_with("UPDATE") {
        return Err("Query must be an UPDATE statement".to_string());
    }

    // Extract table name
    let parts: Vec<&str> = query.split_whitespace().collect();
    let table_name = parts.get(1)
        .ok_or("Missing table name")?;

    let key = get_database_key(database, table_name);

    if !TABLES.contains_key(&key) {
        return Err(format!("Table '{}' does not exist", table_name));
    }

    let mut table_data = TABLES.get_mut(&key).unwrap();

    // Parse SET clause (simplified)
    let set_pos = query_upper.find("SET")
        .ok_or("Missing SET clause")?;

    let where_pos = query_upper.find("WHERE");
    let set_clause = if let Some(wp) = where_pos {
        &query[set_pos + 3..wp]
    } else {
        &query[set_pos + 3..]
    };

    // Parse assignments
    let mut updates = HashMap::new();
    for assignment in set_clause.split(',') {
        let parts: Vec<&str> = assignment.split('=').collect();
        if parts.len() == 2 {
            let col = parts[0].trim();
            let val = parts[1].trim().trim_matches(|c| c == '\'' || c == '"');
            let parsed_val = if let Ok(n) = val.parse::<i64>() {
                Value::Number(n.into())
            } else {
                Value::String(val.to_string())
            };
            updates.insert(col.to_string(), parsed_val);
        }
    }

    let mut affected_rows = 0;

    // Apply WHERE filter if present
    if let Some(wp) = where_pos {
        let where_clause = &query[wp + 5..];
        let where_parts: Vec<&str> = where_clause.split_whitespace().collect();
        if where_parts.len() >= 3 {
            let col = where_parts[0];
            let val = where_parts[2].trim_matches(|c| c == '\'' || c == '"');

            for row in table_data.iter_mut() {
                let matches = row.get(col).map(|v| {
                    match v {
                        Value::String(s) => s == val,
                        Value::Number(n) => n.to_string() == val,
                        _ => false,
                    }
                }).unwrap_or(false);

                if matches {
                    for (k, v) in &updates {
                        row.insert(k.clone(), v.clone());
                    }
                    affected_rows += 1;
                }
            }
        }
    } else {
        // Update all rows
        for row in table_data.iter_mut() {
            for (k, v) in &updates {
                row.insert(k.clone(), v.clone());
            }
            affected_rows += 1;
        }
    }

    Ok(UpdateOutput {
        success: true,
        affected_rows,
    })
}

#[cfg(test)]
mod tests;
