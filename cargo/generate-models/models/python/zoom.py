# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class ZoomMeeting(BaseModel):
    """
    zoom_meeting
    
    ID fields: meeting_id
    """

    agenda: Optional[str] = None
    created_at: datetime = Field(default_factory=datetime.utcnow)
    duration: int = Field(default=60)
    host_id: Optional[str] = None
    join_url: Optional[str] = None
    meeting_id: str
    password: Optional[str] = None
    start_time: Optional[datetime] = None
    start_url: Optional[str] = None
    status: str = Field(default="waiting")
    timezone: Optional[str] = None
    topic: str
    type: str = Field(default="scheduled")
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


class ZoomMeetingRegistrant(BaseModel):
    """
    zoom_meeting_registrant
    
    ID fields: meeting_id, registrant_id
    """

    email: str
    first_name: str
    join_url: Optional[str] = None
    last_name: Optional[str] = None
    meeting_id: str = Field(description="Reference: zoom_meeting.id")
    phone: Optional[str] = None
    registered_at: datetime = Field(default_factory=datetime.utcnow)
    registrant_id: str
    status: str = Field(default="approved")
    class Config:
        from_attributes = True
        populate_by_name = True


class ZoomMeetingParticipant(BaseModel):
    """
    zoom_meeting_participant
    
    ID fields: meeting_id, user_id, join_time
    """

    duration_seconds: Optional[int] = None
    email: Optional[str] = None
    join_time: datetime = Field(default_factory=datetime.utcnow)
    leave_time: Optional[datetime] = None
    meeting_id: str = Field(description="Reference: zoom_meeting.id")
    name: Optional[str] = None
    user_id: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class ZoomWebinar(BaseModel):
    """
    zoom_webinar
    
    ID fields: webinar_id
    """

    agenda: Optional[str] = None
    created_at: datetime = Field(default_factory=datetime.utcnow)
    duration: int = Field(default=60)
    host_id: Optional[str] = None
    join_url: Optional[str] = None
    start_time: Optional[datetime] = None
    timezone: Optional[str] = None
    topic: str
    type: str = Field(default="webinar")
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    webinar_id: str
    class Config:
        from_attributes = True
        populate_by_name = True


class ZoomUser(BaseModel):
    """
    zoom_user
    
    ID fields: user_id
    """

    created_at: datetime = Field(default_factory=datetime.utcnow)
    email: str
    first_name: Optional[str] = None
    last_name: Optional[str] = None
    status: str = Field(default="active")
    timezone: Optional[str] = None
    type: int = Field(default=1)
    user_id: str
    class Config:
        from_attributes = True
        populate_by_name = True


class ZoomRecording(BaseModel):
    """
    zoom_recording
    
    ID fields: meeting_id, recording_id
    """

    download_url: Optional[str] = None
    duration_seconds: Optional[int] = None
    file_size: Optional[int] = None
    file_type: str = Field(default="mp4")
    meeting_id: str = Field(description="Reference: zoom_meeting.id")
    recording_id: str
    share_url: Optional[str] = None
    status: str = Field(default="completed")
    class Config:
        from_attributes = True
        populate_by_name = True


class ZoomRecurrence(BaseModel):
    """
    zoom_recurrence
    
    ID fields: id
    """

    type: str
    repeat_interval: int
    weekly_days: str
    monthly_day: int
    monthly_week: int
    monthly_week_day: int
    end_times: int
    end_date_time: datetime
    class Config:
        from_attributes = True
        populate_by_name = True


class ZoomMeetingSettings(BaseModel):
    """
    zoom_meeting_settings
    
    ID fields: id
    """

    host_video: bool
    participant_video: bool
    join_before_host: bool
    mute_upon_entry: bool
    watermark: bool
    use_pmi: bool
    approval_type: int
    registration_type: int
    audio: str
    auto_recording: str
    waiting_room: bool
    class Config:
        from_attributes = True
        populate_by_name = True


class ZoomWebinarSettings(BaseModel):
    """
    zoom_webinar_settings
    
    ID fields: id
    """

    host_video: bool
    panelists_video: bool
    practice_session: bool
    hd_video: bool
    approval_type: int
    registration_type: int
    audio: str
    auto_recording: str
    close_registration: bool
    show_share_button: bool
    allow_multiple_devices: bool
    class Config:
        from_attributes = True
        populate_by_name = True


class ZoomCustomQuestion(BaseModel):
    """
    zoom_custom_question
    
    ID fields: id
    """

    title: str
    value: str
    class Config:
        from_attributes = True
        populate_by_name = True


class ZoomRegistrant(BaseModel):
    """
    zoom_registrant
    
    ID fields: id
    """

    id: str
    email: str
    first_name: str
    last_name: str
    status: str
    create_time: datetime
    join_url: str
    class Config:
        from_attributes = True
        populate_by_name = True


class ZoomRegistrantId(BaseModel):
    """
    zoom_registrant_id
    
    ID fields: id
    """

    id: str
    email: str
    class Config:
        from_attributes = True
        populate_by_name = True


class ZoomParticipant(BaseModel):
    """
    zoom_participant
    
    ID fields: id
    """

    id: str
    user_id: str
    name: str
    email: str
    join_time: datetime
    leave_time: datetime
    duration: int
    attentiveness_score: str
    status: str
    class Config:
        from_attributes = True
        populate_by_name = True


class ZoomRecordingFile(BaseModel):
    """
    zoom_recording_file
    
    ID fields: id
    """

    id: str
    meeting_id: str
    recording_start: datetime
    recording_end: datetime
    file_type: str
    file_size: int
    download_url: str
    play_url: str
    status: str
    class Config:
        from_attributes = True
        populate_by_name = True


