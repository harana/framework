#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_tags_to_aws_tags() {
        let mut tags = HashMap::new();
        tags.insert("key1".to_string(), Value::String("value1".to_string()));
        tags.insert("key2".to_string(), Value::Number(42.into()));

        let aws_tags = tags_to_aws_tags(&tags);
        assert_eq!(aws_tags.len(), 2);
    }

}
