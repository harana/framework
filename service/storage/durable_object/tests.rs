#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_do_bind_value_from_json_string() {
        let v = serde_json::json!("hello");
        let bind = DOBindValue::from_json(&v);
        matches!(bind, DOBindValue::Text(s) if s == "hello");
    }

    #[test]
    fn test_do_bind_value_from_json_int() {
        let v = serde_json::json!(42);
        let bind = DOBindValue::from_json(&v);
        matches!(bind, DOBindValue::Integer(42));
    }

    #[test]
    fn test_do_bind_value_from_json_float() {
        let v = serde_json::json!(3.14);
        let bind = DOBindValue::from_json(&v);
        matches!(bind, DOBindValue::Real(f) if (f - 3.14).abs() < f64::EPSILON);
    }

    #[test]
    fn test_do_bind_value_from_json_bool() {
        let v = serde_json::json!(true);
        let bind = DOBindValue::from_json(&v);
        matches!(bind, DOBindValue::Bool(true));
    }

    #[test]
    fn test_do_bind_value_from_json_null() {
        let v = serde_json::json!(null);
        let bind = DOBindValue::from_json(&v);
        matches!(bind, DOBindValue::Null);
    }

    #[test]
    fn test_do_bind_value_to_sql_value_text() {
        let bind = DOBindValue::Text("hello".to_string());
        let sql_val = bind.to_sql_value();
        assert_eq!(sql_val, SqlStorageValue::String("hello".to_string()));
    }

    #[test]
    fn test_do_bind_value_to_sql_value_integer() {
        let bind = DOBindValue::Integer(42);
        let sql_val = bind.to_sql_value();
        assert_eq!(sql_val, SqlStorageValue::Integer(42));
    }

    #[test]
    fn test_do_bind_value_to_sql_value_null() {
        let bind = DOBindValue::Null;
        let sql_val = bind.to_sql_value();
        assert_eq!(sql_val, SqlStorageValue::Null);
    }

    #[test]
    fn test_do_bind_value_to_sql_value_opt_text_some() {
        let bind = DOBindValue::OptText(Some("hello".to_string()));
        let sql_val = bind.to_sql_value();
        assert_eq!(sql_val, SqlStorageValue::String("hello".to_string()));
    }

    #[test]
    fn test_do_bind_value_to_sql_value_opt_text_none() {
        let bind = DOBindValue::OptText(None);
        let sql_val = bind.to_sql_value();
        assert_eq!(sql_val, SqlStorageValue::Null);
    }

    #[test]
    fn test_json_to_sql_value_string() {
        let v = serde_json::json!("test");
        let sql_val = json_to_sql_value(&v);
        assert_eq!(sql_val, SqlStorageValue::String("test".to_string()));
    }

    #[test]
    fn test_json_to_sql_value_integer() {
        let v = serde_json::json!(123);
        let sql_val = json_to_sql_value(&v);
        assert_eq!(sql_val, SqlStorageValue::Integer(123));
    }

    #[test]
    fn test_json_to_sql_value_float() {
        let v = serde_json::json!(1.5);
        let sql_val = json_to_sql_value(&v);
        assert_eq!(sql_val, SqlStorageValue::Float(1.5));
    }

    #[test]
    fn test_json_to_sql_value_bool() {
        let v = serde_json::json!(true);
        let sql_val = json_to_sql_value(&v);
        assert_eq!(sql_val, SqlStorageValue::Boolean(true));
    }

    #[test]
    fn test_json_to_sql_value_null() {
        let v = serde_json::json!(null);
        let sql_val = json_to_sql_value(&v);
        assert_eq!(sql_val, SqlStorageValue::Null);
    }
}
