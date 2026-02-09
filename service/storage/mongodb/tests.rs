use mongodb::bson::doc;
use crate::{Entity, FilterCondition};
use super::filter_to_document;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct TestUser {
    id: String,
    name: String,
    email: String,
    age: i32,
}

impl Entity for TestUser {
    fn id(&self) -> &str {
        &self.id
    }

    fn entity_type() -> &'static str {
        "test_user"
    }
}

#[test]
fn test_filter_to_document_eq() {
    let filter = FilterCondition::eq("name", "John");
    let doc = filter_to_document(&filter).unwrap();
    assert_eq!(doc, doc! { "name": "John" });
}

#[test]
fn test_filter_to_document_ne() {
    let filter = FilterCondition::ne("status", "inactive");
    let doc = filter_to_document(&filter).unwrap();
    assert_eq!(doc, doc! { "status": { "$ne": "inactive" } });
}

#[test]
fn test_filter_to_document_gt() {
    let filter = FilterCondition::gt("age", 18);
    let doc = filter_to_document(&filter).unwrap();
    assert_eq!(doc, doc! { "age": { "$gt": 18i64 } });
}

#[test]
fn test_filter_to_document_and() {
    let filter = FilterCondition::and(vec![
        FilterCondition::eq("active", true),
        FilterCondition::gt("age", 18),
    ]);
    let doc = filter_to_document(&filter).unwrap();
    assert!(doc.contains_key("$and"));
}

#[test]
fn test_filter_to_document_or() {
    let filter = FilterCondition::or(vec![
        FilterCondition::eq("role", "admin"),
        FilterCondition::eq("role", "moderator"),
    ]);
    let doc = filter_to_document(&filter).unwrap();
    assert!(doc.contains_key("$or"));
}

#[test]
fn test_filter_to_document_in() {
    let filter = FilterCondition::is_in("status", vec!["active", "pending"]);
    let doc = filter_to_document(&filter).unwrap();
    assert!(doc.get_document("status").unwrap().contains_key("$in"));
}

#[test]
fn test_filter_to_document_is_null() {
    let filter = FilterCondition::is_null("deleted_at");
    let doc = filter_to_document(&filter).unwrap();
    assert!(doc.contains_key("deleted_at"));
}
