# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class SlackWorkspace(BaseModel):
    """
    slack_workspace
    
    ID fields: id
    """

    access_token: Optional[str] = None
    created_at: datetime = Field(default_factory=datetime.utcnow)
    is_active: bool = Field(default=True)
    team_id: str
    team_name: Optional[str] = None
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


class SlackChannel(BaseModel):
    """
    slack_channel
    
    ID fields: workspace_id, channel_id
    """

    channel_id: str
    is_archived: bool = Field(default=False)
    is_private: bool = Field(default=False)
    name: str
    num_members: int = Field(default=0)
    purpose: Optional[str] = None
    topic: Optional[str] = None
    workspace_id: str = Field(description="Reference: slack_workspace.id")
    class Config:
        from_attributes = True
        populate_by_name = True


class SlackUser(BaseModel):
    """
    slack_user
    
    ID fields: workspace_id, user_id
    """

    email: Optional[str] = None
    is_admin: bool = Field(default=False)
    is_bot: bool = Field(default=False)
    name: Optional[str] = None
    real_name: Optional[str] = None
    timezone: Optional[str] = None
    user_id: str
    workspace_id: str = Field(description="Reference: slack_workspace.id")
    class Config:
        from_attributes = True
        populate_by_name = True


class SlackMessageLog(BaseModel):
    """
    slack_message_log
    
    ID fields: channel_id, message_ts
    """

    channel_id: str
    created_at: datetime = Field(default_factory=datetime.utcnow)
    message_ts: str
    status: str = Field(default="sent")
    text: Optional[str] = None
    thread_ts: Optional[str] = None
    user_id: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


