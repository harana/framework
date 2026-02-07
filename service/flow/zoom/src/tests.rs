use super::*;

#[tokio::test]
async fn test_create_meeting() {
    clear_all_data();
    
    let result = create_meeting(
        "Team Standup".to_string(),
        Some(MeetingType::Scheduled),
        Some(Utc::now() + Duration::hours(1)),
        Some(30),
        Some("America/New_York".to_string()),
        Some("Daily team standup meeting".to_string()),
        None,
        None,
        None,
    ).await;

    assert!(result.success);
    assert!(!result.meeting_id.is_empty());
    assert!(result.join_url.contains(&result.meeting_id));
    assert!(result.start_url.contains(&result.meeting_id));
    assert!(result.password.is_some());
}

#[tokio::test]
async fn test_get_meeting() {
    clear_all_data();
    
    let created = create_meeting(
        "Test Meeting".to_string(),
        None,
        None,
        Some(60),
        None,
        None,
        Some("test123".to_string()),
        None,
        None,
    ).await;

    let result = get_meeting(created.meeting_id.clone()).await;

    assert_eq!(result.meeting_id, created.meeting_id);
    assert_eq!(result.topic, "Test Meeting");
    assert_eq!(result.duration, 60);
    assert_eq!(result.password, Some("test123".to_string()));
}

#[tokio::test]
async fn test_update_meeting() {
    clear_all_data();
    
    let created = create_meeting(
        "Original Topic".to_string(),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ).await;

    let update_result = update_meeting(
        created.meeting_id.clone(),
        Some("Updated Topic".to_string()),
        None,
        None,
        Some(90),
        None,
        Some("New agenda".to_string()),
        None,
        None,
    ).await;

    assert!(update_result.success);

    let updated = get_meeting(created.meeting_id).await;
    assert_eq!(updated.topic, "Updated Topic");
    assert_eq!(updated.duration, 90);
    assert_eq!(updated.agenda, Some("New agenda".to_string()));
}

#[tokio::test]
async fn test_delete_meeting() {
    clear_all_data();
    
    let created = create_meeting(
        "Meeting to Delete".to_string(),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ).await;

    let result = delete_meeting(created.meeting_id.clone(), None, None).await;
    assert!(result.success);

    let get_result = get_meeting(created.meeting_id).await;
    assert_eq!(get_result.status, "not_found");
}

#[tokio::test]
async fn test_list_meetings() {
    clear_all_data();
    
    // Create several meetings
    for i in 0..5 {
        create_meeting(
            format!("Meeting {}", i),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        ).await;
    }

    let result = list_meetings(None, None, Some(3), None).await;
    
    assert_eq!(result.meetings.len(), 3);
    assert_eq!(result.total_records, 5);
}

#[tokio::test]
async fn test_registrant_lifecycle() {
    clear_all_data();
    
    let meeting = create_meeting(
        "Webinar with Registration".to_string(),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ).await;

    // Add registrant
    let add_result = add_registrant(
        meeting.meeting_id.clone(),
        "attendee@example.com".to_string(),
        "John".to_string(),
        Some("Doe".to_string()),
        None,
        None,
    ).await;

    assert!(add_result.success);
    assert!(!add_result.registrant_id.is_empty());
    assert!(!add_result.join_url.is_empty());

    // List registrants
    let list_result = list_registrants(
        meeting.meeting_id.clone(),
        Some(RegistrantStatus::Approved),
        None,
        None,
    ).await;

    assert_eq!(list_result.registrants.len(), 1);
    assert_eq!(list_result.registrants[0].email, "attendee@example.com");
}

#[tokio::test]
async fn test_update_registrant_status() {
    clear_all_data();
    
    let meeting = create_meeting(
        "Meeting".to_string(),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ).await;

    let registrant = add_registrant(
        meeting.meeting_id.clone(),
        "test@example.com".to_string(),
        "Test".to_string(),
        None,
        None,
        None,
    ).await;

    let result = update_registrant_status(
        meeting.meeting_id,
        RegistrantAction::Deny,
        vec![ZoomRegistrantId {
            id: registrant.registrant_id,
            email: "test@example.com".to_string(),
        }],
    ).await;

    assert!(result.success);
}

#[tokio::test]
async fn test_create_webinar() {
    clear_all_data();
    
    let result = create_webinar(
        "Product Launch Webinar".to_string(),
        Some(WebinarType::Webinar),
        Some(Utc::now() + Duration::days(7)),
        Some(120),
        Some("UTC".to_string()),
        Some("Join us for our product launch!".to_string()),
        None,
        None,
    ).await;

    assert!(result.success);
    assert!(!result.webinar_id.is_empty());
    assert!(result.join_url.contains("/w/"));
}

#[tokio::test]
async fn test_get_webinar() {
    clear_all_data();
    
    let created = create_webinar(
        "Test Webinar".to_string(),
        None,
        None,
        Some(90),
        None,
        None,
        Some("webinar123".to_string()),
        None,
    ).await;

    let result = get_webinar(created.webinar_id.clone()).await;

    assert_eq!(result.webinar_id, created.webinar_id);
    assert_eq!(result.topic, "Test Webinar");
    assert_eq!(result.duration, 90);
}

#[tokio::test]
async fn test_delete_webinar() {
    clear_all_data();
    
    let created = create_webinar(
        "Webinar to Delete".to_string(),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ).await;

    let result = delete_webinar(created.webinar_id.clone(), None).await;
    assert!(result.success);
}

#[tokio::test]
async fn test_get_user() {
    clear_all_data();
    
    // Test with default "me" user
    let result = get_user(None).await;
    
    assert!(!result.user_id.is_empty());
    assert!(!result.email.is_empty());
    assert_eq!(result.status, "active");
}

#[tokio::test]
async fn test_end_meeting() {
    clear_all_data();
    
    let meeting = create_meeting(
        "Meeting to End".to_string(),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ).await;

    let result = end_meeting(meeting.meeting_id.clone()).await;
    assert!(result.success);

    let ended = get_meeting(meeting.meeting_id).await;
    assert_eq!(ended.status, "ended");
}

#[tokio::test]
async fn test_send_invitation() {
    clear_all_data();
    
    let meeting = create_meeting(
        "Meeting with Invites".to_string(),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ).await;

    let result = send_invitation(
        meeting.meeting_id,
        vec![
            "user1@example.com".to_string(),
            "user2@example.com".to_string(),
        ],
    ).await;

    assert!(result.success);
}

#[tokio::test]
async fn test_send_invitation_nonexistent_meeting() {
    clear_all_data();
    
    let result = send_invitation(
        "nonexistent".to_string(),
        vec!["user@example.com".to_string()],
    ).await;

    assert!(!result.success);
}
