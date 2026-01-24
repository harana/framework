# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class Session(BaseModel):
    """
    Session
    
    Class: session
    ID fields: token
    """

    created_at: datetime = Field(default_factory=datetime.utcnow)
    data: Optional[str] = None
    ip_address: Optional[str] = None
    is_active: bool = Field(default=True)
    last_activity_at: datetime = Field(default_factory=datetime.utcnow)
    token: str
    user_agent: Optional[str] = None
    user_id: str
    class Config:
        from_attributes = True
        populate_by_name = True


class RefreshToken(BaseModel):
    """
    Refresh Token
    
    Class: refresh_token
    ID fields: token
    """

    created_at: datetime = Field(default_factory=datetime.utcnow)
    expires_at: datetime
    is_revoked: bool = Field(default=False)
    revoked_at: Optional[datetime] = None
    session_id: str
    token: str
    class Config:
        from_attributes = True
        populate_by_name = True


class SessionActivity(BaseModel):
    """
    Session Activity
    
    Class: session_activity
    ID fields: session_id, action, created_at
    """

    action: str
    created_at: datetime = Field(default_factory=datetime.utcnow)
    ip_address: Optional[str] = None
    metadata: Optional[str] = None
    session_id: str
    class Config:
        from_attributes = True
        populate_by_name = True


