#[cfg(test)]
mod tests {
    use crate::*;

    #[tokio::test]
    async fn test_autocomplete_main_street() {
        let result = autocomplete("123 main", None, None, None).await.unwrap();
        assert!(!result.suggestions.is_empty());
        assert!(result.suggestions[0].description.contains("Main"));
    }

    #[tokio::test]
    async fn test_autocomplete_with_country() {
        let result = autocomplete("456 oak", Some("US"), Some(3), None).await.unwrap();
        assert!(!result.suggestions.is_empty());
        assert!(result.suggestions.len() <= 3);
        assert_eq!(result.suggestions[0].address.country, "US");
    }

    #[tokio::test]
    async fn test_autocomplete_with_limit() {
        let result = autocomplete("elm", None, Some(2), None).await.unwrap();
        assert!(result.suggestions.len() <= 2);
    }

    #[tokio::test]
    async fn test_autocomplete_empty_query() {
        let result = autocomplete("", None, None, None).await.unwrap();
        assert!(result.suggestions.is_empty());
    }

    #[tokio::test]
    async fn test_normalize_basic() {
        let address = Address {
            street: "123 main st.".to_string(),
            city: "springfield".to_string(),
            state: "illinois".to_string(),
            postal_code: "62701".to_string(),
            country: "US".to_string(),
        };

        let result = normalize(address, None, None).await.unwrap();

        assert_eq!(result.address.street, "123 Main Street");
        assert_eq!(result.address.city, "Springfield");
        assert_eq!(result.address.state, "IL");
        assert_eq!(result.address.country, "US");
    }

    #[tokio::test]
    async fn test_normalize_uppercase_format() {
        let address = Address {
            street: "456 Oak Ave".to_string(),
            city: "Portland".to_string(),
            state: "OR".to_string(),
            postal_code: "97201".to_string(),
            country: "US".to_string(),
        };

        let result = normalize(address, None, Some("uppercase")).await.unwrap();

        assert_eq!(result.address.street, result.address.street.to_uppercase());
        assert_eq!(result.address.city, result.address.city.to_uppercase());
    }

    #[tokio::test]
    async fn test_normalize_street_abbreviations() {
        let address = Address {
            street: "789 elm blvd. apt. 5".to_string(),
            city: "Austin".to_string(),
            state: "TX".to_string(),
            postal_code: "78701".to_string(),
            country: "US".to_string(),
        };

        let result = normalize(address, None, None).await.unwrap();

        assert!(result.address.street.contains("Boulevard"));
        assert!(result.address.street.contains("Apartment"));
    }

    #[tokio::test]
    async fn test_parse_full_address() {
        let result = parse("123 Main Street, Springfield, IL 62701", None).await.unwrap();

        assert_eq!(result.street, "123 Main Street");
        assert_eq!(result.city, "Springfield");
        assert_eq!(result.state, "IL");
        assert_eq!(result.postal_code, "62701");
    }

    #[tokio::test]
    async fn test_parse_with_country() {
        let result = parse("456 Oak Avenue, Portland, OR 97201, US", Some("US"))
            .await
            .unwrap();

        assert_eq!(result.street, "456 Oak Avenue");
        assert_eq!(result.city, "Portland");
        assert_eq!(result.state, "OR");
        assert_eq!(result.country, "US");
    }

    #[tokio::test]
    async fn test_parse_simple_address() {
        let result = parse("789 Elm Boulevard", None).await.unwrap();

        assert_eq!(result.street, "789 Elm Boulevard");
    }

    #[tokio::test]
    async fn test_parse_with_newlines() {
        let result = parse("123 Main St\nSpringfield, IL 62701", None).await.unwrap();

        assert!(!result.street.is_empty());
    }

    #[tokio::test]
    async fn test_validate_valid_address() {
        let address = Address {
            street: "123 Main Street".to_string(),
            city: "Springfield".to_string(),
            state: "IL".to_string(),
            postal_code: "62701".to_string(),
            country: "US".to_string(),
        };

        let result = validate(address, None).await.unwrap();

        assert!(result.valid);
        assert!(result.errors.is_empty());
    }

    #[tokio::test]
    async fn test_validate_missing_street() {
        let address = Address {
            street: "".to_string(),
            city: "Springfield".to_string(),
            state: "IL".to_string(),
            postal_code: "62701".to_string(),
            country: "US".to_string(),
        };

        let result = validate(address, None).await.unwrap();

        assert!(!result.valid);
        assert!(
            result
                .errors
                .iter()
                .any(|e| e.field == "street" && e.code == "REQUIRED")
        );
    }

    #[tokio::test]
    async fn test_validate_invalid_state() {
        let address = Address {
            street: "123 Main Street".to_string(),
            city: "Springfield".to_string(),
            state: "XX".to_string(),
            postal_code: "62701".to_string(),
            country: "US".to_string(),
        };

        let result = validate(address, None).await.unwrap();

        assert!(!result.valid);
        assert!(result.errors.iter().any(|e| e.field == "state" && e.code == "INVALID"));
    }

    #[tokio::test]
    async fn test_validate_invalid_postal_code() {
        let address = Address {
            street: "123 Main Street".to_string(),
            city: "Springfield".to_string(),
            state: "IL".to_string(),
            postal_code: "ABCDE".to_string(),
            country: "US".to_string(),
        };

        let result = validate(address, None).await.unwrap();

        assert!(!result.valid);
        assert!(
            result
                .errors
                .iter()
                .any(|e| e.field == "postal_code" && e.code == "INVALID_FORMAT")
        );
    }

    #[tokio::test]
    async fn test_validate_us_zip_plus_four() {
        let address = Address {
            street: "123 Main Street".to_string(),
            city: "Springfield".to_string(),
            state: "IL".to_string(),
            postal_code: "62701-1234".to_string(),
            country: "US".to_string(),
        };

        let result = validate(address, None).await.unwrap();

        assert!(result.valid);
    }

    #[tokio::test]
    async fn test_validate_canadian_postal_code() {
        let address = Address {
            street: "123 Main Street".to_string(),
            city: "Toronto".to_string(),
            state: "ON".to_string(),
            postal_code: "M5V 2T6".to_string(),
            country: "CA".to_string(),
        };

        let result = validate(address, None).await.unwrap();

        // Canadian addresses don't require US state validation
        let postal_errors: Vec<_> = result.errors.iter().filter(|e| e.field == "postal_code").collect();
        assert!(postal_errors.is_empty());
    }

    #[tokio::test]
    async fn test_validate_uk_postal_code() {
        let address = Address {
            street: "10 Downing Street".to_string(),
            city: "London".to_string(),
            state: "".to_string(),
            postal_code: "SW1A 2AA".to_string(),
            country: "GB".to_string(),
        };

        let result = validate(address, None).await.unwrap();

        let postal_errors: Vec<_> = result.errors.iter().filter(|e| e.field == "postal_code").collect();
        assert!(postal_errors.is_empty());
    }

    #[tokio::test]
    async fn test_validate_missing_city() {
        let address = Address {
            street: "123 Main Street".to_string(),
            city: "".to_string(),
            state: "IL".to_string(),
            postal_code: "62701".to_string(),
            country: "US".to_string(),
        };

        let result = validate(address, None).await.unwrap();

        assert!(!result.valid);
        assert!(result.errors.iter().any(|e| e.field == "city" && e.code == "REQUIRED"));
    }

    #[tokio::test]
    async fn test_validate_with_country_override() {
        let address = Address {
            street: "123 Main Street".to_string(),
            city: "Toronto".to_string(),
            state: "ON".to_string(),
            postal_code: "M5V 2T6".to_string(),
            country: "".to_string(),
        };

        let result = validate(address, Some("CA")).await.unwrap();

        // Should use CA validation rules
        let postal_errors: Vec<_> = result.errors.iter().filter(|e| e.field == "postal_code").collect();
        assert!(postal_errors.is_empty());
    }

    #[tokio::test]
    async fn test_normalize_state_full_name() {
        let address = Address {
            street: "123 Main Street".to_string(),
            city: "Springfield".to_string(),
            state: "California".to_string(),
            postal_code: "90210".to_string(),
            country: "US".to_string(),
        };

        let result = normalize(address, None, None).await.unwrap();

        assert_eq!(result.address.state, "CA");
    }

    #[tokio::test]
    async fn test_validate_short_street() {
        let address = Address {
            street: "AB".to_string(),
            city: "Springfield".to_string(),
            state: "IL".to_string(),
            postal_code: "62701".to_string(),
            country: "US".to_string(),
        };

        let result = validate(address, None).await.unwrap();

        assert!(!result.valid);
        assert!(
            result
                .errors
                .iter()
                .any(|e| e.field == "street" && e.code == "TOO_SHORT")
        );
    }
}
