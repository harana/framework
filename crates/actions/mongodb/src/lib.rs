// Harana Actions - Mongodb Module
// This module provides in-memory MongoDB-like functionality for testing and development.

pub mod output;
#[cfg(test)]
mod tests;

use dashmap::DashMap;
use once_cell::sync::Lazy;
use output::*;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

type Document = HashMap<String, Value>;
type Collection = DashMap<String, Document>;
type Database = DashMap<String, Arc<Collection>>;

static DATABASES: Lazy<DashMap<String, Arc<Database>>> = Lazy::new(DashMap::new);
static INDEXES: Lazy<DashMap<String, Vec<IndexInfo>>> = Lazy::new(DashMap::new);

#[derive(Debug, Clone)]
struct IndexInfo {
    name: String,
    #[allow(dead_code)]
    keys: HashMap<String, Value>,
    #[allow(dead_code)]
    unique: bool,
}

fn get_or_create_database(db_name: &str) -> Arc<Database> {
    DATABASES
        .entry(db_name.to_string())
        .or_insert_with(|| Arc::new(DashMap::new()))
        .clone()
}

fn get_or_create_collection(db_name: &str, coll_name: &str) -> Arc<Collection> {
    let db = get_or_create_database(db_name);
    
    // First check if it exists
    if let Some(existing) = db.get(coll_name) {
        return existing.clone();
    }
    
    // Create new collection
    let new_coll = Arc::new(DashMap::new());
    db.insert(coll_name.to_string(), new_coll.clone());
    new_coll
}

fn matches_filter(doc: &Document, filter: &HashMap<String, Value>) -> bool {
    for (key, filter_value) in filter {
        if let Some(doc_value) = doc.get(key) {
            if !values_match(doc_value, filter_value) {
                return false;
            }
        } else {
            return false;
        }
    }
    true
}

fn values_match(doc_value: &Value, filter_value: &Value) -> bool {
    // Check if filter_value contains operators
    if let Value::Object(filter_obj) = filter_value {
        // This is a filter with operators like {"$gte": 30}
        for (op, op_value) in filter_obj {
            match op.as_str() {
                "$eq" => {
                    if doc_value != op_value {
                        return false;
                    }
                }
                "$ne" => {
                    if doc_value == op_value {
                        return false;
                    }
                }
                "$gt" => {
                    if !compare_values(doc_value, op_value, |a, b| a > b) {
                        return false;
                    }
                }
                "$gte" => {
                    if !compare_values(doc_value, op_value, |a, b| a >= b) {
                        return false;
                    }
                }
                "$lt" => {
                    if !compare_values(doc_value, op_value, |a, b| a < b) {
                        return false;
                    }
                }
                "$lte" => {
                    if !compare_values(doc_value, op_value, |a, b| a <= b) {
                        return false;
                    }
                }
                "$in" => {
                    if let Value::Array(arr) = op_value {
                        if !arr.contains(doc_value) {
                            return false;
                        }
                    }
                }
                _ => {}
            }
        }
        return true;
    }
    
    // No operators, just direct comparison
    doc_value == filter_value
}

fn compare_values<F>(a: &Value, b: &Value, cmp: F) -> bool
where
    F: Fn(f64, f64) -> bool,
{
    match (a, b) {
        (Value::Number(a_num), Value::Number(b_num)) => {
            if let (Some(a_f), Some(b_f)) = (a_num.as_f64(), b_num.as_f64()) {
                return cmp(a_f, b_f);
            }
        }
        _ => {}
    }
    false
}

fn apply_update(doc: &mut Document, update: &HashMap<String, Value>) {
    for (key, value) in update {
        if key == "$set" {
            if let Value::Object(set_fields) = value {
                for (field, field_value) in set_fields {
                    doc.insert(field.clone(), field_value.clone());
                }
            }
        } else if key == "$inc" {
            if let Value::Object(inc_fields) = value {
                for (field, inc_value) in inc_fields {
                    if let Some(Value::Number(current)) = doc.get(field) {
                        if let (Some(curr_f), Some(inc_f)) = (current.as_f64(), inc_value.as_f64()) {
                            doc.insert(field.clone(), json!(curr_f + inc_f));
                        }
                    }
                }
            }
        } else if key == "$unset" {
            if let Value::Object(unset_fields) = value {
                for field in unset_fields.keys() {
                    doc.remove(field);
                }
            }
        } else {
            doc.insert(key.clone(), value.clone());
        }
    }
}

/// Insert a single document into a collection
pub async fn insert(
    collection: &str,
    database: &str,
    document: HashMap<String, Value>,
) -> Result<InsertOutput, String> {
    let coll = get_or_create_collection(database, collection);
    
    let mut doc = document.clone();
    let id = doc
        .get("_id")
        .and_then(|v| v.as_str())
        .map(String::from)
        .unwrap_or_else(|| Uuid::new_v4().to_string());
    
    doc.insert("_id".to_string(), Value::String(id.clone()));
    
    coll.insert(id.clone(), doc);
    
    Ok(InsertOutput {
        inserted_id: id,
        success: true,
    })
}

/// Insert multiple documents into a collection
pub async fn insert_many(
    collection: &str,
    database: &str,
    documents: Vec<HashMap<String, Value>>,
    _ordered: Option<bool>,
) -> Result<InsertManyOutput, String> {
    let coll = get_or_create_collection(database, collection);
    let mut inserted_ids = Vec::new();
    
    for mut doc in documents {
        let id = doc
            .get("_id")
            .and_then(|v| v.as_str())
            .map(String::from)
            .unwrap_or_else(|| Uuid::new_v4().to_string());
        
        doc.insert("_id".to_string(), Value::String(id.clone()));
        coll.insert(id.clone(), doc);
        inserted_ids.push(id);
    }
    
    Ok(InsertManyOutput {
        inserted_count: inserted_ids.len() as i32,
        inserted_ids,
        success: true,
    })
}

/// Find a single document matching the filter
pub async fn find_one(
    collection: &str,
    database: &str,
    filter: HashMap<String, Value>,
    projection: Option<HashMap<String, i32>>,
) -> Result<FindOneOutput, String> {
    let coll = get_or_create_collection(database, collection);
    
    for item in coll.iter() {
        if matches_filter(item.value(), &filter) {
            let mut doc = item.value().clone();
            
            if let Some(proj) = projection {
                let mut projected = HashMap::new();
                for (field, include) in proj {
                    if include != 0 {
                        if let Some(value) = doc.get(&field) {
                            projected.insert(field, value.clone());
                        }
                    }
                }
                doc = projected;
            }
            
            return Ok(FindOneOutput {
                document: Some(doc),
                found: true,
            });
        }
    }
    
    Ok(FindOneOutput {
        document: None,
        found: false,
    })
}

/// Find multiple documents matching the filter
pub async fn find(
    collection: &str,
    database: &str,
    filter: Option<HashMap<String, Value>>,
    limit: Option<i32>,
    projection: Option<HashMap<String, i32>>,
    skip: Option<i32>,
    sort: Option<HashMap<String, i32>>,
) -> Result<FindOutput, String> {
    let coll = get_or_create_collection(database, collection);
    let filter = filter.unwrap_or_default();
    let skip = skip.unwrap_or(0) as usize;
    
    let mut documents: Vec<Document> = coll
        .iter()
        .filter(|item| matches_filter(item.value(), &filter))
        .map(|item| item.value().clone())
        .collect();
    
    // Apply sorting if specified
    if let Some(sort_fields) = sort {
        documents.sort_by(|a, b| {
            for (field, order) in &sort_fields {
                let a_val = a.get(field);
                let b_val = b.get(field);
                
                let cmp = match (a_val, b_val) {
                    (Some(Value::String(a_str)), Some(Value::String(b_str))) => a_str.cmp(b_str),
                    (Some(Value::Number(a_num)), Some(Value::Number(b_num))) => {
                        if let (Some(a_f), Some(b_f)) = (a_num.as_f64(), b_num.as_f64()) {
                            a_f.partial_cmp(&b_f).unwrap_or(std::cmp::Ordering::Equal)
                        } else {
                            std::cmp::Ordering::Equal
                        }
                    }
                    _ => std::cmp::Ordering::Equal,
                };
                
                if cmp != std::cmp::Ordering::Equal {
                    return if *order < 0 { cmp.reverse() } else { cmp };
                }
            }
            std::cmp::Ordering::Equal
        });
    }
    
    // Apply skip and limit
    documents = documents.into_iter().skip(skip).collect();
    if let Some(lim) = limit {
        documents.truncate(lim as usize);
    }
    
    // Apply projection if specified
    if let Some(proj) = projection {
        documents = documents
            .into_iter()
            .map(|doc| {
                let mut projected = HashMap::new();
                for (field, include) in &proj {
                    if *include != 0 {
                        if let Some(value) = doc.get(field) {
                            projected.insert(field.clone(), value.clone());
                        }
                    }
                }
                projected
            })
            .collect();
    }
    
    let count = documents.len() as i32;
    
    Ok(FindOutput {
        count,
        documents,
    })
}

/// Update a single document matching the filter
pub async fn update_one(
    collection: &str,
    database: &str,
    filter: HashMap<String, Value>,
    update: HashMap<String, Value>,
    upsert: Option<bool>,
) -> Result<UpdateOneOutput, String> {
    let coll = get_or_create_collection(database, collection);
    
    for mut item in coll.iter_mut() {
        if matches_filter(item.value(), &filter) {
            apply_update(item.value_mut(), &update);
            return Ok(UpdateOneOutput {
                matched_count: 1,
                modified_count: 1,
                success: true,
                upserted_id: None,
            });
        }
    }
    
    // Handle upsert
    if upsert.unwrap_or(false) {
        let mut new_doc = filter.clone();
        apply_update(&mut new_doc, &update);
        
        let id = new_doc
            .get("_id")
            .and_then(|v| v.as_str())
            .map(String::from)
            .unwrap_or_else(|| Uuid::new_v4().to_string());
        
        new_doc.insert("_id".to_string(), Value::String(id.clone()));
        coll.insert(id.clone(), new_doc);
        
        return Ok(UpdateOneOutput {
            matched_count: 0,
            modified_count: 0,
            success: true,
            upserted_id: Some(id),
        });
    }
    
    Ok(UpdateOneOutput {
        matched_count: 0,
        modified_count: 0,
        success: true,
        upserted_id: None,
    })
}

/// Update multiple documents matching the filter
pub async fn update_many(
    collection: &str,
    database: &str,
    filter: HashMap<String, Value>,
    update: HashMap<String, Value>,
    upsert: Option<bool>,
) -> Result<UpdateManyOutput, String> {
    let coll = get_or_create_collection(database, collection);
    let mut matched = 0;
    let mut modified = 0;
    
    for mut item in coll.iter_mut() {
        if matches_filter(item.value(), &filter) {
            matched += 1;
            apply_update(item.value_mut(), &update);
            modified += 1;
        }
    }
    
    if matched == 0 && upsert.unwrap_or(false) {
        let mut new_doc = filter.clone();
        apply_update(&mut new_doc, &update);
        
        let id = new_doc
            .get("_id")
            .and_then(|v| v.as_str())
            .map(String::from)
            .unwrap_or_else(|| Uuid::new_v4().to_string());
        
        new_doc.insert("_id".to_string(), Value::String(id.clone()));
        coll.insert(id.clone(), new_doc);
        
        return Ok(UpdateManyOutput {
            matched_count: 0,
            modified_count: 0,
            success: true,
            upserted_id: Some(id),
        });
    }
    
    Ok(UpdateManyOutput {
        matched_count: matched,
        modified_count: modified,
        success: true,
        upserted_id: None,
    })
}

/// Replace a single document matching the filter
pub async fn replace_one(
    collection: &str,
    database: &str,
    filter: HashMap<String, Value>,
    replacement: HashMap<String, Value>,
    upsert: Option<bool>,
) -> Result<ReplaceOneOutput, String> {
    let coll = get_or_create_collection(database, collection);
    
    for mut item in coll.iter_mut() {
        if matches_filter(item.value(), &filter) {
            let id = item.value().get("_id").cloned();
            let mut new_doc = replacement.clone();
            
            if let Some(id_val) = id {
                new_doc.insert("_id".to_string(), id_val);
            }
            
            *item.value_mut() = new_doc;
            
            return Ok(ReplaceOneOutput {
                matched_count: 1,
                modified_count: 1,
                success: true,
                upserted_id: None,
            });
        }
    }
    
    // Handle upsert
    if upsert.unwrap_or(false) {
        let mut new_doc = replacement.clone();
        let id = new_doc
            .get("_id")
            .and_then(|v| v.as_str())
            .map(String::from)
            .unwrap_or_else(|| Uuid::new_v4().to_string());
        
        new_doc.insert("_id".to_string(), Value::String(id.clone()));
        coll.insert(id.clone(), new_doc);
        
        return Ok(ReplaceOneOutput {
            matched_count: 0,
            modified_count: 0,
            success: true,
            upserted_id: Some(id),
        });
    }
    
    Ok(ReplaceOneOutput {
        matched_count: 0,
        modified_count: 0,
        success: true,
        upserted_id: None,
    })
}

/// Delete a single document matching the filter
pub async fn delete_one(
    collection: &str,
    database: &str,
    filter: HashMap<String, Value>,
) -> Result<DeleteOneOutput, String> {
    let coll = get_or_create_collection(database, collection);
    
    // Find the first matching key without holding the iterator
    let key_to_delete = coll
        .iter()
        .find(|item| matches_filter(item.value(), &filter))
        .map(|item| item.key().clone());
    
    if let Some(key) = key_to_delete {
        coll.remove(&key);
        return Ok(DeleteOneOutput {
            deleted_count: 1,
            success: true,
        });
    }
    
    Ok(DeleteOneOutput {
        deleted_count: 0,
        success: true,
    })
}

/// Delete multiple documents matching the filter
pub async fn delete_many(
    collection: &str,
    database: &str,
    filter: HashMap<String, Value>,
) -> Result<DeleteManyOutput, String> {
    let coll = get_or_create_collection(database, collection);
    let mut deleted_count = 0;
    
    let keys_to_delete: Vec<String> = coll
        .iter()
        .filter(|item| matches_filter(item.value(), &filter))
        .map(|item| item.key().clone())
        .collect();
    
    for key in keys_to_delete {
        coll.remove(&key);
        deleted_count += 1;
    }
    
    Ok(DeleteManyOutput {
        deleted_count,
        success: true,
    })
}

/// Count documents matching the filter
pub async fn count(
    collection: &str,
    database: &str,
    filter: Option<HashMap<String, Value>>,
) -> Result<CountOutput, String> {
    let coll = get_or_create_collection(database, collection);
    let filter = filter.unwrap_or_default();
    
    let count = coll
        .iter()
        .filter(|item| matches_filter(item.value(), &filter))
        .count() as i32;
    
    Ok(CountOutput { count })
}

/// Aggregate documents using a pipeline (simplified implementation)
pub async fn aggregate(
    collection: &str,
    database: &str,
    pipeline: Vec<HashMap<String, Value>>,
) -> Result<AggregateOutput, String> {
    let coll = get_or_create_collection(database, collection);
    let mut documents: Vec<Document> = coll.iter().map(|item| item.value().clone()).collect();
    
    for stage in pipeline {
        if let Some(match_stage) = stage.get("$match") {
            if let Value::Object(filter) = match_stage {
                let filter_map: HashMap<String, Value> = filter
                    .iter()
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect();
                documents.retain(|doc| matches_filter(doc, &filter_map));
            }
        }
        
        if let Some(limit_stage) = stage.get("$limit") {
            if let Some(limit) = limit_stage.as_i64() {
                documents.truncate(limit as usize);
            }
        }
        
        if let Some(skip_stage) = stage.get("$skip") {
            if let Some(skip) = skip_stage.as_i64() {
                documents = documents.into_iter().skip(skip as usize).collect();
            }
        }
    }
    
    Ok(AggregateOutput { documents })
}

/// Create an index on a collection
pub async fn create_index(
    collection: &str,
    database: &str,
    keys: HashMap<String, Value>,
    name: Option<&str>,
    unique: Option<bool>,
) -> Result<CreateIndexOutput, String> {
    let index_key = format!("{}:{}", database, collection);
    let index_name = name.map(String::from).unwrap_or_else(|| {
        keys.keys()
            .map(|k| format!("{}_{}", k, 1))
            .collect::<Vec<_>>()
            .join("_")
    });
    
    let index_info = IndexInfo {
        name: index_name.clone(),
        keys,
        unique: unique.unwrap_or(false),
    };
    
    INDEXES
        .entry(index_key)
        .or_insert_with(Vec::new)
        .push(index_info);
    
    Ok(CreateIndexOutput {
        index_name,
        success: true,
    })
}

/// Drop an index from a collection
pub async fn drop_index(
    collection: &str,
    database: &str,
    index_name: &str,
) -> Result<DropIndexOutput, String> {
    let index_key = format!("{}:{}", database, collection);
    
    if let Some(mut indexes) = INDEXES.get_mut(&index_key) {
        indexes.retain(|idx| idx.name != index_name);
        return Ok(DropIndexOutput { success: true });
    }
    
    Ok(DropIndexOutput { success: false })
}

/// List all collections in a database
pub async fn list_collections(database: &str) -> Result<ListCollectionsOutput, String> {
    let db = get_or_create_database(database);
    let collections: Vec<String> = db.iter().map(|item| item.key().clone()).collect();
    
    Ok(ListCollectionsOutput { collections })
}

/// Drop a collection from a database
pub async fn drop_collection(
    collection: &str,
    database: &str,
) -> Result<DropCollectionOutput, String> {
    let db = get_or_create_database(database);
    db.remove(collection);
    
    // Also remove associated indexes
    let index_key = format!("{}:{}", database, collection);
    INDEXES.remove(&index_key);
    
    Ok(DropCollectionOutput { success: true })
}

/// Perform bulk write operations
pub async fn bulk_write(
    collection: &str,
    database: &str,
    operations: Vec<HashMap<String, Value>>,
    _ordered: Option<bool>,
) -> Result<BulkWriteOutput, String> {
    let mut inserted_count = 0;
    let mut matched_count = 0;
    let mut modified_count = 0;
    let mut deleted_count = 0;
    let upserted_count = 0;
    
    for op in operations {
        if let Some(insert_op) = op.get("insertOne") {
            if let Value::Object(doc_obj) = insert_op {
                let doc: HashMap<String, Value> = doc_obj
                    .iter()
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect();
                if insert(collection, database, doc).await.is_ok() {
                    inserted_count += 1;
                }
            }
        } else if let Some(update_op) = op.get("updateOne") {
            if let Value::Object(update_obj) = update_op {
                let filter = update_obj
                    .get("filter")
                    .and_then(|v| v.as_object())
                    .map(|obj| {
                        obj.iter()
                            .map(|(k, v)| (k.clone(), v.clone()))
                            .collect()
                    })
                    .unwrap_or_default();
                
                let update = update_obj
                    .get("update")
                    .and_then(|v| v.as_object())
                    .map(|obj| {
                        obj.iter()
                            .map(|(k, v)| (k.clone(), v.clone()))
                            .collect()
                    })
                    .unwrap_or_default();
                
                if let Ok(result) = update_one(collection, database, filter, update, None).await {
                    matched_count += result.matched_count;
                    modified_count += result.modified_count;
                }
            }
        } else if let Some(delete_op) = op.get("deleteOne") {
            if let Value::Object(delete_obj) = delete_op {
                let filter = delete_obj
                    .get("filter")
                    .and_then(|v| v.as_object())
                    .map(|obj| {
                        obj.iter()
                            .map(|(k, v)| (k.clone(), v.clone()))
                            .collect()
                    })
                    .unwrap_or_default();
                
                if let Ok(result) = delete_one(collection, database, filter).await {
                    deleted_count += result.deleted_count;
                }
            }
        }
    }
    
    Ok(BulkWriteOutput {
        success: true,
        modified_count,
        matched_count,
        inserted_count,
        deleted_count,
        upserted_count,
    })
}
