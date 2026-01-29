pub mod output;

use output::*;
use chrono::{DateTime, Duration, Utc, Timelike};
use dashmap::DashMap;
use once_cell::sync::Lazy;
use uuid::Uuid;

/// Global calendar storage
static CALENDARS: Lazy<DashMap<String, Calendar>> = Lazy::new(|| {
    let map = DashMap::new();
    // Create default "primary" calendar
    map.insert("primary".to_string(), Calendar {
        calendar_id: "primary".to_string(),
        name: "Primary".to_string(),
        description: None,
        color: Some("#4285f4".to_string()),
        timezone: "UTC".to_string(),
        primary: true,
        hidden: false,
    });
    map
});

/// Global event storage (calendar_id -> events)
static EVENTS: Lazy<DashMap<String, DashMap<String, CalendarEvent>>> = Lazy::new(|| {
    let map = DashMap::new();
    map.insert("primary".to_string(), DashMap::new());
    map
});

/// Create Calendar Event.
pub async fn create_event(
    end_time: &str,
    start_time: &str,
    title: &str,
    all_day: Option<bool>,
    attendees: Option<Vec<String>>,
    calendar_id: Option<&str>,
    description: Option<&str>,
    location: Option<&str>,
    recurrence: Option<&str>,
    reminders: Option<Vec<CalendarReminder>>,
    timezone: Option<&str>,
) -> Result<CreateEventOutput, String> {
    let calendar_id = calendar_id.unwrap_or("primary");
    let timezone = timezone.unwrap_or("UTC").to_string();
    let all_day = all_day.unwrap_or(false);
    
    // Verify calendar exists
    if !CALENDARS.contains_key(calendar_id) {
        return Err(format!("Calendar not found: {}", calendar_id));
    }
    
    let start = DateTime::parse_from_rfc3339(start_time)
        .map(|dt| dt.with_timezone(&Utc))
        .map_err(|e| format!("Invalid start_time: {}", e))?;
    
    let end = DateTime::parse_from_rfc3339(end_time)
        .map(|dt| dt.with_timezone(&Utc))
        .map_err(|e| format!("Invalid end_time: {}", e))?;
    
    if end <= start {
        return Err("end_time must be after start_time".to_string());
    }
    
    let event_id = Uuid::new_v4().to_string();
    
    let attendee_list: Vec<CalendarAttendee> = attendees
        .unwrap_or_default()
        .into_iter()
        .map(|email| CalendarAttendee {
            email,
            name: None,
            response_status: "needsAction".to_string(),
            optional: false,
        })
        .collect();
    
    let event = CalendarEvent {
        event_id: event_id.clone(),
        calendar_id: calendar_id.to_string(),
        title: title.to_string(),
        description: description.map(|s| s.to_string()),
        start_time: start,
        end_time: end,
        timezone,
        location: location.map(|s| s.to_string()),
        attendees: attendee_list,
        recurrence: recurrence.map(|s| s.to_string()),
        status: "confirmed".to_string(),
        all_day,
        html_link: Some(format!("https://calendar.example.com/event/{}", event_id)),
        reminders: reminders.unwrap_or_default(),
    };
    
    EVENTS
        .entry(calendar_id.to_string())
        .or_insert_with(DashMap::new)
        .insert(event_id.clone(), event);
    
    Ok(CreateEventOutput {
        event_id: event_id.clone(),
        html_link: Some(format!("https://calendar.example.com/event/{}", event_id)),
        success: true,
    })
}

/// Update Calendar Event.
pub async fn update_event(
    event_id: &str,
    attendees: Option<Vec<String>>,
    calendar_id: Option<&str>,
    description: Option<&str>,
    end_time: Option<&str>,
    location: Option<&str>,
    recurrence: Option<&str>,
    reminders: Option<Vec<CalendarReminder>>,
    start_time: Option<&str>,
    timezone: Option<&str>,
    title: Option<&str>,
) -> Result<UpdateEventOutput, String> {
    let calendar_id = calendar_id.unwrap_or("primary");
    
    let calendar_events = EVENTS
        .get(calendar_id)
        .ok_or_else(|| format!("Calendar not found: {}", calendar_id))?;
    
    let mut event = calendar_events
        .get_mut(event_id)
        .ok_or_else(|| format!("Event not found: {}", event_id))?;
    
    if let Some(t) = title {
        event.title = t.to_string();
    }
    if let Some(d) = description {
        event.description = Some(d.to_string());
    }
    if let Some(l) = location {
        event.location = Some(l.to_string());
    }
    if let Some(tz) = timezone {
        event.timezone = tz.to_string();
    }
    if let Some(r) = recurrence {
        event.recurrence = Some(r.to_string());
    }
    if let Some(rem) = reminders {
        event.reminders = rem;
    }
    
    if let Some(st) = start_time {
        event.start_time = DateTime::parse_from_rfc3339(st)
            .map(|dt| dt.with_timezone(&Utc))
            .map_err(|e| format!("Invalid start_time: {}", e))?;
    }
    if let Some(et) = end_time {
        event.end_time = DateTime::parse_from_rfc3339(et)
            .map(|dt| dt.with_timezone(&Utc))
            .map_err(|e| format!("Invalid end_time: {}", e))?;
    }
    
    if let Some(att) = attendees {
        event.attendees = att
            .into_iter()
            .map(|email| CalendarAttendee {
                email,
                name: None,
                response_status: "needsAction".to_string(),
                optional: false,
            })
            .collect();
    }
    
    Ok(UpdateEventOutput {
        event_id: event_id.to_string(),
        success: true,
    })
}

/// Delete Calendar Event.
pub async fn delete_event(
    event_id: &str,
    calendar_id: Option<&str>,
    _send_updates: Option<&str>,
) -> Result<DeleteEventOutput, String> {
    let calendar_id = calendar_id.unwrap_or("primary");
    
    let calendar_events = EVENTS
        .get(calendar_id)
        .ok_or_else(|| format!("Calendar not found: {}", calendar_id))?;
    
    calendar_events
        .remove(event_id)
        .ok_or_else(|| format!("Event not found: {}", event_id))?;
    
    Ok(DeleteEventOutput { success: true })
}

/// Get Calendar Event.
pub async fn get_event(
    event_id: &str,
    calendar_id: Option<&str>,
) -> Result<GetEventOutput, String> {
    let calendar_id = calendar_id.unwrap_or("primary");
    
    let calendar_events = EVENTS
        .get(calendar_id)
        .ok_or_else(|| format!("Calendar not found: {}", calendar_id))?;
    
    let event = calendar_events
        .get(event_id)
        .ok_or_else(|| format!("Event not found: {}", event_id))?;
    
    Ok(GetEventOutput {
        event_id: event.event_id.clone(),
        title: event.title.clone(),
        description: event.description.clone(),
        start_time: event.start_time,
        end_time: event.end_time,
        location: event.location.clone(),
        attendees: event.attendees.clone(),
        status: event.status.clone(),
        html_link: event.html_link.clone(),
    })
}

/// List Calendar Events.
pub async fn list_events(
    calendar_id: Option<&str>,
    end_time: Option<&str>,
    max_results: Option<i32>,
    order_by: Option<&str>,
    query: Option<&str>,
    _single_events: Option<bool>,
    start_time: Option<&str>,
) -> Result<ListEventsOutput, String> {
    let calendar_id = calendar_id.unwrap_or("primary");
    let max_results = max_results.unwrap_or(100) as usize;
    let order_by = order_by.unwrap_or("startTime");
    
    let start_filter = start_time
        .map(|s| DateTime::parse_from_rfc3339(s).map(|dt| dt.with_timezone(&Utc)))
        .transpose()
        .map_err(|e| format!("Invalid start_time: {}", e))?;
    
    let end_filter = end_time
        .map(|s| DateTime::parse_from_rfc3339(s).map(|dt| dt.with_timezone(&Utc)))
        .transpose()
        .map_err(|e| format!("Invalid end_time: {}", e))?;
    
    let calendar_events = EVENTS
        .get(calendar_id)
        .ok_or_else(|| format!("Calendar not found: {}", calendar_id))?;
    
    let mut events: Vec<CalendarEvent> = calendar_events
        .iter()
        .map(|entry| entry.value().clone())
        .filter(|event| {
            // Filter by time range
            if let Some(start) = start_filter {
                if event.end_time < start {
                    return false;
                }
            }
            if let Some(end) = end_filter {
                if event.start_time > end {
                    return false;
                }
            }
            // Filter by query
            if let Some(q) = query {
                let q_lower = q.to_lowercase();
                if !event.title.to_lowercase().contains(&q_lower)
                    && !event.description.as_ref().map_or(false, |d| d.to_lowercase().contains(&q_lower))
                {
                    return false;
                }
            }
            true
        })
        .collect();
    
    // Sort events
    match order_by {
        "updated" => events.sort_by(|a, b| b.start_time.cmp(&a.start_time)),
        _ => events.sort_by(|a, b| a.start_time.cmp(&b.start_time)), // startTime
    }
    
    let total = events.len() as i32;
    events.truncate(max_results);
    
    Ok(ListEventsOutput {
        events,
        next_page_token: None,
        total,
    })
}

/// Create Calendar.
pub async fn create_calendar(
    name: &str,
    color: Option<&str>,
    description: Option<&str>,
    timezone: Option<&str>,
) -> Result<CreateCalendarOutput, String> {
    let calendar_id = Uuid::new_v4().to_string();
    let timezone = timezone.unwrap_or("UTC").to_string();
    
    let calendar = Calendar {
        calendar_id: calendar_id.clone(),
        name: name.to_string(),
        description: description.map(|s| s.to_string()),
        color: color.map(|s| s.to_string()),
        timezone,
        primary: false,
        hidden: false,
    };
    
    CALENDARS.insert(calendar_id.clone(), calendar);
    EVENTS.insert(calendar_id.clone(), DashMap::new());
    
    Ok(CreateCalendarOutput {
        calendar_id,
        success: true,
    })
}

/// Delete Calendar.
pub async fn delete_calendar(
    calendar_id: &str,
) -> Result<DeleteCalendarOutput, String> {
    if calendar_id == "primary" {
        return Err("Cannot delete primary calendar".to_string());
    }
    
    CALENDARS
        .remove(calendar_id)
        .ok_or_else(|| format!("Calendar not found: {}", calendar_id))?;
    
    EVENTS.remove(calendar_id);
    
    Ok(DeleteCalendarOutput { success: true })
}

/// List Calendars.
pub async fn list_calendars(
    max_results: Option<i32>,
    show_hidden: Option<bool>,
) -> Result<ListCalendarsOutput, String> {
    let max_results = max_results.unwrap_or(100) as usize;
    let show_hidden = show_hidden.unwrap_or(false);
    
    let calendars: Vec<Calendar> = CALENDARS
        .iter()
        .filter(|entry| show_hidden || !entry.value().hidden)
        .map(|entry| entry.value().clone())
        .take(max_results)
        .collect();
    
    let total = calendars.len() as i32;
    
    Ok(ListCalendarsOutput { calendars, total })
}

/// Check Availability.
pub async fn check_availability(
    calendar_ids: Vec<String>,
    end_time: &str,
    start_time: &str,
    _timezone: Option<&str>,
) -> Result<CheckAvailabilityOutput, String> {
    let start = DateTime::parse_from_rfc3339(start_time)
        .map(|dt| dt.with_timezone(&Utc))
        .map_err(|e| format!("Invalid start_time: {}", e))?;
    
    let end = DateTime::parse_from_rfc3339(end_time)
        .map(|dt| dt.with_timezone(&Utc))
        .map_err(|e| format!("Invalid end_time: {}", e))?;
    
    let mut busy_periods: Vec<CalendarTimePeriod> = Vec::new();
    
    // Collect all busy periods from specified calendars
    for cal_id in &calendar_ids {
        if let Some(calendar_events) = EVENTS.get(cal_id) {
            for entry in calendar_events.iter() {
                let event = entry.value();
                // Check if event overlaps with the query range
                if event.start_time < end && event.end_time > start {
                    busy_periods.push(CalendarTimePeriod {
                        start_time: event.start_time.max(start),
                        end_time: event.end_time.min(end),
                        calendar_id: cal_id.clone(),
                    });
                }
            }
        }
    }
    
    // Sort busy periods by start time
    busy_periods.sort_by(|a, b| a.start_time.cmp(&b.start_time));
    
    // Calculate free periods
    let mut free_periods: Vec<CalendarTimePeriod> = Vec::new();
    let mut current_time = start;
    
    for busy in &busy_periods {
        if busy.start_time > current_time {
            free_periods.push(CalendarTimePeriod {
                start_time: current_time,
                end_time: busy.start_time,
                calendar_id: "free".to_string(),
            });
        }
        current_time = current_time.max(busy.end_time);
    }
    
    if current_time < end {
        free_periods.push(CalendarTimePeriod {
            start_time: current_time,
            end_time: end,
            calendar_id: "free".to_string(),
        });
    }
    
    Ok(CheckAvailabilityOutput {
        busy_periods,
        free_periods,
    })
}

/// Add Event Attendee.
pub async fn add_attendee(
    email: &str,
    event_id: &str,
    calendar_id: Option<&str>,
    optional: Option<bool>,
    _send_updates: Option<&str>,
) -> Result<AddAttendeeOutput, String> {
    let calendar_id = calendar_id.unwrap_or("primary");
    let optional = optional.unwrap_or(false);
    
    let calendar_events = EVENTS
        .get(calendar_id)
        .ok_or_else(|| format!("Calendar not found: {}", calendar_id))?;
    
    let mut event = calendar_events
        .get_mut(event_id)
        .ok_or_else(|| format!("Event not found: {}", event_id))?;
    
    // Check if attendee already exists
    if event.attendees.iter().any(|a| a.email == email) {
        return Err(format!("Attendee already exists: {}", email));
    }
    
    event.attendees.push(CalendarAttendee {
        email: email.to_string(),
        name: None,
        response_status: "needsAction".to_string(),
        optional,
    });
    
    Ok(AddAttendeeOutput { success: true })
}

/// Remove Event Attendee.
pub async fn remove_attendee(
    email: &str,
    event_id: &str,
    calendar_id: Option<&str>,
    _send_updates: Option<&str>,
) -> Result<RemoveAttendeeOutput, String> {
    let calendar_id = calendar_id.unwrap_or("primary");
    
    let calendar_events = EVENTS
        .get(calendar_id)
        .ok_or_else(|| format!("Calendar not found: {}", calendar_id))?;
    
    let mut event = calendar_events
        .get_mut(event_id)
        .ok_or_else(|| format!("Event not found: {}", event_id))?;
    
    let original_len = event.attendees.len();
    event.attendees.retain(|a| a.email != email);
    
    if event.attendees.len() == original_len {
        return Err(format!("Attendee not found: {}", email));
    }
    
    Ok(RemoveAttendeeOutput { success: true })
}

/// Respond To Event Invitation.
pub async fn respond_to_invitation(
    event_id: &str,
    response: &str,
    calendar_id: Option<&str>,
    _comment: Option<&str>,
) -> Result<RespondToInvitationOutput, String> {
    let calendar_id = calendar_id.unwrap_or("primary");
    
    // Validate response
    let valid_responses = ["accepted", "declined", "tentative"];
    let response_lower = response.to_lowercase();
    if !valid_responses.contains(&response_lower.as_str()) {
        return Err(format!("Invalid response: {}. Must be one of: {:?}", response, valid_responses));
    }
    
    let calendar_events = EVENTS
        .get(calendar_id)
        .ok_or_else(|| format!("Calendar not found: {}", calendar_id))?;
    
    let mut event = calendar_events
        .get_mut(event_id)
        .ok_or_else(|| format!("Event not found: {}", event_id))?;
    
    // Update status based on response
    event.status = response_lower;
    
    Ok(RespondToInvitationOutput { success: true })
}

/// Find Available Slots.
pub async fn find_available_slots(
    attendees: Vec<String>,
    duration_minutes: i32,
    end_time: &str,
    start_time: &str,
    _timezone: Option<&str>,
    working_hours_only: Option<bool>,
) -> Result<FindAvailableSlotsOutput, String> {
    let working_hours_only = working_hours_only.unwrap_or(true);
    
    let start = DateTime::parse_from_rfc3339(start_time)
        .map(|dt| dt.with_timezone(&Utc))
        .map_err(|e| format!("Invalid start_time: {}", e))?;
    
    let end = DateTime::parse_from_rfc3339(end_time)
        .map(|dt| dt.with_timezone(&Utc))
        .map_err(|e| format!("Invalid end_time: {}", e))?;
    
    let duration = Duration::minutes(duration_minutes as i64);
    
    // Collect all busy periods from attendees' calendars
    let mut all_busy: Vec<(DateTime<Utc>, DateTime<Utc>)> = Vec::new();
    
    for attendee in &attendees {
        // Look for calendar by attendee email (simplified - in reality would need mapping)
        if let Some(calendar_events) = EVENTS.get("primary") {
            for entry in calendar_events.iter() {
                let event = entry.value();
                if event.attendees.iter().any(|a| &a.email == attendee) {
                    if event.start_time < end && event.end_time > start {
                        all_busy.push((event.start_time.max(start), event.end_time.min(end)));
                    }
                }
            }
        }
    }
    
    // Sort and merge overlapping busy periods
    all_busy.sort_by(|a, b| a.0.cmp(&b.0));
    let merged_busy = merge_intervals(all_busy);
    
    // Find available slots
    let mut available_slots: Vec<CalendarTimeSlot> = Vec::new();
    let mut current = start;
    
    for (busy_start, busy_end) in merged_busy {
        // Check for slot before this busy period
        while current + duration <= busy_start {
            if !working_hours_only || is_working_hours(current, duration_minutes) {
                available_slots.push(CalendarTimeSlot {
                    start_time: current,
                    end_time: current + duration,
                    duration_minutes,
                });
            }
            current = current + Duration::minutes(30); // 30-minute increments
        }
        current = current.max(busy_end);
    }
    
    // Check for slots after last busy period
    while current + duration <= end {
        if !working_hours_only || is_working_hours(current, duration_minutes) {
            available_slots.push(CalendarTimeSlot {
                start_time: current,
                end_time: current + duration,
                duration_minutes,
            });
        }
        current = current + Duration::minutes(30);
    }
    
    let total = available_slots.len() as i32;
    
    Ok(FindAvailableSlotsOutput {
        available_slots,
        total,
    })
}

/// Merge overlapping intervals
fn merge_intervals(mut intervals: Vec<(DateTime<Utc>, DateTime<Utc>)>) -> Vec<(DateTime<Utc>, DateTime<Utc>)> {
    if intervals.is_empty() {
        return intervals;
    }
    
    intervals.sort_by(|a, b| a.0.cmp(&b.0));
    let mut merged = vec![intervals[0]];
    
    for (start, end) in intervals.into_iter().skip(1) {
        let last = merged.last_mut().unwrap();
        if start <= last.1 {
            last.1 = last.1.max(end);
        } else {
            merged.push((start, end));
        }
    }
    
    merged
}

/// Check if a time slot falls within working hours (9 AM - 5 PM)
fn is_working_hours(time: DateTime<Utc>, duration_minutes: i32) -> bool {
    let start_hour = time.hour();
    let end_time = time + Duration::minutes(duration_minutes as i64);
    let end_hour = end_time.hour();
    
    // Working hours: 9 AM to 5 PM
    start_hour >= 9 && end_hour <= 17 && (end_hour > start_hour || end_time.minute() == 0)
}

#[cfg(test)]
mod tests;
