#[cfg(test)]
mod tests {
    use crate::*;
    use serde_json::json;
    use std::collections::HashMap;

    #[tokio::test]
    async fn test_generate_text_basic() {
        let result = generate_text("Write a poem about Rust", None, None, None)
            .await
            .unwrap();
        
        assert!(!result.text.is_empty());
        assert!(result.text.contains("poem about Rust"));
        assert!(result.tokens_used > 0);
    }

    #[tokio::test]
    async fn test_generate_text_with_max_tokens() {
        let result = generate_text("Hello", None, None, Some(20))
            .await
            .unwrap();
        
        assert!(!result.text.is_empty());
        assert!(result.tokens_used <= 20);
    }

    #[tokio::test]
    async fn test_chat_completion_basic() {
        let mut msg = HashMap::new();
        msg.insert("role".to_string(), json!("user"));
        msg.insert("content".to_string(), json!("Hello AI"));
        
        let result = chat_completion(vec![msg], None, None, None, None)
            .await
            .unwrap();
        
        assert!(!result.response.is_empty());
        assert!(result.response.contains("Hello AI"));
        assert!(result.tokens_used > 0);
    }

    #[tokio::test]
    async fn test_chat_completion_with_system_prompt() {
        let mut msg = HashMap::new();
        msg.insert("role".to_string(), json!("user"));
        msg.insert("content".to_string(), json!("Hello"));
        
        let result = chat_completion(
            vec![msg],
            Some(100),
            Some(0.5),
            Some("You are a helpful assistant"),
            Some("gpt-4"),
        )
        .await
        .unwrap();
        
        assert!(!result.response.is_empty());
        assert!(result.response.contains("helpful assistant"));
    }

    #[tokio::test]
    async fn test_chat_completion_empty_messages() {
        let result = chat_completion(vec![], None, None, None, None).await;
        
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "No messages provided");
    }

    #[tokio::test]
    async fn test_generate_embeddings() {
        let result = generate_embeddings("Hello world", None).await.unwrap();
        
        assert_eq!(result.dimensions, 384);
        assert_eq!(result.embeddings.len(), 384);
        
        // Check embeddings are in valid range [-1, 1]
        for val in &result.embeddings {
            assert!(*val >= -1.0 && *val <= 1.0);
        }
    }

    #[tokio::test]
    async fn test_generate_embeddings_deterministic() {
        let result1 = generate_embeddings("test", None).await.unwrap();
        let result2 = generate_embeddings("test", None).await.unwrap();
        
        // Same input should produce same embeddings
        assert_eq!(result1.embeddings, result2.embeddings);
    }

    #[tokio::test]
    async fn test_generate_image() {
        let result = generate_image("A beautiful sunset", None, None, None)
            .await
            .unwrap();
        
        assert!(!result.base64.is_empty());
        assert!(result.url.starts_with("data:image/png;base64,"));
    }

    #[tokio::test]
    async fn test_generate_image_with_options() {
        let result = generate_image(
            "A cat",
            Some("512x512"),
            Some("dall-e-3"),
            Some("hd"),
        )
        .await
        .unwrap();
        
        assert!(!result.base64.is_empty());
        assert!(!result.url.is_empty());
    }

    #[tokio::test]
    async fn test_transcribe_audio() {
        let audio_data = vec![0u8; 16000]; // 1 second of "audio"
        let result = transcribe_audio(&audio_data, None, None).await.unwrap();
        
        assert!(!result.text.is_empty());
        assert!(result.duration > 0.0);
        assert!(result.text.contains("16000 bytes"));
    }

    #[tokio::test]
    async fn test_transcribe_audio_with_language() {
        let audio_data = vec![0u8; 8000];
        let result = transcribe_audio(&audio_data, Some("whisper"), Some("es"))
            .await
            .unwrap();
        
        assert!(result.text.contains("es"));
    }

    #[tokio::test]
    async fn test_transcribe_audio_empty() {
        let result = transcribe_audio(&[], None, None).await;
        
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "No audio data provided");
    }

    #[tokio::test]
    async fn test_classify_text_positive() {
        let labels = vec!["positive".to_string(), "negative".to_string()];
        let result = classify_text(labels, "This is positive content", None)
            .await
            .unwrap();
        
        assert_eq!(result.label, "positive");
        assert!(result.confidence > 0.5);
        assert_eq!(result.scores.len(), 2);
    }

    #[tokio::test]
    async fn test_classify_text_multiple_labels() {
        let labels = vec![
            "sports".to_string(),
            "technology".to_string(),
            "politics".to_string(),
        ];
        let result = classify_text(labels.clone(), "The new smartphone has amazing technology features", None)
            .await
            .unwrap();
        
        assert_eq!(result.label, "technology");
        assert_eq!(result.scores.len(), 3);
    }

    #[tokio::test]
    async fn test_classify_text_empty_labels() {
        let result = classify_text(vec![], "Some text", None).await;
        
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "No labels provided");
    }

    #[tokio::test]
    async fn test_summarize_text() {
        let text = "This is the first sentence. This is the second sentence. This is the third sentence.";
        let result = summarize_text(text, None, Some(50)).await.unwrap();
        
        assert!(!result.summary.is_empty());
        assert!(result.summary.len() <= 55); // Allow some margin
    }

    #[tokio::test]
    async fn test_summarize_text_long() {
        let text = "First sentence here. Second sentence follows. Third one too. Fourth sentence. Fifth and final.";
        let result = summarize_text(text, None, Some(30)).await.unwrap();
        
        assert!(!result.summary.is_empty());
        assert!(result.summary.len() <= 35);
    }

    #[tokio::test]
    async fn test_summarize_empty_text() {
        let result = summarize_text("", None, Some(100)).await.unwrap();
        
        assert_eq!(result.summary, "");
    }

    #[tokio::test]
    async fn test_extract_entities() {
        let text = "John works at Microsoft in Seattle";
        let result = extract_entities(text, None, None).await.unwrap();
        
        assert!(!result.entities.is_empty());
        
        // Check that capitalized words are found
        let entity_texts: Vec<String> = result
            .entities
            .iter()
            .filter_map(|e| e.get("text").and_then(|v| v.as_str()).map(String::from))
            .collect();
        
        assert!(entity_texts.contains(&"John".to_string()));
        assert!(entity_texts.contains(&"Microsoft".to_string()));
        assert!(entity_texts.contains(&"Seattle".to_string()));
    }

    #[tokio::test]
    async fn test_extract_entities_with_types() {
        let text = "Alice and Bob went to Paris";
        let types = vec!["PERSON".to_string(), "LOCATION".to_string()];
        let result = extract_entities(text, Some(types), None).await.unwrap();
        
        assert!(!result.entities.is_empty());
        
        // Check entity structure
        if let Some(entity) = result.entities.first() {
            assert!(entity.contains_key("text"));
            assert!(entity.contains_key("type"));
            assert!(entity.contains_key("confidence"));
        }
    }

    #[tokio::test]
    async fn test_extract_entities_no_capitalized() {
        let text = "no capitals here";
        let result = extract_entities(text, None, None).await.unwrap();
        
        assert!(result.entities.is_empty());
    }
}
