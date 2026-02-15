# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class CreateMeeting(BaseModel):
    """
    create_meeting
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetMeeting(BaseModel):
    """
    get_meeting
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class UpdateMeeting(BaseModel):
    """
    update_meeting
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteMeeting(BaseModel):
    """
    delete_meeting
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListMeetings(BaseModel):
    """
    list_meetings
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AddRegistrant(BaseModel):
    """
    add_registrant
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListRegistrants(BaseModel):
    """
    list_registrants
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class UpdateRegistrantStatus(BaseModel):
    """
    update_registrant_status
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetParticipants(BaseModel):
    """
    get_participants
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CreateWebinar(BaseModel):
    """
    create_webinar
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetWebinar(BaseModel):
    """
    get_webinar
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteWebinar(BaseModel):
    """
    delete_webinar
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetUser(BaseModel):
    """
    get_user
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListUsers(BaseModel):
    """
    list_users
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetRecording(BaseModel):
    """
    get_recording
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteRecording(BaseModel):
    """
    delete_recording
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListRecordings(BaseModel):
    """
    list_recordings
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class EndMeeting(BaseModel):
    """
    end_meeting
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SendInvitation(BaseModel):
    """
    send_invitation
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ZoomMeeting(BaseModel):
    """
    zoom_meeting
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ZoomRecurrence(BaseModel):
    """
    zoom_recurrence
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ZoomMeetingSettings(BaseModel):
    """
    zoom_meeting_settings
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ZoomWebinarSettings(BaseModel):
    """
    zoom_webinar_settings
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ZoomCustomQuestion(BaseModel):
    """
    zoom_custom_question
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ZoomRegistrant(BaseModel):
    """
    zoom_registrant
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ZoomRegistrantId(BaseModel):
    """
    zoom_registrant_id
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ZoomParticipant(BaseModel):
    """
    zoom_participant
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ZoomUser(BaseModel):
    """
    zoom_user
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ZoomRecordingFile(BaseModel):
    """
    zoom_recording_file
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ZoomRecording(BaseModel):
    """
    zoom_recording
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


