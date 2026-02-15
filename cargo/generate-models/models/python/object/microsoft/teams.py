# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class TeamsTeam(BaseModel):
    """
    teams_team
    
    ID fields: team_id
    """

    created_at: datetime = Field(default_factory=datetime.utcnow)
    description: Optional[str] = None
    display_name: str
    is_archived: bool = Field(default=False)
    team_id: str
    visibility: str = Field(default="private")
    web_url: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class TeamsChannel(BaseModel):
    """
    teams_channel
    
    ID fields: team_id, channel_id
    """

    channel_id: str
    created_at: datetime = Field(default_factory=datetime.utcnow)
    description: Optional[str] = None
    display_name: str
    membership_type: str = Field(default="standard")
    team_id: str = Field(description="Reference: teams_team.id")
    web_url: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class TeamsMember(BaseModel):
    """
    teams_member
    
    ID fields: team_id, user_id
    """

    display_name: Optional[str] = None
    email: Optional[str] = None
    joined_at: datetime = Field(default_factory=datetime.utcnow)
    membership_id: Optional[str] = None
    roles: Optional[str] = None
    team_id: str = Field(description="Reference: teams_team.id")
    user_id: str
    class Config:
        from_attributes = True
        populate_by_name = True


class TeamsMessage(BaseModel):
    """
    teams_message
    
    ID fields: channel_id, message_id
    """

    channel_id: str
    content: Optional[str] = None
    content_type: str = Field(default="text")
    created_at: datetime = Field(default_factory=datetime.utcnow)
    importance: str = Field(default="normal")
    message_id: str
    sender_id: Optional[str] = None
    thread_id: Optional[str] = None
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


class TeamsMeeting(BaseModel):
    """
    teams_meeting
    
    ID fields: meeting_id
    """

    body: Optional[str] = None
    created_at: datetime = Field(default_factory=datetime.utcnow)
    end_time: Optional[datetime] = None
    is_online: bool = Field(default=True)
    join_url: Optional[str] = None
    location: Optional[str] = None
    meeting_id: str
    organizer_id: Optional[str] = None
    start_time: Optional[datetime] = None
    subject: str
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


class TeamsAdaptiveCard(BaseModel):
    """
    teams_adaptive_card
    
    ID fields: id
    """

    body: str
    created_at: datetime = Field(default_factory=datetime.utcnow)
    description: Optional[str] = None
    is_active: bool = Field(default=True)
    schema_version: str = Field(default="1.4")
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


