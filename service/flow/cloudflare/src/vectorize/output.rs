// Harana Actions - Cloudflare Vectorize Module Output Types

use serde::{Deserialize, Serialize};

// insert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsertOutput {
    pub count: i32,
    pub mutation_id: String,
    pub success: bool,
}

// upsert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpsertOutput {
    pub count: i32,
    pub mutation_id: String,
    pub success: bool,
}

// query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryOutput {
    pub count: i32,
    pub matches: Vec<VectorizeMatch>,
}

// query_by_id
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryByIdOutput {
    pub count: i32,
    pub matches: Vec<VectorizeMatch>,
}

// get_by_ids
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetByIdsOutput {
    pub vectors: Vec<VectorizeVector>,
}

// delete_by_ids
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteByIdsOutput {
    pub count: i32,
    pub mutation_id: String,
    pub success: bool,
}

// describe
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DescribeOutput {
    pub configured_dimensions: i32,
    pub description: String,
    pub metric: String,
    pub name: String,
    pub vectors_count: i32,
}

// info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfoOutput {
    pub dimensions: i32,
    pub metric: String,
    pub name: String,
    pub vectors_count: i32,
}

// create_metadata_index
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMetadataIndexOutput {
    pub success: bool,
}

// delete_metadata_index
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteMetadataIndexOutput {
    pub success: bool,
}

// list_metadata_indexes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListMetadataIndexesOutput {
    pub metadata_indexes: Vec<VectorizeMetadataIndex>,
}

// Helper structs

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorizeVector {
    pub id: String,
    pub values: Vec<f32>,
    pub namespace: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorizeMatch {
    pub id: String,
    pub score: f64,
    pub values: Option<Vec<f32>>,
    pub namespace: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorizeFilter {
    pub conditions: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorizeMetadataIndex {
    pub property_name: String,
    pub index_type: String,
}
