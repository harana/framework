// Harana Components - MongoDB Common Utilities

use mongodb::{
    bson::{doc, Bson, Document},
    options::FindOptions,
};
use serde_json::Value;

use crate::{FilterCondition, StorageError, StorageResult};

/// Converts a serde_json Value to BSON.
pub(crate) fn json_value_to_bson(value: &Value) -> StorageResult<Bson> {
    Ok(match value {
        Value::Null => Bson::Null,
        Value::Bool(b) => Bson::Boolean(*b),
        Value::Number(n) => n
            .as_i64()
            .map(Bson::Int64)
            .or_else(|| n.as_f64().map(Bson::Double))
            .ok_or_else(|| StorageError::SerializationError("Failed to convert number to BSON".into()))?,
        Value::String(s) => Bson::String(s.clone()),
        Value::Array(arr) => Bson::Array(arr.iter().map(json_value_to_bson).collect::<StorageResult<_>>()?),
        Value::Object(map) => {
            let doc = map
                .iter()
                .map(|(k, v)| Ok((k.clone(), json_value_to_bson(v)?)))
                .collect::<StorageResult<Document>>()?;
            Bson::Document(doc)
        }
    })
}

/// Converts a BSON document to a JSON value.
pub(crate) fn bson_doc_to_json(doc: &Document) -> StorageResult<Value> {
    serde_json::to_value(doc)
        .map_err(|e| StorageError::SerializationError(format!("Failed to convert BSON to JSON: {e}")))
}

/// Maps a MongoDB error to a StorageError with a context message.
pub(crate) fn map_mongo_error(context: &str) -> impl Fn(mongodb::error::Error) -> StorageError + '_ {
    move |e| StorageError::QueryError(format!("{context}: {e}"))
}

/// Maps a MongoDB error to a ConnectionError.
pub(crate) fn map_connection_error(context: &str) -> impl Fn(mongodb::error::Error) -> StorageError + '_ {
    move |e| StorageError::ConnectionError(format!("{context}: {e}"))
}

/// Builds FindOptions from limit, skip, and optional sort parameters.
pub(crate) fn build_find_options(
    limit: Option<i64>,
    skip: Option<u64>,
    sort: Option<Document>,
    projection: Option<Document>,
) -> FindOptions {
    FindOptions::builder()
        .limit(limit)
        .skip(skip)
        .sort(sort)
        .projection(projection)
        .build()
}

/// Converts a FilterCondition to a MongoDB Document.
pub(crate) fn filter_to_document(filter: &FilterCondition) -> StorageResult<Document> {
    Ok(match filter {
        // Simple comparisons
        FilterCondition::Eq(field, value) => doc! { field: json_value_to_bson(value)? },
        FilterCondition::Ne(field, value) => doc! { field: { "$ne": json_value_to_bson(value)? } },
        FilterCondition::Gt(field, value) => doc! { field: { "$gt": json_value_to_bson(value)? } },
        FilterCondition::Gte(field, value) => doc! { field: { "$gte": json_value_to_bson(value)? } },
        FilterCondition::Lt(field, value) => doc! { field: { "$lt": json_value_to_bson(value)? } },
        FilterCondition::Lte(field, value) => doc! { field: { "$lte": json_value_to_bson(value)? } },

        // Array membership
        FilterCondition::In(field, values) => {
            let bson_values: Vec<Bson> = values.iter().map(json_value_to_bson).collect::<StorageResult<_>>()?;
            doc! { field: { "$in": bson_values } }
        }
        FilterCondition::NotIn(field, values) => {
            let bson_values: Vec<Bson> = values.iter().map(json_value_to_bson).collect::<StorageResult<_>>()?;
            doc! { field: { "$nin": bson_values } }
        }

        // String patterns
        FilterCondition::Contains(field, value) => doc! { field: { "$regex": value, "$options": "i" } },
        FilterCondition::StartsWith(field, value) => doc! { field: { "$regex": format!("^{}", regex::escape(value)) } },
        FilterCondition::EndsWith(field, value) => doc! { field: { "$regex": format!("{}$", regex::escape(value)) } },

        // Null checks
        FilterCondition::IsNull(field) => doc! { field: { "$eq": Bson::Null } },
        FilterCondition::IsNotNull(field) => doc! { field: { "$ne": Bson::Null } },

        // Logical operators
        FilterCondition::And(conditions) => {
            let docs: Vec<Document> = conditions.iter().map(filter_to_document).collect::<StorageResult<_>>()?;
            doc! { "$and": docs }
        }
        FilterCondition::Or(conditions) => {
            let docs: Vec<Document> = conditions.iter().map(filter_to_document).collect::<StorageResult<_>>()?;
            doc! { "$or": docs }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_value_to_bson() {
        assert_eq!(json_value_to_bson(&serde_json::json!(null)).unwrap(), Bson::Null);
        assert_eq!(json_value_to_bson(&serde_json::json!(true)).unwrap(), Bson::Boolean(true));
        assert_eq!(json_value_to_bson(&serde_json::json!(42)).unwrap(), Bson::Int64(42));
        assert_eq!(json_value_to_bson(&serde_json::json!("hello")).unwrap(), Bson::String("hello".to_string()));
    }

    #[test]
    fn test_json_value_to_bson_array() {
        let arr = json_value_to_bson(&serde_json::json!([1, 2, 3])).unwrap();
        match arr {
            Bson::Array(v) => assert_eq!(v.len(), 3),
            _ => panic!("Expected array"),
        }
    }

    #[test]
    fn test_json_value_to_bson_object() {
        let obj = json_value_to_bson(&serde_json::json!({"key": "value"})).unwrap();
        match obj {
            Bson::Document(doc) => assert_eq!(doc.get_str("key").unwrap(), "value"),
            _ => panic!("Expected document"),
        }
    }
}
