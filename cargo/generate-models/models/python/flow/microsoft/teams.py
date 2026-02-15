# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class SendMessage(BaseModel):
    """
    send_message
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ReplyToMessage(BaseModel):
    """
    reply_to_message
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class UpdateMessage(BaseModel):
    """
    update_message
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteMessage(BaseModel):
    """
    delete_message
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SendChatMessage(BaseModel):
    """
    send_chat_message
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CreateChannel(BaseModel):
    """
    create_channel
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteChannel(BaseModel):
    """
    delete_channel
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListChannels(BaseModel):
    """
    list_channels
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetChannelMessages(BaseModel):
    """
    get_channel_messages
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


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


class CancelMeeting(BaseModel):
    """
    cancel_meeting
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListTeams(BaseModel):
    """
    list_teams
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetTeam(BaseModel):
    """
    get_team
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListTeamMembers(BaseModel):
    """
    list_team_members
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AddTeamMember(BaseModel):
    """
    add_team_member
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class RemoveTeamMember(BaseModel):
    """
    remove_team_member
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SendAdaptiveCard(BaseModel):
    """
    send_adaptive_card
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class UploadFile(BaseModel):
    """
    upload_file
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class TeamsMessage(BaseModel):
    """
    teams_message
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class TeamsUser(BaseModel):
    """
    teams_user
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class TeamsAttachment(BaseModel):
    """
    teams_attachment
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class TeamsMention(BaseModel):
    """
    teams_mention
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class TeamsChannel(BaseModel):
    """
    teams_channel
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class TeamsAttendee(BaseModel):
    """
    teams_attendee
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class TeamsTeam(BaseModel):
    """
    teams_team
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class TeamsMember(BaseModel):
    """
    teams_member
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class TeamsAdaptiveCard(BaseModel):
    """
    teams_adaptive_card
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


