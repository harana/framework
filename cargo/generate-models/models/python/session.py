# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class Session(BaseModel):
    """
    session
    
    ID fields: token
    """

    created_at: datetime = Field(default_factory=datetime.utcnow)
    data: Optional[str] = None
    ip_address: Optional[str] = None
    is_active: bool = Field(default=True)
    last_activity_at: datetime = Field(default_factory=datetime.utcnow)
    token: str
    user_agent: Optional[str] = None
    user_id: str = Field(description="Reference: user.id")
    class Config:
        from_attributes = True
        populate_by_name = True


class RefreshToken(BaseModel):
    """
    refresh_token
    
    ID fields: token
    """

    created_at: datetime = Field(default_factory=datetime.utcnow)
    expires_at: datetime
    is_revoked: bool = Field(default=False)
    revoked_at: Optional[datetime] = None
    session_id: str = Field(description="Reference: session.id")
    token: str
    class Config:
        from_attributes = True
        populate_by_name = True


class SessionActivity(BaseModel):
    """
    session_activity
    
    ID fields: session_id, action, created_at
    """

    method: str
    created_at: datetime = Field(default_factory=datetime.utcnow)
    ip_address: Optional[str] = None
    metadata: Optional[str] = None
    session_id: str = Field(description="Reference: session.id")
    class Config:
        from_attributes = True
        populate_by_name = True


class SessionData(BaseModel):
    """
    session_data
    
    ID fields: id
    """

    values: str
    class Config:
        from_attributes = True
        populate_by_name = True


class SessionInfo(BaseModel):
    """
    session_info
    
    ID fields: id
    """

    session_id: str
    user_id: str
    created_at: datetime
    expires_at: datetime
    class Config:
        from_attributes = True
        populate_by_name = True


