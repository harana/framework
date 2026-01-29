#[cfg(test)]
mod tests {
    use crate::*;
    use serde_json::json;

    #[tokio::test]
    async fn test_create_and_list_templates() {
        let tpl = create_template("tpl1", "Hello {{name}}", "Hi", Some("text/plain"), None).await.unwrap();
        assert!(!tpl.template_id.is_empty());

        let list = list_templates(Some(10), Some(0)).await.unwrap();
        assert!(list.total >= 1);
        assert!(list.templates.iter().any(|t| t.get("template_id").is_some()));
    }

    #[tokio::test]
    async fn test_delete_template() {
        let tpl = create_template("tpl-delete", "Body", "S", None, None).await.unwrap();
        let del = delete_template(&tpl.template_id).await.unwrap();
        assert!(del.success);
    }

    #[tokio::test]
    async fn test_send_and_status() {
        let send = send("Hi", "Subject", vec!["user@example.com".to_string()], Some("sender@example.com"), None, None, None, None, None).await.unwrap();
        assert!(!send.message_id.is_empty());

        let st = status(&send.message_id).await.unwrap();
        assert!(st.status == "queued");
    }

    #[tokio::test]
    async fn test_send_bulk() {
        let recipients = vec![
            [ ("email".to_string(), json!("a@example.com")) ].iter().cloned().collect(),
            [ ("email".to_string(), json!("invalid-email")) ].iter().cloned().collect(),
        ];
        let out = send_bulk("Hi", "S", recipients, Some("from@example.com"), None).await.unwrap();
        assert!(out.total >= 1);
    }

    #[tokio::test]
    async fn test_send_template() {
        let tpl = create_template("templ", "Hello {{name}}", "Hi", None, None).await.unwrap();
        let out = send_template(vec!["dest@example.com".to_string()], &tpl.template_id, None, None, Some("from@example.com"), Some([ ("name".to_string(), json!("Alice")) ].iter().cloned().collect()), None, None).await.unwrap();
        assert!(out.success);
    }

    #[tokio::test]
    async fn test_validate_email() {
        let v = validate_email("user@example.com", Some(false), Some(false)).await.unwrap();
        assert!(v.valid);
        let v2 = validate_email("not-an-email", None, None).await.unwrap();
        assert!(!v2.valid);
    }
}
