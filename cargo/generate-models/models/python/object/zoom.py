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


