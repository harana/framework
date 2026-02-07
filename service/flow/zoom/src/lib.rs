pub mod output;

#[cfg(test)]
mod tests;

use dashmap::DashMap;
use once_cell::sync::Lazy;
use chrono::{Utc, Duration};
use uuid::Uuid;

use output::*;

// In-memory storage
static MEETINGS: Lazy<DashMap<String, StoredMeeting>> = Lazy::new(DashMap::new);
static WEBINARS: Lazy<DashMap<String, StoredWebinar>> = Lazy::new(DashMap::new);
static REGISTRANTS: Lazy<DashMap<String, StoredRegistrant>> = Lazy::new(DashMap::new);
static USERS: Lazy<DashMap<String, StoredUser>> = Lazy::new(DashMap::new);
static RECORDINGS: Lazy<DashMap<String, StoredRecording>> = Lazy::new(DashMap::new);
static PARTICIPANTS: Lazy<DashMap<String, Vec<StoredParticipant>>> = Lazy::new(DashMap::new);

fn generate_meeting_id() -> String {
    // Zoom meeting IDs are typically 10-11 digit numbers
    let id: u64 = rand_u64() % 90000000000 + 10000000000;
    id.to_string()
}

fn rand_u64() -> u64 {
    use std::hash::{Hash, Hasher};
    use std::collections::hash_map::DefaultHasher;
    let mut hasher = DefaultHasher::new();
    Uuid::new_v4().hash(&mut hasher);
    hasher.finish()
}

fn generate_id(prefix: &str) -> String {
    format!("{}_{}", prefix, Uuid::new_v4().to_string().replace("-", "")[..16].to_string())
}

fn generate_join_url(meeting_id: &str) -> String {
    format!("https://zoom.us/j/{}", meeting_id)
}

fn generate_start_url(meeting_id: &str) -> String {
    format!("https://zoom.us/s/{}?zak={}", meeting_id, Uuid::new_v4())
}

fn generate_password() -> String {
    Uuid::new_v4().to_string().replace("-", "")[..6].to_string()
}

/// Create a Zoom meeting
pub async fn create_meeting(
    topic: String,
    meeting_type: Option<MeetingType>,
    start_time: Option<chrono::DateTime<Utc>>,
    duration: Option<i32>,
    timezone: Option<String>,
    agenda: Option<String>,
    password: Option<String>,
    settings: Option<ZoomMeetingSettings>,
    recurrence: Option<ZoomRecurrence>,
) -> CreateMeetingOutput {
    let meeting_id = generate_meeting_id();
    let meeting_type = meeting_type.unwrap_or(MeetingType::Scheduled);
    let password = password.or_else(|| Some(generate_password()));
    let join_url = generate_join_url(&meeting_id);
    let start_url = generate_start_url(&meeting_id);
    let now = Utc::now();

    let stored = StoredMeeting {
        meeting_id: meeting_id.clone(),
        topic,
        meeting_type,
        start_time,
        duration: duration.unwrap_or(60),
        timezone: timezone.unwrap_or_else(|| "UTC".to_string()),
        agenda,
        password: password.clone(),
        join_url: join_url.clone(),
        start_url: start_url.clone(),
        host_id: "me".to_string(),
        status: "waiting".to_string(),
        settings: settings.unwrap_or_default(),
        recurrence,
        created_at: now,
    };

    MEETINGS.insert(meeting_id.clone(), stored);

    CreateMeetingOutput {
        meeting_id,
        join_url,
        start_url,
        password,
        success: true,
    }
}

/// Get a Zoom meeting
pub async fn get_meeting(meeting_id: String) -> GetMeetingOutput {
    if let Some(entry) = MEETINGS.get(&meeting_id) {
        GetMeetingOutput {
            meeting_id: entry.meeting_id.clone(),
            topic: entry.topic.clone(),
            r#type: entry.meeting_type.as_i32(),
            start_time: entry.start_time,
            duration: entry.duration,
            timezone: entry.timezone.clone(),
            agenda: entry.agenda.clone(),
            password: entry.password.clone(),
            join_url: entry.join_url.clone(),
            host_id: entry.host_id.clone(),
            status: entry.status.clone(),
        }
    } else {
        GetMeetingOutput {
            meeting_id,
            topic: String::new(),
            r#type: 0,
            start_time: None,
            duration: 0,
            timezone: String::new(),
            agenda: None,
            password: None,
            join_url: String::new(),
            host_id: String::new(),
            status: "not_found".to_string(),
        }
    }
}

/// Update a Zoom meeting
pub async fn update_meeting(
    meeting_id: String,
    topic: Option<String>,
    meeting_type: Option<MeetingType>,
    start_time: Option<chrono::DateTime<Utc>>,
    duration: Option<i32>,
    timezone: Option<String>,
    agenda: Option<String>,
    password: Option<String>,
    settings: Option<ZoomMeetingSettings>,
) -> UpdateMeetingOutput {
    let mut success = false;

    if let Some(mut entry) = MEETINGS.get_mut(&meeting_id) {
        if let Some(t) = topic {
            entry.topic = t;
        }
        if let Some(mt) = meeting_type {
            entry.meeting_type = mt;
        }
        if let Some(st) = start_time {
            entry.start_time = Some(st);
        }
        if let Some(d) = duration {
            entry.duration = d;
        }
        if let Some(tz) = timezone {
            entry.timezone = tz;
        }
        if let Some(a) = agenda {
            entry.agenda = Some(a);
        }
        if let Some(p) = password {
            entry.password = Some(p);
        }
        if let Some(s) = settings {
            entry.settings = s;
        }
        success = true;
    }

    UpdateMeetingOutput { success }
}

/// Delete a Zoom meeting
pub async fn delete_meeting(
    meeting_id: String,
    _occurrence_id: Option<String>,
    _schedule_for_reminder: Option<bool>,
) -> DeleteMeetingOutput {
    let success = MEETINGS.remove(&meeting_id).is_some();
    DeleteMeetingOutput { success }
}

/// List Zoom meetings
pub async fn list_meetings(
    user_id: Option<String>,
    _list_type: Option<ListMeetingsType>,
    page_size: Option<i32>,
    _next_page_token: Option<String>,
) -> ListMeetingsOutput {
    let _user_id = user_id.unwrap_or_else(|| "me".to_string());
    let page_size = page_size.unwrap_or(30) as usize;

    let meetings: Vec<ZoomMeeting> = MEETINGS
        .iter()
        .take(page_size)
        .map(|entry| {
            let m = entry.value();
            ZoomMeeting {
                meeting_id: m.meeting_id.clone(),
                topic: m.topic.clone(),
                r#type: m.meeting_type.as_i32(),
                start_time: m.start_time,
                duration: m.duration,
                timezone: m.timezone.clone(),
                password: m.password.clone(),
                join_url: m.join_url.clone(),
                start_url: m.start_url.clone(),
                host_id: m.host_id.clone(),
                status: m.status.clone(),
            }
        })
        .collect();

    let total = MEETINGS.len() as i32;

    ListMeetingsOutput {
        meetings,
        total_records: total,
        next_page_token: None,
    }
}

/// Add a meeting registrant
pub async fn add_registrant(
    meeting_id: String,
    email: String,
    first_name: String,
    last_name: Option<String>,
    phone: Option<String>,
    custom_questions: Option<Vec<ZoomCustomQuestion>>,
) -> AddRegistrantOutput {
    let registrant_id = generate_id("reg");
    let join_url = format!("{}?tk={}", generate_join_url(&meeting_id), Uuid::new_v4());
    let now = Utc::now();

    let stored = StoredRegistrant {
        id: registrant_id.clone(),
        meeting_id,
        email,
        first_name,
        last_name,
        phone,
        custom_questions: custom_questions.unwrap_or_default(),
        status: RegistrantStatus::Approved,
        join_url: join_url.clone(),
        created_at: now,
    };

    REGISTRANTS.insert(registrant_id.clone(), stored);

    AddRegistrantOutput {
        registrant_id,
        join_url,
        success: true,
    }
}

/// List meeting registrants
pub async fn list_registrants(
    meeting_id: String,
    status: Option<RegistrantStatus>,
    page_size: Option<i32>,
    _next_page_token: Option<String>,
) -> ListRegistrantsOutput {
    let page_size = page_size.unwrap_or(30) as usize;
    let status = status.unwrap_or(RegistrantStatus::Approved);

    let registrants: Vec<ZoomRegistrant> = REGISTRANTS
        .iter()
        .filter(|entry| entry.meeting_id == meeting_id && entry.status == status)
        .take(page_size)
        .map(|entry| {
            let r = entry.value();
            ZoomRegistrant {
                id: r.id.clone(),
                email: r.email.clone(),
                first_name: r.first_name.clone(),
                last_name: r.last_name.clone(),
                status: format!("{:?}", r.status).to_lowercase(),
                create_time: r.created_at,
                join_url: r.join_url.clone(),
            }
        })
        .collect();

    let total = registrants.len() as i32;

    ListRegistrantsOutput {
        registrants,
        total_records: total,
        next_page_token: None,
    }
}

/// Update registrant status
pub async fn update_registrant_status(
    meeting_id: String,
    action: RegistrantAction,
    registrants: Vec<ZoomRegistrantId>,
) -> UpdateRegistrantStatusOutput {
    let new_status = match action {
        RegistrantAction::Approve => RegistrantStatus::Approved,
        RegistrantAction::Deny => RegistrantStatus::Denied,
        RegistrantAction::Cancel => RegistrantStatus::Denied,
    };

    for reg_id in registrants {
        if let Some(mut entry) = REGISTRANTS.get_mut(&reg_id.id) {
            if entry.meeting_id == meeting_id {
                entry.status = new_status;
            }
        }
    }

    UpdateRegistrantStatusOutput { success: true }
}

/// Get meeting participants
pub async fn get_participants(
    meeting_id: String,
    _participant_type: Option<ParticipantType>,
    page_size: Option<i32>,
    _next_page_token: Option<String>,
) -> GetParticipantsOutput {
    let page_size = page_size.unwrap_or(30) as usize;

    let participants: Vec<ZoomParticipant> = PARTICIPANTS
        .get(&meeting_id)
        .map(|entry| {
            entry
                .iter()
                .take(page_size)
                .map(|p| ZoomParticipant {
                    id: p.id.clone(),
                    user_id: p.user_id.clone(),
                    name: p.name.clone(),
                    email: p.email.clone(),
                    join_time: p.join_time,
                    leave_time: p.leave_time,
                    duration: p.duration,
                    attentiveness_score: None,
                    status: p.status.clone(),
                })
                .collect()
        })
        .unwrap_or_default();

    let total = participants.len() as i32;

    GetParticipantsOutput {
        participants,
        total_records: total,
        next_page_token: None,
    }
}

/// Create a Zoom webinar
pub async fn create_webinar(
    topic: String,
    webinar_type: Option<WebinarType>,
    start_time: Option<chrono::DateTime<Utc>>,
    duration: Option<i32>,
    timezone: Option<String>,
    agenda: Option<String>,
    password: Option<String>,
    settings: Option<ZoomWebinarSettings>,
) -> CreateWebinarOutput {
    let webinar_id = generate_meeting_id();
    let webinar_type = webinar_type.unwrap_or(WebinarType::Webinar);
    let password = password.or_else(|| Some(generate_password()));
    let join_url = format!("https://zoom.us/w/{}", webinar_id);
    let start_url = format!("https://zoom.us/w/{}?zak={}", webinar_id, Uuid::new_v4());
    let now = Utc::now();

    let stored = StoredWebinar {
        webinar_id: webinar_id.clone(),
        topic,
        webinar_type,
        start_time,
        duration: duration.unwrap_or(60),
        timezone: timezone.unwrap_or_else(|| "UTC".to_string()),
        agenda,
        password: password.clone(),
        join_url: join_url.clone(),
        start_url: start_url.clone(),
        host_id: "me".to_string(),
        settings: settings.unwrap_or_default(),
        created_at: now,
    };

    WEBINARS.insert(webinar_id.clone(), stored);

    CreateWebinarOutput {
        webinar_id,
        join_url,
        start_url,
        password,
        success: true,
    }
}

/// Get a Zoom webinar
pub async fn get_webinar(webinar_id: String) -> GetWebinarOutput {
    if let Some(entry) = WEBINARS.get(&webinar_id) {
        GetWebinarOutput {
            webinar_id: entry.webinar_id.clone(),
            topic: entry.topic.clone(),
            r#type: entry.webinar_type.as_i32(),
            start_time: entry.start_time,
            duration: entry.duration,
            timezone: entry.timezone.clone(),
            agenda: entry.agenda.clone(),
            join_url: entry.join_url.clone(),
            host_id: entry.host_id.clone(),
        }
    } else {
        GetWebinarOutput {
            webinar_id,
            topic: String::new(),
            r#type: 0,
            start_time: None,
            duration: 0,
            timezone: String::new(),
            agenda: None,
            join_url: String::new(),
            host_id: String::new(),
        }
    }
}

/// Delete a Zoom webinar
pub async fn delete_webinar(
    webinar_id: String,
    _occurrence_id: Option<String>,
) -> DeleteWebinarOutput {
    let success = WEBINARS.remove(&webinar_id).is_some();
    DeleteWebinarOutput { success }
}

/// Get a Zoom user
pub async fn get_user(user_id: Option<String>) -> GetUserOutput {
    let user_id = user_id.unwrap_or_else(|| "me".to_string());

    if let Some(entry) = USERS.get(&user_id) {
        GetUserOutput {
            user_id: entry.id.clone(),
            email: entry.email.clone(),
            first_name: entry.first_name.clone(),
            last_name: entry.last_name.clone(),
            r#type: entry.user_type,
            status: format!("{:?}", entry.status).to_lowercase(),
            created_at: entry.created_at,
            timezone: entry.timezone.clone(),
        }
    } else {
        // Return default/mock user for "me"
        let now = Utc::now();
        GetUserOutput {
            user_id,
            email: "user@example.com".to_string(),
            first_name: "Default".to_string(),
            last_name: "User".to_string(),
            r#type: 1,
            status: "active".to_string(),
            created_at: now,
            timezone: "UTC".to_string(),
        }
    }
}

/// List Zoom users
pub async fn list_users(
    _status: Option<UserStatus>,
    _role_id: Option<String>,
    page_size: Option<i32>,
    _next_page_token: Option<String>,
) -> ListUsersOutput {
    let page_size = page_size.unwrap_or(30) as usize;

    let users: Vec<ZoomUser> = USERS
        .iter()
        .take(page_size)
        .map(|entry| {
            let u = entry.value();
            ZoomUser {
                id: u.id.clone(),
                email: u.email.clone(),
                first_name: u.first_name.clone(),
                last_name: u.last_name.clone(),
                r#type: u.user_type,
                status: format!("{:?}", u.status).to_lowercase(),
                created_at: u.created_at,
                timezone: u.timezone.clone(),
            }
        })
        .collect();

    let total = users.len() as i32;

    ListUsersOutput {
        users,
        total_records: total,
        next_page_token: None,
    }
}

/// Get meeting recording
pub async fn get_recording(meeting_id: String) -> GetRecordingOutput {
    if let Some(entry) = RECORDINGS.get(&meeting_id) {
        if !entry.deleted {
            let files: Vec<ZoomRecordingFile> = entry
                .files
                .iter()
                .map(|f| ZoomRecordingFile {
                    id: f.id.clone(),
                    meeting_id: meeting_id.clone(),
                    recording_start: f.recording_start,
                    recording_end: f.recording_end,
                    file_type: f.file_type.clone(),
                    file_size: f.file_size,
                    download_url: f.download_url.clone(),
                    play_url: f.play_url.clone(),
                    status: f.status.clone(),
                })
                .collect();

            return GetRecordingOutput {
                download_url: files.first().map(|f| f.download_url.clone()),
                share_url: Some(format!("https://zoom.us/rec/share/{}", entry.id)),
                duration: entry.duration,
                recording_files: files,
            };
        }
    }

    GetRecordingOutput {
        download_url: None,
        share_url: None,
        duration: 0,
        recording_files: Vec::new(),
    }
}

/// Delete meeting recording
pub async fn delete_recording(
    meeting_id: String,
    _action: Option<RecordingDeleteAction>,
) -> DeleteRecordingOutput {
    let mut success = false;

    if let Some(mut entry) = RECORDINGS.get_mut(&meeting_id) {
        entry.deleted = true;
        success = true;
    }

    DeleteRecordingOutput { success }
}

/// List cloud recordings
pub async fn list_recordings(
    user_id: Option<String>,
    _from_date: Option<chrono::DateTime<Utc>>,
    _to_date: Option<chrono::DateTime<Utc>>,
    page_size: Option<i32>,
    _next_page_token: Option<String>,
) -> ListRecordingsOutput {
    let _user_id = user_id.unwrap_or_else(|| "me".to_string());
    let page_size = page_size.unwrap_or(30) as usize;

    let recordings: Vec<ZoomRecording> = RECORDINGS
        .iter()
        .filter(|entry| !entry.deleted)
        .take(page_size)
        .map(|entry| {
            let r = entry.value();
            ZoomRecording {
                id: r.id.clone(),
                meeting_id: r.meeting_id.clone(),
                topic: r.topic.clone(),
                r#type: r.recording_type,
                start_time: r.start_time,
                duration: r.duration,
                total_size: r.total_size,
                recording_count: r.files.len() as i32,
                recording_files: r
                    .files
                    .iter()
                    .map(|f| ZoomRecordingFile {
                        id: f.id.clone(),
                        meeting_id: r.meeting_id.clone(),
                        recording_start: f.recording_start,
                        recording_end: f.recording_end,
                        file_type: f.file_type.clone(),
                        file_size: f.file_size,
                        download_url: f.download_url.clone(),
                        play_url: f.play_url.clone(),
                        status: f.status.clone(),
                    })
                    .collect(),
            }
        })
        .collect();

    let total = recordings.len() as i32;

    ListRecordingsOutput {
        recordings,
        total_records: total,
        next_page_token: None,
    }
}

/// End a meeting
pub async fn end_meeting(meeting_id: String) -> EndMeetingOutput {
    let mut success = false;

    if let Some(mut entry) = MEETINGS.get_mut(&meeting_id) {
        entry.status = "ended".to_string();
        success = true;
    }

    EndMeetingOutput { success }
}

/// Send meeting invitation
pub async fn send_invitation(
    meeting_id: String,
    emails: Vec<String>,
) -> SendInvitationOutput {
    // In a real implementation, this would send emails
    // For mock, we just verify the meeting exists
    let success = MEETINGS.contains_key(&meeting_id) && !emails.is_empty();
    SendInvitationOutput { success }
}

// Utility functions for testing
#[cfg(test)]
pub fn clear_all_data() {
    MEETINGS.clear();
    WEBINARS.clear();
    REGISTRANTS.clear();
    USERS.clear();
    RECORDINGS.clear();
    PARTICIPANTS.clear();
}
