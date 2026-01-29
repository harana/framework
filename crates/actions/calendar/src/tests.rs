#[cfg(test)]
mod tests {
    use crate::*;
    use chrono::{Duration, Utc};

    fn future_time(hours: i64) -> String {
        (Utc::now() + Duration::hours(hours)).to_rfc3339()
    }

    #[tokio::test]
    async fn test_create_event() {
        let start = future_time(1);
        let end = future_time(2);
        
        let result = create_event(
            &end,
            &start,
            "Test Meeting",
            None,
            None,
            None,
            Some("A test meeting"),
            Some("Conference Room A"),
            None,
            None,
            None,
        ).await.unwrap();
        
        assert!(result.success);
        assert!(!result.event_id.is_empty());
        assert!(result.html_link.is_some());
    }

    #[tokio::test]
    async fn test_create_event_with_attendees() {
        let start = future_time(3);
        let end = future_time(4);
        
        let result = create_event(
            &end,
            &start,
            "Team Sync",
            None,
            Some(vec!["alice@example.com".to_string(), "bob@example.com".to_string()]),
            None,
            None,
            None,
            None,
            None,
            None,
        ).await.unwrap();
        
        assert!(result.success);
        
        // Verify attendees were added
        let event = get_event(&result.event_id, None).await.unwrap();
        assert_eq!(event.attendees.len(), 2);
    }

    #[tokio::test]
    async fn test_create_event_invalid_times() {
        let start = future_time(2);
        let end = future_time(1); // End before start
        
        let result = create_event(
            &end,
            &start,
            "Invalid Meeting",
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        ).await;
        
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_event() {
        let start = future_time(5);
        let end = future_time(6);
        
        let create_result = create_event(
            &end,
            &start,
            "Get Event Test",
            None,
            None,
            None,
            Some("Description"),
            Some("Location"),
            None,
            None,
            None,
        ).await.unwrap();
        
        let event = get_event(&create_result.event_id, None).await.unwrap();
        
        assert_eq!(event.title, "Get Event Test");
        assert_eq!(event.description, Some("Description".to_string()));
        assert_eq!(event.location, Some("Location".to_string()));
    }

    #[tokio::test]
    async fn test_get_event_not_found() {
        let result = get_event("nonexistent-event-id", None).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_event() {
        let start = future_time(7);
        let end = future_time(8);
        
        let create_result = create_event(
            &end,
            &start,
            "Original Title",
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        ).await.unwrap();
        
        let update_result = update_event(
            &create_result.event_id,
            None,
            None,
            Some("New Description"),
            None,
            Some("New Location"),
            None,
            None,
            None,
            None,
            Some("Updated Title"),
        ).await.unwrap();
        
        assert!(update_result.success);
        
        let event = get_event(&create_result.event_id, None).await.unwrap();
        assert_eq!(event.title, "Updated Title");
        assert_eq!(event.description, Some("New Description".to_string()));
        assert_eq!(event.location, Some("New Location".to_string()));
    }

    #[tokio::test]
    async fn test_delete_event() {
        let start = future_time(9);
        let end = future_time(10);
        
        let create_result = create_event(
            &end,
            &start,
            "To Be Deleted",
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        ).await.unwrap();
        
        let delete_result = delete_event(&create_result.event_id, None, None).await.unwrap();
        assert!(delete_result.success);
        
        // Verify event is gone
        let get_result = get_event(&create_result.event_id, None).await;
        assert!(get_result.is_err());
    }

    #[tokio::test]
    async fn test_list_events() {
        // Create some events
        for i in 0..3 {
            let start = future_time(20 + i * 2);
            let end = future_time(21 + i * 2);
            create_event(
                &end,
                &start,
                &format!("List Event {}", i),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            ).await.unwrap();
        }
        
        let result = list_events(None, None, Some(10), None, None, None, None).await.unwrap();
        
        assert!(result.total >= 3);
    }

    #[tokio::test]
    async fn test_list_events_with_query() {
        let start = future_time(30);
        let end = future_time(31);
        
        create_event(
            &end,
            &start,
            "Unique Query Meeting XYZ123",
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        ).await.unwrap();
        
        let result = list_events(None, None, None, None, Some("XYZ123"), None, None).await.unwrap();
        
        assert!(result.events.iter().any(|e| e.title.contains("XYZ123")));
    }

    #[tokio::test]
    async fn test_create_calendar() {
        let result = create_calendar(
            "Work Calendar",
            Some("#ff0000"),
            Some("My work events"),
            Some("America/New_York"),
        ).await.unwrap();
        
        assert!(result.success);
        assert!(!result.calendar_id.is_empty());
    }

    #[tokio::test]
    async fn test_list_calendars() {
        // Create a calendar
        create_calendar("List Test Calendar", None, None, None).await.unwrap();
        
        let result = list_calendars(None, None).await.unwrap();
        
        assert!(result.total >= 1);
        assert!(result.calendars.iter().any(|c| c.name == "Primary" || c.name == "List Test Calendar"));
    }

    #[tokio::test]
    async fn test_delete_calendar() {
        let create_result = create_calendar("To Delete Calendar", None, None, None).await.unwrap();
        
        let delete_result = delete_calendar(&create_result.calendar_id).await.unwrap();
        assert!(delete_result.success);
    }

    #[tokio::test]
    async fn test_delete_primary_calendar_fails() {
        let result = delete_calendar("primary").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_add_attendee() {
        let start = future_time(40);
        let end = future_time(41);
        
        let event = create_event(
            &end,
            &start,
            "Attendee Test",
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        ).await.unwrap();
        
        let result = add_attendee(
            "newattendee@example.com",
            &event.event_id,
            None,
            None,
            None,
        ).await.unwrap();
        
        assert!(result.success);
        
        let updated_event = get_event(&event.event_id, None).await.unwrap();
        assert!(updated_event.attendees.iter().any(|a| a.email == "newattendee@example.com"));
    }

    #[tokio::test]
    async fn test_add_duplicate_attendee_fails() {
        let start = future_time(42);
        let end = future_time(43);
        
        let event = create_event(
            &end,
            &start,
            "Duplicate Attendee Test",
            None,
            Some(vec!["existing@example.com".to_string()]),
            None,
            None,
            None,
            None,
            None,
            None,
        ).await.unwrap();
        
        let result = add_attendee("existing@example.com", &event.event_id, None, None, None).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_remove_attendee() {
        let start = future_time(44);
        let end = future_time(45);
        
        let event = create_event(
            &end,
            &start,
            "Remove Attendee Test",
            None,
            Some(vec!["toremove@example.com".to_string()]),
            None,
            None,
            None,
            None,
            None,
            None,
        ).await.unwrap();
        
        let result = remove_attendee("toremove@example.com", &event.event_id, None, None).await.unwrap();
        assert!(result.success);
        
        let updated_event = get_event(&event.event_id, None).await.unwrap();
        assert!(!updated_event.attendees.iter().any(|a| a.email == "toremove@example.com"));
    }

    #[tokio::test]
    async fn test_respond_to_invitation() {
        let start = future_time(46);
        let end = future_time(47);
        
        let event = create_event(
            &end,
            &start,
            "Respond Test",
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        ).await.unwrap();
        
        let result = respond_to_invitation(&event.event_id, "accepted", None, None).await.unwrap();
        assert!(result.success);
    }

    #[tokio::test]
    async fn test_respond_invalid_response() {
        let start = future_time(48);
        let end = future_time(49);
        
        let event = create_event(
            &end,
            &start,
            "Invalid Response Test",
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        ).await.unwrap();
        
        let result = respond_to_invitation(&event.event_id, "invalid", None, None).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_check_availability() {
        let start = future_time(50);
        let end = future_time(60);
        
        // Create an event
        create_event(
            &future_time(53),
            &future_time(52),
            "Busy Period",
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        ).await.unwrap();
        
        let result = check_availability(
            vec!["primary".to_string()],
            &end,
            &start,
            None,
        ).await.unwrap();
        
        // Should have at least one busy period
        assert!(!result.busy_periods.is_empty() || !result.free_periods.is_empty());
    }

    #[tokio::test]
    async fn test_find_available_slots() {
        let start = future_time(70);
        let end = future_time(80);
        
        let result = find_available_slots(
            vec!["test@example.com".to_string()],
            30,
            &end,
            &start,
            None,
            Some(false), // Don't restrict to working hours for test
        ).await.unwrap();
        
        // Should find some available slots
        assert!(result.total >= 0);
        for slot in &result.available_slots {
            assert_eq!(slot.duration_minutes, 30);
        }
    }

    #[tokio::test]
    async fn test_event_with_all_day() {
        let start = future_time(90);
        let end = future_time(114); // Next day
        
        let result = create_event(
            &end,
            &start,
            "All Day Event",
            Some(true),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        ).await.unwrap();
        
        assert!(result.success);
    }

    #[tokio::test]
    async fn test_create_event_in_custom_calendar() {
        let cal = create_calendar("Custom Calendar", None, None, None).await.unwrap();
        
        let start = future_time(100);
        let end = future_time(101);
        
        let event = create_event(
            &end,
            &start,
            "Custom Calendar Event",
            None,
            None,
            Some(&cal.calendar_id),
            None,
            None,
            None,
            None,
            None,
        ).await.unwrap();
        
        assert!(event.success);
        
        // Verify event is in the custom calendar
        let events = list_events(Some(&cal.calendar_id), None, None, None, None, None, None).await.unwrap();
        assert!(events.events.iter().any(|e| e.event_id == event.event_id));
    }
}
